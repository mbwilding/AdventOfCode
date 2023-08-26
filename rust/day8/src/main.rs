use eframe::egui;
use common::read_lines;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Day 8: Treetop Tree House",
        Default::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

struct App {
    grid: Vec<Vec<u8>>,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            grid: read_lines("../!data/day8/real.txt")
                .expect("Failed to read lines from file")
                .map(|line| {
                    line.chars()
                        .map(|ch| ch.to_digit(10).unwrap() as u8)
                        .collect::<Vec<u8>>()
                })
                .collect(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            draw_grid(ui, &self.grid);
        });
    }
}

fn draw_grid(ui: &mut egui::Ui, grid: &Vec<Vec<u8>>) {
    // Define cell size
    let cell_size = egui::vec2(16.0, 16.0);

    let rows = grid.len();
    if rows == 0 { return; }  // Return early if grid is empty

    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let rect_min = egui::pos2(col as f32 * cell_size.x, row as f32 * cell_size.y);
            let rect_max = rect_min + cell_size;
            let rect = egui::Rect::from_min_max(rect_min, rect_max);

            // Paint cell background (optional)
            ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);

            // Paint cell border
            ui.painter().rect_stroke(rect, 0.0, (1.0, egui::Color32::BLACK));

            // Display the cell's number from the grid
            let number = grid[row][col];
            let number_position = rect.center();
            ui.painter().text(
                number_position,
                egui::Align2::CENTER_CENTER,
                number.to_string(),
                egui::FontId::default(),
                egui::Color32::BLACK
            );
        }
    }
}
