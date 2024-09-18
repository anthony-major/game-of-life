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
        let width = 400;
        let height = 400;

        let seed: Vec<Coord> = (0..(width * height))
            .filter_map(|i| {
                if rand::random::<f32>() < 0.05 {
                    Some(Coord::new(i / height, i % width))
                } else {
                    None
                }
            })
            .collect();

        // let seed = vec![
        //     Coord::new(200, 200),
        //     Coord::new(199, 200),
        //     Coord::new(200, 199),
        //     Coord::new(201, 200),
        //     Coord::new(201, 201),
        // ];

        let board = Board::new(seed);

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

            for cell in cells {
                let min = Pos2::new((cell.column * 2) as f32, (cell.row * 2) as f32);
                let max = Pos2::new(min.x + 2.0, min.y + 2.0);

                ui.painter().rect_filled(
                    Rect::from_min_max(min, max),
                    Rounding::ZERO,
                    Color32::BLACK,
                );
            }
        });

        let delta_time = Instant::now() - self.previous_frame_time;

        self.timer += delta_time;

        if self.timer >= Duration::from_millis(17) {
            self.board.update();
            self.timer = Duration::ZERO;
        }

        ctx.request_repaint();

        self.previous_frame_time = Instant::now();
    }
}
