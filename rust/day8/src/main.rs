mod app;
mod models;

use crate::app::App;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Day 8: Treetop Tree House",
        eframe::NativeOptions {
            initial_window_size: Some(eframe::egui::Vec2 {
                x: 1584.0,
                y: 1584.0,
            }),
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
