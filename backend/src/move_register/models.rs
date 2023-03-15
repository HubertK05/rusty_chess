use std::fmt::{Display, self};

use dyn_clone::DynClone;

use crate::{move_generator::models::{PieceType, Color, Square}, board_setup::models::Board};

#[derive(Debug, PartialEq)]
pub enum MoveError {
    OutOfBounds,
    MoveBlocked,
    MoveFilteredOut,
    PieceNotFound,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub piece: PieceType,
    pub from: Square,
    pub to: Square,
}

impl Move {
    pub fn new(piece: PieceType, from: Square, to: Square) -> Self {
        Self {
            piece,
            from,
            to,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Capture {
    pub piece: PieceType,
    pub from: Square,
    pub to: Square,
}

impl Capture {
    pub fn new(piece: PieceType, from: Square, to: Square) -> Self {
        Self {
            piece,
            from,
            to,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CastleMove {
    pub castle_type: CastleType,
}

impl CastleMove {
    pub fn new(castle_type: CastleType) -> Self {
        Self {
            castle_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, Clone)]
pub struct EnPassantMove {
    pub from: Square,
    pub to: Square,
}

impl EnPassantMove {
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub trait ChessMove: std::fmt::Debug + std::fmt::Display + DynClone {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError>;
    fn is_double_pawn_move(&self) -> bool { false }
    fn from(&self) -> Square;
    fn to(&self) -> Square;
    fn move_type(&self) -> MoveType;
}

dyn_clone::clone_trait_object!(ChessMove);

impl ChessMove for Move {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        todo!()
    }

    fn from(&self) -> Square {
        todo!()
    }

    fn to(&self) -> Square {
        todo!()
    }

    fn move_type(&self) -> MoveType {
        todo!()
    }
}

impl ChessMove for Capture {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        todo!()
    }

    fn from(&self) -> Square {
        todo!()
    }

    fn to(&self) -> Square {
        todo!()
    }

    fn move_type(&self) -> MoveType {
        todo!()
    }
}

impl ChessMove for EnPassantMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        todo!()
    }

    fn from(&self) -> Square {
        todo!()
    }

    fn to(&self) -> Square {
        todo!()
    }

    fn move_type(&self) -> MoveType {
        todo!()
    }
}

impl ChessMove for PromotionMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        todo!()
    }

    fn from(&self) -> Square {
        todo!()
    }

    fn to(&self) -> Square {
        todo!()
    }

    fn move_type(&self) -> MoveType {
        todo!()
    }
}

impl ChessMove for PromotionCapture {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        todo!()
    }

    fn from(&self) -> Square {
        todo!()
    }

    fn to(&self) -> Square {
        todo!()
    }

    fn move_type(&self) -> MoveType {
        todo!()
    }
}

impl ChessMove for CastleMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        todo!()
    }

    fn from(&self) -> Square {
        todo!()
    }

    fn to(&self) -> Square {
        todo!()
    }

    fn move_type(&self) -> MoveType {
        todo!()
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_letter = self.piece;
        let from_file_letter = (self.from.0 as u8 + 97) as char;
        let from_rank_number = self.from.1 + 1;
        let to_file_letter = (self.to.0 as u8 + 97) as char;
        let to_rank_number = self.to.1 + 1;
        write!(f, "{piece_letter}{from_file_letter}{from_rank_number}-{to_file_letter}{to_rank_number}")
    }
}

impl Display for Capture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_letter = self.piece;
        let from_file_letter = (self.from.0 as u8 + 97) as char;
        let from_rank_number = self.from.1 + 1;
        let to_file_letter = (self.to.0 as u8 + 97) as char;
        let to_rank_number = self.to.1 + 1;
        write!(f, "{piece_letter}x{from_file_letter}{from_rank_number}-{to_file_letter}{to_rank_number}")
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
        write!(f, "{from_file_letter}x{to_file_letter}{to_rank_number}={piece_letter}")
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
