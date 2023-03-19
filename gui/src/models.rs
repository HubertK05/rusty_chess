use crate::{ChessGui, Assets};

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

impl ToString for ChessPiece {
    fn to_string(&self) -> String {
        let piece_letter = self.piece_type.to_string();
        match self.color {
            Color::White => piece_letter,
            Color::Black => piece_letter.to_lowercase(),
        }
    }
}

impl From<([[&str; 8]; 8], Assets)> for ChessGui {
    fn from(val: ([[&str; 8]; 8], Assets)) -> Self {
        let mut res = Self::new_empty(val.1);
        for rank in (0..8).rev() {
            for file in 0..8 {
                res.board[rank][file] = match val.0[7 - rank][file] {
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
