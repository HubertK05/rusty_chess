use egui::{Ui, ColorImage, Vec2};
use egui_extras::RetainedImage;

pub struct Square(pub usize, pub usize);

impl ToString for Square {
    fn to_string(&self) -> String {
        format!("{}{}", ((self.0 + 97) as u8) as char, self.1 + 1)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PieceType {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ToString for PieceType {
    fn to_string(&self) -> String {
        match self {
            PieceType::Pawn => "P".to_string(),
            PieceType::Knight => "N".to_string(),
            PieceType::Bishop => "B".to_string(),
            PieceType::Rook => "R".to_string(),
            PieceType::Queen => "Q".to_string(),
            PieceType::King => "K".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl ChessPiece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }
}

impl ToString for ChessPiece {
    fn to_string(&self) -> String {
        let piece_letter = self.piece_type.to_string();
        match self.color {
            Color::White => piece_letter,
            Color::Black => piece_letter.to_lowercase(),
        }
    }
}

impl From<[[&str; 8]; 8]> for Board {
    fn from(val: [[&str; 8]; 8]) -> Self {
        let mut res = Self::new_empty();
        for rank in (0..8).rev() {
            for file in 0..8 {
                res.board[rank][file] = match val[7 - rank][file] {
                    "P" => Some(ChessPiece::new(PieceType::Pawn, Color::White)),
                    "N" => Some(ChessPiece::new(PieceType::Knight, Color::White)),
                    "B" => Some(ChessPiece::new(PieceType::Bishop, Color::White)),
                    "R" => Some(ChessPiece::new(PieceType::Rook, Color::White)),
                    "Q" => Some(ChessPiece::new(PieceType::Queen, Color::White)),
                    "K" => Some(ChessPiece::new(PieceType::King, Color::White)),
                    "p" => Some(ChessPiece::new(PieceType::Pawn, Color::Black)),
                    "n" => Some(ChessPiece::new(PieceType::Knight, Color::Black)),
                    "b" => Some(ChessPiece::new(PieceType::Bishop, Color::Black)),
                    "r" => Some(ChessPiece::new(PieceType::Rook, Color::Black)),
                    "q" => Some(ChessPiece::new(PieceType::Queen, Color::Black)),
                    "k" => Some(ChessPiece::new(PieceType::King, Color::Black)),
                    _ => None,
                }
            }
        }
        res
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
        Self {
            wp: load_img("src/assets/wp.png", "white-pawn"),
            bp: load_img("src/assets/bp.png", "black-pawn"),
            wn: load_img("src/assets/wn.png", "white-knight"),
            bn: load_img("src/assets/bn.png", "black-knight"),
            wb: load_img("src/assets/wb.png", "white-bishop"),
            bb: load_img("src/assets/bb.png", "black-bishop"),
            wr: load_img("src/assets/wr.png", "white-rook"),
            br: load_img("src/assets/br.png", "black-rook"),
            wq: load_img("src/assets/wq.png", "white-queen"),
            bq: load_img("src/assets/bq.png", "black-queen"),
            wk: load_img("src/assets/wk.png", "white-king"),
            bk: load_img("src/assets/bk.png", "black-king"),
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
    pub assets: Assets,
    pub options: UserOptions,
}

impl ChessGui {
    pub fn new_game(assets: Assets, options: UserOptions) -> Self {
        Self {
            board: Board::from([
                ["r", "n", "b", "q", "k", "b", "n", "r"],
                ["p", "p", "p", "p", "p", "p", "p", "p"],
                [" ", " ", " ", " ", " ", " ", " ", " "],
                [" ", " ", " ", " ", " ", " ", " ", " "],
                [" ", " ", " ", " ", " ", " ", " ", " "],
                [" ", " ", " ", " ", " ", " ", " ", " "],
                ["P", "P", "P", "P", "P", "P", "P", "P"],
                ["R", "N", "B", "Q", "K", "B", "N", "R"],
            ]),
            assets,
            options,
        }
    }

    pub fn new_empty(assets: Assets, options: UserOptions) -> Self {
        Self { board: Board { board: [[None; 8]; 8] }, assets, options }
    }
}

impl eframe::App for ChessGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(3., 3.);
            self.board.ui(ui, &self.assets);
        });
    }
}

pub struct UserOptions;

impl UserOptions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        todo!()
    }
}

#[derive(Default)]
pub struct Board {
    pub board: [[Option<ChessPiece>; 8]; 8],
}

fn load_img(path: &str, name: &str) -> RetainedImage {
    let img = image::open(path).unwrap().to_rgba8();
    let pixels = img.pixels().map(|x| {
        [x.0[0], x.0[1], x.0[2], x.0[3]].into_iter()
    }).flatten().collect::<Vec<u8>>();
    RetainedImage::from_color_image(name, ColorImage::from_rgba_unmultiplied([60, 60], &pixels))
}
