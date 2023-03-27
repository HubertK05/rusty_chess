pub mod models;
pub mod additions;

use additions::{new_bg, paint_max_rect};
use backend::{board_setup::models::Board, move_generator::models::{Color, Moves, Square}};
use eframe::epaint::{self, RectShape};
use egui::{
    Color32, CursorIcon, Id, InnerResponse, LayerId, Order, Rect, Sense, Shape, Ui, Vec2, layers::ShapeIdx,
};
use models::{Assets, ChessGui};

fn board_piece(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
    let is_dragged = ui.memory(|mem| mem.is_being_dragged(id));
    if !is_dragged {
        let response = ui.scope(body).response;
        let response = ui.interact(response.rect, id, Sense::drag());
        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }
    } else {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);

        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }
}

fn board_square<R>(ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let margin = Vec2::splat(0.);
    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());
    InnerResponse::new(ret, response)
}

fn chess_ui(state: &mut ChessGui, ui: &mut Ui) {
    let id_source = Id::new("piece id");
    let mut source_sq: Option<(usize, usize)> = None;
    let mut drop_sq: Option<(usize, usize)> = None;
    let bg = new_bg(ui);
    ui.allocate_ui(Vec2::splat(501.), |ui| {
        let (outer_rect, _) = ui.allocate_exact_size(Vec2::splat(517.), Sense::hover());
        let inner_rect = outer_rect.shrink2(Vec2::splat(8.));
        ui.allocate_ui_at_rect(inner_rect, |ui| {
            egui::Grid::new("some id")
            .show(ui, |ui| {
                let mut shapes: Vec<Vec<ShapeIdx>> = Vec::new();
                let mut rects: Vec<Vec<Rect>> = Vec::new();
                for i in 0..8 {
                    shapes.push(Vec::new());
                    rects.push(Vec::new());
                    for j in 0..8 {
                        let idx = ui.painter().add(Shape::Rect(RectShape {
                            rect: Rect::NOTHING,
                            rounding: ui.visuals().widgets.inactive.rounding,
                            fill: Color32::TRANSPARENT,
                            stroke: ui.visuals().widgets.inactive.bg_stroke,
                        }));
                        shapes[i].push(idx);
                        rects[i].push(Rect::NOTHING);
                    }
                }
                for (rank_idx, rank) in state.board.board.clone().into_iter().enumerate().rev() {
                    ui.columns(8, |uis| {
                        for (file, sq) in rank.clone().into_iter().enumerate() {
                            let ui = &mut uis[file];
                            let response = board_square(ui, |ui| {
                                ui.set_min_size(Vec2::new(60., 60.));
                                ui.set_max_size(Vec2::new(60., 60.));
                                paint_max_rect(ui, shapes[rank_idx][file], Color32::TRANSPARENT);
                                rects[rank_idx][file] = ui.max_rect();
                                ui.centered_and_justified(|ui| {
                                    let piece_id = id_source.with(rank_idx).with(file);
                                    match sq {
                                        Some(p) => board_piece(ui, piece_id, |ui| {
                                            state.assets.display_piece(ui, p.piece_type(), p.color());
                                        }),
                                        None => (),
                                    };
                                    if ui.memory(|mem| mem.is_being_dragged(piece_id)) {
                                        source_sq = Some((file, rank_idx));
                                    }
                                });
                            })
                            .response;
                            if ui.memory(|mem| mem.is_anything_being_dragged())
                                && response.hovered()
                            {
                                drop_sq = Some((file, rank_idx));
                            }
                        }
                    });
                    ui.end_row();
                }
                let legal_destinations = source_sq.map(|sq| state.legal_moves.search_with_from(Square(sq.0 as i8, sq.1 as i8)));
                // println!("{:?}", source_sq, /*legal_destinations*/);
                for rank in 0..8 {
                    for file in 0..8 {
                        let color = {
                            if let Some(destinations) = &legal_destinations {
                                let Moves(filtered_moves) = destinations.search_with_to(Square(file as i8, rank as i8));
                                if filtered_moves.len() != 0 {
                                    Color32::from_rgb(128, 0, 0)
                                } else if (file + rank) % 2 == 0 {
                                    Color32::from_rgb(78, 44, 0)
                                } else {
                                    Color32::from_rgb(245, 235, 155)
                                }
                            } else if (file + rank) % 2 == 0 {
                                Color32::from_rgb(78, 44, 0)
                            } else {
                                Color32::from_rgb(245, 235, 155)
                            }
                        };
                        // println!("{:?}", shapes[rank][file].fill);
                        ui.painter().set(shapes[rank][file], RectShape {
                            rect: rects[rank][file],
                            rounding: ui.visuals().widgets.inactive.rounding,
                            fill: color,
                            stroke: ui.visuals().widgets.inactive.bg_stroke,
                        });
                    }
                }
            });
        });
    });

    if let (Some((drag_file, drag_rank)), Some((drop_file, drop_rank))) = (source_sq, drop_sq) {
        if ui.input(|i| i.pointer.any_released()) {
            // let piece = state.board.board[drag_rank][drag_file].take();
            // state.board.board[drop_rank][drop_file] = piece;
            if let Some(chosen_move) = state.legal_moves.find(Square(drag_file as i8, drag_rank as i8), Square(drop_file as i8, drop_rank as i8)) {
                if let Err(_) = state.board.register_move(&*chosen_move) {
                    println!("oops, couldn't register the move");
                };
                state.gen_legal_moves_from_pos(state.board.turn);
            }
        }
    }

    // paint_max_rect(ui, bg, Color32::from_rgb(128, 88, 19));
}
