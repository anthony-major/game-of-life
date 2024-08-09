use std::time::{Duration, Instant};

use crate::board::{Board, Coord};
use eframe::egui::{self, Color32, Pos2, Rect, Rounding};

pub struct Game {
    board: Board,
    previous_frame_time: Instant,
    timer: Duration,
}

impl Game {
    pub fn new() -> Self {
        let width: usize = 400;
        let height: usize = 400;
        let seed: Vec<Coord> = (0..(width * height))
            .filter_map(|i| {
                if rand::random() {
                    Some(Coord::new(i / height, i % width))
                } else {
                    None
                }
            })
            .collect();
        let mut board = Board::new(width, height);

        board.seed(seed);

        Self {
            board: board,
            previous_frame_time: Instant::now(),
            timer: Duration::ZERO,
        }
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let cells = self.board.cells();

            for row in 0..self.board.height() {
                for column in 0..self.board.width() {
                    let i = self.board.get_index(row, column);

                    if cells[i] {
                        let min = Pos2::new((column * 2) as f32, (row * 2) as f32);
                        let max = Pos2::new(min.x + 2.0, min.y + 2.0);

                        ui.painter().rect_filled(
                            Rect::from_min_max(min, max),
                            Rounding::ZERO,
                            Color32::BLACK,
                        );
                    }
                }
            }
        });

        let delta_time = Instant::now() - self.previous_frame_time;

        self.timer += delta_time;

        if self.timer >= Duration::from_millis(100) {
            self.board.update();
            self.timer = Duration::ZERO;
        }

        ctx.request_repaint();

        self.previous_frame_time = Instant::now();
    }
}
