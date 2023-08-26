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
    cell_size: egui::Vec2,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
            cell_size: egui::vec2(16.0, 16.0),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let grid = &self.grid;
            let cell_size = &self.cell_size;

            let rows = grid.len();
            if rows == 0 { return; }
            let cols = grid[0].len();

            let mouse_inside_window = ctx.input(|i| i.pointer.has_pointer());
            let mouse_pos = ctx.input(|i| i.pointer.hover_pos().unwrap_or_default());

            let mut closest_cell = None;

            // This is the threshold beyond which we won't highlight any cells
            let highlight_threshold = cell_size.x.max(cell_size.y); // Using the largest dimension of the cell

            if mouse_inside_window {
                let mut min_distance = f32::MAX;

                for row in 0..rows {
                    for col in 0..cols {
                        let rect_center = egui::pos2((col as f32 + 0.5) * cell_size.x, (row as f32 + 0.5) * cell_size.y);
                        let distance = mouse_pos.distance(rect_center);
                        if distance < min_distance && distance < highlight_threshold {
                            min_distance = distance;
                            closest_cell = Some((row, col));
                        }
                    }
                }
            }

            for row in 0..rows {
                for col in 0..cols {
                    let rect_min = egui::pos2(col as f32 * cell_size.x, row as f32 * cell_size.y);
                    let rect_center = rect_min + (*cell_size * 0.5);
                    let text_color = if closest_cell == Some((row, col)) {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::WHITE
                    };
                    ui.painter().text(
                        rect_center,
                        egui::Align2::CENTER_CENTER,
                        grid[row][col],
                        egui::FontId::default(),
                        text_color
                    );
                }
            }
        });
    }
}
