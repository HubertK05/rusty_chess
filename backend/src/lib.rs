use std::{ops::{Add, AddAssign}, fmt::{Display, self}};

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
#[derive(Debug, Clone, Copy)]
pub struct PiecePosition {
    file: i8,
    rank: i8,
}

impl PiecePosition {
    fn new(file: usize, rank: usize) -> Self {
        Self {
            file: file.try_into().unwrap(),
            rank: rank.try_into().unwrap(),
        }
    }

    fn verify_bounds(&self) -> Result<(), MoveError> {
        if self.file < 0 || self.file > 7 || self.rank < 0 || self.rank > 7 { Err(MoveError::OutOfBounds) } else { Ok(()) }
    }
}

impl From<[i8; 2]> for PiecePosition {
    fn from(val: [i8; 2]) -> Self {
        Self {
            file: val[0].try_into().unwrap(),
            rank: val[1].try_into().unwrap(),
        }
    }
}

impl Add<[i8; 2]> for PiecePosition {
    type Output = PiecePosition;

    fn add(self, rhs: [i8; 2]) -> Self::Output {
        let file = self.file as i8 + rhs[0];
        let rank = self.rank as i8 + rhs[1];
        PiecePosition {
            file: file.try_into().unwrap(),
            rank: rank.try_into().unwrap(),
        }
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

#[derive(Debug)]
struct Pawn {
    color: Color,
    position: PiecePosition,
    enpassantable: bool,
}

#[derive(Debug)]
struct Knight {
    color: Color,
    position: PiecePosition,
}

#[derive(Debug)]
struct Bishop {
    color: Color,
    position: PiecePosition,
}

#[derive(Debug)]
struct Rook {
    color: Color,
    position: PiecePosition,
    has_moved: bool,
}

#[derive(Debug)]
struct Queen {
    color: Color,
    position: PiecePosition,
}

#[derive(Debug)]
struct King {
    color: Color,
    position: PiecePosition,
    has_moved: bool,
}
pub struct Board {
    pub board: [[Option<Box<dyn ChessPiece>>;8];8],
}

impl Board {
    fn new() -> Self {
        Self {
            board: Default::default()
        }
    }

    fn get_square<'a>(&'a self, position: PiecePosition) -> Option<&dyn ChessPiece> {
        self.board[position.file as usize][position.rank as usize].as_deref()
    }
    
    fn check_if_empty(&self, mut pos: PiecePosition, dir: MoveDir, max_moves: usize) -> bool {
        let transition: [i8; 2] = (&dir).try_into().unwrap();
        for _ in 0..max_moves {
            pos += transition;
            if self.get_square(pos).is_some() { return false };
        }
        true
    }
}

impl TryFrom<[[&str; 8]; 8]> for Board {
    type Error = BoardError;

    fn try_from(val: [[&str; 8]; 8]) -> Result<Board, Self::Error> {
        let mut res = Board::new();
        for i in 0..res.board.len() {
            for j in 0..8 {
                match val[7 - j][i] {
                    "" | " " | "." => (),
                    "p" => res.board[i][j] = Some(Box::new(Pawn { color: Color::Black, position: PiecePosition::new(i, j), enpassantable: false})),
                    "n" => res.board[i][j] = Some(Box::new(Knight { color: Color::Black, position: PiecePosition::new(i, j)})),
                    "b" => res.board[i][j] = Some(Box::new(Bishop { color: Color::Black, position: PiecePosition::new(i, j)})),
                    "r" => res.board[i][j] = Some(Box::new(Rook { color: Color::Black, position: PiecePosition::new(i, j), has_moved: false})),
                    "q" => res.board[i][j] = Some(Box::new(Queen { color: Color::Black, position: PiecePosition::new(i, j)})),
                    "k" => res.board[i][j] = Some(Box::new(King { color: Color::Black, position: PiecePosition::new(i, j), has_moved: false})),
                    "P" => res.board[i][j] = Some(Box::new(Pawn { color: Color::White, position: PiecePosition::new(i, j), enpassantable: false})),
                    "N" => res.board[i][j] = Some(Box::new(Knight { color: Color::White, position: PiecePosition::new(i, j)})),
                    "B" => res.board[i][j] = Some(Box::new(Bishop { color: Color::White, position: PiecePosition::new(i, j)})),
                    "R" => res.board[i][j] = Some(Box::new(Rook { color: Color::White, position: PiecePosition::new(i, j), has_moved: false})),
                    "Q" => res.board[i][j] = Some(Box::new(Queen { color: Color::White, position: PiecePosition::new(i, j)})),
                    "K" => res.board[i][j] = Some(Box::new(King { color: Color::White, position: PiecePosition::new(i, j), has_moved: false})),
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

#[derive(Debug)]
pub struct Move {
    pub piece: PieceType,
    pub from: PiecePosition,
    pub to: PiecePosition,
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_letter = self.piece;
        let file_letter = (self.to.file as u8 + 97) as char;
        let rank_number = self.to.rank + 1;
        write!(f, "{piece_letter}{file_letter}{rank_number}")
    }
}

impl Move {
    fn new(piece: PieceType, from: PiecePosition, to: PiecePosition) -> Self {
        Self {
            piece,
            from,
            to,
        }
    }

    fn new_relative(piece: PieceType, from: PiecePosition, coords: [i8; 2]) -> Self {
        Self {
            piece,
            from,
            to: from + coords,
        }
    }
}

#[derive(Debug)]
enum MoveError {
    OutOfBounds,
    MoveBlocked,
}

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

#[derive(Debug)]
pub struct Moves(pub Vec<Move>);

impl Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            let _ = write!(f, "{}, ", &self.0[i]);
        }
        Ok(())
    }
}
impl Moves {
    pub fn get_all_moves(board: &Board, color: Color) -> Self {
        let mut res = Moves(vec![]);
        for i in 0..board.board.len() {
            for j in 0..board.board[i].len() {
                match &board.board[i][j] {
                    Some(s) if s.color() == color => s.get_moves(board, &mut res),
                    _ => (),
                }
            }
        }
        res
    }

    fn add_move(&mut self, board: &Board, start_pos: PiecePosition, coords: [i8; 2], piece_type: PieceType) -> Result<(), MoveError> {
        let destination = start_pos + coords;
        if destination.verify_bounds().is_err() { return Err(MoveError::OutOfBounds) }
        if board.get_square(start_pos + coords).is_none() {
            Ok((*self).0.push(Move::new(piece_type, start_pos, destination)))
        } else { Err(MoveError::MoveBlocked) }
    }

    fn add_capture(&mut self, board: &Board, color: Color, start_pos: PiecePosition, coords: [i8; 2], piece_type: PieceType) -> Result<(), MoveError> {
        let destination = start_pos + coords;
        if destination.verify_bounds().is_err() { return Err(MoveError::OutOfBounds) }
        let piece = board.get_square(start_pos + coords);
        match piece {
            Some(x) if x.color() != color => Ok((*self).0.push(Move::new(piece_type, start_pos, destination))),
            _ => Err(MoveError::MoveBlocked),
        }
    }

    fn add_moves(&mut self, board: &Board, start_pos: PiecePosition, coords: &[[i8; 2]], piece_type: PieceType) {
        for elem in coords {
            let _ = self.add_move(board, start_pos, *elem, piece_type);
        }
    }

    fn add_move_series(&mut self, board: &Board, start_pos: PiecePosition, color: Color, dirs: &[MoveDir], piece_type: PieceType, max_moves: usize) {
        for elem in dirs {
            let mut move_coords = start_pos;
            let mut prev_move_coords = move_coords;
            let translation: [i8; 2] = <&MoveDir as TryInto<[i8; 2]>>::try_into(elem).unwrap();
            for _ in 0..max_moves {
                prev_move_coords = move_coords;
                move_coords += translation;
                if let Err(_) = self.add_move(board, prev_move_coords, translation, piece_type) { break };
            }
            if piece_type != PieceType::Pawn {
                let _ = self.add_capture(board, color, prev_move_coords,   translation, piece_type);
            }
        }
    }

    fn add_captures(&mut self, board: &Board, start_pos: PiecePosition, color: Color, coords: &[[i8; 2]], piece_type: PieceType) {
        for elem in coords {
            let _ = self.add_capture(board, color, start_pos, *elem, piece_type);
        }
    }

    fn add_en_passant(&mut self, board: &Board, start_pos: PiecePosition, color: Color) {
        for elem in [[1, 0], [-1, 0]] {
            if (start_pos + elem).verify_bounds().is_err() { continue }
            let piece = board.get_square(start_pos + elem);
            match piece {
                Some(x) if x.color() != color && x.is_enpassantable() => {
                    let move_coords = match color {
                        Color::White => [elem[0], elem[1] + 1],
                        Color::Black => [elem[0], elem[1] - 1],
                    };
                    (*self).0.push(Move::new_relative(PieceType::Pawn, start_pos, move_coords))
                },
                _ => (),
            }
        }
    }

    fn add_castling(&mut self, board: &Board, start_pos: PiecePosition) {
        if board.check_if_empty(start_pos, MoveDir::Right, 2) {
            let piece = board.get_square(start_pos + [3, 0]);
            match piece {
                Some(x) if !x.has_moved() || x.piece_type() == PieceType::Rook => {
                    (*self).0.push(Move::new_relative(PieceType::King, start_pos, [2, 0]))
                },
                _ => (),
            }
        }
        if !board.check_if_empty(start_pos, MoveDir::Left, 3) { return };
        let piece = board.get_square(start_pos + [-4, 0]);
        match piece {
            Some(x) if !x.has_moved() || x.piece_type() == PieceType::Rook => {
                (*self).0.push(Move::new_relative(PieceType::King, start_pos, [-3, 0]))
            },
            _ => (),
        }
    }
}
pub trait ChessPiece: fmt::Display + fmt::Debug {
    fn get_moves(&self, board: &Board, moves: &mut Moves);
    fn color(&self) -> Color;
    fn is_enpassantable(&self) -> bool { false }
    fn has_moved(&self) -> bool { false }
    fn piece_type(&self) -> PieceType;
}
const WHITE_PAWN_DOUBLE_MOVE_RANK: i8 = 1;
const BLACK_PAWN_DOUBLE_MOVE_RANK: i8 = 6;
const MAX_MOVES_IN_A_SERIES: usize = 7;
impl ChessPiece for Pawn {
    fn get_moves(&self, board: &Board, moves: &mut Moves) {
        match self.color {
            Color::White => {
                let series_length = if self.position.rank == WHITE_PAWN_DOUBLE_MOVE_RANK { 2 } else { 1 };
                moves.add_move_series(board, self.position, self.color, &[MoveDir::Up], self.piece_type(), series_length);
                moves.add_captures(board, self.position, self.color, &[[-1, 1], [1, 1]], self.piece_type());
            },
            Color::Black => {
                let series_length = if self.position.rank == BLACK_PAWN_DOUBLE_MOVE_RANK { 2 } else { 1 };
                moves.add_move_series(board, self.position, self.color, &[MoveDir::Down], self.piece_type(), series_length);
                moves.add_captures(board, self.position, self.color, &[[-1, -1], [1, -1]], self.piece_type());
            },
        };
        moves.add_en_passant(board, self.position, self.color);
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

    fn has_moved(&self) -> bool {
        false
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
    fn get_moves(&self, board: &Board, moves: &mut Moves) {
        moves.add_moves(board, self.position, &[[2, 1], [1, 2], [-2, 1], [-1, 2], [2, -1], [1, -2], [-2, -1], [-1, -2]], self.piece_type());
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Knight
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

impl ChessPiece for King {
    fn get_moves(&self, board: &Board, moves: &mut Moves) {
        moves.add_moves(board, self.position, &[[-1, 1], [0, 1], [1, 1], [-1, 0], [1, 0], [-1, -1], [0, -1], [1, -1]], self.piece_type());
        moves.add_castling(board, self.position);
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
    fn get_moves(&self, board: &Board, moves: &mut Moves) {
        moves.add_move_series(board, self.position, self.color, &[MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right], self.piece_type(), MAX_MOVES_IN_A_SERIES);
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
    fn get_moves(&self, board: &Board, moves: &mut Moves) {
        moves.add_move_series(board, self.position, self.color, &[MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight], self.piece_type(), MAX_MOVES_IN_A_SERIES);
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Bishop
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

impl ChessPiece for Queen {
    fn get_moves(&self, board: &Board, moves: &mut Moves) {
        moves.add_move_series(board, self.position, self.color, &[MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right, MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight], self.piece_type(), MAX_MOVES_IN_A_SERIES);
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Queen
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