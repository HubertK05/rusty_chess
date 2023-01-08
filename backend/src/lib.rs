pub mod move_register;
use std::{ops::{Add, AddAssign, Mul, DerefMut}, fmt::{Display, self}, collections::{HashSet, HashMap, BTreeSet}, hash::Hash};
use move_register::{Move, ChessMove, Capture, EnPassantMove, CastleMove, CastleType, PromotionMove, CastleLength};
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

impl TryFrom<&str> for FenPieceType {
    type Error = BoardError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "P" => Ok(FenPieceType::WhitePawn),
            "N" => Ok(FenPieceType::WhiteKnight),
            "K" => Ok(FenPieceType::WhiteKing),
            "R" => Ok(FenPieceType::WhiteRook),
            "B" => Ok(FenPieceType::WhiteBishop),
            "Q" => Ok(FenPieceType::WhiteQueen),
            "p" => Ok(FenPieceType::BlackPawn),
            "n" => Ok(FenPieceType::BlackKnight),
            "k" => Ok(FenPieceType::BlackKing),
            "r" => Ok(FenPieceType::BlackRook),
            "b" => Ok(FenPieceType::BlackBishop),
            "q" => Ok(FenPieceType::BlackQueen),
            _ => Err(BoardError::ConversionFailure),
        }
    }
}

impl TryFrom<char> for Box<dyn ChessPiece> {
    type Error = BoardError;

    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            'P' => Ok(Box::new(Pawn { position: Vec2(0, 0), color: Color::White})),
            'N' => Ok(Box::new(Knight { position: Vec2(0, 0), color: Color::White })),
            'K' => Ok(Box::new(King { position: Vec2(0, 0), color: Color::White })),
            'R' => Ok(Box::new(Rook { position: Vec2(0, 0), color: Color::White })),
            'B' => Ok(Box::new(Bishop { position: Vec2(0, 0), color: Color::White })),
            'Q' => Ok(Box::new(Queen { position: Vec2(0, 0), color: Color::White })),
            'p' => Ok(Box::new(Pawn { position: Vec2(0, 0), color: Color::Black})),
            'n' => Ok(Box::new(Knight { position: Vec2(0, 0), color: Color::Black })),
            'k' => Ok(Box::new(King { position: Vec2(0, 0), color: Color::Black })),
            'r' => Ok(Box::new(Rook { position: Vec2(0, 0), color: Color::Black })),
            'b' => Ok(Box::new(Bishop { position: Vec2(0, 0), color: Color::Black })),
            'q' => Ok(Box::new(Queen { position: Vec2(0, 0), color: Color::Black })),
            _ => Err(BoardError::ConversionFailure),
        }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2(pub i8, pub i8);

impl Vec2 {
    fn verify_bounds(&self) -> Result<(), MoveError> {
        if self.0 < 0 || self.0 > 7 || self.1 < 0 || self.1 > 7 { Err(MoveError::OutOfBounds) } else { Ok(()) }
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

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl TryFrom<&str> for Vec2 {
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
                                .map_err(|_e| BoardError::ConversionFailure)?;

        Ok(Vec2(file_number, rank_number))
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
struct Pawn {
    color: Color,
    position: Vec2,
}

#[derive(Debug, Clone)]
struct Knight {
    color: Color,
    position: Vec2,
}

#[derive(Debug, Clone)]
struct Bishop {
    color: Color,
    position: Vec2,
}

#[derive(Debug, Clone)]
struct Rook {
    color: Color,
    position: Vec2,
}

#[derive(Debug, Clone)]
struct Queen {
    color: Color,
    position: Vec2,
}

#[derive(Debug, Clone)]
struct King {
    color: Color,
    position: Vec2,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[Option<Box<dyn ChessPiece>>;8];8],
    pub turn: Color,
    pub castling: HashSet<CastleType>,
    pub en_passant_square: Option<Vec2>,
    pub half_move_timer_50: u8,
    pub full_move_number: u16,
    pub mating_material: (u8, u8),
}

impl Board {
    fn get_square<'a>(&'a self, position: Vec2) -> Option<&dyn ChessPiece> {
        let Some(rank) = self.board.get(position.0 as usize) else {
            return None;
        };

        match rank.get(position.1 as usize) {
            Some(piece) => piece.as_deref(),
            None => None,
        }
    }
    
    fn check_castling(&self, mut pos: Vec2, attacked: &Attacked, castle_length: CastleLength) -> bool {
        let transition = match castle_length {
            CastleLength::Short => Vec2(1, 0),
            CastleLength::Long => Vec2(-1, 0),
        };
        for _ in 0..2 {
            pos += transition;
            if self.get_square(pos).is_some() || attacked.0.contains(&pos) { return false };
        }
        if castle_length == CastleLength::Long {
            pos += transition;
            if self.get_square(pos).is_some() { return false };
        }
        true
    }

    fn set_en_passant_square(&mut self, val: Option<Vec2>) {
        self.en_passant_square = val;
    }

    fn modify_castling_rights(&mut self) {
        if self.get_square(Vec2(4, 0)).is_none() {
            self.castling.remove(&CastleType::WhiteLong);
            self.castling.remove(&CastleType::WhiteShort);
        }

        if self.get_square(Vec2(4, 7)).is_none() {
            self.castling.remove(&CastleType::BlackLong);
            self.castling.remove(&CastleType::BlackShort);
        }

        for (position, piece_type, castle_type) in [
            (Vec2(0, 0), FenPieceType::WhiteRook, CastleType::WhiteLong),
            (Vec2(7, 0), FenPieceType::WhiteRook, CastleType::WhiteShort),
            (Vec2(0, 7), FenPieceType::BlackRook, CastleType::BlackLong),
            (Vec2(7, 7), FenPieceType::BlackRook, CastleType::BlackShort),
        ] {
            match self.get_square(position) {
                Some(p) if p.fen_piece_type() == piece_type => (),
                _ => { self.castling.remove(&castle_type); },
            }
        }
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
            for rank in 0..8 {
                let piece = val.get_square(Vec2(rank, file));
                if let Some(p) = piece {
                    if empty_counter != 0 {
                        res.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }
                    res.push_str(&p.fen_piece_type().to_string());
                } else { empty_counter += 1 }
            }
            if empty_counter != 0 { res.push_str(&empty_counter.to_string()) }
            if file != 0 { res.push('/') }
        }

        match val.turn {
            Color::White => res.push_str(" w "),
            Color::Black => res.push_str(" b "),
        }

        let mut castling_rights = String::new();
        if val.castling.contains(&CastleType::WhiteShort) { castling_rights.push('K') }
        if val.castling.contains(&CastleType::WhiteLong) { castling_rights.push('Q') }
        if val.castling.contains(&CastleType::BlackShort) { castling_rights.push('k') }
        if val.castling.contains(&CastleType::BlackLong) { castling_rights.push('q') }

        if castling_rights.is_empty() {
            res.push('-');
        } else {
            res.push_str(&castling_rights);
        }

        res.push_str(" - ");
        res.push_str(&val.half_move_timer_50.to_string());
        res.push(' ');
        res.push_str(&val.full_move_number.to_string());

        FenNotation(res)
    }
}

impl TryFrom<FenNotation> for Board {
    type Error = BoardError;

    fn try_from(val: FenNotation) -> Result<Self, Self::Error> {
        let board_data = val.0.split_whitespace().collect::<Vec<&str>>();
        let position = board_data[0];

        let mut rank = 7_usize;
        let mut file = 0_usize;

        let mut board: [[Option<Box<dyn ChessPiece>>; 8]; 8] = Default::default();
        let mut mating_material = (0, 0);

        for char in position.chars() {
            match char {
                '1'..='8' => file += char.to_string().parse::<usize>().map_err(|_e| BoardError::ConversionFailure)?,
                '/' => {
                    rank -= 1;
                    file = 0;
                },
                _ => {
                    let mut piece: Box<dyn ChessPiece> = char.try_into().map_err(|_e| BoardError::ConversionFailure)?;
                    piece.deref_mut().set_position(Vec2(file as i8, rank as i8));
                    match piece.color() {
                        Color::White => mating_material.0 += piece.mating_material_points(),
                        Color::Black => mating_material.1 += piece.mating_material_points(),
                    }
                    board[file][rank] = Some(piece);
                    file += 1;
                },
            }
        }

        let turn = match board_data[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => unreachable!(),
        };

        let mut castling = HashSet::new();
        for letter in board_data[2].chars() {
            match letter {
                'K' => castling.insert(CastleType::WhiteShort),
                'k' => castling.insert(CastleType::BlackShort),
                'Q' => castling.insert(CastleType::WhiteLong),
                'q' => castling.insert(CastleType::BlackLong),
                '-' => false,
                _ => return Err(BoardError::ConversionFailure),
            };
        }

        let en_passant_square = match board_data[3] {
            "-" => None,
            sq => Some(Vec2::try_from(sq)?),
        };

        let half_move_timer_50 = board_data[4].parse::<u8>().map_err(|_e| BoardError::ConversionFailure)?; 
        let full_move_number = board_data[5].parse::<u16>().map_err(|_e| BoardError::ConversionFailure)?;

        Ok(
            Board {
                board,
                turn,
                castling,
                en_passant_square,
                half_move_timer_50,
                full_move_number,
                mating_material,
            }
        )
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

pub struct MoveDirs(BTreeSet<MoveDir>);

impl MoveDirs {
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

#[derive(Debug, Clone)]
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
        let restrictions = MoveRestrictionData::new(board, color.opp());
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

    fn add_move(&mut self, board: &Board, piece: &impl ChessPiece, coords: Vec2, restrictions: &MoveRestrictionData) -> Result<(), MoveError> {
        let destination = piece.position() + coords;
        destination.verify_bounds()?;
        if board.get_square(destination).is_some() { return Err(MoveError::MoveBlocked) };
        destination.verify_with_checked_squares(&restrictions.check_squares)?;
        (*self).0.push(Box::new(Move::new(piece.piece_type(), piece.position(), destination)));
        Ok(())
    }

    fn add_capture(&mut self, board: &Board, color: Color, start_pos: Vec2, coords: Vec2, piece_type: PieceType, restrictions: &MoveRestrictionData) -> Result<(), MoveError> {
        let destination = start_pos + coords;
        destination.verify_bounds()?;
        destination.verify_with_checked_squares(&restrictions.check_squares)?;
        match board.get_square(destination) {
            Some(x) if x.color() == color.opp() => {
                (*self).0.push(Box::new(Capture::new(piece_type, start_pos, destination)));
            },
            _ => (),
        }
        Ok(())
    }

    fn add_move_series(&mut self, board: &Board, piece: &impl ChessPiece, dirs: &MoveDirs, max_moves: usize, restrictions: &MoveRestrictionData) {
        for elem in &dirs.0 {
            let translation: Vec2 = Vec2::from(elem);
            for i in 1..(max_moves + 1) {
                match self.add_move(board, piece, translation * (i as i8), restrictions) {
                    Err(MoveError::OutOfBounds) => break,
                    Err(MoveError::MoveBlocked) => {
                        let _ = self.add_capture(board, piece.color(), piece.position(), (translation * (i as i8)).into(), piece.piece_type(), restrictions);
                        break
                    },
                    _ => (),
                };
            }
        }
    }
}

pub struct Attacked(pub HashSet<Vec2>);

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

    fn add_attacked_series(&mut self, board: &Board, start_pos: Vec2, dirs: &[MoveDir], max_moves: usize, color: Color) {
        for direction in dirs {
            let mut move_coords = start_pos;
            let translation = Vec2::from(direction);
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
    
    fn insert_attacked_squares_relative(&mut self, start_pos: Vec2, squares: &[Vec2]) {
        for elem in squares {
            let pos = start_pos + *elem;
            if let Ok(_) = pos.verify_bounds() {
                self.0.insert(pos);
            }
        }
    }
}

pub struct CheckSquares(HashSet<Vec2>, pub u8);
pub struct EnPassantCheckSquare(Option<Vec2>);

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

    fn get_checked_from_a_piece(&mut self, board: &Board, start_pos: Vec2, dirs: &[MoveDir], color: Color, max_moves: usize) {
        for direction in dirs {
            let translation = Vec2::from(direction);
            let mut move_coords = start_pos;
            let mut temp_res: Vec<Vec2> = vec![move_coords];
            for _ in 0..max_moves {
                move_coords += translation;
                if let Err(_) = move_coords.verify_bounds() { break }
                match board.get_square(move_coords) {
                    Some(piece) if piece.piece_type() == PieceType::King && piece.color() == color.opp() => {
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

    fn get_en_passantable_check_square(&mut self, board: &Board, start_pos: Vec2, color: Color) {
        let (captures, en_passant_square) = match color {
            Color::White => ([Vec2(-1, 1), Vec2(1, 1)], Vec2(0, -1)),
            Color::Black => ([Vec2(-1, -1), Vec2(1, -1)], Vec2(0, 1)),
        };
        for capture in captures {
            match board.get_square(start_pos + capture) {
                Some(piece) if piece.piece_type() == PieceType::King && piece.color() == color.opp() => {
                    self.0 = Some(start_pos + en_passant_square);
                    return;
                },
                _ => (),
            }
        }
    }
}

pub struct PinSquares(pub HashMap<Vec2, PinDir>);

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

    fn get_pins_from_a_piece(&mut self, board: &Board, start_pos: Vec2, dirs: &[MoveDir], color: Color, max_moves: usize) {
        for direction in dirs {
            let mut en_passant_edge_case = None;
            let translation = Vec2::from(direction);
            let mut move_coords = start_pos;
            let mut temp_res: Vec<Vec2> = vec![move_coords];
            let mut pieces: Vec<&dyn ChessPiece> = vec![];
            for _ in 0..max_moves {
                move_coords += translation;
                if let Err(_) = move_coords.verify_bounds() { break }
                match board.get_square(move_coords) {
                    Some(piece) if piece.color() == color.opp() && piece.piece_type() == PieceType::King => {
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
                    Some(piece) if piece.color() == color.opp() => {
                        if pieces.len() == 0 {
                            pieces.push(piece);
                            temp_res.push(move_coords); 
                        } else { break }
                    },
                    Some(piece) => {
                        match board.en_passant_square {
                            Some(sq) => {
                                match piece.color() {
                                    Color::White => if sq != piece.position() + Vec2(0, -1) { break },
                                    Color::Black => if sq != piece.position() + Vec2(0, 1) { break },
                                }
                            },
                            _ => break,
                        }
                        if !(*direction == MoveDir::Left || *direction == MoveDir::Right) { break }
                        let mut pawn_neighbors = 0;
                        for translation in [Vec2(-1, 0), Vec2(1, 0)] {
                            if let Some(n) = board.get_square(move_coords + translation) {
                                if n.piece_type() == PieceType::Pawn && n.color() == color.opp() {
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
    fn get_checked(&self, _board: &Board, _checked: &mut CheckSquares) {}
    fn get_en_passant_checked(&self, _board: &Board, _checked: &mut EnPassantCheckSquare) {}
    fn get_pins(&self, _board: &Board, _pins: &mut PinSquares) {}
    fn color(&self) -> Color;
    fn piece_type(&self) -> PieceType;
    fn fen_piece_type(&self) -> FenPieceType;
    fn position(&self) -> Vec2;
    fn pin_direction<'a>(&'a self, data: &'a MoveRestrictionData) -> Option<&'a PinDir> {
        data.pin_squares.0.get(&self.position())
    }
    fn set_position(&mut self, pos: Vec2);
    fn mating_material_points(&self) -> u8 { 3 }
}

dyn_clone::clone_trait_object!(ChessPiece);

const WHITE_PAWN_DOUBLE_MOVE_RANK: i8 = 1;
const BLACK_PAWN_DOUBLE_MOVE_RANK: i8 = 6;
const WHITE_PROMOTION_RANK: i8 = 7;
const BLACK_PROMOTION_RANK: i8 = 0;
const MAX_MOVES_IN_A_SERIES: usize = 7;

const WHITE_PAWN_MOVES: [MoveDir; 1] = [MoveDir::Up];
const WHITE_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::UpLeft, MoveDir::UpRight];

const BLACK_PAWN_MOVES: [MoveDir; 1] = [MoveDir::Down];
const BLACK_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::DownLeft, MoveDir::DownRight];

const KNIGHT_MOVES: [Vec2; 8] = [Vec2(2, 1), Vec2(2, -1), Vec2(-2, 1), Vec2(-2, -1), Vec2(1, 2), Vec2(1, -2), Vec2(-1, 2), Vec2(-1, -2)];

const KING_MOVES: [MoveDir; 8] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right, MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

const ROOK_MOVES: [MoveDir; 4] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

const BISHOP_MOVES: [MoveDir; 4] = [MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

const QUEEN_MOVES: [MoveDir; 8] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right, MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

impl ChessPiece for Pawn {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        let pin_direction = self.pin_direction(restrictions);
        let move_dirs: MoveDirs;
        let capture_dirs: MoveDirs;
        let series_length: i32;
        let promotion_rank: i8;
        match self.color {
            Color::White => {
                move_dirs = MoveDirs(BTreeSet::from(WHITE_PAWN_MOVES)).intersection_with_pin_dir(pin_direction);
                capture_dirs = MoveDirs(BTreeSet::from(WHITE_PAWN_CAPTURES)).intersection_with_pin_dir(pin_direction);
                series_length = if self.position.1 == WHITE_PAWN_DOUBLE_MOVE_RANK { 2 } else { 1 };
                promotion_rank = WHITE_PROMOTION_RANK;
            }
            Color::Black => {
                move_dirs = MoveDirs(BTreeSet::from(BLACK_PAWN_MOVES)).intersection_with_pin_dir(pin_direction);
                capture_dirs = MoveDirs(BTreeSet::from(BLACK_PAWN_CAPTURES)).intersection_with_pin_dir(pin_direction);
                series_length = if self.position.1 == BLACK_PAWN_DOUBLE_MOVE_RANK { 2 } else { 1 };
                promotion_rank = BLACK_PROMOTION_RANK;
            }
        }

        for elem in &move_dirs.0 {
            for i in 1..series_length + 1 {
                let destination = self.position + Vec2::from(elem) * (i as i8);
                if destination.verify_bounds().is_err() { break };
                match board.get_square(destination) {
                    None => {
                        if destination.verify_with_checked_squares(&restrictions.check_squares).is_err() { continue };
                        if destination.1 == promotion_rank {
                            moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Queen, self.color)));
                            moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Knight, self.color)));
                            moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Bishop, self.color)));
                            moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Rook, self.color)));
                        } else {
                            moves.0.push(Box::new(Move::new(PieceType::Pawn, self.position, destination)));
                        }
                    },
                    Some(_) => break,
                }
            }
        }

        for elem in &capture_dirs.0 {
            let destination = self.position + Vec2::from(elem);
            if destination.verify_bounds().is_err() { continue }
            if destination.verify_with_checked_squares(&restrictions.check_squares).is_err() { continue };
            match board.get_square(destination) {
                Some(x) if x.color() == self.color.opp() => {
                    if destination.1 == promotion_rank {
                        moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Queen, self.color)));
                        moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Knight, self.color)));
                        moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Bishop, self.color)));
                        moves.0.push(Box::new(PromotionMove::new(self.position, destination, move_register::PromotedPieceType::Rook, self.color)));
                    } else {
                        moves.0.push(Box::new(Capture::new(PieceType::Pawn, self.position, destination)));
                    }
                },
                _ => (),
            }
        }

        if pin_direction == Some(&PinDir::EnPassantBlock) { return }
        for elem in &capture_dirs.0 {
            let translation: Vec2 = Vec2::from(elem);
            let destination = self.position + translation;
            if destination.verify_bounds().is_err() { continue }
            match board.en_passant_square {
                Some(sq) if sq == destination => (),
                _ => continue,
            };
            let piece = board.get_square(self.position + Vec2(translation.0, 0));
            match piece {
                Some(x) if x.color() == self.color.opp()
                && (destination.verify_with_checked_squares(&restrictions.check_squares).is_ok()
                || destination.verify_with_en_passant_checked_square(&restrictions.en_passant_check).is_ok()) => {
                    moves.0.push(Box::new(EnPassantMove::new(self.position, destination)));
                },
                _ => (),
            }
        }
    }

    fn get_attacked(&self, _board: &Board, attacked: &mut Attacked) {
        match self.color {
            Color::White => attacked.insert_attacked_squares_relative(self.position, &[Vec2(-1, 1), Vec2(1, 1)]),
            Color::Black => attacked.insert_attacked_squares_relative(self.position, &[Vec2(-1, -1), Vec2(1, -1)]),
        }
    }

    fn get_checked(&self, board: &Board, checked: &mut CheckSquares) {
        match self.color {
            Color::White => checked.get_checked_from_a_piece(board, self.position, &WHITE_PAWN_CAPTURES, self.color, 1),
            Color::Black => checked.get_checked_from_a_piece(board, self.position, &BLACK_PAWN_CAPTURES, self.color, 1),
        }
    }

    fn get_en_passant_checked(&self, board: &Board, checked: &mut EnPassantCheckSquare) {
        // if !self.enpassantable { return }
        checked.get_en_passantable_check_square(board, self.position, self.color);
    }

    fn color(&self) -> Color { self.color }

    fn piece_type(&self) -> PieceType { PieceType::Pawn }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhitePawn,
            Color::Black => FenPieceType::BlackPawn,
        }
    }

    fn position(&self) -> Vec2 { self.position }

    fn set_position(&mut self, pos: Vec2) { self.position = pos; }
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
        for elem in &KNIGHT_MOVES {
            let destination = self.position + *elem;
            if destination.verify_bounds().is_err() { continue };
            if destination.verify_with_checked_squares(&restrictions.check_squares).is_err() { continue };
            match board.get_square(destination) {
                Some(x) if x.color() == self.color.opp() => {
                    moves.0.push(Box::new(Capture::new(self.piece_type(), self.position, destination)));
                },
                None => moves.0.push(Box::new(Move::new(self.piece_type(), self.position, destination))),
                _ => (),
            }
        }
    }

    fn get_attacked(&self, _board: &Board, attacked: &mut Attacked) {
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

    fn color(&self) -> Color { self.color }

    fn piece_type(&self) -> PieceType { PieceType::Knight }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteKnight,
            Color::Black => FenPieceType::BlackKnight,
        }
    }

    fn position(&self) -> Vec2 { self.position }

    fn set_position(&mut self, pos: Vec2) { self.position = pos; }

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

impl King {
    fn add_castling(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData, castle_type: CastleType) {
        let castle_color = castle_type.color();
        match castle_color {
            Color::White => if self.position != Vec2(4, 0) { return }
            Color::Black => if self.position != Vec2(4, 7) { return }
        }

        let castle_length = castle_type.length();

        if board.check_castling(self.position, &restrictions.attacked, castle_length) && board.castling.contains(&castle_type) {
            moves.0.push(Box::new(CastleMove::new(castle_type, self.position)))
        }
    }
}

impl ChessPiece for King {
    fn get_moves(&self, board: &Board, moves: &mut Moves, restrictions: &MoveRestrictionData) {
        for elem in &KING_MOVES {
            let destination = self.position + Vec2::from(elem);
            if destination.verify_bounds().is_err() { continue };
            if destination.verify_with_attacked_squares(&restrictions.attacked).is_err() { continue };
            match board.get_square(destination) {
                Some(x) if x.color() == self.color.opp() => {
                    moves.0.push(Box::new(Capture::new(self.piece_type(), self.position, destination)));
                },
                None => moves.0.push(Box::new(Move::new(self.piece_type(), self.position, destination))),
                _ => (),
            }
        }

        if restrictions.check_squares.1 == 0 { 
            self.add_castling(board, moves, restrictions, CastleType::new(self.color, CastleLength::Short));
            self.add_castling(board, moves, restrictions, CastleType::new(self.color, CastleLength::Long));
        }
    }

    fn get_attacked(&self, _board: &Board, attacked: &mut Attacked) {
        attacked.insert_attacked_squares_relative(self.position, &[Vec2(-1, 1), Vec2(0, 1), Vec2(1, 1), Vec2(-1, 0), Vec2(1, 0), Vec2(-1, -1), Vec2(0, -1), Vec2(1, -1)])
    }

    fn color(&self) -> Color { self.color }

    fn piece_type(&self) -> PieceType { PieceType::King }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteKing,
            Color::Black => FenPieceType::BlackKing,
        }
    }

    fn position(&self) -> Vec2 { self.position }

    fn set_position(&mut self, pos: Vec2) { self.position = pos; }

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
        moves.add_move_series(board, self, &enabled_moves, MAX_MOVES_IN_A_SERIES, restrictions);
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

    fn color(&self) -> Color { self.color }

    fn piece_type(&self) -> PieceType { PieceType::Rook }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteRook,
            Color::Black => FenPieceType::BlackRook,
        }
    }

    fn position(&self) -> Vec2 { self.position }

    fn set_position(&mut self, pos: Vec2) { self.position = pos; }
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
        moves.add_move_series(board, self, &enabled_moves, MAX_MOVES_IN_A_SERIES, restrictions);
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

    fn color(&self) -> Color { self.color }

    fn piece_type(&self) -> PieceType { PieceType::Bishop }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteBishop,
            Color::Black => FenPieceType::BlackBishop,
        }
    }

    fn position(&self) -> Vec2 { self.position }

    fn set_position(&mut self, pos:Vec2) { self.position = pos; }

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
        moves.add_move_series(board, self, &enabled_moves, MAX_MOVES_IN_A_SERIES, restrictions);
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

    fn color(&self) -> Color { self.color }

    fn piece_type(&self) -> PieceType { PieceType::Queen }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::White => FenPieceType::WhiteQueen,
            Color::Black => FenPieceType::BlackQueen,
        }
    }

    fn position(&self) -> Vec2 { self.position }

    fn set_position(&mut self, pos: Vec2) { self.position = pos; }
}

impl Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::Black => write!(f, "q"),
            Color::White => write!(f, "Q"),
        }
    }
}