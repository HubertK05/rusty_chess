pub mod models;
pub mod additions;

use additions::{new_bg, paint_max_rect};
use backend::{board_setup::models::FenNotation, move_generator::models::{Moves, Square, MoveRestrictionData, ChessPiece}};
use eframe::epaint::RectShape;
use egui::{
    Color32, CursorIcon, Id, InnerResponse, LayerId, Order, Rect, Sense, Shape, Ui, Vec2, layers::ShapeIdx,
};
use models::{ChessGui, GameState};

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
    let (_rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());
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
                    for _ in 0..8 {
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
                let mut board_iter: Vec<(usize, [Option<ChessPiece>; 8])> = state.board.board.clone().into_iter().enumerate().collect();
                if state.reversed {
                    board_iter.reverse();
                };

                for (rank_idx, rank) in board_iter.into_iter().rev() {
                    ui.columns(8, |uis| {
                        let mut rank_iter = rank.clone();
                        if state.reversed {
                            rank_iter.reverse();
                        };
                        for (file, sq) in rank_iter.into_iter().enumerate() {
                            let file_idx = if state.reversed {
                                7 - file
                            } else {
                                file
                            };

                            let ui = &mut uis[file];
                            let response = board_square(ui, |ui| {
                                ui.set_min_size(Vec2::new(60., 60.));
                                ui.set_max_size(Vec2::new(60., 60.));
                                paint_max_rect(ui, shapes[rank_idx][file_idx], Color32::TRANSPARENT);
                                rects[rank_idx][file_idx] = ui.max_rect();
                                ui.centered_and_justified(|ui| {
                                    let piece_id = id_source.with(rank_idx).with(file);
                                    match sq {
                                        Some(p) => board_piece(ui, piece_id, |ui| {
                                            state.assets.display_piece(ui, p.piece_type(), p.color());
                                        }),
                                        None => (),
                                    };
                                    if ui.memory(|mem| mem.is_being_dragged(piece_id)) {
                                        source_sq = Some((file_idx, rank_idx));
                                    }
                                });
                            })
                            .response;
                            if ui.memory(|mem| mem.is_anything_being_dragged())
                                && response.hovered()
                            {
                                drop_sq = Some((file_idx, rank_idx));
                            }
                        }
                    });
                    ui.end_row();
                }
                
                let legal_destinations = source_sq.map(|sq| state.legal_moves.search_with_from(Square(sq.0 as i8, sq.1 as i8)));

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
            if let Some(chosen_move) = state.legal_moves.find(Square(drag_file as i8, drag_rank as i8), Square(drop_file as i8, drop_rank as i8)) {
                if let Err(_) = state.board.register_move(chosen_move) {
                    println!("oops, couldn't register the move");
                };
                state.gen_legal_moves_from_pos(state.board.turn);
                let position_count = *state.repetition_map.entry(FenNotation::from(&state.board).to_draw_fen()).and_modify(|x| *x += 1).or_insert(1);
                if state.board.half_move_timer_50 > 100 {
                    state.game_state = GameState::Done("Draw by the 50 move rule".into());
                }
                else if state.board.mating_material.0 < 3 && state.board.mating_material.1 < 3 {
                    state.game_state = GameState::Done("Draw by insufficitent mating material".into());
                }
                else if state.legal_moves.0.is_empty() {
                    if MoveRestrictionData::get(&state.board, state.board.turn).check_squares.checks_amount != 0 {
                        state.game_state = GameState::Done(format!("{} wins by checkmate", state.board.turn.opp().to_string()));
                    } else {
                        state.game_state = GameState::Done("Draw by stalemate".into());
                    }
                }
                else if position_count >= 3 {
                    state.game_state = GameState::Done("Draw by threefold repetition".into());
                }
            }
        }
    }

    paint_max_rect(ui, bg, Color32::from_rgb(128, 88, 19))
}
