pub mod move_register;
use std::{ops::{Add, AddAssign, Mul}, fmt::{Display, self}, collections::{HashSet, HashMap, BTreeMap, BTreeSet}, hash::Hash, char::MAX, pin::Pin};
use move_register::{Move, ChessMove, Capture, EnPassantMove, CastleMove, CastleType, PromotionMove};
use dyn_clone::DynClone;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FenPieceType {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl ToString for FenPieceType {
    fn to_string(&self) -> String {
        let str = match self {
            FenPieceType::WhitePawn => "P",
            FenPieceType::WhiteKnight => "N",
            FenPieceType::WhiteBishop => "B",
            FenPieceType::WhiteRook => "R",
            FenPieceType::WhiteQueen => "Q",
            FenPieceType::WhiteKing => "K",
            FenPieceType::BlackPawn => "p",
            FenPieceType::BlackKnight => "n",
            FenPieceType::BlackBishop => "b",
            FenPieceType::BlackRook => "r",
            FenPieceType::BlackQueen => "q",
            FenPieceType::BlackKing => "k",
        };
        str.into()
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PiecePosition {
    pub file: i8,
    pub rank: i8,
}

impl PiecePosition {
    fn new(file: i8, rank: i8) -> Self {
        Self {
            file,
            rank,
        }
    }

    fn verify_bounds(&self) -> Result<(), MoveError> {
        if self.file < 0 || self.file > 7 || self.rank < 0 || self.rank > 7 { Err(MoveError::OutOfBounds) } else { Ok(()) }
    }

    fn verify_with_checked_squares(&self, squares: &CheckSquares) -> Result<(), MoveError> {
        match squares.1 {
            0 => Ok(()),
            1 => if squares.0.contains(&self) { Ok(()) } else { Err(MoveError::MoveFilteredOut) },
            2 => Err(MoveError::MoveFilteredOut),
            _ => unreachable!(),
        }
    }

    fn verify_with_en_passant_checked_square(&self, sq: &EnPassantCheckSquare) -> Result<(), MoveError> {
        match sq.0 {
            Some(s) => if self == &s { Ok(()) } else { Err(MoveError::MoveFilteredOut) },
            None => Err(MoveError::MoveFilteredOut),
        }
    }

    fn verify_with_attacked_squares(&self, squares: &Attacked) -> Result<(), MoveError> {
        if squares.0.contains(&self) { Err(MoveError::MoveFilteredOut) } else { Ok(()) }
    }
}
impl Display for PiecePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_letter = (self.file as u8 + 97) as char;
        let rank_number = self.rank + 1;
        write!(f, "{file_letter}{rank_number}")
    }
}

impl From<[i8; 2]> for PiecePosition {
    fn from(val: [i8; 2]) -> Self {
        Self {
            file: val[0],
            rank: val[1],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2(pub i8, pub i8);

impl From<&MoveDir> for Vec2 {
    fn from(val: &MoveDir) -> Self {
        match val {
            MoveDir::Up => Vec2(0, 1),
            MoveDir::UpRight => Vec2(1, 1),
            MoveDir::Right => Vec2(1, 0),
            MoveDir::DownRight => Vec2(1, -1),
            MoveDir::Down => Vec2(0, -1),
            MoveDir::DownLeft => Vec2(-1, -1),
            MoveDir::Left => Vec2(-1, 0),
            MoveDir::UpLeft => Vec2(-1, 1),
        }
    }
}

impl Mul<i8> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i8) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Add<[i8; 2]> for PiecePosition {
    type Output = PiecePosition;

    fn add(self, rhs: [i8; 2]) -> Self::Output {
        let file = self.file + rhs[0];
        let rank = self.rank + rhs[1];
        PiecePosition {
            file,
            rank,
        }
    }
}

impl Add<Vec2> for PiecePosition {
    type Output = PiecePosition;

    fn add(self, rhs: Vec2) -> Self::Output {
        let file = self.file + rhs.0;
        let rank = self.rank + rhs.1;
        Self::new(file, rank)
    }
}

impl AddAssign<[i8; 2]> for PiecePosition {
    fn add_assign(&mut self, rhs: [i8; 2]) {
        let file = self.file as i8 + rhs[0];
        let rank = self.rank as i8 + rhs[1];
        self.file = file.try_into().unwrap();
        self.rank = rank.try_into().unwrap();
    }
}

impl AddAssign<Vec2> for PiecePosition {
    fn add_assign(&mut self, rhs: Vec2) {
        let file = self.file as i8 + rhs.0;
        let rank = self.rank as i8 + rhs.1;
        self.file = file.try_into().unwrap();
        self.rank = rank.try_into().unwrap();
    }
}

impl From<Vec2> for [i8; 2] {
    fn from(val: Vec2) -> Self {
        [val.0, val.1]
    }
}

impl From<[i8; 2]> for Vec2 {
    fn from(val: [i8; 2]) -> Self {
        Vec2(val[0], val[1])
    }
}

impl From<PiecePosition> for Vec2 {
    fn from(val: PiecePosition) -> Self {
        Vec2(val.file, val.rank)
    }
}

impl From<Vec2> for PiecePosition {
    fn from(val: Vec2) -> Self {
        PiecePosition { file: val.0, rank: val.1 }
    }
}

#[derive(Debug, Clone)]
struct Pawn {
    color: Color,
    position: PiecePosition,
    enpassantable: bool,
}

#[derive(Debug, Clone)]
struct Knight {
    color: Color,
    position: PiecePosition,
}

#[derive(Debug, Clone)]
struct Bishop {
    color: Color,
    position: PiecePosition,
}

#[derive(Debug, Clone)]
struct Rook {
    color: Color,
    position: PiecePosition,
    has_moved: bool,
}

#[derive(Debug, Clone)]
struct Queen {
    color: Color,
    position: PiecePosition,
}

#[derive(Debug, Clone)]
struct King {
    color: Color,
    position: PiecePosition,
    has_moved: bool,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[Option<Box<dyn ChessPiece>>;8];8],
    pub en_passant_square: Option<PiecePosition>,
    pub half_move_timer_50: u8,
    pub half_move_number: u16,
    pub white_mating_material: u8,
    pub black_mating_material: u8,
}

impl Board {
    fn new() -> Self {
        Self {
            board: Default::default(),
            en_passant_square: None,
            half_move_timer_50: 0,
            half_move_number: 0,
            white_mating_material: 0,
            black_mating_material: 0,
        }
    }

    fn get_square<'a>(&'a self, position: PiecePosition) -> Option<&dyn ChessPiece> {
        let Some(rank) = self.board.get(position.file as usize) else {
            return None;
        };

        match rank.get(position.rank as usize) {
            Some(piece) => piece.as_deref(),
            None => None,
        }
    }
    
    fn check_castling(&self, mut pos: PiecePosition, attacked: &Attacked, castle_type: CastleType) -> bool {
        let transition = match castle_type {
            CastleType::Short => [1, 0],
            CastleType::Long => [-1, 0],
        };
        for _ in 0..2 {
            pos += transition;
            if self.get_square(pos).is_some() || attacked.0.contains(&pos) { return false };
        }
        if castle_type == CastleType::Long {
            pos += transition;
            if self.get_square(pos).is_some() { return false };
        }
        true
    }

    fn set_en_passant_square(&mut self, val: Option<PiecePosition>) {
        if let Some(sq) = self.en_passant_square {
            let piece_option = self.board[sq.file as usize][sq.rank as usize].as_mut();
            if let Some(piece) = piece_option {
                piece.set_en_passantable(false);
            }
        }
        self.en_passant_square = val;
    }
}

#[derive(Debug)]
pub struct FenNotation(pub String);

impl FenNotation {
    pub fn to_draw_fen(&self) -> String {
        let mut split_fen = self.0.split_whitespace();
        [split_fen.next().expect("wrong fen"), split_fen.next().expect("wrong fen"), split_fen.next().expect("wrong fen")].join(" ")
    }
}

impl From<&Board> for FenNotation {
    fn from(val: &Board) -> Self {
        let mut res = String::new();
        for file in (0..8).rev() {
            let mut empty_counter = 0;
            for rank in (0..8) {
                let piece = val.get_square(PiecePosition::from(Vec2(rank, file)));
                if let Some(p) = piece {
                    if empty_counter != 0 {
                        res.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }
                    res.push_str(&p.fen_piece_type().to_string());
                } else { empty_counter += 1}
            }
            if empty_counter != 0 {
                res.push_str(&empty_counter.to_string());
                empty_counter = 0;
            }
            if file != 0 { res.push('/') }
        }

        res.push(' ');

        match val.half_move_number % 2 {
            0 => res.push('b'),
            1 => res.push('w'),
            _ => unreachable!(),
        }

        let mut castling_rights = String::new();
        match val.get_square(PiecePosition::from(Vec2(4, 0))) {
            Some(piece) if piece.fen_piece_type() == FenPieceType::WhiteKing && !piece.has_moved() => {
                match val.get_square(PiecePosition::from(Vec2(7, 0))) {
                    Some(piece) if piece.fen_piece_type() == FenPieceType::WhiteRook && !piece.has_moved() => {
                        castling_rights.push('K')
                    },
                    _ => (),
                }
                match val.get_square(PiecePosition::from(Vec2(0, 0))) {
                    Some(piece) if piece.fen_piece_type() == FenPieceType::WhiteRook && !piece.has_moved() => {
                        castling_rights.push('Q')
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        res.push(' ');

        match val.get_square(PiecePosition::from(Vec2(4, 7))) {
            Some(piece) if piece.fen_piece_type() == FenPieceType::BlackKing && !piece.has_moved() => {
                match val.get_square(PiecePosition::from(Vec2(7, 7))) {
                    Some(piece) if piece.fen_piece_type() == FenPieceType::BlackRook && !piece.has_moved() => {
                        castling_rights.push('k')
                    },
                    _ => (),
                }
                match val.get_square(PiecePosition::from(Vec2(0, 7))) {
                    Some(piece) if piece.fen_piece_type() == FenPieceType::BlackRook && !piece.has_moved() => {
                        castling_rights.push('q')
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        if castling_rights.is_empty() {
            res.push('-');
        } else {
            res.push_str(&castling_rights);
        }

        res.push_str(" - ");
        res.push_str(&val.half_move_timer_50.to_string());
        res.push(' ');
        let move_number = val.half_move_number / 2 + 1;
        res.push_str(&move_number.to_string());

        FenNotation(res)
    }
}

impl TryFrom<[[&str; 8]; 8]> for Board {
    type Error = BoardError;

    fn try_from(val: [[&str; 8]; 8]) -> Result<Board, Self::Error> {
        let mut res = Board::new();
        for i in 0..res.board.len() {
            let file = i as i8;
            for j in 0..8 {
                let rank = j as i8;
                match val[7 - j][i] {
                    "" | " " | "." => (),
                    "p" => res.board[i][j] = {
                        res.black_mating_material += 3;
                        Some(Box::new(Pawn { color: Color::Black, position: PiecePosition::new(file, rank), enpassantable: false}))
                    },
                    "n" => res.board[i][j] = {
                        res.black_mating_material += 1;
                        Some(Box::new(Knight { color: Color::Black, position: PiecePosition::new(file, rank)}))
                    },
                    "b" => res.board[i][j] = {
                        res.black_mating_material += 2;
                        Some(Box::new(Bishop { color: Color::Black, position: PiecePosition::new(file, rank)}))
                    },
                    "r" => res.board[i][j] = {
                        res.black_mating_material += 3;
                        Some(Box::new(Rook { color: Color::Black, position: PiecePosition::new(file, rank), has_moved: false}))
                    },
                    "q" => res.board[i][j] = {
                        res.black_mating_material += 3;
                        Some(Box::new(Queen { color: Color::Black, position: PiecePosition::new(file, rank)}))
                    },
                    "k" => res.board[i][j] = {
                        Some(Box::new(King { color: Color::Black, position: PiecePosition::new(file, rank), has_moved: false}))
                    },
                    "P" => res.board[i][j] = {
                        res.white_mating_material += 3;
                        Some(Box::new(Pawn { color: Color::White, position: PiecePosition::new(file, rank), enpassantable: false}))
                    },
                    "N" => res.board[i][j] = {
                        res.white_mating_material += 1;
                        Some(Box::new(Knight { color: Color::White, position: PiecePosition::new(file, rank)}))
                    },
                    "B" => res.board[i][j] = {
                        res.white_mating_material += 2;
                        Some(Box::new(Bishop { color: Color::White, position: PiecePosition::new(file, rank)}))
                    },
                    "R" => res.board[i][j] = {
                        res.white_mating_material += 3;
                        Some(Box::new(Rook { color: Color::White, position: PiecePosition::new(file, rank), has_moved: false}))
                    },
                    "Q" => res.board[i][j] = {
                        res.white_mating_material += 3;
                        Some(Box::new(Queen { color: Color::White, position: PiecePosition::new(file, rank)}))
                    },
                    "K" => res.board[i][j] = {
                        Some(Box::new(King { color: Color::White, position: PiecePosition::new(file, rank), has_moved: false}))
                    },
                    _ => return Err(BoardError::ConversionFailure),
                }
            }
        }
        Ok(res)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..8).rev() {
            for j in 0..8 {
                let _ = match self.board[j][i].as_ref() {
                    Some(s) => write!(f, "{s}"),
                    None => write!(f, "."),
                };
            }
            let _ = write!(f, "\n");
        };
        
        Ok(())
    }
}

#[derive(Debug)]
pub enum BoardError {
    ConversionFailure,
}

#[derive(Debug, PartialEq)]
pub enum MoveError {
    OutOfBounds,
    MoveBlocked,
    MoveFilteredOut,
    PieceNotFound,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MoveDir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl TryFrom<&MoveDir> for [i8; 2] {
    type Error = MoveError;
    
    fn try_from(val: &MoveDir) -> Result<Self, Self::Error> {
        let variant = match val {
            MoveDir::Up => [0, 1],
            MoveDir::UpRight => [1, 1],
            MoveDir::Right => [1, 0],
            MoveDir::DownRight => [1, -1],
            MoveDir::Down => [0, -1],
            MoveDir::DownLeft  =>[-1, -1],
            MoveDir::Left => [-1, 0],
            MoveDir::UpLeft => [-1, 1],
        };
        Ok(variant)
    }
}

pub struct MoveDirs(BTreeSet<MoveDir>);

impl MoveDirs {
    fn new() -> Self {
        Self(BTreeSet::new())
    }

    fn intersection_with_pin_dir(self, other: Option<&PinDir>) -> Self {
        let Some(pin_dir) = other else { return self };
        MoveDirs(self.0.intersection(&MoveDirs::from(pin_dir).0).copied().collect())
    }
}

impl From<&PinDir> for MoveDirs {
    fn from(val: &PinDir) -> Self {
        match val {
            PinDir::Vertical => MoveDirs(BTreeSet::from([MoveDir::Up, MoveDir::Down])),
            PinDir::Horizontal => MoveDirs(BTreeSet::from([MoveDir::Left, MoveDir::Right])),
            PinDir::LeftDiagonal => MoveDirs(BTreeSet::from([MoveDir::DownRight, MoveDir::UpLeft])),
            PinDir::RightDiagonal => MoveDirs(BTreeSet::from([MoveDir::DownLeft, MoveDir::UpRight])),
            PinDir::EnPassantBlock => MoveDirs(BTreeSet::from([MoveDir::Up, MoveDir::Down, MoveDir::UpRight, MoveDir::DownLeft, MoveDir::Right, MoveDir::Left, MoveDir::DownRight, MoveDir::UpLeft])),
        }
    }
}

#[derive(Debug)]
pub struct Moves(pub Vec<Box<dyn ChessMove>>);

impl Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            let _ = write!(f, "{}. {}, ", i, &self.0[i]);
        }
        Ok(())
    }
}

impl Moves {
    pub fn get_all_moves(board: &Board, color: Color) -> Self {
        let mut res = Moves(vec![]);
        let restrictions = match color {
            Color::White => MoveRestrictionData::new(board, Color::Black),
            Color::Black => MoveRestrictionData::new(board, Color::White),
        };
        for i in 0..board.board.len() {
            for j in 0..board.board[i].len() {
                match &board.board[i][j] {
                    Some(s) if s.color() == color => s.get_moves(board, &mut res, &restrictions),
                    _ => (),
                }
            }
        }
        res
    }

    fn add_move(&mut self, board: &Board, start_pos: PiecePosition, coords: [i8; 2], piece_type: PieceType, restrictions: &MoveRestrictionData, color: Color) -> Result<(), MoveError> {
        let destination = start_pos + coords;
        let _ = destination.verify_bounds()?;
        if board.get_square(destination).is_none() {
            match piece_type {
                PieceType::King => destination.verify_with_attacked_squares(&restrictions.attacked)?,
                _ => destination.verify_with_checked_squares(&restrictions.check_squares)?,
            }
            if piece_type == PieceType::Pawn && ((start_pos.rank == WHITE_PROMOTION_RANK && color == Color::White) || (start_pos.rank == BLACK_PROMOTION_RANK && color == Color::Black)) {
                (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Queen, color)));
                (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Knight, color)));
                (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Bishop, color)));
                (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Rook, color)));
            } else {
                (*self).0.push(Box::new(Move::new(piece_type, start_pos, destination)));
            }
        } else { return Err(MoveError::MoveBlocked) }
        Ok(())
    }

    fn add_capture(&mut self, board: &Board, color: Color, start_pos: PiecePosition, coords: Vec2, piece_type: PieceType, restrictions: &MoveRestrictionData) -> () {
        let destination = start_pos + coords;
        if destination.verify_bounds().is_err() { return }
        let piece = board.get_square(destination);
        let is_legal = match piece_type {
            PieceType::King => destination.verify_with_attacked_squares(&restrictions.attacked).is_ok(),
            _ => destination.verify_with_checked_squares(&restrictions.check_squares).is_ok(),
        };
        match piece {
            Some(x) if x.color() != color && is_legal => {
                if piece_type == PieceType::Pawn && ((start_pos.rank == WHITE_PROMOTION_RANK && color == Color::White) || (start_pos.rank == BLACK_PROMOTION_RANK && color == Color::Black)) {
                    (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Queen, color)));
                    (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Knight, color)));
                    (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Bishop, color)));
                    (*self).0.push(Box::new(PromotionMove::new(start_pos, destination, move_register::PromotedPieceType::Rook, color)));
                } else {
                    (*self).0.push(Box::new(Capture::new(piece_type, start_pos, destination)));
                }
            },
            _ => (),
        }
    }

    fn add_special_moves(&mut self, board: &Board, start_pos: PiecePosition, coords: &[[i8; 2]], piece_type: PieceType, restrictions: &MoveRestrictionData, color: Color) {
        for elem in coords {
            let _ = self.add_move(board, start_pos, *elem, piece_type, restrictions, color);
            let _ = self.add_capture(board, color, start_pos, Vec2::from(*elem), piece_type, restrictions);
        }
    }

    fn add_move_series(&mut self, board: &Board, start_pos: PiecePosition, color: Color, dirs: &MoveDirs, piece_type: PieceType, max_moves: usize, restrictions: &MoveRestrictionData) {
        for elem in &dirs.0 {
            let mut move_coords = start_pos;
            let mut prev_move_coords = move_coords;
            let translation: Vec2 = <&MoveDir as Into<Vec2>>::into(&elem);
            for i in 1..(max_moves + 1) {
                prev_move_coords = move_coords;
                move_coords += translation * (i as i8);
                match self.add_move(board, start_pos, (translation * (i as i8)).into(), piece_type, restrictions, color) {
                    Err(MoveError::OutOfBounds) => break,
                    Err(MoveError::MoveBlocked) => {
                        if piece_type != PieceType::Pawn {
                            let _ = self.add_capture(board, color, start_pos, (translation * (i as i8)).into(), piece_type, restrictions);
                        }
                        break
                    },
                    _ => (),
                };
            }
        }
    }

    fn add_captures(&mut self, board: &Board, start_pos: PiecePosition, color: Color, dirs: &MoveDirs, piece_type: PieceType, restrictions: &MoveRestrictionData) {
        for elem in &dirs.0 {
            let _ = self.add_capture(board, color, start_pos, <&MoveDir as Into<Vec2>>::into(&elem), piece_type, restrictions);
        }
    }

    fn add_en_passant(&mut self, board: &Board, start_pos: PiecePosition, color: Color, dirs: &MoveDirs, restrictions: &MoveRestrictionData) {
        for elem in &dirs.0 {
            let translation: [i8; 2] = <&MoveDir as TryInto<[i8; 2]>>::try_into(&elem).unwrap();
            let destination = start_pos + translation;
            if destination.verify_bounds().is_err() { continue }
            let mut translation = Vec2::from(elem);
            translation.1 = 0;
            let piece = board.get_square(start_pos + translation);
            match piece {
                Some(x) if x.color() != color && x.is_enpassantable()
                && (destination.verify_with_checked_squares(&restrictions.check_squares).is_ok()
                || destination.verify_with_en_passant_checked_square(&restrictions.en_passant_check).is_ok()) => {
                    let the_move = EnPassantMove::new(start_pos, destination);
                    (*self).0.push(Box::new(the_move));
                },
                _ => (),
            }
        }
    }

    fn add_castling(&mut self, board: &Board, start_pos: PiecePosition, restrictions: &MoveRestrictionData, color: Color) {
        let legal_start_positions: HashSet<Vec2> = HashSet::from([Vec2(4, 0), Vec2(4, 7)]);
        if !legal_start_positions.contains(&Vec2::from(start_pos)) { return }
        if restrictions.check_squares.1 != 0 { return }
        if board.check_castling(start_pos, &restrictions.attacked, CastleType::Short) {
            let piece = board.get_square(start_pos + [3, 0]);
            let destination = start_pos + [2, 0];
            match piece {
                Some(x) if !x.has_moved() && x.piece_type() == PieceType::Rook && x.color() == color && destination.verify_with_checked_squares(&restrictions.check_squares).is_ok() => {
                    (*self).0.push(Box::new(CastleMove::new(CastleType::Short, start_pos)))
                },
                _ => (),
            }
        }
        if !board.check_castling(start_pos, &restrictions.attacked, CastleType::Long) { return };
        let piece = board.get_square(start_pos + [-4, 0]);
        let destination = start_pos + [-2, 0];
        match piece {
            Some(x) if !x.has_moved() && x.piece_type() == PieceType::Rook && x.color() == color && destination.verify_with_checked_squares(&restrictions.check_squares).is_ok() => {
                (*self).0.push(Box::new(CastleMove::new(CastleType::Long, start_pos)))
            },
            _ => (),
        }
    }
}

pub struct Attacked(pub HashSet<PiecePosition>);

impl Display for Attacked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{}, ", elem);
        }
        Ok(())
    }
}

impl Attacked {
    pub fn get_attacked_squares(board: &Board, color: Color) -> Self {
        let mut res = Attacked(HashSet::new());
        for i in 0..board.board.len() {
            for j in 0..board.board[i].len() {
                match &board.board[i][j] {
                    Some(s) if s.color() == color => s.get_attacked(board, &mut res),
                    _ => (),
                }
            }
        }
        res
    }

    fn add_attacked_series(&mut self, board: &Board, start_pos: PiecePosition, dirs: &[MoveDir], max_moves: usize, color: Color) {
        for direction in dirs {
            let mut move_coords = start_pos;
            let translation: [i8; 2] = <&MoveDir as TryInto<[i8; 2]>>::try_into(direction).unwrap();
            for _ in 0..max_moves {
                move_coords += translation;
                if let Ok(_) = move_coords.verify_bounds() {
                    self.0.insert(move_coords);
                } else { break }
                match board.get_square(move_coords) {
                    Some(piece) if piece.piece_type() != PieceType::King || piece.color() == color => break,
                    _ => (),
                };
            }
        }
    }
    
    fn insert_attacked_squares_relative(&mut self, start_pos: PiecePosition, squares: &[[i8; 2]]) {
        for elem in squares {
            let pos = start_pos + *elem;
            if let Ok(_) = pos.verify_bounds() {
                self.0.insert(pos);
            }
        }
    }
}

pub struct CheckSquares(HashSet<PiecePosition>, pub u8);
pub struct EnPassantCheckSquare(Option<PiecePosition>);

impl Display for CheckSquares {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{}, ", elem);
        }
        Ok(())
    }
}

impl CheckSquares {
    pub fn get_all_checked_squares(board: &Board, color: Color) -> Self {
        let mut res = CheckSquares(HashSet::new(), 0);
        for i in 0..board.board.len() {
            for j in 0..board.board[i].len() {
                match &board.board[i][j] {
                    Some(s) if s.color() == color => s.get_checked(board, &mut res),
                    _ => (),
                }
            }
        }
        res
    }

    fn get_checked_from_a_piece(&mut self, board: &Board, start_pos: PiecePosition, dirs: &[MoveDir], color: Color, max_moves: usize) {
        for direction in dirs {
            let translation = <&MoveDir as TryInto<[i8; 2]>>::try_into(direction).unwrap();
            let mut move_coords = start_pos;
            let mut temp_res: Vec<PiecePosition> = vec![move_coords];
            for _ in 0..max_moves {
                move_coords += translation;
                if let Err(_) = move_coords.verify_bounds() { break }
                match board.get_square(move_coords) {
                    Some(piece) if piece.piece_type() == PieceType::King && piece.color() != color => {
                        temp_res.into_iter().for_each(|square| {
                            self.0.insert(square);
                        });
                        self.1 += 1;
                        return;
                    },
                    Some(_) => break,
                    None => temp_res.push(move_coords),
                }
            }
        }
    }
}

impl EnPassantCheckSquare {
    pub fn get_all_en_passant_check_squares(board: &Board, color: Color) -> Self {
        let mut res = EnPassantCheckSquare(None);
        for i in 0..board.board.len() {
            for j in 0..board.board[i].len() {
                match &board.board[i][j] {
                    Some(s) if s.color() == color => s.get_en_passant_checked(board, &mut res),
                    _ => (),
                }
            }
        }
        res
    }

    fn get_en_passantable_check_square(&mut self, board: &Board, start_pos: PiecePosition, color: Color) {
        let (captures, en_passant_square) = match color {
            Color::White => ([[-1, 1], [1, 1]], [0, -1]),
            Color::Black => ([[-1, -1], [1, -1]], [0, 1]),
        };
        for capture in captures {
            match board.get_square(start_pos + capture) {
                Some(piece) if piece.piece_type() == PieceType::King && piece.color() != color => {
                    self.0 = Some(start_pos + en_passant_square);
                    return;
                },
                _ => (),
            }
        }
    }
}

pub struct PinSquares(pub HashMap<PiecePosition, PinDir>);

impl Display for PinSquares {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for elem in self.0.iter() {
            let _ = write!(f, "{} ({:?}), ", elem.0, elem.1);
        }
        Ok(())
    }
}

impl PinSquares {
    pub fn get_all_pin_squares(board: &Board, color: Color) -> Self {
        let mut res = PinSquares(HashMap::new());
        for i in 0..board.board.len() {
            for j in 0..board.board[i].len() {
                match &board.board[i][j] {
                    Some(s) if s.color() == color => s.get_pins(board, &mut res),
                    _ => (),
                }
            }
        }
        res
    }

    fn get_pins_from_a_piece(&mut self, board: &Board, start_pos: PiecePosition, dirs: &[MoveDir], color: Color, max_moves: usize) {
        for direction in dirs {
            let mut en_passant_edge_case = None;
            let translation = <&MoveDir as TryInto<[i8; 2]>>::try_into(direction).unwrap();
            let mut move_coords = start_pos;
            let mut temp_res: Vec<PiecePosition> = vec![move_coords];
            let mut pieces: Vec<&dyn ChessPiece> = vec![];
            for _ in 0..max_moves {
                move_coords += translation;
                if let Err(_) = move_coords.verify_bounds() { break }
                match board.get_square(move_coords) {
                    Some(piece) if piece.color() != color && piece.piece_type() == PieceType::King => {
                        if pieces.len() == 0 { break } else {
                            if let Some(sq) = en_passant_edge_case {
                                self.0.insert(sq, PinDir::EnPassantBlock);
                                return
                            }
                            temp_res.into_iter().for_each(|piece| {
                                self.0.insert(piece, <&MoveDir as Into<PinDir>>::into(direction));
                            });
                            return
                        }
                    },
                    Some(piece) if piece.color() != color => {
                        if pieces.len() == 0 {
                            pieces.push(piece);
                            temp_res.push(move_coords); 
                        } else { break }
                    },
                    Some(piece) if piece.is_enpassantable() => {
                        if !(*direction == MoveDir::Left || *direction == MoveDir::Right) { break }
                        let mut pawn_neighbors = 0;
                        for translation in [Vec2(-1, 0), Vec2(1, 0)] {
                            if let Some(n) = board.get_square(move_coords + translation) {
                                if n.piece_type() == PieceType::Pawn && n.color() != color {
                                    pawn_neighbors += 1;
                                    en_passant_edge_case = Some(move_coords + translation);
                                }
                            };
                        }
                        // println!("Amount of pawn neighbors: {}", pawn_neighbors);
                        if pawn_neighbors != 1 {
                            break
                        }
                    },
                    Some(_) => break,
                    _ => { temp_res.push(move_coords) },
                }
            }
        }
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

impl MoveRestrictionData {
    pub fn new(board: &Board, color: Color) -> Self {
        Self {
            attacked: Attacked::get_attacked_squares(board, color),
            check_squares: CheckSquares::get_all_checked_squares(board, color),
            en_passant_check: EnPassantCheckSquare::get_all_en_passant_check_squares(board, color),
            pin_squares: PinSquares::get_all_pin_squares(board, color),
        }
    }
}

pub trait ChessPiece: fmt::Display + fmt::Debug + DynClone {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restriction: &MoveRestrictionData);
    fn get_attacked(&self, board: &Board, attacked: &mut Attacked);
    fn get_checked(&self, board: &Board, checked: &mut CheckSquares) {}
    fn get_en_passant_checked(&self, board: &Board, checked: &mut EnPassantCheckSquare) {}
    fn get_pins(&self, board: &Board, pins: &mut PinSquares) {}
    fn color(&self) -> Color;
    fn is_enpassantable(&self) -> bool { false }
    fn has_moved(&self) -> bool { false }
    fn piece_type(&self) -> PieceType;
    fn fen_piece_type(&self) -> FenPieceType;
    fn position(&self) -> PiecePosition;
    fn pin_direction<'a>(&'a self, data: &'a MoveRestrictionData) -> Option<&'a PinDir> {
        data.pin_squares.0.get(&self.position())
    }
    fn can_promote(&self) -> bool { false }
    fn set_position(&mut self, pos: PiecePosition) -> ();
    fn set_en_passantable(&mut self, val: bool) -> () {}
    fn set_moved(&mut self, val: bool) -> () {}
    fn mating_material_points(&self) -> u8 { 3 }
}

dyn_clone::clone_trait_object!(ChessPiece);

const WHITE_PAWN_DOUBLE_MOVE_RANK: i8 = 1;
const BLACK_PAWN_DOUBLE_MOVE_RANK: i8 = 6;
const WHITE_PROMOTION_RANK: i8 = 6;
const BLACK_PROMOTION_RANK: i8 = 1;
const MAX_MOVES_IN_A_SERIES: usize = 7;

const WHITE_PAWN_MOVES: [MoveDir; 1] = [MoveDir::Up];
const WHITE_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::UpLeft, MoveDir::UpRight];

const BLACK_PAWN_MOVES: [MoveDir; 1] = [MoveDir::Down];
const BLACK_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::DownLeft, MoveDir::DownRight];

const KNIGHT_MOVES: [[i8; 2]; 8] = [[2, 1], [1, 2], [-2, 1], [-1, 2], [2, -1], [1, -2], [-2, -1], [-1, -2]];

const KING_MOVES: [MoveDir; 8] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right, MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

const ROOK_MOVES: [MoveDir; 4] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

const BISHOP_MOVES: [MoveDir; 4] = [MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

const QUEEN_MOVES: [MoveDir; 8] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right, MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

impl ChessPiece for Pawn {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        let pin_direction = self.pin_direction(restrictions);
        match self.color {
            Color::White => {
                let white_moves = MoveDirs(BTreeSet::from(WHITE_PAWN_MOVES)).intersection_with_pin_dir(pin_direction);
                let white_captures = MoveDirs(BTreeSet::from(WHITE_PAWN_CAPTURES)).intersection_with_pin_dir(pin_direction);
                let series_length = if self.position.rank == WHITE_PAWN_DOUBLE_MOVE_RANK { 2 } else { 1 };
                moves.add_move_series(board, self.position, self.color, &white_moves, self.piece_type(), series_length, restrictions);
                moves.add_captures(board, self.position, self.color, &white_captures, self.piece_type(), restrictions);
                if !(pin_direction == Some(&PinDir::EnPassantBlock)) {
                    moves.add_en_passant(board, self.position, self.color, &white_captures, restrictions);
                }
            },
            Color::Black => {
                let black_moves = MoveDirs(BTreeSet::from(BLACK_PAWN_MOVES)).intersection_with_pin_dir(pin_direction);
                let black_captures = MoveDirs(BTreeSet::from(BLACK_PAWN_CAPTURES)).intersection_with_pin_dir(pin_direction);
                let series_length = if self.position.rank == BLACK_PAWN_DOUBLE_MOVE_RANK { 2 } else { 1 };
                moves.add_move_series(board, self.position, self.color, &black_moves, self.piece_type(), series_length, restrictions);
                moves.add_captures(board, self.position, self.color, &black_captures, self.piece_type(), restrictions);
                if !(pin_direction == Some(&PinDir::EnPassantBlock)) {
                    moves.add_en_passant(board, self.position, self.color, &black_captures, restrictions);
                }
            },
        };
    }
    
    fn get_attacked(&self, board: &Board, attacked: &mut Attacked) {
        match self.color {
            Color::White => attacked.insert_attacked_squares_relative(self.position, &[[-1, 1], [1, 1]]),
            Color::Black => attacked.insert_attacked_squares_relative(self.position, &[[-1, -1], [1, -1]]),
        }
    }

    fn get_checked(&self, board: &Board, checked: &mut CheckSquares) {
        match self.color {
            Color::White => checked.get_checked_from_a_piece(board, self.position, &WHITE_PAWN_CAPTURES, self.color, 1),
            Color::Black => checked.get_checked_from_a_piece(board, self.position, &BLACK_PAWN_CAPTURES, self.color, 1),
        }
    }

    fn get_en_passant_checked(&self, board: &Board, checked: &mut EnPassantCheckSquare) {
        if !self.enpassantable { return }
        checked.get_en_passantable_check_square(board, self.position, self.color);
    }

    fn color(&self) -> Color {
        self.color
    }

    fn is_enpassantable(&self) -> bool {
        self.enpassantable
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhitePawn,
            Color::Black => FenPieceType::BlackPawn,
        }
    }

    fn has_moved(&self) -> bool {
        false
    }

    fn position(&self) -> PiecePosition {
        self.position
    }

    fn can_promote(&self) -> bool {
        (self.position.rank == WHITE_PROMOTION_RANK && self.color == Color::White) || (self.position.rank == BLACK_PROMOTION_RANK && self.color == Color::Black)
    }

    fn set_position(&mut self, pos: PiecePosition) -> () {
        self.position = pos;
    }

    fn set_en_passantable(&mut self, val: bool) -> () {
        self.enpassantable = val;
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

impl ChessPiece for Knight {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        if self.pin_direction(restrictions).is_some() { return };
        moves.add_special_moves(board, self.position, &KNIGHT_MOVES, self.piece_type(), restrictions, self.color);
    }

    fn get_attacked(&self, board: &Board, attacked: &mut Attacked) {
        attacked.insert_attacked_squares_relative(self.position, &KNIGHT_MOVES);
    }

    fn get_checked(&self, board: &Board, checked: &mut CheckSquares) {
        for translation in KNIGHT_MOVES {
            let destination = self.position + translation;
            if let Err(_) = destination.verify_bounds() { continue }
            match board.get_square(destination) {
                Some(piece) if piece.piece_type() == PieceType::King && piece.color() != self.color => {
                    checked.0.insert(self.position);
                    checked.1 += 1;
                    return
                },
                _ => (),
            }
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Knight
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteKnight,
            Color::Black => FenPieceType::BlackKnight,
        }
    }

    fn position(&self) -> PiecePosition {
        self.position
    }

    fn set_position(&mut self, pos: PiecePosition) -> () {
        self.position = pos;
    }

    fn mating_material_points(&self) -> u8 { 1 }
}

impl Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "n"),
            Color::White => write!(f, "N"),
        }
    }
}

impl ChessPiece for King {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        moves.add_move_series(board, self.position, self.color, &MoveDirs(BTreeSet::from(KING_MOVES)), self.piece_type(), 1, restrictions);
        if self.has_moved { return }
        moves.add_castling(board, self.position, restrictions, self.color);
    }

    fn get_attacked(&self, board: &Board, attacked: &mut Attacked) {
        attacked.insert_attacked_squares_relative(self.position, &[[-1, 1], [0, 1], [1, 1], [-1, 0], [1, 0], [-1, -1], [0, -1], [1, -1]])
    }

    fn color(&self) -> Color {
        self.color
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn piece_type(&self) -> PieceType {
        PieceType::King
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteKing,
            Color::Black => FenPieceType::BlackKing,
        }
    }

    fn position(&self) -> PiecePosition {
        self.position
    }

    fn set_position(&mut self, pos: PiecePosition) -> () {
        self.position = pos;
    }

    fn set_moved(&mut self, val: bool) -> () {
        self.has_moved = val;
    }

    fn mating_material_points(&self) -> u8 { 0 }
}

impl Display for King {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "k"),
            Color::White => write!(f, "K"),
        }
    }
}

impl ChessPiece for Rook {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        let pin_direction = self.pin_direction(restrictions);
        let enabled_moves = MoveDirs(BTreeSet::from(ROOK_MOVES)).intersection_with_pin_dir(pin_direction);
        moves.add_move_series(board, self.position, self.color, &enabled_moves, self.piece_type(), MAX_MOVES_IN_A_SERIES, restrictions);
    }

    fn get_attacked(&self, board: &Board, attacked: &mut Attacked) {
        attacked.add_attacked_series(board, self.position, &ROOK_MOVES, MAX_MOVES_IN_A_SERIES, self.color)
    }

    fn get_checked(&self, board: &Board, checked: &mut CheckSquares) {
        checked.get_checked_from_a_piece(board, self.position, &ROOK_MOVES, self.color, MAX_MOVES_IN_A_SERIES);
    }

    fn get_pins(&self, board: &Board, pins: &mut PinSquares) {
        pins.get_pins_from_a_piece(board, self.position, &ROOK_MOVES, self.color, MAX_MOVES_IN_A_SERIES);
    }

    fn color(&self) -> Color {
        self.color
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Rook
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteRook,
            Color::Black => FenPieceType::BlackRook,
        }
    }

    fn position(&self) -> PiecePosition {
        self.position
    }

    fn set_position(&mut self, pos: PiecePosition) -> () {
        self.position = pos;
    }

    fn set_moved(&mut self, val: bool) -> () {
        self.has_moved = val;
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

impl ChessPiece for Bishop {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        let pin_direction = self.pin_direction(restrictions);
        let enabled_moves = MoveDirs(BTreeSet::from(BISHOP_MOVES)).intersection_with_pin_dir(pin_direction);
        moves.add_move_series(board, self.position, self.color, &enabled_moves, self.piece_type(), MAX_MOVES_IN_A_SERIES, restrictions);
    }

    fn get_attacked(&self, board: &Board, attacked: &mut Attacked) {
        attacked.add_attacked_series(board, self.position, &BISHOP_MOVES, MAX_MOVES_IN_A_SERIES, self.color)
    }

    fn get_checked(&self, board: &Board, checked: &mut CheckSquares) {
        checked.get_checked_from_a_piece(board, self.position, &BISHOP_MOVES, self.color, MAX_MOVES_IN_A_SERIES);
    }

    fn get_pins(&self, board: &Board, pins: &mut PinSquares) {
        pins.get_pins_from_a_piece(board, self.position, &BISHOP_MOVES, self.color, MAX_MOVES_IN_A_SERIES);
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Bishop
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteBishop,
            Color::Black => FenPieceType::BlackBishop,
        }
    }

    fn position(&self) -> PiecePosition {
        self.position
    }

    fn set_position(&mut self, pos: PiecePosition) -> () {
        self.position = pos;
    }

    fn mating_material_points(&self) -> u8 { 2 }
}

impl Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "b"),
            Color::White => write!(f, "B"),
        }
    }
}

impl ChessPiece for Queen {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        let pin_direction = self.pin_direction(restrictions);
        let enabled_moves = MoveDirs(BTreeSet::from(QUEEN_MOVES)).intersection_with_pin_dir(pin_direction);
        moves.add_move_series(board, self.position, self.color, &enabled_moves, self.piece_type(), MAX_MOVES_IN_A_SERIES, restrictions);
    }

    fn get_attacked(&self, board: &Board, attacked: &mut Attacked) {
        attacked.add_attacked_series(board, self.position, &QUEEN_MOVES, MAX_MOVES_IN_A_SERIES, self.color)
    }

    fn get_checked(&self, board: &Board, checked: &mut CheckSquares) {
        checked.get_checked_from_a_piece(board, self.position, &QUEEN_MOVES, self.color, MAX_MOVES_IN_A_SERIES);
    }

    fn get_pins(&self, board: &Board, pins: &mut PinSquares) {
        pins.get_pins_from_a_piece(board, self.position, &QUEEN_MOVES, self.color, MAX_MOVES_IN_A_SERIES);
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Queen
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteQueen,
            Color::Black => FenPieceType::BlackQueen,
        }
    }

    fn position(&self) -> PiecePosition {
        self.position
    }

    fn set_position(&mut self, pos: PiecePosition) -> () {
        self.position = pos;
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