use std::time::{Duration, Instant};

use crate::board::{Board, Coord};
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, TextEdit};

pub struct Game {
    board: Board,
    previous_frame_time: Instant,
    timer: Duration,
    speed: u32,
    speed_input: String,
    cell_size: u32,
}

impl Default for Game {
    fn default() -> Self {
        let width = 64;
        let height = 64;

        let seed: Vec<Coord> = (0..width * height)
            .filter_map(|i| {
                if rand::random::<f32>() < 0.1 {
                    Some(Coord::new(i / height, i % width))
                } else {
                    None
                }
            })
            .collect();

        let board = Board::new(seed);

        Self {
            board: board,
            previous_frame_time: Instant::now(),
            timer: Duration::ZERO,
            speed: 2,
            speed_input: 2.to_string(),
            cell_size: 5,
        }
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("control_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Speed:");
                let response = ui.add(
                    TextEdit::singleline(&mut self.speed_input)
                        .desired_width(0.0)
                        .clip_text(false)
                        .hint_text(self.speed.to_string()),
                );
                response.request_focus();
                ui.label("updates/second");

                if response.changed() {
                    self.speed = self.speed_input.parse::<u32>().unwrap_or(self.speed);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let cells = self.board.cells();

            for cell in cells {
                let min = Pos2::new(
                    (cell.column * self.cell_size as i32) as f32,
                    (cell.row * self.cell_size as i32) as f32,
                );
                let max = Pos2::new(min.x + self.cell_size as f32, min.y + self.cell_size as f32);

                ui.painter().rect_filled(
                    Rect::from_min_max(min, max),
                    Rounding::ZERO,
                    Color32::BLACK,
                );
            }
        });

        let delta_time = Instant::now() - self.previous_frame_time;

        self.timer += delta_time;

        if self.speed > 0 && self.timer >= Duration::from_millis(1000 / self.speed as u64) {
            self.board.update();
            self.timer = Duration::ZERO;
        }

        ctx.request_repaint();

        self.previous_frame_time = Instant::now();
    }
}
