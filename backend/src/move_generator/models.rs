use std::{fmt::{Display, self}, collections::{HashSet, HashMap}, ops::{Mul, Add, Sub}};

use dyn_clone::DynClone;

use crate::{board_setup::models::{BoardError, Board, FenPieceType}, move_register::models::{ChessMove, MoveError}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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

#[derive(Debug, Clone, Copy)]
pub struct Square(pub i8);

impl From<Square> for (i8, i8) {
    fn from(val: Square) -> Self {
        (val.0 / 8, val.0 % 8)
    }
}

pub trait CheckedAdd<T = Self> {
    type Output;

    fn c_add(self, rhs: T) -> Option<Self::Output>;
}

pub trait CheckedAddAssign<T = Self> {
    type Output;

    fn c_add_assign(self, rhs: T) -> Option<Self::Output>;
}

impl Add<i8> for Square {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<i8> for Square {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl CheckedAdd<Offset> for Square {
    type Output = Self;

    fn c_add(self, rhs: Offset) -> Option<Self::Output> {
        let rank_number = self.0 / 8 + rhs.0;
        let file_number = self.0 % 8 + rhs.1;
        if !rank_number.is_negative() && rank_number <= 7 && !file_number.is_negative() && file_number <= 7 {
            Some(Self(rank_number * 8 + file_number))
        } else {
            None
        }
    }
}

impl CheckedAddAssign<Offset> for Square {
    type Output = Self;

    fn c_add_assign(self, rhs: Offset) -> Option<Self::Output> {
        let rank_number = self.0 / 8 + rhs.0;
        let file_number = self.0 % 8 + rhs.1;
        if !rank_number.is_negative() && rank_number <= 7 && !file_number.is_negative() && file_number <= 7 {
            Some(Self(rank_number * 8 + file_number))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
            MoveDir::Up => Offset(1, 0),
            MoveDir::UpRight => Offset(1, 1),
            MoveDir::Right => Offset(0, 1),
            MoveDir::DownRight => Offset(-1, 1),
            MoveDir::Down => Offset(-1, 0),
            MoveDir::DownLeft => Offset(-1, -1),
            MoveDir::Left => Offset(0, -1),
            MoveDir::UpLeft => Offset(1, -1),
        }
    }
}

impl TryFrom<&str> for Square {
    type Error = BoardError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        if val.len() != 2 { return Err(BoardError::ConversionFailure) }
        let mut val = val.chars();

        let file_number = ((val.next().ok_or(BoardError::ConversionFailure)? as u8) - 97) as i8;
        let rank_number = val
                                .next()
                                .ok_or(BoardError::ConversionFailure)?
                                .to_string()
                                .parse::<i8>()
                                .map_err(|_e| BoardError::ConversionFailure)? - 1;

        Ok(Square(rank_number * 8 + file_number))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_number = self.0 / 8;
        let file_number = (self.0 as u8 % 8 + 97) as char;
        write!(f, "{}{}", file_number, rank_number)
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} ranks, {} files)", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct Pawn {
    pub color: Color,
    pub position: Square,
}

#[derive(Debug, Clone)]
pub struct Knight {
    pub color: Color,
    pub position: Square,
}

#[derive(Debug, Clone)]
pub struct Bishop {
    pub color: Color,
    pub position: Square,
}

#[derive(Debug, Clone)]
pub struct Rook {
    pub color: Color,
    pub position: Square,
}

#[derive(Debug, Clone)]
pub struct Queen {
    pub color: Color,
    pub position: Square,
}

#[derive(Debug, Clone)]
pub struct King {
    pub color: Color,
    pub position: Square,
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
pub struct Moves(pub Vec<Box<dyn ChessMove>>);

impl Moves {
    pub fn add_move(&mut self, m: Box<dyn ChessMove>) {
        self.0.push(m);
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

pub struct Attacked(pub HashSet<Square>);

impl Display for Attacked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{}, ", elem);
        }
        Ok(())
    }
}

pub struct CheckSquares(HashSet<Square>, pub u8);
pub struct EnPassantCheckSquare(Option<Square>);

impl Display for CheckSquares {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{}, ", elem);
        }
        Ok(())
    }
}

pub struct PinSquares(pub HashMap<Square, PinDir>);

impl Display for PinSquares {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{} ({:?}), ", elem.0, elem.1);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PinDir {
    Vertical,
    Horizontal,
    LeftDiagonal,
    RightDiagonal,
    EnPassantBlock,
}

impl From<&MoveDir> for PinDir {
    fn from(val: &MoveDir) -> Self {
        match val {
            MoveDir::Up | MoveDir::Down => PinDir::Vertical,
            MoveDir::UpRight | MoveDir::DownLeft => PinDir::RightDiagonal,
            MoveDir::Right | MoveDir::Left => PinDir::Horizontal,
            MoveDir::DownRight | MoveDir::UpLeft => PinDir::LeftDiagonal,
        }
    }
}

pub struct MoveRestrictionData {
    pub attacked: Attacked,
    pub check_squares: CheckSquares,
    pub en_passant_check: EnPassantCheckSquare,
    pub pin_squares: PinSquares,
}

// impl MoveRestrictionData {
//     pub fn new(board: &Board, color: Color) -> Self {
//         Self {
//             attacked: Attacked::get_attacked_squares(board, color),
//             check_squares: CheckSquares::get_all_checked_squares(board, color),
//             en_passant_check: EnPassantCheckSquare::get_all_en_passant_check_squares(board, color),
//             pin_squares: PinSquares::get_all_pin_squares(board, color),
//         }
//     }
// }

impl Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "p"),
            Color::White => write!(f, "P"),
        }
    }
}

impl Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "n"),
            Color::White => write!(f, "N"),
        }
    }
}

impl Display for King {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "k"),
            Color::White => write!(f, "K"),
        }
    }
}

impl Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "r"),
            Color::White => write!(f, "R"),
        }
    }
}

impl Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "b"),
            Color::White => write!(f, "B"),
        }
    }
}

impl Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "q"),
            Color::White => write!(f, "Q"),
        }
    }
}
