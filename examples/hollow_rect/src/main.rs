#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label("空心矩形示例：");

        let available = ui.available_size();
        let margin = 40.0;
        let rect_size = (available.x - 2.0 * margin)
            .min(available.y - 2.0 * margin)
            .max(10.0);

        let center = egui::pos2(available.x / 2.0, available.y / 2.0 + 15.0);
        let outer = egui::Rect::from_center_size(center, egui::vec2(rect_size, rect_size));

        let inner_size = rect_size * 0.6;
        let inner = egui::Rect::from_center_size(center, egui::vec2(inner_size, inner_size));

        let hollow = egui::Shape::hollow_rect(
            outer,
            inner,
            0.0,
            egui::Color32::from_rgba_unmultiplied(255, 100, 100, 128),
            egui::Stroke {
                width: 3.0,
                color: egui::Color32::from_rgb(255, 50, 50),
            },
            egui::Stroke::NONE,
        );

        ui.painter().add(hollow);
    }
}
