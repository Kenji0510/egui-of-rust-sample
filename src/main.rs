use core::panic;

use egui::ScrollArea;
use opencv::{
    core::{Mat, Vec3b},
    imgcodecs,
    prelude::*,
    text,
    videoio::{VideoCapture, CAP_ANY},
};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "egui app",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            //Ok(Box::new(MyApp::default()))
            Ok(Box::new(MyApp::new(&cc.egui_ctx)))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    count: u16,
    age: u16,
    frame: Mat,
    texture_handle: Option<egui::TextureHandle>,
}

impl MyApp {
    fn new(ctx: &egui::Context) -> Self {
        let mut video = VideoCapture::from_file(
            "/home/kenji/workspace/Rust/opencv_sample/data/AutowareDemoVideo.m4v",
            CAP_ANY,
        )
        .unwrap();
        let mut frame = Mat::default();

        if video.read(&mut frame).unwrap() {
            if frame.empty() {
                panic!("Frame is empty!");
            }
        } else {
            panic!("Failed to read frame!");
        }

        let texture_handle = Some(Self::mat_to_texture(ctx, &frame));

        MyApp {
            count: 0,
            age: 0,
            frame,
            texture_handle,
        }
    }

    fn mat_to_texture(ctx: &egui::Context, mat: &Mat) -> egui::TextureHandle {
        // Convert Mat to egui::ColorImage
        let size = [mat.cols() as usize, mat.rows() as usize];
        let mut pixels = Vec::with_capacity(size[0] * size[1] * 4);
        for y in 0..mat.rows() {
            for x in 0..mat.cols() {
                let pixel = mat.at_2d::<Vec3b>(y, x).unwrap();
                pixels.push(pixel[2]); // R
                pixels.push(pixel[1]); // G
                pixels.push(pixel[0]); // B
                pixels.push(255); // A
            }
        }
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        ctx.load_texture("frame_texture", color_image, Default::default())
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("Hello, welcome to egui!");
                ui.label("Good afetrnoon!");

                ui.add(egui::Slider::new(&mut self.age, 0..=100).text("age"));
                if ui.button("Increment").clicked() {
                    self.age += 1;
                }

                if let Some(texture_handle) = &self.texture_handle {
                    ui.image(texture_handle);
                }

                // ui.add(
                //     //egui::Image::new(egui::include_image!("../data/crab.png"))
                //     egui::Image::new(egui::include_image!("../data/crab.png"))
                //         .max_width(400.0)
                //         .max_height(400.0)
                //         .rounding(5.0),
                // );
            });

            if ui.button("Click here").clicked() {
                println!("Button clicked!");
                self.count += 1;
                println!("Count: {}", self.count);
            }
        });
    }
}
