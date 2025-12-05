use egui::Ui;

use crate::{FrcUi, nt_paths, nt_util::NTValueType};

pub fn show_input_bindings(ui: &mut Ui, app: &mut FrcUi) {
    if let Some(NTValueType::String(s)) = app.listened_values.get(nt_paths::CURRENT_STATE) {
        if s == "IdleToIntake" {
            ui.label("A: Do something | B: Do other thing | ...");
        } else if s == "IdleToShoot" {
            ui.label("A: Do something | B: Do other thing | ...");
        } else if s == "Intake" {
            ui.label("A: Do something | B: Do other thing | ...");
        } else if s == "Shooting" {
            ui.label("A: Do something | B: Do other thing | ...");
        } else if s == "Autodrive" {
            ui.label("A: Do something | B: Do other thing | ...");
        } else if s == "IdleDebug" {
            ui.label("A: Do something | B: Do other thing | ...");
        } else if s == "ManualIntake" {
            ui.label("A: Do something | B: Do other thing | ...");
        } else {
            ui.label("Unknown state - bindings will appear here, like A: Intake | ...");
        }
    } else {
        ui.label("Unknown state - bindings will appear here, like A: Intake | ...");
    }
}
