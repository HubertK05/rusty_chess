use std::{collections::HashSet, fmt::Display};

use crate::{move_generator::{models::{Color, Square, PieceType}, ChessPiece}, move_register::{models::{CastleType, MoveError, MoveType}, ChessMove}};

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

// impl TryFrom<char> for Box<dyn ChessPiece> {
//     type Error = BoardError;

//     fn try_from(val: char) -> Result<Self, Self::Error> {
//         match val {
//             'P' => Ok(Box::new(Pawn { position: Vec2(0, 0), color: Color::White})),
//             'N' => Ok(Box::new(Knight { position: Vec2(0, 0), color: Color::White })),
//             'K' => Ok(Box::new(King { position: Vec2(0, 0), color: Color::White })),
//             'R' => Ok(Box::new(Rook { position: Vec2(0, 0), color: Color::White })),
//             'B' => Ok(Box::new(Bishop { position: Vec2(0, 0), color: Color::White })),
//             'Q' => Ok(Box::new(Queen { position: Vec2(0, 0), color: Color::White })),
//             'p' => Ok(Box::new(Pawn { position: Vec2(0, 0), color: Color::Black})),
//             'n' => Ok(Box::new(Knight { position: Vec2(0, 0), color: Color::Black })),
//             'k' => Ok(Box::new(King { position: Vec2(0, 0), color: Color::Black })),
//             'r' => Ok(Box::new(Rook { position: Vec2(0, 0), color: Color::Black })),
//             'b' => Ok(Box::new(Bishop { position: Vec2(0, 0), color: Color::Black })),
//             'q' => Ok(Box::new(Queen { position: Vec2(0, 0), color: Color::Black })),
//             _ => Err(BoardError::ConversionFailure),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[Option<Box<dyn ChessPiece>>;8];8],
    pub turn: Color,
    pub castling: AvailableCastles,
    pub en_passant_square: Option<Square>,
    pub half_move_timer_50: u8,
    pub full_move_number: u16,
    pub mating_material: (u8, u8),
    pub king_positions: (Square, Square),
}

impl Board {
    pub fn get_square(&self, sq: Square) -> Option<&dyn ChessPiece> {
        let Square(file_number, rank_number) = sq;
        self.board.get(rank_number as usize)?.get(file_number as usize)?.as_deref()
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

    pub fn take_piece(&mut self, sq: Square) -> Result<Box<dyn ChessPiece>, MoveError> {
        if !sq.is_in_bounds() {
            return Err(MoveError::OutOfBounds);
        }
        self.board[sq.1 as usize][sq.0 as usize].take().ok_or(MoveError::PieceNotFound)
    }

    pub fn place_piece(&mut self, mut p: Box<dyn ChessPiece>, sq: Square) -> Result<(), MoveError> {
        if !sq.is_in_bounds() {
            return Err(MoveError::OutOfBounds);
        }
        p.set_position(sq);
        Ok(self.board[sq.1 as usize][sq.0 as usize] = Some(p))
    }

    pub fn set_castling(&mut self, m: &dyn ChessMove) {
        if m.from() == Square(0, 0) || m.from() == Square(4, 0) {
            self.castling.white_long = false;
        }
        if m.from() == Square(7, 0) || m.from() == Square(4, 0) {
            self.castling.white_short = false;
        }
        if m.to() == Square(0, 7) || m.to() == Square(4, 7) {
            self.castling.black_long = false;
        }
        if m.to() == Square(7, 7) || m.to() == Square(4, 7) {
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

    pub fn register_move(&mut self, m: &dyn ChessMove) -> Result<(), MoveError> {
        self.increment_half_move_timer();
        match m.move_type() {
            MoveType::Capture | MoveType::EnPassantMove | MoveType::PromotionMove | MoveType::PromotionCapture => self.reset_half_move_timer(),
            _ => (),
        };

        self.set_ep_target_square(None);

        m.register_move(self)?;
        
        self.advance_turn();
        if self.turn == Color::White {
            self.increment_full_move_timer();
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AvailableCastles {
    pub white_short: bool,
    pub white_long: bool,
    pub black_short: bool,
    pub black_long: bool,
}

#[derive(Debug)]
pub struct FenNotation(pub String);

impl FenNotation {
    pub fn to_draw_fen(&self) -> String {
        let mut split_fen = self.0.split_whitespace();
        [split_fen.next().expect("wrong fen"), split_fen.next().expect("wrong fen"), split_fen.next().expect("wrong fen")].join(" ")
    }
}

// impl From<&Board> for FenNotation {
//     fn from(val: &Board) -> Self {
//         let mut res = String::new();
//         for file in (0..8).rev() {
//             let mut empty_counter = 0;
//             for rank in 0..8 {
//                 let piece = val.get_square(Vec2(rank, file));
//                 if let Some(p) = piece {
//                     if empty_counter != 0 {
//                         res.push_str(&empty_counter.to_string());
//                         empty_counter = 0;
//                     }
//                     res.push_str(&p.fen_piece_type().to_string());
//                 } else { empty_counter += 1 }
//             }
//             if empty_counter != 0 { res.push_str(&empty_counter.to_string()) }
//             if file != 0 { res.push('/') }
//         }

//         match val.turn {
//             Color::White => res.push_str(" w "),
//             Color::Black => res.push_str(" b "),
//         }

//         let mut castling_rights = String::new();
//         if val.castling.contains(&CastleType::WhiteShort) { castling_rights.push('K') }
//         if val.castling.contains(&CastleType::WhiteLong) { castling_rights.push('Q') }
//         if val.castling.contains(&CastleType::BlackShort) { castling_rights.push('k') }
//         if val.castling.contains(&CastleType::BlackLong) { castling_rights.push('q') }

//         if castling_rights.is_empty() {
//             res.push('-');
//         } else {
//             res.push_str(&castling_rights);
//         }

//         res.push_str(" - ");
//         res.push_str(&val.half_move_timer_50.to_string());
//         res.push(' ');
//         res.push_str(&val.full_move_number.to_string());

//         FenNotation(res)
//     }
// }

// impl TryFrom<FenNotation> for Board {
//     type Error = BoardError;

//     fn try_from(val: FenNotation) -> Result<Self, Self::Error> {
//         let board_data = val.0.split_whitespace().collect::<Vec<&str>>();
//         let position = board_data[0];

//         let mut rank = 7_usize;
//         let mut file = 0_usize;

//         let mut board: [[Option<Box<dyn ChessPiece>>; 8]; 8] = Default::default();
//         let mut mating_material = (0, 0);

//         for char in position.chars() {
//             match char {
//                 '1'..='8' => file += char.to_string().parse::<usize>().map_err(|_e| BoardError::ConversionFailure)?,
//                 '/' => {
//                     rank -= 1;
//                     file = 0;
//                 },
//                 _ => {
//                     let mut piece: Box<dyn ChessPiece> = char.try_into().map_err(|_e| BoardError::ConversionFailure)?;
//                     piece.deref_mut().set_position(Vec2(file as i8, rank as i8));
//                     match piece.color() {
//                         Color::White => mating_material.0 += piece.mating_material_points(),
//                         Color::Black => mating_material.1 += piece.mating_material_points(),
//                     }
//                     board[file][rank] = Some(piece);
//                     file += 1;
//                 },
//             }
//         }

//         let turn = match board_data[1] {
//             "w" => Color::White,
//             "b" => Color::Black,
//             _ => unreachable!(),
//         };

//         let mut castling = HashSet::new();
//         for letter in board_data[2].chars() {
//             match letter {
//                 'K' => castling.insert(CastleType::WhiteShort),
//                 'k' => castling.insert(CastleType::BlackShort),
//                 'Q' => castling.insert(CastleType::WhiteLong),
//                 'q' => castling.insert(CastleType::BlackLong),
//                 '-' => false,
//                 _ => return Err(BoardError::ConversionFailure),
//             };
//         }

//         let en_passant_square = match board_data[3] {
//             "-" => None,
//             sq => Some(Vec2::try_from(sq)?),
//         };

//         let half_move_timer_50 = board_data[4].parse::<u8>().map_err(|_e| BoardError::ConversionFailure)?; 
//         let full_move_number = board_data[5].parse::<u16>().map_err(|_e| BoardError::ConversionFailure)?;

//         Ok(
//             Board {
//                 board,
//                 turn,
//                 castling,
//                 en_passant_square,
//                 half_move_timer_50,
//                 full_move_number,
//                 mating_material,
//             }
//         )
//     }
// }

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
