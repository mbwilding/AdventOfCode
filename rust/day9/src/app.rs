use crate::models::{Direction, Move};
use common::read_lines;
use eframe::egui;
use std::ops::Deref;

pub struct App {
    moves: Vec<Move>,
    current_move_index: usize,
    speed: f32,        // The speed of the iteration, can be seconds per move
    elapsed_time: f32, // Time since the last move was shown
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl Default for App {
    fn default() -> Self {
        let moves: Vec<_> = read_lines("../!data/day9/mock.txt")
            .expect("Failed to read lines from file")
            .map(|line| Move::new(line))
            .collect();

        Self {
            moves,
            current_move_index: 0,
            speed: 1.0,
            elapsed_time: 0.0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Day 9: Rope Bridge");
            ui.separator();

            // Slider to control the speed
            ui.horizontal(|ui| {
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed, 0.1..=5.0));
            });

            // Update current move based on elapsed time and speed
            //self.elapsed_time += frame.delta_time();
            if self.elapsed_time >= self.speed {
                self.current_move_index = (self.current_move_index + 1) % self.moves.len();
                self.elapsed_time = 0.0;
            }

            // Display the current move
            ui.label(format!(
                "{}: {:?}",
                self.current_move_index, &self.moves[self.current_move_index]
            ));
        });
    }
}
