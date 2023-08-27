use common::read_lines;
use eframe::egui;
use eframe::egui::Vec2;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Day 8: Treetop Tree House",
        eframe::NativeOptions {
            initial_window_size: Some(Vec2 {
                x: 1584.0,
                y: 1584.0,
            }),
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

struct App {
    forest: Forest,
    cell_size: Vec2,
    visible_trees: usize,
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

        Self {
            forest,
            cell_size: egui::vec2(16.0, 16.0),
            visible_trees,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let forest = &self.forest.trees;
            let cell_size = &self.cell_size;

            let rows = forest.len();
            if rows == 0 {
                return;
            }
            let cols = forest[0].len();

            let mouse_inside_window = ctx.input(|i| i.pointer.has_pointer());
            let mouse_pos = ctx.input(|i| i.pointer.hover_pos().unwrap_or_default());

            let mut closest_cell = None;

            let highlight_threshold = cell_size.x.max(cell_size.y); // Using the largest dimension of the cell

            if mouse_inside_window {
                let mut min_distance = f32::MAX;

                for row in 0..rows {
                    for col in 0..cols {
                        let rect_center = egui::pos2(
                            (col as f32 + 0.5) * cell_size.x,
                            (row as f32 + 0.5) * cell_size.y,
                        );
                        let distance = mouse_pos.distance(rect_center);
                        if distance < min_distance && distance < highlight_threshold {
                            min_distance = distance;
                            closest_cell = Some((row, col));
                        }
                    }
                }
            } else {
                egui::Window::new("Information")
                    .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ctx, |ui| {
                        ui.label(format!("Total Visible Trees: {}", self.visible_trees));
                    });
            }

            for row in 0..rows {
                for col in 0..cols {
                    let rect_min = egui::pos2(col as f32 * cell_size.x, row as f32 * cell_size.y);
                    let rect_center = rect_min + (*cell_size * 0.5);
                    let text_color = if closest_cell == Some((row, col)) {
                        egui::Color32::WHITE
                    } else if closest_cell.is_some()
                        && forest[closest_cell.unwrap().0][closest_cell.unwrap().1]
                            .trees_that_make_me_visible
                            .iter()
                            .any(|pos| pos == &Pos { row, col })
                    {
                        egui::Color32::YELLOW
                    } else {
                        if forest[row][col].visible {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        }
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

#[derive(PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

impl Default for Pos {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}

struct Tree {
    height: u8,
    visible: bool,
    trees_that_make_me_visible: Vec<Pos>,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            height: 0,
            visible: false,
            trees_that_make_me_visible: vec![],
        }
    }
}

struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn calculate_visible_trees(&mut self) -> usize {
        let mut visible_count = 0;

        for row in 0..self.trees.len() {
            for col in 0..self.trees[0].len() {
                let (is_visible, trees_making_visible) = self.check_visibility(row, col);
                if is_visible {
                    self.trees[row][col].visible = true;
                    self.trees[row][col].trees_that_make_me_visible = trees_making_visible;
                    visible_count += 1;
                } else {
                    self.trees[row][col].visible = false;
                }
            }
        }

        visible_count
    }

    fn check_visibility(&self, row: usize, col: usize) -> (bool, Vec<Pos>) {
        let mut trees_making_visible = Vec::new();
        let current_tree_height = self.trees[row][col].height;
        let mut top_visible = false;
        let mut bottom_visible = false;
        let mut left_visible = false;
        let mut right_visible = false;

        // If the tree is on the boundary, it is visible
        if row == 0 || row == self.trees.len() - 1 || col == 0 || col == self.trees[0].len() - 1 {
            return (true, trees_making_visible);
        }

        // Check from top
        for row in (0..row).rev() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if row == 0 {
                top_visible = true;
            }
        }

        // Check from bottom
        for row in row + 1..self.trees.len() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if row == self.trees.len() - 1 {
                bottom_visible = true;
            }
        }

        // Check from left
        for col in (0..col).rev() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if col == 0 {
                left_visible = true;
            }
        }

        // Check from right
        for col in col + 1..self.trees[0].len() {
            if self.trees[row][col].height < current_tree_height {
                trees_making_visible.push(Pos { row, col });
            } else {
                break;
            }
            if col == self.trees[0].len() - 1 {
                right_visible = true;
            }
        }

        if top_visible || bottom_visible || left_visible || right_visible {
            (true, trees_making_visible)
        } else {
            (false, vec![])
        }
    }
}
