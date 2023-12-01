use crate::models::{Forest, Pos, Tree};
use common_2022::read_lines;
use eframe::egui;
use eframe::egui::Vec2;

pub struct App {
    forest: Forest,
    cell_size: Vec2,
    visible_trees: usize,
    max_scenic_score: u32,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl Default for App {
    fn default() -> Self {
        let mut forest = Forest {
            trees: read_lines("../!data/day8/real.txt")
                .expect("Failed to read lines from file")
                .map(|line| {
                    line.chars()
                        .map(|ch| Tree {
                            height: ch.to_digit(10).unwrap() as u8,
                            ..Default::default()
                        })
                        .collect::<Vec<Tree>>()
                })
                .collect::<Vec<Vec<Tree>>>(),
        };

        let visible_trees = forest.calculate_visible_trees();
        let max_scenic_score = forest.calculate_max_scenic_score();

        Self {
            forest,
            cell_size: egui::vec2(16.0, 16.0),
            visible_trees,
            max_scenic_score,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let forest = &self.forest.trees;
        let cell_size = &self.cell_size;

        let rows = forest.len();
        if rows == 0 {
            return;
        }
        let cols = forest[0].len();

        // Do one start
        let cell = self.cell_size;
        let window_size = Vec2 {
            x: cols as f32 * cell.x,
            y: rows as f32 * cell.y,
        };
        frame.set_window_size(window_size);
        // Do once end

        egui::CentralPanel::default().show(ctx, |ui| {
            let mouse_inside_window = ctx.input(|i| i.pointer.has_pointer());
            let mouse_pos = ctx.input(|i| i.pointer.hover_pos().unwrap_or_default());

            let highlight_threshold = cell_size.x.max(cell_size.y);

            let mut closest_cell = None;
            if mouse_inside_window {
                closest_cell = (0..rows)
                    .flat_map(|row| (0..cols).map(move |col| Pos { row, col }))
                    .map(|pos| {
                        let rect_center = egui::pos2(
                            (pos.col as f32 + 0.5) * cell_size.x,
                            (pos.row as f32 + 0.5) * cell_size.y,
                        );
                        let distance = mouse_pos.distance(rect_center);
                        (pos, distance)
                    })
                    .filter(|&(_, distance)| distance < highlight_threshold)
                    .min_by(|&(_, distance1), &(_, distance2)| {
                        distance1
                            .partial_cmp(&distance2)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(pos, _)| pos);
            } else {
                egui::Window::new("Information")
                    .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ctx, |ui| {
                        ui.label(format!("Visible Trees: {}", self.visible_trees));
                        ui.label(format!("Max Scenic Score: {}", self.max_scenic_score));
                    });
            }

            for row in 0..rows {
                for col in 0..cols {
                    let rect_min = egui::pos2(col as f32 * cell_size.x, row as f32 * cell_size.y);
                    let rect_center = rect_min + (*cell_size * 0.5);

                    let text_color =
                        if mouse_inside_window && closest_cell == Some(Pos { row, col }) {
                            egui::Color32::WHITE
                        } else if closest_cell.as_ref().map_or(false, |c| {
                            forest[c.row][c.col]
                                .trees_in_range
                                .iter()
                                .any(|pos| pos == &Pos { row, col })
                        }) {
                            egui::Color32::YELLOW
                        } else if forest[row][col].visible {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        };

                    ui.painter().text(
                        rect_center,
                        egui::Align2::CENTER_CENTER,
                        forest[row][col].height,
                        egui::FontId::default(),
                        text_color,
                    );
                }
            }
        });
    }
}
