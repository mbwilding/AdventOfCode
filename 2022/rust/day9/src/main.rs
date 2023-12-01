mod app;
mod models;

use crate::app::App;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Day 9: Rope Bridge",
        eframe::NativeOptions {
            initial_window_size: Some(eframe::egui::Vec2 { x: 500.0, y: 500.0 }),
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
