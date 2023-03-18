// /// Something to view in the demo windows
// pub trait View {
//     fn ui(&mut self, ui: &mut egui::Ui);
// }

// /// Something to view
// pub trait Demo {
//     /// `&'static` so we can also use it as a key to store open/close state.
//     fn name(&self) -> &'static str;

//     /// Show windows, etc
//     fn show(&mut self, ctx: &egui::Context, open: &mut bool);
// }

// use egui::*;

// pub fn drag_source(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
//     let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(id));

//     if !is_being_dragged {
//         let response = ui.scope(body).response;

//         // Check for drags:
//         let response = ui.interact(response.rect, id, Sense::drag());
//         if response.hovered() {
//             ui.ctx().set_cursor_icon(CursorIcon::Grab);
//         }
//     } else {
//         ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

//         // Paint the body to a new layer:
//         let layer_id = LayerId::new(Order::Tooltip, id);
//         let response = ui.with_layer_id(layer_id, body).response;

//         // Now we move the visuals of the body to where the mouse is.
//         // Normally you need to decide a location for a widget first,
//         // because otherwise that widget cannot interact with the mouse.
//         // However, a dragged component cannot be interacted with anyway
//         // (anything with `Order::Tooltip` always gets an empty [`Response`])
//         // So this is fine!

//         if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
//             let delta = pointer_pos - response.rect.center();
//             ui.ctx().translate_layer(layer_id, delta);
//         }
//     }
// }

// pub fn drop_target<R>(
//     ui: &mut Ui,
//     can_accept_what_is_being_dragged: bool,
//     body: impl FnOnce(&mut Ui) -> R,
// ) -> InnerResponse<R> {
//     let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());

//     let margin = Vec2::splat(4.0);

//     let outer_rect_bounds = ui.available_rect_before_wrap();
//     let inner_rect = outer_rect_bounds.shrink2(margin);
//     let where_to_put_background = ui.painter().add(Shape::Noop);
//     let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
//     let ret = body(&mut content_ui);
//     let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
//     let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

//     let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
//         ui.visuals().widgets.active
//     } else {
//         ui.visuals().widgets.inactive
//     };

//     let mut fill = style.bg_fill;
//     let mut stroke = style.bg_stroke;
//     if is_being_dragged && !can_accept_what_is_being_dragged {
//         fill = ui.visuals().gray_out(fill);
//         stroke.color = ui.visuals().gray_out(stroke.color);
//     }

//     ui.painter().set(
//         where_to_put_background,
//         epaint::RectShape {
//             rounding: style.rounding,
//             fill,
//             stroke,
//             rect,
//         },
//     );

//     InnerResponse::new(ret, response)
// }

// #[derive(Clone, PartialEq, Eq)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// pub struct DragAndDropDemo {
//     /// columns with items
//     columns: Vec<Vec<String>>,
// }

// impl Default for DragAndDropDemo {
//     fn default() -> Self {
//         Self {
//             columns: vec![
//                 vec!["Item A", "Item B", "Item C"],
//                 vec!["Item D", "Item E"],
//                 vec!["Item F", "Item G", "Item H"],
//             ]
//             .into_iter()
//             .map(|v| v.into_iter().map(ToString::to_string).collect())
//             .collect(),
//         }
//     }
// }

// impl Demo for DragAndDropDemo {
//     fn name(&self) -> &'static str {
//         "✋ Drag and Drop"
//     }

//     fn show(&mut self, ctx: &Context, open: &mut bool) {
//         use View as _;
//         Window::new(self.name())
//             .open(open)
//             .default_size(vec2(256.0, 256.0))
//             .vscroll(false)
//             .resizable(false)
//             .show(ctx, |ui| self.ui(ui));
//     }
// }

// impl View for DragAndDropDemo {
//     fn ui(&mut self, ui: &mut Ui) {
//         ui.label("This is a proof-of-concept of drag-and-drop in egui.");
//         ui.label("Drag items between columns.");

//         let id_source = "my_drag_and_drop_demo";
//         let mut source_col_row = None;
//         let mut drop_col = None;
//         ui.columns(self.columns.len(), |uis| {
//             for (col_idx, column) in self.columns.clone().into_iter().enumerate() {
//                 let ui = &mut uis[col_idx];
//                 let can_accept_what_is_being_dragged = true; // We accept anything being dragged (for now) ¯\_(ツ)_/¯
//                 let response = drop_target(ui, can_accept_what_is_being_dragged, |ui| {
//                     ui.set_min_size(vec2(64.0, 100.0));
//                     for (row_idx, item) in column.iter().enumerate() {
//                         let item_id = Id::new(id_source).with(col_idx).with(row_idx);
//                         drag_source(ui, item_id, |ui| {
//                             let response = ui.add(Label::new(item).sense(Sense::click()));
//                             response.context_menu(|ui| {
//                                 if ui.button("Remove").clicked() {
//                                     self.columns[col_idx].remove(row_idx);
//                                     ui.close_menu();
//                                 }
//                             });
//                         });

//                         if ui.memory(|mem| mem.is_being_dragged(item_id)) {
//                             source_col_row = Some((col_idx, row_idx));
//                         }
//                     }
//                 })
//                 .response;

//                 let response = response.context_menu(|ui| {
//                     if ui.button("New Item").clicked() {
//                         self.columns[col_idx].push("New Item".to_owned());
//                         ui.close_menu();
//                     }
//                 });

//                 let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());
//                 if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
//                     drop_col = Some(col_idx);
//                 }
//             }
//         });

//         if let Some((source_col, source_row)) = source_col_row {
//             if let Some(drop_col) = drop_col {
//                 if ui.input(|i| i.pointer.any_released()) {
//                     // do the drop:
//                     let item = self.columns[source_col].remove(source_row);
//                     self.columns[drop_col].push(item);
//                 }
//             }
//         }
//     }
// }

use std::fmt::Display;

use eframe::{CreationContext, epaint};
use egui::{Context, Ui, Rect, Vec2, Shape, Sense, InnerResponse, Color32, Stroke};

fn board_square<R>(ui: &mut Ui, idx: usize, body: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let margin = Vec2::splat(0.);
    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let background = ui.painter().add(Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());
    let style = ui.visuals().widgets.inactive;

    let color = if idx % 2 == 0 {
        Color32::from_rgb(78, 44, 0)
    } else {
        Color32::from_rgb(240, 245, 152)
    };

    ui.painter().set(
        background,
        epaint::RectShape {
            rounding: style.rounding,
            fill: color,
            stroke: style.bg_stroke,
            rect,
        },
    );

    InnerResponse::new(ret, response)
}

#[derive(Default, Clone)]
pub struct ChessGui {
    pub board: [Option<String>; 8],
}

impl ChessGui {
    pub fn new(cc: &CreationContext) -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new("some id").show(ui, |ui| {
            for rank in (0..8).rev() {
                ui.columns(8, |uis| {
                    for (file, sq) in self.board.clone().into_iter().enumerate() {
                        let ui = &mut uis[file];
                        ui.horizontal(|ui| {
                            board_square(ui, rank + file, |ui| {
                                ui.set_min_size(Vec2::new(60., 60.));
                                ui.set_max_size(Vec2::new(60., 60.));
                                ui.centered_and_justified(|ui| {
                                    ui.label(Square(file, rank).to_string());
                                });
                            });
                        });
                    }
                });
                ui.end_row();
            }
        });
    }
}

impl eframe::App for ChessGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(3., 3.);
            self.ui(ui);
        });
    }
}

pub struct Square(pub usize, pub usize);

impl ToString for Square {
    fn to_string(&self) -> String {
        format!("{}{}", ((self.0 + 97) as u8) as char, self.1 + 1)
    }
}
