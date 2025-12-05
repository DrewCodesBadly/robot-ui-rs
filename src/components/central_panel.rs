use std::{iter::repeat_n, ptr::slice_from_raw_parts};

use egui::{
    CentralPanel, Color32, ColorImage, Frame, Image, Pos2, SidePanel, TopBottomPanel,
    load::SizedTexture,
};
use opencv::{
    core::{CV_8U, CV_8UC3, Mat, MatExpr, MatExprTraitConst, MatTrait, MatTraitConst, Vector},
    imgcodecs::imencode,
    traits::Boxed,
    videoio::VideoCaptureTrait,
};

use crate::{
    FrcUi, components::input_descriptions::show_input_bindings, nt_paths, nt_util::NTValueType,
};

pub fn central_panel(ctx: &egui::Context, app: &mut FrcUi) {
    SidePanel::left("LeftCamerasPanel").show(ctx, |ui| {
        ui.vertical(|ui| {
            for (name, capture) in &mut app.camera_streams {
                ui.label(format!("Camera Feed: {}", name));
                let mut mat = Mat::default();
                // If capture failed, show a blank image.
                if capture.read(&mut mat).ok().filter(|b| *b).is_none() {
                    // Black screen
                    mat = Mat::zeros(480, 640, CV_8UC3)
                        .expect("base mat should be valid")
                        .to_mat()
                        .expect("base mat should be valid");

                    // Changing image (for testing purposes)
                    // let zeros_mat = Mat::zeros(480, 640, CV_8UC3)
                    //     .expect("base mat should be valid")
                    //     .to_mat()
                    //     .expect("base mat should be valid");
                    // app.tmp = (app.tmp + 1) % 256;
                    // println!("{}", app.tmp);
                    // let _ = opencv::core::add(
                    //     &zeros_mat,
                    //     &(app.tmp as f64),
                    //     &mut mat,
                    //     &Mat::ones(480, 640, CV_8U).unwrap().to_mat().unwrap(),
                    //     CV_8UC3,
                    // );
                }
                let unsafe_slice =
                    unsafe { slice_from_raw_parts(mat.data(), mat.total() * 3).as_ref() };
                if let Some(slice) = unsafe_slice {
                    let image =
                        ColorImage::from_rgb([mat.cols() as usize, mat.rows() as usize], slice);
                    let tex =
                        ctx.load_texture(&format!("camera-{}", name), image, Default::default());
                    ui.add(
                        Image::new(SizedTexture::from_handle(&tex))
                            .maintain_aspect_ratio(true)
                            .shrink_to_fit(),
                    );
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

        show_input_bindings(ui, app);
    });
}
