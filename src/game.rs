use std::{
    time::{Duration, Instant},
    u32,
};

use crate::board::{Board, Coord};
use eframe::egui::{self, Color32, Pos2, Rect, Rounding, TextEdit};

pub struct Game {
    board: Board,
    previous_frame_time: Instant,
    timer: Duration,
    update_speed: u32,
    update_speed_input: String,
    cell_size: u32,
    cell_size_input: String,
    camera: Pos2,
    camera_x_input: String,
    camera_y_input: String,
    camera_speed: u32,
    camera_speed_input: String,
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
            update_speed: 1,
            update_speed_input: 1.to_string(),
            cell_size: 5,
            cell_size_input: 5.to_string(),
            camera: Pos2::ZERO,
            camera_x_input: 0.to_string(),
            camera_y_input: 0.to_string(),
            camera_speed: 5,
            camera_speed_input: 5.to_string(),
        }
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("control_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Update speed:");
                let update_speed_input_response = ui.add(
                    TextEdit::singleline(&mut self.update_speed_input)
                        .desired_width(0.0)
                        .clip_text(false)
                        .hint_text(self.update_speed.to_string()),
                );
                if update_speed_input_response.changed() {
                    self.update_speed = self
                        .update_speed_input
                        .parse::<u32>()
                        .unwrap_or(self.update_speed);
                }
                ui.label("updates/second");

                ui.add_space(10.0);

                ui.label("Camera speed:");
                let camera_speed_input_response = ui.add(
                    TextEdit::singleline(&mut self.camera_speed_input)
                        .desired_width(0.0)
                        .clip_text(false)
                        .hint_text(self.camera_speed.to_string()),
                );
                if camera_speed_input_response.changed() {
                    self.camera_speed = self
                        .camera_speed_input
                        .parse::<u32>()
                        .unwrap_or(self.camera_speed)
                        .clamp(1, u32::MAX);
                }
                ui.label("px");

                ui.add_space(10.0);

                ui.label("Camera Pos:");
                let camera_x_input_response = ui.add(
                    TextEdit::singleline(&mut self.camera_x_input)
                        .desired_width(0.0)
                        .clip_text(false)
                        .hint_text(self.camera.x.to_string()),
                );
                if camera_x_input_response.changed() {
                    self.camera.x = -self
                        .camera_x_input
                        .parse::<i32>()
                        .unwrap_or(self.camera.x as i32) as f32;
                }
                ui.label(",");
                let camera_y_input_response = ui.add(
                    TextEdit::singleline(&mut self.camera_y_input)
                        .desired_width(0.0)
                        .clip_text(false)
                        .hint_text(self.camera.y.to_string()),
                );
                if camera_y_input_response.changed() {
                    self.camera.y = -self
                        .camera_y_input
                        .parse::<i32>()
                        .unwrap_or(self.camera.y as i32) as f32;
                }

                ui.add_space(10.0);

                ui.label("Cell size:");
                let cell_size_input_response = ui.add(
                    TextEdit::singleline(&mut self.cell_size_input)
                        .desired_width(0.0)
                        .clip_text(false)
                        .hint_text(self.cell_size.to_string()),
                );
                if cell_size_input_response.changed() {
                    self.cell_size = self
                        .cell_size_input
                        .parse::<u32>()
                        .unwrap_or(self.cell_size)
                        .clamp(1, u32::MAX);
                }
                ui.label("px");
            });

            if ui.button("Reset camera").clicked() {
                self.camera.x = 0.0;
                self.camera.y = 0.0;
            }

            ui.horizontal(|ui| {
                ui.label(format!("Generation: {}", self.board.generation()));
                ui.label(format!("Population: {}", self.board.population()));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.ctx().memory(|m| m.focused().is_none()) {
                ui.input(|i| {
                    // We move the camera opposite of what is inputted to make the calculations for moving the cells simpler.
                    if i.key_pressed(egui::Key::ArrowLeft) {
                        self.camera.x += self.camera_speed as f32;
                    } else if i.key_pressed(egui::Key::ArrowRight) {
                        self.camera.x -= self.camera_speed as f32;
                    } else if i.key_pressed(egui::Key::ArrowUp) {
                        self.camera.y += self.camera_speed as f32;
                    } else if i.key_pressed(egui::Key::ArrowDown) {
                        self.camera.y -= self.camera_speed as f32;
                    }
                });
                self.camera_x_input = (-self.camera.x as i32).to_string();
                self.camera_y_input = (-self.camera.y as i32).to_string();
            }

            let cells = self.board.cells();

            for cell in cells {
                let min = Pos2::new(
                    (cell.column * self.cell_size as i32) as f32 + self.camera.x,
                    (cell.row * self.cell_size as i32) as f32 + self.camera.y,
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

        if self.update_speed > 0
            && self.timer >= Duration::from_millis(1000 / self.update_speed as u64)
        {
            self.board.update();
            self.timer = Duration::ZERO;
        }

        ctx.request_repaint();

        self.previous_frame_time = Instant::now();
    }
}
