use egui::{ComboBox, Layout, TopBottomPanel};
use ntcore_sys::NT_SetString;

use crate::{
    FrcUi, nt_paths,
    nt_util::{NTValueType, get_entry_handle, to_wpi_string},
};

pub fn bottom_panel(ctx: &egui::Context, app: &mut FrcUi) {
    TopBottomPanel::bottom("BottomPanel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Connection Settings").clicked() {
                    app.settings_modal_open = true;
                }
                let auto_chooser_box = ComboBox::new("AutoChooserBox", "Currently selected auto:");
                let mut selected = {
                    let val = app.listened_values.get(nt_paths::AUTO_CHOOSER_ACTIVE);
                    if let Some(NTValueType::String(s)) = val {
                        s.to_owned()
                    } else {
                        String::from("None")
                    }
                };
                auto_chooser_box
                    .selected_text(selected.clone())
                    .show_ui(ui, |inner_ui| {
                        if let Some(NTValueType::StringArray(arr)) =
                            app.listened_values.get(nt_paths::AUTO_CHOOSER_OPTIONS)
                        {
                            for option in arr {
                                inner_ui.selectable_value(&mut selected, option.to_owned(), option);
                            }
                        }
                    });

                unsafe {
                    NT_SetString(
                        get_entry_handle(nt_paths::AUTO_CHOOSER_ACTIVE, app.nt),
                        0,
                        &to_wpi_string(&selected),
                    )
                };
            });
        });
    });
}
