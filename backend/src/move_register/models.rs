use crate::{move_generator::models::{Color, PieceType, Square}, board_setup::models::Board};
use std::fmt::{self, Display};

use super::{move_register_move, capture_register_move, en_passant_register_move, promotion_register_move, promotion_capture_register_move, castle_move_register_move};

#[derive(Debug, PartialEq)]
pub enum MoveError {
    OutOfBounds,
    PieceNotFound,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub piece: PieceType,
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn new(piece: PieceType, from: Square, to: Square) -> Self {
        Self { piece, from, to }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Capture {
    pub piece: PieceType,
    pub from: Square,
    pub to: Square,
}

impl Capture {
    pub fn new(piece: PieceType, from: Square, to: Square) -> Self {
        Self { piece, from, to }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CastleMove {
    pub castle_type: CastleType,
}

impl CastleMove {
    pub fn new(castle_type: CastleType) -> Self {
        Self { castle_type }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CastleType {
    WhiteShort,
    WhiteLong,
    BlackShort,
    BlackLong,
}

#[derive(PartialEq, Clone, Copy)]
pub enum CastleLength {
    Short,
    Long,
}

#[derive(Debug, Clone, Copy)]
pub struct EnPassantMove {
    pub from: Square,
    pub to: Square,
}

impl EnPassantMove {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PromotionMove {
    pub from: Square,
    pub to: Square,
    pub to_piece: PromotedPieceType,
    pub color: Color,
}

impl PromotionMove {
    pub fn new(from: Square, to: Square, to_piece: PromotedPieceType, color: Color) -> Self {
        Self {
            from,
            to,
            to_piece,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PromotionCapture {
    pub from: Square,
    pub to: Square,
    pub to_piece: PromotedPieceType,
    pub color: Color,
}

impl PromotionCapture {
    pub fn new(from: Square, to: Square, to_piece: PromotedPieceType, color: Color) -> Self {
        Self {
            from,
            to,
            to_piece,
            color,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PromotedPieceType {
    Queen,
    Knight,
    Bishop,
    Rook,
}

#[derive(PartialEq)]
pub enum MoveType {
    Move,
    Capture,
    EnPassantMove,
    CastleMove,
    PromotionMove,
    PromotionCapture,
}

#[derive(Debug, Clone, Copy)]
pub enum ChessMove {
    Move(Move),
    Capture(Capture),
    CastleMove(CastleMove),
    EnPassantMove(EnPassantMove),
    PromotionMove(PromotionMove),
    PromotionCapture(PromotionCapture),
}

impl ChessMove {
    pub fn register_move(self, board: &mut Board) -> Result<(), MoveError> {
        match self {
            ChessMove::Move(mov) => move_register_move(mov, board),
            ChessMove::Capture(mov) => capture_register_move(mov, board),
            ChessMove::CastleMove(mov) => castle_move_register_move(mov, board),
            ChessMove::EnPassantMove(mov) => en_passant_register_move(mov, board),
            ChessMove::PromotionMove(mov) => promotion_register_move(mov, board),
            ChessMove::PromotionCapture(mov) => promotion_capture_register_move(mov, board),
        }
    }

    pub fn from(&self) -> Square {
        match self {
            ChessMove::Move(mov) => mov.from,
            ChessMove::Capture(mov) => mov.from,
            ChessMove::CastleMove(mov) => {
                match mov.castle_type {
                    CastleType::WhiteShort | CastleType::WhiteLong => Square(4, 0),
                    CastleType::BlackShort | CastleType::BlackLong => Square(4, 7),
                }
            },
            ChessMove::EnPassantMove(mov) => mov.from,
            ChessMove::PromotionMove(mov) => mov.from,
            ChessMove::PromotionCapture(mov) => mov.from,
        }
    }
    
    pub fn to(&self) -> Square {
        match self {
            ChessMove::Move(mov) => mov.to,
            ChessMove::Capture(mov) => mov.to,
            ChessMove::CastleMove(mov) => {
                match mov.castle_type {
                    CastleType::WhiteShort => Square(6, 0),
                    CastleType::WhiteLong => Square(2, 0),
                    CastleType::BlackShort => Square(6, 7),
                    CastleType::BlackLong => Square(2, 7),
                }
            },
            ChessMove::EnPassantMove(mov) => mov.to,
            ChessMove::PromotionMove(mov) => mov.to,
            ChessMove::PromotionCapture(mov) => mov.to,
        }
    }
    
    pub fn move_type(&self) -> MoveType {
        match self {
            ChessMove::Move(_) => MoveType::Move,
            ChessMove::Capture(_) => MoveType::Capture,
            ChessMove::CastleMove(_) => MoveType::CastleMove,
            ChessMove::EnPassantMove(_) => MoveType::EnPassantMove,
            ChessMove::PromotionMove(_) => MoveType::PromotionMove,
            ChessMove::PromotionCapture(_) => MoveType::PromotionCapture,
        }
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessMove::Move(m) => write!(f, "{m}"),
            ChessMove::Capture(m) => write!(f, "{m}"),
            ChessMove::CastleMove(m) => write!(f, "{m}"),
            ChessMove::EnPassantMove(m) => write!(f, "{m}"),
            ChessMove::PromotionMove(m) => write!(f, "{m}"),
            ChessMove::PromotionCapture(m) => write!(f, "{m}"),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_letter = self.piece;
        let to_file_letter = (self.to.0 as u8 + 97) as char;
        let to_rank_number = self.to.1 + 1;
        write!(
            f,
            "{piece_letter}{to_file_letter}{to_rank_number}"
        )
    }
}

impl Display for Capture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_type = self.piece;
        let piece_letter = if piece_type == PieceType::Pawn {
            (self.from.0 as u8 + 97) as char
        } else {
            char::from(self.piece)
        };
        let to_file_letter = (self.to.0 as u8 + 97) as char;
        let to_rank_number = self.to.1 + 1;
        write!(
            f,
            "{piece_letter}x{to_file_letter}{to_rank_number}"
        )
    }
}

impl Display for CastleMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.castle_type {
            CastleType::WhiteLong | CastleType::BlackLong => write!(f, "O-O-O"),
            CastleType::WhiteShort | CastleType::BlackShort => write!(f, "O-O"),
        }
    }
}

impl Display for EnPassantMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_file_letter = (self.from.0 as u8 + 97) as char;
        let to_file_letter = (self.to.0 as u8 + 97) as char;
        let to_rank_number = self.to.1 + 1;
        write!(f, "{from_file_letter}x{to_file_letter}{to_rank_number}")
    }
}

impl Display for PromotionMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to_file_letter = (self.to.0 as u8 + 97) as char;
        let to_rank_number = self.to.1 + 1;
        let piece_letter = &self.to_piece;
        write!(f, "{to_file_letter}{to_rank_number}={piece_letter}")
    }
}

impl Display for PromotionCapture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_file_letter = (self.from.0 as u8 + 97) as char;
        let to_file_letter = (self.to.0 as u8 + 97) as char;
        let to_rank_number = self.to.1 + 1;
        let piece_letter = &self.to_piece;
        write!(
            f,
            "{from_file_letter}x{to_file_letter}{to_rank_number}={piece_letter}"
        )
    }
}

impl Display for PromotedPieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromotedPieceType::Queen => write!(f, "Q"),
            PromotedPieceType::Knight => write!(f, "N"),
            PromotedPieceType::Bishop => write!(f, "B"),
            PromotedPieceType::Rook => write!(f, "R"),
        }
    }
}
