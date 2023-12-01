mod app;
mod models;

use crate::app::App;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Day 8: Treetop Tree House",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
