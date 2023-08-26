use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Day 8: Treetop Tree House",
        Default::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

struct App {
    label: String,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            label: "Welcome to Treetop Tree House!".to_owned(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.label);
        });
    }
}

