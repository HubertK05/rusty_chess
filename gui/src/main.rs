use eframe::egui;
use egui::Vec2;
use gui::models::{Assets, ChessGui};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(768., 616.));
    native_options.resizable = false;
    let assets = Assets::new();
    let _ = eframe::run_native(
        "Rusty Chess",
        native_options,
        Box::new(|_cc| Box::new(ChessGui::new_game(assets))),
    );
}
