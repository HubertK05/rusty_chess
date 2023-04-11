use std::collections::BTreeMap;
use std::thread::JoinHandle;
use std::env;

use backend::{board_setup::models::{Board, FenNotation}, move_generator::models::{PieceType, Color, Moves}, move_register::models::ChessMove};
use egui::{Ui, ColorImage, Vec2, Color32, Button};
use egui_extras::RetainedImage;

use crate::{additions::{new_bg, paint_max_rect}, chess_ui};

pub struct Square(pub usize, pub usize);

impl ToString for Square {
    fn to_string(&self) -> String {
        format!("{}{}", ((self.0 + 97) as u8) as char, self.1 + 1)
    }
}

pub struct Assets {
    pub wb: RetainedImage,
    pub wk: RetainedImage,
    pub wn: RetainedImage,
    pub wp: RetainedImage,
    pub wq: RetainedImage,
    pub wr: RetainedImage,
    pub bb: RetainedImage,
    pub bk: RetainedImage,
    pub bn: RetainedImage,
    pub bp: RetainedImage,
    pub bq: RetainedImage,
    pub br: RetainedImage,
}

impl Assets {
    pub fn new() -> Self {
        let exe_path = env::current_exe().unwrap();
        // let dir_path = exe_path.parent().unwrap().as_os_str().to_str().unwrap();
        let dir_path = "src";
        Self {
            wp: load_img(&(dir_path.to_string() + "/assets/wp.png"), "white-pawn"),
            bp: load_img(&(dir_path.to_string() + "/assets/bp.png"), "black-pawn"),
            wn: load_img(&(dir_path.to_string() + "/assets/wn.png"), "white-knight"),
            bn: load_img(&(dir_path.to_string() + "/assets/bn.png"), "black-knight"),
            wb: load_img(&(dir_path.to_string() + "/assets/wb.png"), "white-bishop"),
            bb: load_img(&(dir_path.to_string() + "/assets/bb.png"), "black-bishop"),
            wr: load_img(&(dir_path.to_string() + "/assets/wr.png"), "white-rook"),
            br: load_img(&(dir_path.to_string() + "/assets/br.png"), "black-rook"),
            wq: load_img(&(dir_path.to_string() + "/assets/wq.png"), "white-queen"),
            bq: load_img(&(dir_path.to_string() + "/assets/bq.png"), "black-queen"),
            wk: load_img(&(dir_path.to_string() + "/assets/wk.png"), "white-king"),
            bk: load_img(&(dir_path.to_string() + "/assets/bk.png"), "black-king"),
        }
    }

    pub fn display_piece(&self, ui: &mut Ui, piece_type: PieceType, color: Color) {
        match (piece_type, color) {
            (PieceType::Pawn, Color::White) => self.wp.show(ui),
            (PieceType::Pawn, Color::Black) => self.bp.show(ui),
            (PieceType::Knight, Color::White) => self.wn.show(ui),
            (PieceType::Knight, Color::Black) => self.bn.show(ui),
            (PieceType::Bishop, Color::White) => self.wb.show(ui),
            (PieceType::Bishop, Color::Black) => self.bb.show(ui),
            (PieceType::Rook, Color::White) => self.wr.show(ui),
            (PieceType::Rook, Color::Black) => self.br.show(ui),
            (PieceType::Queen, Color::White) => self.wq.show(ui),
            (PieceType::Queen, Color::Black) => self.bq.show(ui),
            (PieceType::King, Color::White) => self.wk.show(ui),
            (PieceType::King, Color::Black) => self.bk.show(ui),
        };
    }
}

pub struct ChessGui {
    pub board: Board,
    pub legal_moves: Moves,
    pub game_state: GameState,
    pub repetition_map: BTreeMap<u64, u8>,
    pub reversed: bool,
    pub assets: Assets,
    pub bot_thread: Option<JoinHandle<ChessMove>>,
    pub bot_settings: (bool, bool)
}

impl ChessGui {
    pub fn new_game(assets: Assets) -> Self {
        let board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()))
        .unwrap();
        let legal_moves = Moves::get_all_moves(&board, Color::White);
        Self {
            board,
            legal_moves,
            game_state: GameState::Ongoing,
            repetition_map: BTreeMap::from([(board.hash_board(), 1)]),
            reversed: false,
            assets,
            bot_thread: None,
            bot_settings: (false, false)
        }
    }

    pub fn new_empty(assets: Assets) -> Self {
        Self {
            board: Board::try_from(FenNotation("8/8/8/8/8/8/8/8 w - - 0 1".to_string())).unwrap(),
            legal_moves: Moves(Vec::new()),
            game_state: GameState::Ongoing,
            repetition_map: BTreeMap::new(),
            reversed: false,
            assets,
            bot_thread: None,
            bot_settings: (false, false)
        }
    }

    pub fn reset_game(&mut self) {
        self.bot_thread = None;
        self.board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).unwrap();
        self.legal_moves = Moves::get_all_moves(&self.board, Color::White);
        self.game_state = GameState::Ongoing;
        self.repetition_map = BTreeMap::from([(self.board.hash_board(), 1)]);
    }

    pub fn reverse_view(&mut self) {
        self.reversed = !self.reversed
    }

    pub fn gen_legal_moves_from_pos(&mut self, color: Color) {
        self.legal_moves = Moves::get_all_moves(&self.board, color);
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let bg = new_bg(ui);
        
        ui.allocate_ui(Vec2::splat(0.), |ui| {
            ui.horizontal(|ui| {
                ui.add_space(15.5);
                ui.set_max_height(600.);
                ui.vertical(|ui| {
                    ui.add_space(40.);
                    ui.allocate_ui(Vec2::splat(0.), |ui| {
                        chess_ui(self, ui);
                    });
                    ui.add_space(40.);
                });
                ui.add_space(4.);
                ui.vertical(|ui| {
                    ui.allocate_ui(Vec2::new(200., 600.), |ui| {
                        ui.add_space(73.5);
                        if ui.add_sized(Vec2::new(200., 72.5), Button::new("Reset game")).clicked() {
                            self.reset_game();
                        }
                        if ui.add_sized(Vec2::new(200., 72.5), Button::new("Reverse view")).clicked() {
                            self.reverse_view();
                        }
                        self.game_state.show(ui, self.board.turn);
                        let button_text_bot_white = if self.bot_settings.0 {
                            "Toggle bot for White (on)"
                        } else {
                            "Toggle bot for White (off)"
                        };
                        let button_text_bot_black = if self.bot_settings.1 {
                            "Toggle bot for Black (on)"
                        } else {
                            "Toggle bot for Black (off)"
                        };
                        if ui.add_sized(Vec2::new(200., 72.5), Button::new(button_text_bot_white)).clicked() {
                            self.bot_settings.0 = !self.bot_settings.0;
                        };
                        if ui.add_sized(Vec2::new(200., 72.5), Button::new(button_text_bot_black)).clicked() {
                            self.bot_settings.1 = !self.bot_settings.1;
                        };
                        ui.add_space(73.5);
                    });
                });
                ui.add_space(12.5);
            });
        });
        paint_max_rect(ui, bg, Color32::from_rgb(32, 20, 0));
    }

    pub fn get_bot_settings(&self, color: Color) -> bool {
        match color {
            Color::White => self.bot_settings.0,
            Color::Black => self.bot_settings.1,
        }
    }
}

impl eframe::App for ChessGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(3., 3.);
            ui.scope(|ui| {
                ui.set_max_size(Vec2::splat(600.));
                self.ui(ui);
            });
        });
        ctx.request_repaint();
    }
}

#[derive(PartialEq)]
pub enum GameState {
    Ongoing,
    Done(String),
}

impl GameState {
    fn show(&self, ui: &mut Ui, color: Color) {
        ui.allocate_ui(Vec2::new(200., 148.), |ui| {
            match self {
                GameState::Ongoing => {
                    let bg = new_bg(ui);
                    ui.centered_and_justified(|ui| {
                        match color {
                            Color::White => {
                                ui.label("White's turn");
                                paint_max_rect(ui, bg, Color32::WHITE)
                            },
                            Color::Black => {
                                ui.label("Black's turn");
                                paint_max_rect(ui, bg, Color32::BLACK)
                            },
                        };
                    });
                },
                GameState::Done(msg) => {
                    ui.centered_and_justified(|ui| {
                        ui.label(msg);
                    });
                },
            }
        });
    }

    pub fn is_ongoing(&self) -> bool {
        *self == GameState::Ongoing
    }

    pub fn is_done(&self) -> bool {
        *self != GameState::Ongoing
    }
}

fn load_img(path: &str, name: &str) -> RetainedImage {
    let img = image::open(path).unwrap().to_rgba8();
    let pixels = img.pixels().map(|x| {
        [x.0[0], x.0[1], x.0[2], x.0[3]].into_iter()
    }).flatten().collect::<Vec<u8>>();
    RetainedImage::from_color_image(name, ColorImage::from_rgba_unmultiplied([60, 60], &pixels))
}
