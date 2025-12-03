use egui::TopBottomPanel;

use crate::{
    nt_paths,
    nt_util::{ListenedValues, NTValueType, format_game_time},
};

pub fn top_bar(ctx: &egui::Context, values: &ListenedValues) {
    TopBottomPanel::top("TopBar").show(ctx, |ui| {
        ui.columns(3, |columns| {
            // Lunite count
            columns[0].centered_and_justified(|ui| {
                let lunites = values.get(nt_paths::LUNITE_COUNT);
                if let Some(NTValueType::Double(n)) = lunites {
                    ui.label(format!("Lunite Count: {}", n.floor() as i32));
                } else {
                    ui.label("Lunite Count: Unknown");
                }
            });

            // Game timer
            columns[1].centered_and_justified(|ui| {
                ui.heading(format_game_time(values.get(nt_paths::GAME_TIME).and_then(
                    |v| {
                        if let NTValueType::Double(f) = *v {
                            Some(f)
                        } else {
                            None
                        }
                    },
                )));
            });

            // State
            columns[2].centered_and_justified(|ui| {
                let state = values.get(nt_paths::CURRENT_STATE);
                if let Some(NTValueType::String(s)) = state {
                    ui.label(format!("Current State: {}", s));
                } else {
                    ui.label("Current State: Unknown");
                }
            });
        });
    });
}
