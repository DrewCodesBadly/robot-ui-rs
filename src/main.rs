use std::collections::HashMap;

use egui::{Context, DragValue, Id, Modal};
use ntcore_sys::{
    NT_CreateInstance, NT_GetDouble, NT_GetDoubleArray, NT_GetString, NT_Inst, NT_SetServerTeam,
    NT_StartClient4,
};
use opencv::videoio::{CAP_ANY, VideoCapture};

use crate::{
    nt_paths::LUNITE_COUNT,
    nt_util::{ListenedValues, NTValueType, from_wpi_string, get_entry_handle, to_wpi_string},
};

mod components;
mod nt_paths;
mod nt_util;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "FRC UI",
        native_options,
        Box::new(|cc| Ok(Box::new(FrcUi::new(cc)))),
    );
}

struct FrcUi {
    team_number: u32,
    port: u32,
    nt: NT_Inst,
    camera_ips: HashMap<String, String>,
    camera_streams: HashMap<String, VideoCapture>,
    settings_modal_open: bool,
    listened_values: ListenedValues,

    tmp: u32,
}

impl FrcUi {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut camera_ips = HashMap::new();
        // Add cameras here
        // camera_ips.insert(String::from("ll-front"), String::from("10.87.26.11:5800"));
        // camera_ips.insert(String::from("ll-back"), String::from("10.87.26.12:5800"));
        camera_ips.insert(String::from("ll-front"), String::from("0.0.0.0:5800"));
        camera_ips.insert(String::from("ll-back"), String::from("0.0.0.0:5800"));

        // Set up NT
        let nt = unsafe { NT_CreateInstance() };
        unsafe { NT_StartClient4(nt, &to_wpi_string("FRC_UI")) };

        // Start listening to needed values
        let mut listened_values = HashMap::new();
        // add_listener(&mut listened_values, nt_paths::GAME_TIME, nt);
        // add_listener(&mut listened_values, nt_paths::CURRENT_STATE, nt);
        // add_listener(&mut listened_values, nt_paths::KNOWN_LUNITE_POSITIONS, nt);
        // add_listener(&mut listened_values, nt_paths::LUNITE_COUNT, nt);
        // add_listener(&mut listened_values, nt_paths::ROBOT_2D_POSITION, nt);

        let mut s = Self {
            settings_modal_open: false,
            team_number: 8726,
            port: 5810,
            camera_streams: HashMap::new(),
            nt,
            camera_ips,
            listened_values,

            tmp: 200,
        };

        s.try_reconnect();
        s.update_cameras();

        s
    }

    // connects to rio
    fn try_reconnect(&mut self) {
        unsafe {
            NT_SetServerTeam(self.nt, self.team_number, self.port);
        }
    }

    // Sets up new camera streams based on the updated IP addresses.
    fn update_cameras(&mut self) {
        for (k, v) in &self.camera_ips {
            if let Ok(cap) = VideoCapture::from_file(&format!("http://{}", v), CAP_ANY) {
                // This seems to block, causing slow startup.
                // TODO: Async this - and other camera operations too...
                self.camera_streams.insert(k.clone(), cap);
            } else {
                println!("Failed to initialize camera {} with ip {}", k, v);
            }
        }
    }

    // scuffed way to do this, but alas, listeners cause a crash I can't debug easily.
    fn update_nt_values(&mut self) {
        // gameTime
        let game_time =
            unsafe { NT_GetDouble(get_entry_handle(nt_paths::GAME_TIME, self.nt), -1.0) };
        let lunite_count =
            unsafe { NT_GetDouble(get_entry_handle(nt_paths::LUNITE_COUNT, self.nt), -1.0) };
        let mut current_state = to_wpi_string("Unknown");
        unsafe {
            NT_GetString(
                get_entry_handle(nt_paths::CURRENT_STATE, self.nt),
                &to_wpi_string("Unknown"),
                &mut current_state,
            )
        };
        for string in [
            nt_paths::ROBOT_2D_POSITION.to_string(),
            nt_paths::KNOWN_LUNITE_POSITIONS.to_string(),
        ] {
            let arr = Vec::<f64>::new();
            let mut arr_len = 0usize;
            let out_ptr = unsafe {
                NT_GetDoubleArray(
                    get_entry_handle(&string, self.nt),
                    arr.as_ptr(),
                    0,
                    &mut arr_len,
                )
            };
            if arr_len > 0 {
                self.listened_values.insert(string, unsafe {
                    NTValueType::DoubleArray(Vec::from_raw_parts(out_ptr, arr_len, arr_len))
                });
            }
        }

        self.listened_values.insert(
            nt_paths::CURRENT_STATE.to_string(),
            NTValueType::String(from_wpi_string(current_state)),
        );
        if game_time != -1.0 {
            self.listened_values.insert(
                nt_paths::GAME_TIME.to_string(),
                NTValueType::Double(game_time),
            );
        }
        if lunite_count != -1.0 {
            self.listened_values.insert(
                nt_paths::LUNITE_COUNT.to_string(),
                NTValueType::Double(lunite_count),
            );
        }
    }
}

impl eframe::App for FrcUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        ctx.include_bytes(
            "bytes://bbots25-field.png",
            include_bytes!("assets/bbots25-field.png"),
        );

        self.update_nt_values();

        components::top_bar::top_bar(ctx, &self.listened_values);

        components::central_panel::central_panel(ctx, self);

        if self.settings_modal_open {
            let modal = Modal::new(Id::new("Settings Modal")).show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Connection Settings");
                    ui.separator();
                    ui.heading("Camera IP Addresses - include ports! e.x. 1.2.3.4:5800");
                    for entry in &mut self.camera_ips {
                        ui.horizontal(|ui| {
                            ui.label(entry.0);
                            ui.text_edit_singleline(entry.1);
                        });
                    }

                    ui.heading("RoboRIO connection settings");
                    ui.horizontal(|ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.horizontal_centered(|ui| {
                                ui.label("Team Number: ");
                                ui.add(DragValue::new(&mut self.team_number).speed(1))
                            });
                            ui.horizontal_centered(|ui| {
                                ui.label("Port: ");
                                ui.add(DragValue::new(&mut self.port).speed(1))
                            })
                        });
                    });

                    ui.separator();
                    ui.label("This will stall for a while while trying to connect to cameras!");
                    ui.label("Don't worry, it didn't crash.");
                    ui.label("Maybe I'll multithread this in the future to avoid this...");
                    if ui.button("Save, Reconnect and Close").clicked() {
                        self.try_reconnect();
                        self.update_cameras();
                        self.settings_modal_open = false;
                    }
                });
            });

            if modal.should_close() {
                self.try_reconnect();
                self.update_cameras();
                self.settings_modal_open = false;
            }
        }

        ctx.request_repaint(); // spam repaint just to be safe. lots of values change.
    }
}
