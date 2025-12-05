use std::ptr::slice_from_raw_parts;

use egui::{ColorImage, Image, SidePanel, load::SizedTexture};
use opencv::{
    core::{CV_8UC3, Mat, MatExprTraitConst, MatTraitConst},
    videoio::VideoCaptureTrait,
};

use crate::FrcUi;

pub fn left_panel(ctx: &egui::Context, app: &mut FrcUi) {
    SidePanel::left("LeftCamerasPanel").show(ctx, |ui| {
        ui.vertical(|ui| {
            for (name, capture) in &mut app.camera_streams {
                ui.weak(format!("Camera Feed: {}", name));
                let mut mat = Mat::default();
                // If capture failed, show a blank image.
                if capture.read(&mut mat).ok().filter(|b| *b).is_none() {
                    // Black screen
                    mat = Mat::zeros(480, 640, CV_8UC3)
                        .expect("base mat should be valid")
                        .to_mat()
                        .expect("base mat should be valid");
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
}
