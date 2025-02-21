use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

use egui::ScrollArea;
use opencv::{
    core::{Mat, Vec3b},
    imgproc,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "egui app",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 720.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            //Ok(Box::new(MyApp::default()))
            Ok(Box::new(MyApp::new(&cc.egui_ctx)))
        }),
    )
}

//#[derive(Default)]
struct MyApp {
    count: u16,
    age: u16,
    texture_handle: Option<egui::TextureHandle>,
    frame_receiver: Receiver<egui::ColorImage>,
    timer: Instant,
}

// impl Default for MyApp {
//     fn default() -> Self {
//         let mut video = VideoCapture::from_file(
//             "/home/kenji/workspace/Rust/opencv_sample/data/AutowareDemoVideo.m4v",
//             CAP_ANY,
//         )
//         .unwrap();
//         let mut frame = Mat::default();
//     }
// }

impl MyApp {
    fn new(ctx: &egui::Context) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut video = VideoCapture::from_file(
                "/home/kenji/workspace/Rust/egui_sample/data/sample02.m4v",
                CAP_ANY,
            )
            .expect("Failed to open video file!");
            let mut frame = Mat::default();

            let mut rgba_frame = Mat::default();

            loop {
                if let Ok(read_suucess) = video.read(&mut frame) {
                    if !read_suucess || frame.empty() {
                        eprintln!("Video ended or frame is empty!");
                        break;
                    }

                    // let size = [frame.cols() as usize, frame.rows() as usize];
                    // let mut pixels = Vec::with_capacity(size[0] * size[1] * 4);
                    // for y in 0..frame.rows() {
                    //     for x in 0..frame.cols() {
                    //         let pixel = frame.at_2d::<Vec3b>(y, x).unwrap();
                    //         pixels.push(pixel[2]);
                    //         pixels.push(pixel[1]);
                    //         pixels.push(pixel[0]);
                    //         pixels.push(255);
                    //     }
                    // }
                    // let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

                    // if tx.send(color_image).is_err() {
                    //     eprintln!("Failed to send frame to main thread!");
                    //     break;
                    // }

                    if let Err(e) =
                        imgproc::cvt_color(&frame, &mut rgba_frame, imgproc::COLOR_BGR2RGBA, 0)
                    {
                        eprintln!("Failed to convert color: {}", e);
                        break;
                    }

                    if let Ok(rgba_bytes) = rgba_frame.data_bytes() {
                        let size = [rgba_frame.cols() as usize, rgba_frame.rows() as usize];
                        let color_image =
                            egui::ColorImage::from_rgba_unmultiplied(size, rgba_bytes);
                        if tx.send(color_image).is_err() {
                            break;
                        }
                    } else {
                        eprintln!("Failed to get rgba_frame data bytes!");
                        break;
                    }
                } else {
                    eprintln!("Failed to read frame!");
                    break;
                }
                thread::sleep(Duration::from_millis(16));
            }
        });

        let texture_handle = match rx.try_recv() {
            Ok(color_image) => {
                Some(ctx.load_texture("frame_texture", color_image, Default::default()))
            }
            Err(_) => None,
        };

        Self {
            count: 0,
            age: 0,
            texture_handle,
            frame_receiver: rx,
            timer: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(color_image) = self.frame_receiver.try_recv() {
            self.texture_handle =
                Some(ctx.load_texture("frame_texture", color_image, Default::default()));
            println!("Frame received!");
        }

        ctx.request_repaint();

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
            });

            if ui.button("Click here").clicked() {
                println!("Button clicked!");
                self.count += 1;
                println!("Count: {}", self.count);
            }
        });
    }
}
