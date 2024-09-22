use eframe::{
    egui::{Style, Vec2, ViewportBuilder, Visuals},
    NativeOptions,
};
use game::Game;

mod board;
mod game;

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(Vec2::new(750.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Conway's Game of Life- egui",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::light(),
                ..Default::default()
            });

            Ok(Box::new(Game::default()))
        }),
    )
}
