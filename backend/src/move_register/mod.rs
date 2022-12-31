use std::fmt::{Display, self};

use dyn_clone::DynClone;

use crate::{PieceType, Board, MoveError, Vec2, Queen, Color, Bishop, Rook, Knight};

#[derive(Debug, Clone)]
pub struct Move {
    pub piece: PieceType,
    pub from: Vec2,
    pub to: Vec2,
}

impl Move {
    pub fn new(piece: PieceType, from: Vec2, to: Vec2) -> Self {
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
    pub from: Vec2,
    pub to: Vec2,
}

impl Capture {
    pub fn new(piece: PieceType, from: Vec2, to: Vec2) -> Self {
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
    pub from: Vec2,
}

impl CastleMove {
    pub fn new(castle_type: CastleType, from: Vec2) -> Self {
        Self {
            castle_type,
            from,
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

impl CastleType {
    pub fn new(color: Color, length: CastleLength) -> Self {
        match (color, length) {
            (Color::White, CastleLength::Short) => Self::WhiteShort,
            (Color::White, CastleLength::Long) => Self::WhiteLong,
            (Color::Black, CastleLength::Short) => Self::BlackShort,
            (Color::Black, CastleLength::Long) => Self::BlackLong,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            CastleType::WhiteShort | CastleType::WhiteLong => Color::White,
            CastleType::BlackShort | CastleType::BlackLong => Color::Black,
        }
    }

    pub fn length(&self) -> CastleLength {
        match self {
            CastleType::WhiteShort | CastleType::BlackShort => CastleLength::Short,
            CastleType::WhiteLong | CastleType::BlackLong => CastleLength::Long,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum CastleLength {
    Short,
    Long,
}

#[derive(Debug, Clone)]
pub struct EnPassantMove {
    pub from: Vec2,
    pub to: Vec2,
}

impl EnPassantMove {
    pub fn new(from: Vec2, to: Vec2) -> Self {
        Self {
            from,
            to,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PromotionMove {
    pub from: Vec2,
    pub to: Vec2,
    pub to_piece: PromotedPieceType,
    pub color: Color,
}

impl PromotionMove {
    pub fn new(from: Vec2, to: Vec2, to_piece: PromotedPieceType, color: Color) -> Self {
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
}

pub trait ChessMove: std::fmt::Debug + std::fmt::Display + DynClone {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError>;
    fn is_double_pawn_move(&self) -> bool { false }
    fn to(&self) -> Vec2;
    fn move_type(&self) -> MoveType;
}

dyn_clone::clone_trait_object!(ChessMove);

impl ChessMove for Move {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let piece = board.board[self.from.0 as usize][self.from.1 as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
        if piece.piece_type() == PieceType::Pawn { board.half_move_timer_50 = 0 } else { board.half_move_timer_50 += 1 };
        piece.set_position(self.to);
        let en_passant_square = if self.is_double_pawn_move() {
            piece.set_en_passantable(true);
            Some(self.to)
        } else { None };
        board.set_en_passant_square(en_passant_square);
        // set moved
        board.board[self.to.0 as usize][self.to.1 as usize] = board.board[self.from.0 as usize][self.from.1 as usize].take();
        board.modify_castling_rights();
        Ok(())
    }

    fn is_double_pawn_move(&self) -> bool {
        self.piece == PieceType::Pawn && (self.to.1 - self.from.1).abs() == 2
    }

    fn to(&self) -> Vec2 { self.to }

    fn move_type(&self) -> MoveType { MoveType::Move }
}

impl ChessMove for Capture {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let captured_piece = board.get_square(self.to).ok_or(MoveError::PieceNotFound)?;
        match captured_piece.color() {
            Color::White => board.mating_material.0 -= captured_piece.mating_material_points(),
            Color::Black => board.mating_material.1 -= captured_piece.mating_material_points(),
        }

        let piece = board.board[self.from.0 as usize][self.from.1 as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
        piece.set_position(self.to);
        board.board[self.to.0 as usize][self.to.1 as usize] = board.board[self.from.0 as usize][self.from.1 as usize].take();
        board.set_en_passant_square(None);
        board.half_move_timer_50 = 0;
        board.modify_castling_rights();
        Ok(())
    }

    fn to(&self) -> Vec2 { self.to }

    fn move_type(&self) -> MoveType { MoveType::Capture }
}

impl ChessMove for CastleMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let castle_1 = self.from.1;
        let (king_target, rook_start, rook_target) = match self.castle_type {
            CastleType::WhiteLong | CastleType::BlackLong => (Vec2(2, castle_1), Vec2(0, castle_1), Vec2(3, castle_1)),
            CastleType::WhiteShort | CastleType::BlackShort => (Vec2(6, castle_1), Vec2(7, castle_1), Vec2(5, castle_1)),
        };
        {
            let piece = board.board[self.from.0 as usize][self.from.1 as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
            piece.set_position(Vec2::from(king_target));
            let piece = board.board[rook_start.0 as usize][rook_start.1 as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
            piece.set_position(Vec2::from(rook_target));
        }
        board.board[king_target.0 as usize][king_target.1 as usize] = board.board[self.from.0 as usize][self.from.1 as usize].take();
        board.board[rook_target.0 as usize][rook_target.1 as usize] = board.board[rook_start.0 as usize][rook_start.1 as usize].take();
        board.set_en_passant_square(None);
        board.half_move_timer_50 += 1;
        board.modify_castling_rights();
        Ok(())
    }

    fn to(&self) -> Vec2 {
        match self.castle_type {
            CastleType::WhiteShort | CastleType::BlackShort => self.from + Vec2(2, 0),
            CastleType::WhiteLong | CastleType::BlackLong => self.from + Vec2(-3, 0),
        }
    }

    fn move_type(&self) -> MoveType { MoveType::CastleMove }
}

impl ChessMove for EnPassantMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let captured_piece = board.get_square(Vec2::from(Vec2(self.to.0, self.from.1))).ok_or(MoveError::PieceNotFound)?;
        match captured_piece.color() {
            Color::White => board.mating_material.0 -= 3,
            Color::Black => board.mating_material.1 -= 3,
        }

        let piece = board.board[self.from.0 as usize][self.from.1 as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
        piece.set_position(self.to);
        board.board[self.to.0 as usize][self.to.1 as usize] = board.board[self.from.0 as usize][self.from.1 as usize].take();
        board.board[self.to.0 as usize][self.from.1 as usize] = None;
        board.set_en_passant_square(None);
        board.half_move_timer_50 = 0;
        Ok(())
    }

    fn to(&self) -> Vec2 { self.to }

    fn move_type(&self) -> MoveType { MoveType::EnPassantMove }
}

impl ChessMove for PromotionMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let promoting_pawn = board.get_square(self.from).ok_or(MoveError::PieceNotFound)?;

        board.board[self.to.0 as usize][self.to.1 as usize] = match self.to_piece {
            PromotedPieceType::Queen => Some(Box::new(Queen { color: self.color, position: self.to })),
            PromotedPieceType::Knight => {
                match promoting_pawn.color() {
                    Color::White => board.mating_material.0 -= 2,
                    Color::Black => board.mating_material.1 -= 2,
                };
                Some(Box::new(Knight { color: self.color, position: self.to }))
            },
            PromotedPieceType::Bishop => {
                match promoting_pawn.color() {
                    Color::White => board.mating_material.0 -= 1,
                    Color::Black => board.mating_material.1 -= 1,
                };
                Some(Box::new(Bishop { color: self.color, position: self.to }))
            },
            PromotedPieceType::Rook => Some(Box::new(Rook { color: self.color, position: self.to })),
        };
        board.board[self.from.0 as usize][self.from.1 as usize] = None;
        board.set_en_passant_square(None);
        board.half_move_timer_50 = 0;
        Ok(())
    }

    fn to(&self) -> Vec2 { self.to }

    fn move_type(&self) -> MoveType { MoveType::PromotionMove }
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
        let from_0_letter = (self.from.0 as u8 + 97) as char;
        let from_1_number = self.from.1 + 1;
        let to_0_letter = (self.to.0 as u8 + 97) as char;
        let to_1_number = self.to.1 + 1;
        write!(f, "{piece_letter}x{from_0_letter}{from_1_number}-{to_0_letter}{to_1_number}")
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
        let file_from_letter = (self.from.0 as u8 + 97) as char;
        let file_to_letter = (self.to.0 as u8 + 97) as char;
        let rank_number = self.to.1 + 1;
        write!(f, "{file_from_letter}x{file_to_letter}{rank_number}")
    }
}

impl Display for PromotionMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let file_letter = (self.to.0 as u8 + 97) as char;
        let rank_number = self.to.1 + 1;
        let piece_letter = &self.to_piece;
        write!(f, "{file_letter}{rank_number}={piece_letter}")
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