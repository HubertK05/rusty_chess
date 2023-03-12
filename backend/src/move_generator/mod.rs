use std::fmt;

use dyn_clone::DynClone;

use crate::{board_setup::models::{Board, FenPieceType}, move_register::models::{MoveError, ChessMove, Move, Capture, PromotionMove, PromotedPieceType, PromotionCapture, EnPassantMove, MoveType}};

use self::models::{Moves, MoveRestrictionData, Attacked, CheckSquares, EnPassantCheckSquare, PinSquares, Color, PieceType, Square, Pawn, MoveDir, Offset, CheckedAdd, Rook, Bishop, Queen, Knight, King};

pub mod models;

pub trait ChessPiece: fmt::Display + fmt::Debug + DynClone {
    fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<Box<dyn ChessMove>>;
    fn get_en_passant_checked(&self, _board: &Board, _checked: &mut EnPassantCheckSquare) {}
    fn get_pins(&self, _board: &Board, _pins: &mut PinSquares) {}
    fn color(&self) -> Color;
    fn piece_type(&self) -> PieceType;
    fn fen_piece_type(&self) -> FenPieceType;
    fn position(&self) -> Square;
    // fn pin_direction<'a>(&'a self, data: &'a MoveRestrictionData) -> Option<&'a PinDir> {
    //     data.pin_squares.0.get(&self.position())
    // }
    fn set_position(&mut self, pos: Square);
    fn mating_material_points(&self) -> u8 { 3 }
}

dyn_clone::clone_trait_object!(ChessPiece);

const WHITE_PAWN_DOUBLE_MOVE_RANK: i8 = 1;
const BLACK_PAWN_DOUBLE_MOVE_RANK: i8 = 6;
const WHITE_PROMOTION_RANK: i8 = 7;
const BLACK_PROMOTION_RANK: i8 = 0;
const MAX_MOVES_IN_A_SERIES: usize = 7;

const WHITE_PAWN_MOVE: MoveDir = MoveDir::Up;
const WHITE_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::UpLeft, MoveDir::UpRight];

const BLACK_PAWN_MOVE: MoveDir = MoveDir::Down;
const BLACK_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::DownLeft, MoveDir::DownRight];

const PROMOTED_PIECE_TYPES: [PromotedPieceType; 4] = [PromotedPieceType::Queen, PromotedPieceType::Rook, PromotedPieceType::Bishop, PromotedPieceType::Knight];

const KNIGHT_MOVES: [Offset; 8] = [Offset(2, 1), Offset(1, 2), Offset(-1, 2), Offset(-2, 1), Offset(-2, -1), Offset(-1, -2), Offset(1, -2), Offset(2, -1)];

const KING_MOVES: [Offset; 8] = [Offset(1, 0), Offset(1, 1), Offset(0, 1), Offset(-1, 1), Offset(-1, 0), Offset(-1, -1), Offset(0, -1), Offset(1, -1)];

const ROOK_MOVES: [MoveDir; 4] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

const BISHOP_MOVES: [MoveDir; 4] = [MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

const QUEEN_MOVES: [MoveDir; 8] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right, MoveDir::UpLeft, MoveDir::DownLeft, MoveDir::UpRight, MoveDir::DownRight];

impl ChessPiece for Pawn {
    fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<Box<dyn ChessMove>> {
        let mut a = get_pawn_moves(board, self.position, self.color);
        a.extend(get_pawn_captures(board, self.position, self.color));
        a.extend(get_en_passant(board, self.position, self.color));
        a
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::Black => FenPieceType::BlackPawn,
            Color::White => FenPieceType::WhitePawn,
        }
    }

    fn position(&self) -> Square {
        self.position
    }

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for Rook {
    fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<Box<dyn ChessMove>> {
        let mut a: Vec<Box<dyn ChessMove>> = Vec::new();
        for dir in ROOK_MOVES {
            a.extend(get_move_series(board, self.position, dir, MAX_MOVES_IN_A_SERIES as u8, self));
        }
        a
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Rook
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::Black => FenPieceType::BlackRook,
            Color::White => FenPieceType::WhiteRook,
        }
    }

    fn position(&self) -> Square {
        self.position
    }

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for Bishop {
    fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<Box<dyn ChessMove>> {
        let mut a: Vec<Box<dyn ChessMove>> = Vec::new();
        for dir in BISHOP_MOVES {
            a.extend(get_move_series(board, self.position, dir, MAX_MOVES_IN_A_SERIES as u8, self));
        }
        a
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Bishop
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::Black => FenPieceType::BlackBishop,
            Color::White => FenPieceType::WhiteBishop,
        }
    }

    fn position(&self) -> Square {
        self.position
    }

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for Queen {
    fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<Box<dyn ChessMove>> {
        let mut a: Vec<Box<dyn ChessMove>> = Vec::new();
        for dir in QUEEN_MOVES {
            a.extend(get_move_series(board, self.position, dir, MAX_MOVES_IN_A_SERIES as u8, self));
        }
        a
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Queen
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::Black => FenPieceType::BlackQueen,
            Color::White => FenPieceType::WhiteQueen,
        }
    }

    fn position(&self) -> Square {
        self.position
    }

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for Knight {
    fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<Box<dyn ChessMove>> {
        let mut a: Vec<Box<dyn ChessMove>> = Vec::new();
        for offset in KNIGHT_MOVES {
            a.extend(get_move(board, self.position, offset, self));
        }
        a
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::Knight
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::Black => FenPieceType::BlackKnight,
            Color::White => FenPieceType::WhiteKnight,
        }
    }

    fn position(&self) -> Square {
        self.position
    }

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for King {
    fn get_moves(&self, board: &Board, restriction: &MoveRestrictionData) -> Vec<Box<dyn ChessMove>> {
        let mut a: Vec<Box<dyn ChessMove>> = Vec::new();
        for offset in KING_MOVES {
            a.extend(get_move(board, self.position, offset, self));
        }
        a
    }

    fn color(&self) -> Color {
        self.color
    }

    fn piece_type(&self) -> PieceType {
        PieceType::King
    }

    fn fen_piece_type(&self) -> FenPieceType {
        match self.color {
            Color::Black => FenPieceType::BlackKing,
            Color::White => FenPieceType::WhiteKing,
        }
    }

    fn position(&self) -> Square {
        self.position
    }

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

fn get_move_series(board: &Board, start: Square, dir: MoveDir, count: u8, piece: &dyn ChessPiece) -> Vec<Box<dyn ChessMove>> {
    let translation = Offset::from(dir);
    let mut res: Vec<Box<dyn ChessMove>> = Vec::new();
    for i in 1..=count {
        let m = get_move(board, start, translation * i as i8, piece);
        let end = match m {
            Some(ref x) if (*x).move_type() == MoveType::Move => false,
            _ => true,
        };
        res.extend(m);

        if end {
            break
        }
    }

    res
}

fn get_pawn_moves(board: &Board, start: Square, color: Color) -> Vec<Box<dyn ChessMove>> {
    let mut res: Vec<Box<dyn ChessMove>> = Vec::new();
    let (translation, double_move_rank, promotion_rank) = match color {
        Color::White => (Offset::from(WHITE_PAWN_MOVE), WHITE_PAWN_DOUBLE_MOVE_RANK, WHITE_PROMOTION_RANK),
        Color::Black => (Offset::from(BLACK_PAWN_MOVE), BLACK_PAWN_DOUBLE_MOVE_RANK, BLACK_PROMOTION_RANK),
    };

    let pos = <(i8, i8)>::from(start);
    let count = if pos.0 == double_move_rank { 2 } else { 1 };

    for i in 1..=count {
        if let Some(sq) = start.c_add(translation * i as i8) {
            if board.get_square(sq).is_some() {
                break
            }
            let (rank_number, _) = <(i8, i8)>::from(sq);
            if rank_number == promotion_rank {
                for pp in PROMOTED_PIECE_TYPES {
                    res.push(Box::new(PromotionMove::new(start, sq, pp, color)));
                }
                break
            }
        } else {
            break
        }
    }

    res
}

fn get_pawn_captures(board: &Board, start: Square, color: Color) -> Vec<Box<dyn ChessMove>> {
    let mut res: Vec<Box<dyn ChessMove>> = Vec::new();
    let (pawn_captures, promotion_rank) = match color {
        Color::White => (WHITE_PAWN_CAPTURES, WHITE_PROMOTION_RANK),
        Color::Black => (BLACK_PAWN_CAPTURES, BLACK_PROMOTION_RANK),
    };

    for move_dir in pawn_captures {
        let Some(sq) = start.c_add(Offset::from(move_dir)) else {
            continue
        };

        let Some(p) = board.get_square(sq) else {
            continue
        };
        
        let (rank_number, _) = <(i8, i8)>::from(sq);
        if p.color() != color && rank_number == promotion_rank {
            for pp in PROMOTED_PIECE_TYPES {
                res.push(Box::new(PromotionCapture::new(start, sq, pp, color)));
            }
        } else if p.color() != color {
            res.push(Box::new(Capture::new(PieceType::Pawn, start, sq)));
        }
    }

    res
}

fn get_en_passant(board: &Board, start: Square, color: Color) -> Option<Box<dyn ChessMove>> {
    let target_sq = board.en_passant_square?;
    if (target_sq.0 - start.0).abs() != 1 {
        return None;
    }
    if board.get_square(target_sq)?.color() != color {
        match color {
            Color::White => Some(Box::new(EnPassantMove::new(start, target_sq + 8))),
            Color::Black => Some(Box::new(EnPassantMove::new(start, target_sq - 8))),  
        }
    } else {
        None
    }
}

fn get_move(board: &Board, start: Square, offset: Offset, piece: &dyn ChessPiece) -> Option<Box<dyn ChessMove>> {
    let sq = start.c_add(offset)?;
    match board.get_square(sq) {
        Some(p) if p.color() == piece.color() => None,
        Some(_) => Some(Box::new(Capture::new(piece.piece_type(), start, sq))),
        None => Some(Box::new(Move::new(piece.piece_type(), start, sq)))
    }
}
