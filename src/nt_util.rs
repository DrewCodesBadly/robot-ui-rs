use std::{collections::HashMap, ffi::c_void};

use ntcore_sys::{
    NT_AddListener, NT_Event, NT_EventFlags_NT_EVENT_VALUE_ALL, NT_GetEntry, NT_Inst,
    NT_Type_NT_BOOLEAN, NT_Type_NT_DOUBLE, NT_Type_NT_DOUBLE_ARRAY, NT_Type_NT_STRING, WPI_String,
};

pub type ListenedValues = HashMap<String, NTValueType>;

pub fn to_wpi_string(s: &str) -> WPI_String {
    WPI_String {
        str_: s.as_ptr().cast(),
        len: s.len(),
    }
}

pub fn from_wpi_string(s: WPI_String) -> String {
    let original_string = unsafe { String::from_raw_parts(s.str_.cast_mut().cast(), s.len, s.len) };

    // Clone, since original data might disappear.
    return original_string.clone();
}

pub enum NTValueType {
    Unknown,
    Boolean(bool),
    BooleanArray(Vec<bool>),
    Double(f64),
    DoubleArray(Vec<f64>),
    Float(f32),
    FloatArray(Vec<f32>),
    Integer(i32),
    IntegerArray(Vec<i32>),
    String(String),
    StringArray(Vec<String>),
}

/// IMPORTANT: Assumes this HashMap will live for the duration of the app and the listener will always be needed.
/// Consider a different implementation if the HashMap reference may eventually be invalid.
pub fn add_listener(map: &mut HashMap<String, NTValueType>, entry_name: &str, inst: NT_Inst) {
    let entry = unsafe { NT_GetEntry(inst, &to_wpi_string(entry_name)) };
    unsafe {
        NT_AddListener(
            entry,
            NT_EventFlags_NT_EVENT_VALUE_ALL,
            (&raw mut *map).cast(),
            Some(nt_update),
        )
    };
}

// Does this segfault? Not clear on whether or not the topicInfo field is always there.
pub extern "C" fn nt_update(data: *mut c_void, event: *const NT_Event) {
    let mut map = unsafe { data.cast::<HashMap<String, NTValueType>>().read() };
    let event = unsafe { event.read() };

    let value = unsafe {
        match event.data.valueData.value.type_ {
            NT_Type_NT_BOOLEAN => {
                NTValueType::Boolean(event.data.valueData.value.data.v_boolean == 1)
            }
            NT_Type_NT_DOUBLE => NTValueType::Double(event.data.valueData.value.data.v_double),
            NT_Type_NT_DOUBLE_ARRAY => NTValueType::DoubleArray(
                Vec::from_raw_parts(
                    event.data.valueData.value.data.arr_double.arr,
                    event.data.valueData.value.data.arr_double.size,
                    event.data.valueData.value.data.arr_double.size,
                )
                .clone(),
            ),
            NT_Type_NT_STRING => {
                NTValueType::String(from_wpi_string(event.data.valueData.value.data.v_string))
            }

            // TODO: Implement.
            _ => NTValueType::Unknown,
        }
    };

    let name = from_wpi_string(unsafe { event.data.topicInfo.name });

    map.insert(name, value);
}

pub fn format_game_time(time: Option<f64>) -> String {
    if let Some(f) = time {
        let time_s = f.ceil() as i32;
        format!("{}:{}", time_s / 60, time_s % 60)
    } else {
        String::from("--:--")
    }
}
