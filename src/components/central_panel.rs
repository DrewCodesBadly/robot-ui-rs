use std::{iter::repeat_n, ptr::slice_from_raw_parts};

use egui::{
    Button, CentralPanel, Color32, ColorImage, ComboBox, FontId, Frame, Image, Pos2, SidePanel,
    TextFormat, TopBottomPanel, load::SizedTexture, text::LayoutJob,
};
use ntcore_sys::NT_SetString;
use opencv::{
    core::{CV_8U, CV_8UC3, Mat, MatExpr, MatExprTraitConst, MatTrait, MatTraitConst, Vector},
    imgcodecs::imencode,
    traits::Boxed,
    videoio::VideoCaptureTrait,
};

use crate::{
    FrcUi,
    components::input_descriptions::show_input_bindings,
    nt_paths,
    nt_util::{NTValueType, format_game_time, get_entry_handle, to_wpi_string},
};

pub fn central_panel(ctx: &egui::Context, app: &mut FrcUi) {
    CentralPanel::default().show(ctx, |ui| {
        // Get bot position
        let bot_pos = if let Some(NTValueType::DoubleArray(arr)) =
            app.listened_values.get(nt_paths::ROBOT_2D_POSITION)
        {
            if arr.len() >= 3 {
                Some((arr[0] / 16.4592, 1.0 - arr[1] / 8.2296, arr[2]))
            } else {
                None // Should never happen.
            }
        } else {
            None
        };

        let lunite_positions = if let Some(NTValueType::DoubleArray(arr)) =
            app.listened_values.get(nt_paths::KNOWN_LUNITE_POSITIONS)
        {
            if arr.len() > 0 { Some(arr) } else { None }
        } else {
            None
        };

        Frame::canvas(ui.style()).show(ui, |ui| {
            let img = Image::new("bytes://bbots25-field.png").shrink_to_fit();
            ui.add(img);
            let ui_rect = ui.min_rect();
            if let Some(pos) = bot_pos {
                let center_pos = Pos2::new(
                    ui_rect.min.x + pos.0 as f32 * ui_rect.width(),
                    ui_rect.min.y + pos.1 as f32 * ui_rect.height(),
                );
                ui.painter()
                    .circle_filled(center_pos, 15.0, Color32::from_rgb(30, 30, 150));
                ui.painter()
                    .circle_filled(center_pos, 5.0, Color32::from_rgb(255, 0, 0));
            }

            if let Some(positions) = lunite_positions {
                for pos in positions.chunks(2) {
                    if pos.len() < 2 {
                        continue; // bad chunk at the end for some reason, most likely.
                    }
                    let center_pos = Pos2::new(
                        ui_rect.min.x + (pos[0] / 16.4592) as f32 * ui_rect.width(),
                        ui_rect.min.y + 1.0 - (pos[1] / 8.2296) as f32 * ui_rect.height(),
                    );
                    ui.painter()
                        .circle_filled(center_pos, 5.0, Color32::from_rgb(0, 255, 0));
                }
            }
        });
        ui.separator();
        let gt_string = format!(
            "{} - ",
            format_game_time(app.listened_values.get(nt_paths::GAME_TIME).and_then(|v| {
                if let NTValueType::Double(f) = *v {
                    Some(f)
                } else {
                    None
                }
            }))
        );
        let mut job = LayoutJob::default();
        job.append(
            &gt_string,
            0.0,
            TextFormat {
                font_id: FontId::proportional(25.0),
                ..Default::default()
            },
        );
        if let Some(NTValueType::Boolean(true)) =
            app.listened_values.get(nt_paths::FMS_IS_RED_ALLIANCE)
        {
            job.append(
                "RED ",
                0.0,
                TextFormat {
                    font_id: FontId::proportional(25.0),
                    color: Color32::from_rgb(255, 50, 50),
                    ..Default::default()
                },
            );
        } else {
            job.append(
                "BLUE ",
                0.0,
                TextFormat {
                    font_id: FontId::proportional(25.0),
                    color: Color32::from_rgb(50, 50, 255),
                    ..Default::default()
                },
            );
        }
        job.append(
            "ALLIANCE",
            0.0,
            TextFormat {
                font_id: FontId::proportional(25.0),
                ..Default::default()
            },
        );

        ui.label(job);
        ui.separator();

        ui.horizontal(|ui| {
            ui.columns(3, |columns| {
                // State
                columns[0].centered_and_justified(|ui| {
                    let state = app.listened_values.get(nt_paths::CURRENT_STATE);
                    if let Some(NTValueType::String(s)) = state {
                        ui.label(format!("Current State: {}", s));
                    } else {
                        ui.label("Current State: Unknown");
                    }
                });

                // Lunite count
                columns[1].centered_and_justified(|ui| {
                    let lunites = app.listened_values.get(nt_paths::LUNITE_COUNT);
                    if let Some(NTValueType::Double(n)) = lunites {
                        ui.label(format!("Lunite Count: {}", n.floor() as i32));
                    } else {
                        ui.label("Lunite Count: Unknown");
                    }
                });

                // columns[2].centered_and_justified(|ui| ui.label("Add more here later maybe?"));
            });
        });
        ui.separator();
        show_input_bindings(ui, app);
    });
}
