use egui::ScrollArea;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "egui app",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    count: u16,
    age: u16,
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

                ui.add(
                    egui::Image::new(egui::include_image!("../data/crab.png"))
                        .max_width(400.0)
                        .max_height(400.0)
                        .rounding(5.0),
                );
            });

            if ui.button("Click here").clicked() {
                println!("Button clicked!");
                self.count += 1;
                println!("Count: {}", self.count);
            }
        });
    }
}
