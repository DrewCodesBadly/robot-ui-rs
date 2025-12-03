use std::collections::HashMap;

use egui::{CentralPanel, Context, DragValue, Id, Modal, SidePanel};
use ntcore_sys::{NT_CreateInstance, NT_Inst, NT_SetServerTeam, NT_StartClient4};

use crate::nt_util::{ListenedValues, NTValueType, add_listener, to_wpi_string};

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
    cameras: HashMap<String, String>,
    settings_modal_open: bool,
    listened_values: ListenedValues,
}

impl FrcUi {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut cameras = HashMap::new();
        // Add cameras here
        cameras.insert(String::from("ll-front"), String::from("0.0.0.0:5800"));
        cameras.insert(String::from("ll-back"), String::from("0.0.0.0:5800"));

        // Set up NT
        let nt = unsafe { NT_CreateInstance() };
        unsafe { NT_StartClient4(nt, &to_wpi_string("FRC_UI")) };

        // Start listening to needed values
        let mut listened_values = HashMap::new();
        add_listener(&mut listened_values, nt_paths::GAME_TIME, nt);
        add_listener(&mut listened_values, nt_paths::CURRENT_STATE, nt);
        add_listener(&mut listened_values, nt_paths::KNOWN_LUNITE_POSITIONS, nt);
        add_listener(&mut listened_values, nt_paths::LUNITE_COUNT, nt);
        add_listener(&mut listened_values, nt_paths::ROBOT_2D_POSITION, nt);

        Self {
            settings_modal_open: false,
            team_number: 1234,
            port: 5810,
            nt,
            cameras,
            listened_values,
        }
    }

    // connects to rio
    fn try_reconnect(&mut self) {
        unsafe {
            NT_SetServerTeam(self.nt, self.team_number, self.port);
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

        components::top_bar::top_bar(ctx, &self.listened_values);

        components::central_panel::central_panel(ctx, self);

        if self.settings_modal_open {
            let modal = Modal::new(Id::new("Settings Modal")).show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Connection Settings");
                    ui.separator();
                    ui.heading("Camera IP Addresses - include ports! e.x. 1.2.3.4:5800");
                    for entry in &mut self.cameras {
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
                        if ui.button("Connect to Robot").clicked() {
                            self.try_reconnect()
                        }
                    });

                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.settings_modal_open = false;
                    }
                });
            });

            if modal.should_close() {
                self.settings_modal_open = false;
            }
        }
    }
}
