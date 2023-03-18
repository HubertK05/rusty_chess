pub mod models;
use std::fmt::Display;
use crate::models::{ChessPiece, PieceType, Color};

use eframe::{CreationContext, epaint};
use egui::{Context, Ui, Rect, Vec2, Shape, Sense, InnerResponse, Color32, Stroke, Id, CursorIcon, LayerId, Order, Label};

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
        Color32::from_rgb(245, 235, 155)
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

#[derive(Default, Clone, Debug)]
pub struct ChessGui {
    pub board: [[Option<ChessPiece>; 8]; 8],
}

impl ChessGui {
    pub fn new_game(cc: &CreationContext) -> Self {
        Self::from([
            ["r", "n", "b", "q", "k", "b", "n", "r"],
            ["p", "p", "p", "p", "p", "p", "p", "p"],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            [" ", " ", " ", " ", " ", " ", " ", " "],
            ["P", "P", "P", "P", "P", "P", "P", "P"],
            ["R", "N", "B", "Q", "K", "B", "N", "R"],
        ])
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let id_source = Id::new("piece id");
        let mut source_sq: Option<(usize, usize)> = None;
        let mut drop_sq: Option<(usize, usize)> = None;
        let response = egui::Grid::new("some id").show(ui, |ui| {
            for (rank_idx, rank) in self.board.clone().into_iter().enumerate().rev() {
                ui.columns(8, |uis| {
                    for (file, sq) in rank.clone().into_iter().enumerate() {
                        let ui = &mut uis[file];
                        ui.horizontal(|ui| {
                            let response = board_square(ui, rank_idx + file, |ui| {
                                ui.set_min_size(Vec2::new(60., 60.));
                                ui.set_max_size(Vec2::new(60., 60.));
                                ui.centered_and_justified(|ui| {
                                    let piece_id = id_source.with(rank_idx).with(file);
                                    let response_opt = match sq {
                                        Some(p) => board_piece(ui, piece_id, |ui| { ui.add(Label::new(p.to_string()).sense(Sense::click())); }),
                                        None => { ui.scope(|ui| ui.label("".to_string())); },
                                    };
                                    if ui.memory(|mem| mem.is_being_dragged(piece_id)) {
                                        source_sq = Some((file, rank_idx));
                                    }
                                });
                            }).response;
                            if ui.memory(|mem| mem.is_anything_being_dragged()) && response.hovered() {
                                drop_sq = Some((file, rank_idx));
                            }
                        });
                    }
                });
                ui.end_row();
            }
        }).response;

        if let (Some((drag_file, drag_rank)), Some((drop_file, drop_rank))) = (source_sq, drop_sq) {
            if ui.input(|i| i.pointer.any_released()) {
                println!("{:?} {:?}", source_sq, drop_sq);
                let piece = self.board[drag_rank][drag_file].take();
                self.board[drop_rank][drop_file] = piece;
            }
        }
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

impl From<[[&str; 8]; 8]> for ChessGui {
    fn from(val: [[&str; 8]; 8]) -> Self {
        let mut res = ChessGui::default();
        for rank in (0..8).rev() {
            for file in 0..8 {
                res.board[rank][file] = match val[7 - rank][file] {
                    "P" => Some(ChessPiece { piece_type: PieceType::Pawn, color: Color::White }),
                    "N" => Some(ChessPiece { piece_type: PieceType::Knight, color: Color::White }),
                    "B" => Some(ChessPiece { piece_type: PieceType::Bishop, color: Color::White }),
                    "R" => Some(ChessPiece { piece_type: PieceType::Rook, color: Color::White }),
                    "Q" => Some(ChessPiece { piece_type: PieceType::Queen, color: Color::White }),
                    "K" => Some(ChessPiece { piece_type: PieceType::King, color: Color::White }),
                    "p" => Some(ChessPiece { piece_type: PieceType::Pawn, color: Color::Black }),
                    "n" => Some(ChessPiece { piece_type: PieceType::Knight, color: Color::Black }),
                    "b" => Some(ChessPiece { piece_type: PieceType::Bishop, color: Color::Black }),
                    "r" => Some(ChessPiece { piece_type: PieceType::Rook, color: Color::Black }),
                    "q" => Some(ChessPiece { piece_type: PieceType::Queen, color: Color::Black }),
                    "k" => Some(ChessPiece { piece_type: PieceType::King, color: Color::Black }),
                    _ => None,
                }
            }
        }
        res
    }
}

pub enum BoardError {
    ParseFailure,
}
