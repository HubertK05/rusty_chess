use eframe::egui;
use egui::Vec2;
use gui::models::{Assets, UserOptions, ChessGui};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540., 540.));
    let assets = Assets::new();
    let user_options = UserOptions::new();
    let _ = eframe::run_native(
        "Rusty Chess",
        native_options,
        Box::new(|_cc| Box::new(ChessGui::new_game(assets, user_options))),
    );
}
