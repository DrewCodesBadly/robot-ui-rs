use egui::{CentralPanel, Color32, Frame, Image, Pos2, SidePanel, TopBottomPanel};
use opencv::{
    core::{Mat, Vector},
    imgcodecs::imencode,
    videoio::VideoCaptureTrait,
};

use crate::{FrcUi, nt_paths, nt_util::NTValueType};

pub fn central_panel(ctx: &egui::Context, app: &mut FrcUi) {
    SidePanel::left("LeftCamerasPanel").show(ctx, |ui| {
        ui.vertical(|ui| {
            for (name, capture) in &mut app.camera_streams {
                let mut mat = Mat::default();
                if let Ok(true) = capture.read(&mut mat) {
                    // re-encoding is jank but whatever I guess.
                    // its easier and im tired.
                    let mut buffer = Vector::new();
                    if let Ok(true) = imencode(".png", &mat, &mut buffer, &Vector::new()) {
                        ui.add(Image::from_bytes(
                            "bytes://camera_bytes",
                            buffer.as_slice().to_vec(),
                        ));
                    }
                }
            }
        });
    });

    TopBottomPanel::bottom("BottomPanel").show(ctx, |ui| {
        if ui.button("Connection Settings").clicked() {
            app.settings_modal_open = true;
        }
    });

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
    });
}
