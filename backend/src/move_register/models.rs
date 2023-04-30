use crate::{
    board_setup::models::Board,
    move_generator::models::{PieceType, Square},
};
use std::fmt::{self, Display};

use super::{
    capture_register_move, castle_move_register_move, en_passant_register_move, move_register_move,
    promotion_capture_register_move, promotion_register_move,
};

#[derive(Debug, PartialEq)]
pub enum MoveError {
    OutOfBounds,
    PieceNotFound,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CastleType {
    WhiteShort,
    WhiteLong,
    BlackShort,
    BlackLong,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PromotedPieceType {
    Queen,
    Knight,
    Bishop,
    Rook,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveType {
    Move(PieceType),
    Capture(PieceType),
    EnPassantMove,
    CastleMove(CastleType),
    PromotionMove(PromotedPieceType),
    PromotionCapture(PromotedPieceType),
}

#[derive(PartialEq, Clone, Copy)]
pub enum RawMoveType {
    Move,
    Capture,
    EnPassantMove,
    CastleMove,
    PromotionMove,
    PromotionCapture,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChessMove {
    pub move_type: MoveType,
    pub from: Square,
    pub to: Square,
}

impl ChessMove {
    pub fn register_move(self, board: &mut Board) -> Result<(), MoveError> {
        match self.move_type {
            MoveType::Move(_) => move_register_move(self.from, self.to, board),
            MoveType::Capture(_) => capture_register_move(self.from, self.to, board),
            MoveType::CastleMove(castle_type) => castle_move_register_move(castle_type, board),
            MoveType::EnPassantMove => en_passant_register_move(self.from, self.to, board),
            MoveType::PromotionMove(to_piece) => {
                promotion_register_move(self.from, self.to, to_piece, board)
            }
            MoveType::PromotionCapture(to_piece) => {
                promotion_capture_register_move(self.from, self.to, to_piece, board)
            }
        }
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.move_type {
            MoveType::Move(piece) => {
                let piece_letter = piece;
                let to_file_letter = (self.to.0 as u8 + 97) as char;
                let to_rank_number = self.to.1 + 1;
                write!(f, "{piece_letter}{to_file_letter}{to_rank_number}")
            }
            MoveType::Capture(piece) => {
                let piece_type = piece;
                let piece_letter = if piece_type == PieceType::Pawn {
                    (self.from.0 as u8 + 97) as char
                } else {
                    char::from(piece)
                };
                let to_file_letter = (self.to.0 as u8 + 97) as char;
                let to_rank_number = self.to.1 + 1;
                write!(f, "{piece_letter}x{to_file_letter}{to_rank_number}")
            }
            MoveType::CastleMove(_) => match self.to.0 {
                2 => write!(f, "O-O-O"),
                6 => write!(f, "O-O"),
                _ => unreachable!(),
            },
            MoveType::EnPassantMove => {
                let from_file_letter = (self.from.0 as u8 + 97) as char;
                let to_file_letter = (self.to.0 as u8 + 97) as char;
                let to_rank_number = self.to.1 + 1;
                write!(f, "{from_file_letter}x{to_file_letter}{to_rank_number}")
            }
            MoveType::PromotionMove(piece) => {
                let to_file_letter = (self.to.0 as u8 + 97) as char;
                let to_rank_number = self.to.1 + 1;
                let piece_letter = piece;
                write!(f, "{to_file_letter}{to_rank_number}={piece_letter}")
            }
            MoveType::PromotionCapture(piece) => {
                let from_file_letter = (self.from.0 as u8 + 97) as char;
                let to_file_letter = (self.to.0 as u8 + 97) as char;
                let to_rank_number = self.to.1 + 1;
                let piece_letter = piece;
                write!(
                    f,
                    "{from_file_letter}x{to_file_letter}{to_rank_number}={piece_letter}"
                )
            }
        }
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
