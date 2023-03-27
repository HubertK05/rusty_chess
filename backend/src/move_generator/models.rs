use crate::{
    board_setup::models::{Board, BoardError},
    move_register::ChessMove,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    ops::{Add, Mul, Sub},
};

use super::restrictions::{get_attacked, get_checked, get_pins};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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

    pub fn get_all_moves(board: &Board, color: Color) -> Self {
        let restrictions = MoveRestrictionData::get(board, color);
        let mut res: Vec<Box<dyn ChessMove>> = Vec::new();
        for rank in 0..=7 {
            for file in 0..=7 {
                if let Some(p) = board.get_square(Square(file, rank)) {
                    if p.color() == color {
                        res.extend(p.get_moves(board, &restrictions));
                    }
                }
            }
        }

        Moves(res)
    }

    pub fn search_with_from(&self, from: Square) -> Self {
        Self(self.0.iter().cloned().filter(|mov| mov.from() == from).collect())
    }

    pub fn search_with_to(&self, to: Square) -> Self {
        Self(self.0.iter().cloned().filter(|mov| mov.to() == to).collect())
    }

    pub fn find(&self, from: Square, to: Square) -> Option<Box<dyn ChessMove>> {
        self.0.iter().cloned().filter(|mov| mov.from() == from && mov.to() == to).next()
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
