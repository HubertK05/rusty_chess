use std::fmt::{Display, self};

use crate::{PieceType, PiecePosition, Board, MoveError, Vec2, Queen, Color, Bishop, Rook, Knight, ChessPiece};

#[derive(Debug)]
pub struct Move {
    pub piece: PieceType,
    pub from: PiecePosition,
    pub to: PiecePosition,
}

impl Move {
    pub fn new(piece: PieceType, from: PiecePosition, to: PiecePosition) -> Self {
        Self {
            piece,
            from,
            to,
        }
    }
}

#[derive(Debug)]
pub struct Capture {
    pub piece: PieceType,
    pub from: PiecePosition,
    pub to: PiecePosition,
}

impl Capture {
    pub fn new(piece: PieceType, from: PiecePosition, to: PiecePosition) -> Self {
        Self {
            piece,
            from,
            to,
        }
    }
}

#[derive(Debug)]
pub struct CastleMove {
    pub castle_type: CastleType,
    pub from: PiecePosition,
}

impl CastleMove {
    pub fn new(castle_type: CastleType, from: PiecePosition) -> Self {
        Self {
            castle_type,
            from,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CastleType {
    Short,
    Long,
}

#[derive(Debug)]
pub struct EnPassantMove {
    pub from: PiecePosition,
    pub to: PiecePosition,
}

impl EnPassantMove {
    pub fn new(from: PiecePosition, to: PiecePosition) -> Self {
        Self {
            from,
            to,
        }
    }
}

#[derive(Debug)]
pub struct PromotionMove {
    pub from: PiecePosition,
    pub to: PiecePosition,
    pub to_piece: PromotedPieceType,
    pub color: Color,
}

impl PromotionMove {
    pub fn new(from: PiecePosition, to: PiecePosition, to_piece: PromotedPieceType, color: Color) -> Self {
        Self {
            from,
            to,
            to_piece,
            color,
        }
    }
}

#[derive(Debug)]
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

pub trait ChessMove: std::fmt::Debug + std::fmt::Display {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError>;
    fn is_double_pawn_move(&self) -> bool { false }
    fn to(&self) -> PiecePosition;
    fn move_type(&self) -> MoveType;
}

impl ChessMove for Move {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let piece = board.board[self.from.file as usize][self.from.rank as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
        if piece.piece_type() == PieceType::Pawn { board.half_move_timer_50 = 0 } else { board.half_move_timer_50 += 1 };
        piece.set_position(self.to);
        piece.set_moved(true);
        let en_passant_square = if self.is_double_pawn_move() {
            piece.set_en_passantable(true);
            Some(self.to)
        } else { None };
        board.set_en_passant_square(en_passant_square);
        board.board[self.to.file as usize][self.to.rank as usize] = board.board[self.from.file as usize][self.from.rank as usize].take();
        Ok(())
    }

    fn is_double_pawn_move(&self) -> bool {
        self.piece == PieceType::Pawn && (self.to.rank - self.from.rank).abs() == 2
    }

    fn to(&self) -> PiecePosition { self.to }

    fn move_type(&self) -> MoveType { MoveType::Move }
}

impl ChessMove for Capture {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let captured_piece = board.get_square(self.to).ok_or(MoveError::PieceNotFound)?;
        match captured_piece.color() {
            Color::White => board.white_mating_material -= captured_piece.mating_material_points(),
            Color::Black => board.black_mating_material -= captured_piece.mating_material_points(),
        }

        let piece = board.board[self.from.file as usize][self.from.rank as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
        piece.set_position(self.to);
        piece.set_moved(true);
        board.board[self.to.file as usize][self.to.rank as usize] = board.board[self.from.file as usize][self.from.rank as usize].take();
        board.set_en_passant_square(None);
        board.half_move_timer_50 = 0;
        Ok(())
    }

    fn to(&self) -> PiecePosition { self.to }

    fn move_type(&self) -> MoveType { MoveType::Capture }
}

impl ChessMove for CastleMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let castle_rank = self.from.rank;
        let (king_target, rook_start, rook_target) = match self.castle_type {
            CastleType::Long => (Vec2(2, castle_rank), Vec2(0, castle_rank), Vec2(3, castle_rank)),
            CastleType::Short => (Vec2(6, castle_rank), Vec2(7, castle_rank), Vec2(5, castle_rank)),
        };
        {
            let piece = board.board[self.from.file as usize][self.from.rank as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
            piece.set_position(PiecePosition::from(king_target));
            piece.set_moved(true);
            let piece = board.board[rook_start.0 as usize][rook_start.1 as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
            piece.set_position(PiecePosition::from(rook_target));
            piece.set_moved(true);
        }
        board.board[king_target.0 as usize][king_target.1 as usize] = board.board[self.from.file as usize][self.from.rank as usize].take();
        board.board[rook_target.0 as usize][rook_target.1 as usize] = board.board[rook_start.0 as usize][rook_start.1 as usize].take();
        board.set_en_passant_square(None);
        board.half_move_timer_50 += 1;
        Ok(())
    }

    fn to(&self) -> PiecePosition {
        match self.castle_type {
            CastleType::Short => self.from + Vec2(2, 0),
            CastleType::Long => self.from + Vec2(-3, 0),
        }
    }

    fn move_type(&self) -> MoveType { MoveType::CastleMove }
}

impl ChessMove for EnPassantMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let captured_piece = board.get_square(self.to).ok_or(MoveError::PieceNotFound)?;
        match captured_piece.color() {
            Color::White => board.white_mating_material -= 3,
            Color::Black => board.black_mating_material -= 3,
        }

        let piece = board.board[self.from.file as usize][self.from.rank as usize].as_mut().ok_or(MoveError::PieceNotFound)?;
        piece.set_position(self.to);
        board.board[self.to.file as usize][self.to.rank as usize] = board.board[self.from.file as usize][self.from.rank as usize].take();
        board.board[self.to.file as usize][self.from.rank as usize] = None;
        board.set_en_passant_square(None);
        board.half_move_timer_50 = 0;
        Ok(())
    }

    fn to(&self) -> PiecePosition { self.to }

    fn move_type(&self) -> MoveType { MoveType::EnPassantMove }
}

impl ChessMove for PromotionMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let _ = self.from.verify_bounds()?;
        let _ = self.to.verify_bounds()?;
        let promoting_pawn = board.get_square(self.from).ok_or(MoveError::PieceNotFound)?;

        board.board[self.to.file as usize][self.to.rank as usize] = match self.to_piece {
            PromotedPieceType::Queen => Some(Box::new(Queen { color: self.color, position: self.to })),
            PromotedPieceType::Knight => {
                match promoting_pawn.color() {
                    Color::White => board.white_mating_material -= 2,
                    Color::Black => board.black_mating_material -= 2,
                };
                Some(Box::new(Knight { color: self.color, position: self.to }))
            },
            PromotedPieceType::Bishop => {
                match promoting_pawn.color() {
                    Color::White => board.white_mating_material -= 1,
                    Color::Black => board.black_mating_material -= 1,
                };
                Some(Box::new(Bishop { color: self.color, position: self.to }))
            },
            PromotedPieceType::Rook => Some(Box::new(Rook { color: self.color, position: self.to, has_moved: true })),
        };
        board.board[self.from.file as usize][self.from.rank as usize] = None;
        board.set_en_passant_square(None);
        board.half_move_timer_50 = 0;
        Ok(())
    }

    fn to(&self) -> PiecePosition { self.to }

    fn move_type(&self) -> MoveType { MoveType::PromotionMove }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_letter = self.piece;
        let from_file_letter = (self.from.file as u8 + 97) as char;
        let from_rank_number = self.from.rank + 1;
        let to_file_letter = (self.to.file as u8 + 97) as char;
        let to_rank_number = self.to.rank + 1;
        write!(f, "{piece_letter}{from_file_letter}{from_rank_number}-{to_file_letter}{to_rank_number}")
    }
}

impl Display for Capture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_letter = self.piece;
        let from_file_letter = (self.from.file as u8 + 97) as char;
        let from_rank_number = self.from.rank + 1;
        let to_file_letter = (self.to.file as u8 + 97) as char;
        let to_rank_number = self.to.rank + 1;
        write!(f, "{piece_letter}x{from_file_letter}{from_rank_number}-{to_file_letter}{to_rank_number}")
    }
}

impl Display for CastleMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.castle_type {
            CastleType::Long => write!(f, "O-O-O"),
            CastleType::Short => write!(f, "O-O"),
        }
    }
}

impl Display for EnPassantMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_from_letter = (self.from.file as u8 + 97) as char;
        let file_to_letter = (self.to.file as u8 + 97) as char;
        let rank_number = self.to.rank + 1;
        write!(f, "{file_from_letter}x{file_to_letter}{rank_number}")
    }
}

impl Display for PromotionMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let file_letter = (self.to.file as u8 + 97) as char;
        let rank_number = self.to.rank + 1;
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