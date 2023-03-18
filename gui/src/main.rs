use eframe::egui;
use egui::{Rect, Vec2, Pos2, Color32, Stroke, Sense, Ui, Window, vec2};
use gui::ChessGui;
// use gui::{DragAndDropDemo, Demo, View};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540., 540.));
    let _ = eframe::run_native("Rusty Chess", native_options, Box::new(|cc| Box::new(ChessGui::new_game(cc))));
}

// #[derive(Default)]
// struct MyEguiApp {
//     demo: DragAndDropDemo,
// }

// impl MyEguiApp {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
//         // Restore app state using cc.storage (requires the "persistence" feature).
//         // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
//         // for e.g. egui::PaintCallback.
//         Self::default()
//     }
// }

// let (rect, _response) = ui.allocate_at_least(self.size, Sense::hover());
//     let color = if i % 2 == 0 {
//         Color32::WHITE
//     } else {
//         Color32::BLACK
//     };
//     ui.painter().rect(
//         rect,
//         0.,
//         color,
//         Stroke::new(0., Color32::WHITE)
//     );

struct BoardSquare {
    size: Vec2,
    rank: usize,
    file: usize,
}

impl Default for BoardSquare {
    fn default() -> Self {
        Self {
            size: Vec2 { x: 30., y: 30. },
            rank: 0,
            file: 0
        }
    }
}

impl BoardSquare {
    fn ui(&mut self, ui: &mut Ui) {
        let (rect, _response) = ui.allocate_at_least(self.size, Sense::hover());
        let color = if self.rank + self.file % 2 == 0 {
            Color32::WHITE
        } else {
            Color32::BLACK
        };
        ui.painter().rect(
            rect,
            0.,
            color,
            Stroke::new(0., Color32::WHITE)
        );
    }
}
