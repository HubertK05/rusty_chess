use std::fmt::Display;

use thiserror::Error;

use crate::{
    move_generator::{
        models::{Color, PieceType, Square, ChessPiece},
    },
    move_register::{
        models::{MoveError, MoveType, ChessMove, PromotedPieceType},
    }, chess_bot::zobrist::zobrist_hash,
};

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

impl Display for FenPieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        write!(f, "{}", str)
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

impl TryFrom<&str> for PieceType {
    type Error = BoardError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "P" => Ok(PieceType::Pawn),
            "N" => Ok(PieceType::Knight),
            "K" => Ok(PieceType::King),
            "R" => Ok(PieceType::Rook),
            "B" => Ok(PieceType::Bishop),
            "Q" => Ok(PieceType::Queen),
            _ => Err(BoardError::ConversionFailure),
        }
    }
}

impl TryFrom<&str> for PromotedPieceType {
    type Error = BoardError;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "N" => Ok(PromotedPieceType::Knight),
            "R" => Ok(PromotedPieceType::Rook),
            "B" => Ok(PromotedPieceType::Bishop),
            "Q" => Ok(PromotedPieceType::Queen),
            _ => Err(BoardError::ConversionFailure),
        }
    }
}

impl TryFrom<(char, Square)> for ChessPiece {
    type Error = BoardError;

    fn try_from(val: (char, Square)) -> Result<Self, Self::Error> {
        let color = if val.0.is_lowercase() {
            Color::Black
        } else {
            Color::White
        };

        let piece_type = match val.0.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'k' => PieceType::King,
            'r' => PieceType::Rook,
            'b' => PieceType::Bishop,
            'q' => PieceType::Queen,
            _ => return Err(BoardError::ConversionFailure),
        };

        Ok(ChessPiece {
            piece_type,
            position: val.1,
            color,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: [[Option<ChessPiece>; 8]; 8],
    pub turn: Color,
    pub castling: AvailableCastles,
    pub en_passant_square: Option<Square>,
    pub half_move_timer_50: u8,
    pub full_move_number: u16,
    pub mating_material: (u8, u8),
    pub king_positions: (Square, Square),
}

impl Board {
    pub fn new_empty() -> Self {
        Board::try_from(FenNotation("8/8/8/8/8/8/8/8 w - - 0 1".to_string())).unwrap()
    }

    pub fn new_game() -> Self {
        Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())).unwrap()
    }

    pub fn get_square(&self, sq: Square) -> Option<ChessPiece> {
        let Square(file_number, rank_number) = sq;
        self.board
            .get(rank_number as usize)?
            .get(file_number as usize)?
            .as_ref()
            .copied()
    }

    pub fn set_ep_target_square(&mut self, sq: Option<Square>) {
        self.en_passant_square = sq
    }

    pub fn set_king_position(&mut self, sq: Square, color: Color) {
        match color {
            Color::White => self.king_positions.0 = sq,
            Color::Black => self.king_positions.1 = sq,
        }
    }

    pub fn change_mating_material(&mut self, color: Color, points: i8) {
        if points.is_negative() {
            match color {
                Color::White => self.mating_material.0 -= -points as u8,
                Color::Black => self.mating_material.1 -= -points as u8,
            }
        } else {
            match color {
                Color::White => self.mating_material.0 += points as u8,
                Color::Black => self.mating_material.1 += points as u8,
            }
        }
    }

    pub fn take_piece(&mut self, sq: Square) -> Result<ChessPiece, MoveError> {
        if !sq.is_in_bounds() {
            return Err(MoveError::OutOfBounds);
        }
        self.board[sq.1 as usize][sq.0 as usize]
            .take()
            .ok_or(MoveError::PieceNotFound)
    }

    pub fn place_piece(&mut self, mut p: ChessPiece, sq: Square) -> Result<(), MoveError> {
        if !sq.is_in_bounds() {
            return Err(MoveError::OutOfBounds);
        }
        p.set_position(sq);
        Ok(self.board[sq.1 as usize][sq.0 as usize] = Some(p))
    }

    pub fn set_castling(&mut self, m: ChessMove) {
        if m.from == Square(0, 0) || m.to == Square(0, 0) || m.from == Square(4, 0) {
            self.castling.white_long = false;
        }
        if m.from == Square(7, 0) || m.to == Square(7, 0) || m.from == Square(4, 0) {
            self.castling.white_short = false;
        }
        if m.from == Square(0, 7) || m.to == Square(0, 7) || m.from == Square(4, 7) {
            self.castling.black_long = false;
        }
        if m.from == Square(7, 7) || m.to == Square(7, 7) || m.from == Square(4, 7) {
            self.castling.black_short = false;
        }
    }

    pub fn increment_half_move_timer(&mut self) {
        self.half_move_timer_50 += 1;
    }

    pub fn reset_half_move_timer(&mut self) {
        self.half_move_timer_50 = 0;
    }

    pub fn increment_full_move_timer(&mut self) {
        self.full_move_number += 1;
    }

    pub fn advance_turn(&mut self) {
        self.turn = self.turn.opp();
    }

    pub fn register_move(&mut self, m: ChessMove) -> Result<(), MoveError> {
        self.increment_half_move_timer();
        match m.move_type {
            MoveType::Capture(_)
            | MoveType::EnPassantMove
            | MoveType::PromotionMove(_)
            | MoveType::PromotionCapture(_) => self.reset_half_move_timer(),
            _ => (),
        };

        self.set_ep_target_square(None);
        self.set_castling(m);

        m.register_move(self)?;

        self.advance_turn();
        if self.turn == Color::White {
            self.increment_full_move_timer();
        }

        Ok(())
    }

    pub fn hash_board(&self) -> u64 {
        zobrist_hash(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AvailableCastles {
    pub white_short: bool,
    pub white_long: bool,
    pub black_short: bool,
    pub black_long: bool,
}

impl AvailableCastles {
    pub fn all_false() -> Self {
        Self {
            white_short: false,
            white_long: false,
            black_short: false,
            black_long: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FenNotation(pub String);

impl FenNotation {
    pub fn to_draw_fen(&self) -> String {
        let mut split_fen = self.0.split_whitespace();
        [
            split_fen.next().expect("wrong fen"),
            split_fen.next().expect("wrong fen"),
            split_fen.next().expect("wrong fen"),
        ]
        .join(" ")
    }
}

impl From<&Board> for FenNotation {
    fn from(val: &Board) -> Self {
        let mut res = String::new();
        for file in (0..8).rev() {
            let mut empty_counter = 0;
            for rank in 0..8 {
                let piece = val.get_square(Square(rank, file));
                if let Some(p) = piece {
                    if empty_counter != 0 {
                        res.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }
                    res.push_str(&p.fen_piece_type().to_string());
                } else {
                    empty_counter += 1
                }
            }
            if empty_counter != 0 {
                res.push_str(&empty_counter.to_string())
            }
            if file != 0 {
                res.push('/')
            }
        }

        match val.turn {
            Color::White => res.push_str(" w "),
            Color::Black => res.push_str(" b "),
        }

        let mut castling_rights = String::new();
        if val.castling.white_short {
            castling_rights.push('K')
        }
        if val.castling.white_long {
            castling_rights.push('Q')
        }
        if val.castling.black_short {
            castling_rights.push('k')
        }
        if val.castling.black_long {
            castling_rights.push('q')
        }

        if castling_rights.is_empty() {
            res.push('-');
        } else {
            res.push_str(&castling_rights);
        }

        res.push(' ');
        if let Some(sq) = &val.en_passant_square {
            res.push_str(&sq.to_string());
        } else {
            res.push('-');
        }
        res.push(' ');
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

        let mut board: [[Option<ChessPiece>; 8]; 8] = Default::default();
        let mut mating_material = (0, 0);

        let mut white_king_pos = Square(0, 0);
        let mut black_king_pos = Square(0, 0);

        for char in position.chars() {
            match char {
                '1'..='8' => {
                    file += char
                        .to_string()
                        .parse::<usize>()
                        .map_err(|_e| BoardError::ConversionFailure)?
                }
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                _ => {
                    let pos = Square(file as i8, rank as i8);
                    let mut piece: ChessPiece = (char, pos)
                        .try_into()
                        .map_err(|_e| BoardError::ConversionFailure)?;
                    if piece.piece_type == PieceType::King {
                        match piece.color {
                            Color::White => white_king_pos = pos,
                            Color::Black => black_king_pos = pos,
                        }
                    }
                    piece.set_position(pos);
                    match piece.color {
                        Color::White => mating_material.0 += piece.mating_material_points(),
                        Color::Black => mating_material.1 += piece.mating_material_points(),
                    }
                    board[rank][file] = Some(piece);
                    file += 1;
                }
            }
        }

        let turn = match board_data[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => unreachable!(),
        };

        let mut castling = AvailableCastles::all_false();
        for letter in board_data[2].chars() {
            match letter {
                'K' => castling.white_short = true,
                'k' => castling.black_short = true,
                'Q' => castling.white_long = true,
                'q' => castling.black_long = true,
                '-' => (),
                _ => return Err(BoardError::ConversionFailure),
            };
        }

        let en_passant_square = match board_data[3] {
            "-" => None,
            sq => Some(Square::try_from(sq)?),
        };

        let half_move_timer_50 = board_data[4]
            .parse::<u8>()
            .map_err(|_e| BoardError::ConversionFailure)?;
        let full_move_number = board_data[5]
            .parse::<u16>()
            .map_err(|_e| BoardError::ConversionFailure)?;

        Ok(Board {
            board,
            turn,
            castling,
            en_passant_square,
            half_move_timer_50,
            full_move_number,
            mating_material,
            king_positions: (white_king_pos, black_king_pos),
        })
    }
}

impl Display for FenNotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(val) = self;
        write!(f, "{val}")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..8).rev() {
            for j in 0..8 {
                let _ = match self.board[i][j].as_ref() {
                    Some(s) => write!(f, "{s}"),
                    None => write!(f, "."),
                };
            }
            let _ = write!(f, "\n");
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum BoardError {
    #[error("Failed to parse the board")]
    ConversionFailure,
}
