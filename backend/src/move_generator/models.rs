use crate::{
    board_setup::models::{Board, BoardError, FenPieceType}, move_register::models::{ChessMove, PromotedPieceType, MoveType, RawMoveType},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Debug},
    ops::{Add, Mul, Sub},
};

use super::{restrictions::{get_attacked, get_checked, get_pins}, pawn_get_moves, knight_get_moves, bishop_get_moves, rook_get_moves, queen_get_moves, king_get_moves};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<PromotedPieceType> for PieceType {
    fn from(val: PromotedPieceType) -> Self {
        match val {
            PromotedPieceType::Queen => PieceType::Queen,
            PromotedPieceType::Knight => PieceType::Knight,
            PromotedPieceType::Bishop => PieceType::Bishop,
            PromotedPieceType::Rook => PieceType::Rook,
        }
    }
}

impl From<PieceType> for char {
    fn from(val: PieceType) -> Self {
        match val {
            PieceType::Pawn => ' ',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self {
            PieceType::Pawn => write!(f, ""),
            PieceType::Knight => write!(f, "N"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::King => write!(f, "K"),
        };
        Ok(())
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opp(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        let str = match self {
            Color::White => "White",
            Color::Black => "Black",
        };
        str.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(pub i8, pub i8);

impl Square {
    pub fn is_in_bounds(&self) -> bool {
        !self.0.is_negative() && self.0 <= 7 && !self.1.is_negative() && self.1 <= 7
    }
}

pub trait CheckedAdd<T = Self> {
    type Output;

    fn c_add(self, rhs: T) -> Option<Self::Output>;
}

impl Add<Offset> for Square {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Offset> for Square {
    type Output = Self;

    fn sub(self, rhs: Offset) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub for Square {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        Offset(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl CheckedAdd<Offset> for Square {
    type Output = Self;

    fn c_add(self, rhs: Offset) -> Option<Self::Output> {
        let res = Self(self.0 + rhs.0, self.1 + rhs.1);
        if res.is_in_bounds() {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Offset(pub i8, pub i8);

impl Mul<i8> for Offset {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl From<MoveDir> for Offset {
    fn from(val: MoveDir) -> Self {
        match val {
            MoveDir::Up => Offset(0, 1),
            MoveDir::UpRight => Offset(1, 1),
            MoveDir::Right => Offset(1, 0),
            MoveDir::DownRight => Offset(1, -1),
            MoveDir::Down => Offset(0, -1),
            MoveDir::DownLeft => Offset(-1, -1),
            MoveDir::Left => Offset(-1, 0),
            MoveDir::UpLeft => Offset(-1, 1),
        }
    }
}

impl TryFrom<&str> for Square {
    type Error = BoardError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        if val.len() != 2 {
            return Err(BoardError::ConversionFailure);
        }
        let mut val = val.chars();

        let file_number = ((val.next().ok_or(BoardError::ConversionFailure)? as u8) - 97) as i8;
        let rank_number = val
            .next()
            .ok_or(BoardError::ConversionFailure)?
            .to_string()
            .parse::<i8>()
            .map_err(|_e| BoardError::ConversionFailure)?
            - 1;

        Ok(Square(file_number, rank_number))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_number = self.1 + 1;
        let file_number = (self.0 as u8 + 97) as char;
        write!(f, "{}{}", file_number, rank_number)
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} ranks, {} files)", self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: Square,
}

impl ChessPiece {
    pub fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<ChessMove> {
        match self.piece_type {
            PieceType::Pawn => pawn_get_moves(self, board, restriction).collect(),
            PieceType::Knight => knight_get_moves(self, board, restriction).collect(),
            PieceType::Bishop => bishop_get_moves(self, board, restriction).collect(),
            PieceType::Rook => rook_get_moves(self, board, restriction).collect(),
            PieceType::Queen => queen_get_moves(self, board, restriction).collect(),
            PieceType::King => king_get_moves(self, board, restriction).collect(),
        }
    }
    
    pub fn fen_piece_type(&self) -> FenPieceType {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => FenPieceType::WhitePawn,
            (PieceType::Pawn, Color::Black) => FenPieceType::BlackPawn,
            (PieceType::Knight, Color::White) => FenPieceType::WhiteKnight,
            (PieceType::Knight, Color::Black) => FenPieceType::BlackKnight,
            (PieceType::Bishop, Color::White) => FenPieceType::WhiteBishop,
            (PieceType::Bishop, Color::Black) => FenPieceType::BlackBishop,
            (PieceType::Rook, Color::White) => FenPieceType::WhiteRook,
            (PieceType::Rook, Color::Black) => FenPieceType::BlackRook,
            (PieceType::Queen, Color::White) => FenPieceType::WhiteQueen,
            (PieceType::Queen, Color::Black) => FenPieceType::BlackQueen,
            (PieceType::King, Color::White) => FenPieceType::WhiteKing,
            (PieceType::King, Color::Black) => FenPieceType::BlackKing,
        }
    }
    
    pub fn set_position(&mut self, pos: Square) {
        self.position = pos;
    }
    
    pub fn mating_material_points(&self) -> u8 {
        match self.piece_type {
            PieceType::Pawn => 3,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 3,
            PieceType::King => 0,
        }
    }
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.fen_piece_type(), f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveDir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Debug, Clone)]
pub struct Moves(pub Vec<ChessMove>);

impl Moves {
    pub fn add_move(&mut self, m: ChessMove) {
        self.0.push(m);
    }

    pub fn get_all_moves(board: &Board, color: Color) -> Self {
        let restrictions = MoveRestrictionData::get(board, color);
        let mut res: Vec<ChessMove> = Vec::new();
        for rank in 0..=7 {
            for file in 0..=7 {
                if let Some(p) = board.get_square(Square(file, rank)) {
                    if p.color == color {
                        res.extend(p.get_moves(board, &restrictions));
                    }
                }
            }
        }

        Moves(res)
    }

    pub fn search_with_from(&self, from: Square) -> Self {
        Self(self.0.iter().copied().filter(|mov| mov.from == from).collect())
    }

    pub fn search_with_from_file(&self, from_file: i8) -> Self {
        Self(self.0.iter().copied().filter(|mov| mov.from.0 == from_file).collect())
    }

    pub fn search_with_from_rank(&self, from_rank: i8) -> Self {
        Self(self.0.iter().copied().filter(|mov| mov.from.1 == from_rank).collect())
    }

    pub fn search_with_to(&self, to: Square) -> Self {
        Self(self.0.iter().copied().filter(|mov| mov.to == to).collect())
    }
    
    pub fn search_with_piece_type(&self, piece_type: PieceType) -> Self {
        Self(self.0.iter().copied().filter(|&mov| match mov.move_type {
            MoveType::Move(pt) | MoveType::Capture(pt) => piece_type == pt,
            MoveType::EnPassantMove | MoveType::PromotionMove(_) | MoveType::PromotionCapture(_) => piece_type == PieceType::Pawn,
            MoveType::CastleMove(_) => piece_type == PieceType::King,
        }).collect())
    }

    pub fn search_with_promoted_piece_type(&self, piece_type: PromotedPieceType) -> Self {
        Self(self.0.iter().copied().filter(|&mov| match mov.move_type {
            MoveType::Move(_) | MoveType::Capture(_) | MoveType::EnPassantMove | MoveType::CastleMove(_) => false,
            MoveType::PromotionMove(ppt) | MoveType::PromotionCapture(ppt) => ppt == piece_type,
        }).collect())
    }

    pub fn search_with_raw_move_types(&self, move_types: &[RawMoveType]) -> Self {
        Self(self.0.iter().copied().filter(|&mov| match mov.move_type {
            MoveType::Move(_) => move_types.iter().any(|&move_type| move_type == RawMoveType::Move),
            MoveType::Capture(_) => move_types.iter().any(|&move_type| move_type == RawMoveType::Capture),
            MoveType::EnPassantMove => move_types.iter().any(|&move_type| move_type == RawMoveType::EnPassantMove),
            MoveType::CastleMove(_) => move_types.iter().any(|&move_type| move_type == RawMoveType::CastleMove),
            MoveType::PromotionMove(_) => move_types.iter().any(|&move_type| move_type == RawMoveType::PromotionMove),
            MoveType::PromotionCapture(_) => move_types.iter().any(|&move_type| move_type == RawMoveType::PromotionCapture),
        }).collect())
    }

    pub fn find(&self, from: Square, to: Square) -> Option<ChessMove> {
        self.0.iter().copied().filter(|mov| mov.from == from && mov.to == to).next()
    }
}

impl Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            let _ = write!(f, "{}. {}, ", i, &self.0[i]);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Attacked(pub HashSet<Square>);

impl Display for Attacked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{}, ", elem);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CheckSquares {
    pub squares: HashSet<Square>,
    pub checks_amount: u8,
}

impl Display for CheckSquares {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.squares.iter() {
            let _ = write!(f, "{}, ", elem);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct PinSquares(pub HashMap<Square, PinDir>);

impl Display for PinSquares {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{} ({:?}), ", elem.0, elem.1);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PinDir {
    Vertical,
    Horizontal,
    LeftDiagonal,
    RightDiagonal,
    EnPassantBlock,
}

impl From<MoveDir> for PinDir {
    fn from(val: MoveDir) -> Self {
        match val {
            MoveDir::Up | MoveDir::Down => PinDir::Vertical,
            MoveDir::UpRight | MoveDir::DownLeft => PinDir::RightDiagonal,
            MoveDir::Right | MoveDir::Left => PinDir::Horizontal,
            MoveDir::DownRight | MoveDir::UpLeft => PinDir::LeftDiagonal,
        }
    }
}

#[derive(Debug)]
pub struct MoveRestrictionData {
    pub attacked: Attacked,
    pub check_squares: CheckSquares,
    pub pin_squares: PinSquares,
}

impl MoveRestrictionData {
    pub fn get(board: &Board, color: Color) -> Self {
        Self {
            attacked: get_attacked(board, color),
            check_squares: get_checked(board, color),
            pin_squares: get_pins(board, color),
        }
    }
}
