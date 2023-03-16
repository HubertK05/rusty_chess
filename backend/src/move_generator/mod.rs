use std::fmt;

use dyn_clone::DynClone;

use crate::{
    board_setup::models::{AvailableCastles, Board, FenPieceType},
    move_register::{
        models::{
            Capture, CastleMove, CastleType, EnPassantMove, Move, MoveType, PromotedPieceType,
            PromotionCapture, PromotionMove,
        },
        ChessMove,
    },
};

use self::{
    models::{
        Attacked, Bishop, CheckedAdd, Color, King, Knight, MoveDir, MoveRestrictionData, Offset,
        Pawn, PieceType, PinDir, Queen, Rook, Square,
    },
    restrictions::{filter_with_checked, filter_with_pins},
};

pub mod models;
pub mod restrictions;

pub trait ChessPiece: fmt::Display + fmt::Debug + DynClone {
    fn get_moves(
        &self,
        board: &Board,
        restriction: &MoveRestrictionData,
    ) -> Vec<Box<dyn ChessMove>>;
    fn color(&self) -> Color;
    fn piece_type(&self) -> PieceType;
    fn fen_piece_type(&self) -> FenPieceType;
    fn set_position(&mut self, pos: Square);
    fn mating_material_points(&self) -> u8 {
        3
    }
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

const PROMOTED_PIECE_TYPES: [PromotedPieceType; 4] = [
    PromotedPieceType::Queen,
    PromotedPieceType::Rook,
    PromotedPieceType::Bishop,
    PromotedPieceType::Knight,
];

const KNIGHT_MOVES: [Offset; 8] = [
    Offset(2, 1),
    Offset(1, 2),
    Offset(-1, 2),
    Offset(-2, 1),
    Offset(-2, -1),
    Offset(-1, -2),
    Offset(1, -2),
    Offset(2, -1),
];

const KING_MOVES: [Offset; 8] = [
    Offset(1, 0),
    Offset(1, 1),
    Offset(0, 1),
    Offset(-1, 1),
    Offset(-1, 0),
    Offset(-1, -1),
    Offset(0, -1),
    Offset(1, -1),
];

const ROOK_MOVES: [MoveDir; 4] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

const BISHOP_MOVES: [MoveDir; 4] = [
    MoveDir::UpLeft,
    MoveDir::DownLeft,
    MoveDir::UpRight,
    MoveDir::DownRight,
];

const QUEEN_MOVES: [MoveDir; 8] = [
    MoveDir::Up,
    MoveDir::Down,
    MoveDir::Left,
    MoveDir::Right,
    MoveDir::UpLeft,
    MoveDir::DownLeft,
    MoveDir::UpRight,
    MoveDir::DownRight,
];

impl ChessPiece for Pawn {
    fn get_moves(
        &self,
        board: &Board,
        restriction: &MoveRestrictionData,
    ) -> Vec<Box<dyn ChessMove>> {
        let mut all_moves = get_pawn_moves(board, self.position, self.color);
        all_moves.extend(get_pawn_captures(board, self.position, self.color));

        let en_passant = get_en_passant(board, self.position, self.color);
        if let Some(ep) = &en_passant {
            if restriction.pin_squares.0.get(&ep.from()) != Some(&PinDir::EnPassantBlock) {
                all_moves.extend(en_passant);
            }
        };

        let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
        let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
        all_moves
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

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for Rook {
    fn get_moves(
        &self,
        board: &Board,
        restriction: &MoveRestrictionData,
    ) -> Vec<Box<dyn ChessMove>> {
        let mut all_moves: Vec<Box<dyn ChessMove>> = Vec::new();
        for dir in ROOK_MOVES {
            all_moves.extend(get_move_series(
                board,
                self.position,
                dir,
                MAX_MOVES_IN_A_SERIES as u8,
                self,
            ));
        }
        let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
        let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
        all_moves
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

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for Bishop {
    fn get_moves(
        &self,
        board: &Board,
        restriction: &MoveRestrictionData,
    ) -> Vec<Box<dyn ChessMove>> {
        let mut all_moves: Vec<Box<dyn ChessMove>> = Vec::new();
        for dir in BISHOP_MOVES {
            all_moves.extend(get_move_series(
                board,
                self.position,
                dir,
                MAX_MOVES_IN_A_SERIES as u8,
                self,
            ));
        }
        let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
        let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
        all_moves
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

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }

    fn mating_material_points(&self) -> u8 {
        2
    }
}

impl ChessPiece for Queen {
    fn get_moves(
        &self,
        board: &Board,
        restriction: &MoveRestrictionData,
    ) -> Vec<Box<dyn ChessMove>> {
        let mut all_moves: Vec<Box<dyn ChessMove>> = Vec::new();
        for dir in QUEEN_MOVES {
            all_moves.extend(get_move_series(
                board,
                self.position,
                dir,
                MAX_MOVES_IN_A_SERIES as u8,
                self,
            ));
        }
        let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
        let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
        all_moves
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

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }
}

impl ChessPiece for Knight {
    fn get_moves(
        &self,
        board: &Board,
        restriction: &MoveRestrictionData,
    ) -> Vec<Box<dyn ChessMove>> {
        let mut all_moves: Vec<Box<dyn ChessMove>> = Vec::new();
        for offset in KNIGHT_MOVES {
            all_moves.extend(get_move(board, self.position, offset, self));
        }
        let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
        let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
        all_moves
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

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }

    fn mating_material_points(&self) -> u8 {
        1
    }
}

impl ChessPiece for King {
    fn get_moves(
        &self,
        board: &Board,
        restriction: &MoveRestrictionData,
    ) -> Vec<Box<dyn ChessMove>> {
        let mut all_moves: Vec<Box<dyn ChessMove>> = Vec::new();
        for offset in KING_MOVES {
            if let Some(m) = get_move(board, self.position, offset, self) {
                if !restriction.attacked.0.contains(&m.to()) {
                    all_moves.push(m);
                }
            }
        }
        if restriction.check_squares.checks_amount == 0 {
            all_moves.extend(get_castles(
                board,
                self.position,
                &board.castling,
                &restriction.attacked,
                self.color,
            ));
        }

        all_moves
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

    fn set_position(&mut self, pos: Square) {
        self.position = pos
    }

    fn mating_material_points(&self) -> u8 {
        0
    }
}

fn get_move_series(
    board: &Board,
    start: Square,
    dir: MoveDir,
    count: u8,
    piece: &dyn ChessPiece,
) -> Vec<Box<dyn ChessMove>> {
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
            break;
        }
    }

    res
}

fn get_pawn_moves(board: &Board, start: Square, color: Color) -> Vec<Box<dyn ChessMove>> {
    let mut res: Vec<Box<dyn ChessMove>> = Vec::new();
    let (translation, double_move_rank, promotion_rank) = match color {
        Color::White => (
            Offset::from(WHITE_PAWN_MOVE),
            WHITE_PAWN_DOUBLE_MOVE_RANK,
            WHITE_PROMOTION_RANK,
        ),
        Color::Black => (
            Offset::from(BLACK_PAWN_MOVE),
            BLACK_PAWN_DOUBLE_MOVE_RANK,
            BLACK_PROMOTION_RANK,
        ),
    };

    let count = if start.1 == double_move_rank { 2 } else { 1 };

    for i in 1..=count {
        let Some(sq) = start.c_add(translation * i as i8) else {
            break
        };

        if board.get_square(sq).is_some() {
            break;
        }

        if sq.1 == promotion_rank {
            for pp in PROMOTED_PIECE_TYPES {
                res.push(Box::new(PromotionMove::new(start, sq, pp, color)));
            }
            break;
        } else {
            res.push(Box::new(Move::new(PieceType::Pawn, start, sq)));
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

        if p.color() != color && sq.1 == promotion_rank {
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
    if target_sq.1 - start.1 != 0 || (target_sq.0 - start.0).abs() != 1 {
        return None;
    }
    if board.get_square(target_sq)?.color() != color {
        let res = match color {
            Color::White => Square(target_sq.0, target_sq.1 + 1),
            Color::Black => Square(target_sq.0, target_sq.1 - 1),
        };
        Some(Box::new(EnPassantMove::new(start, res)))
    } else {
        None
    }
}

fn get_move(
    board: &Board,
    start: Square,
    offset: Offset,
    piece: &dyn ChessPiece,
) -> Option<Box<dyn ChessMove>> {
    let sq = start.c_add(offset)?;
    match board.get_square(sq) {
        Some(p) if p.color() == piece.color() => None,
        Some(_) => Some(Box::new(Capture::new(piece.piece_type(), start, sq))),
        None => Some(Box::new(Move::new(piece.piece_type(), start, sq))),
    }
}

fn get_castles(
    board: &Board,
    pos: Square,
    castles: &AvailableCastles,
    attacked: &Attacked,
    color: Color,
) -> Vec<Box<dyn ChessMove>> {
    let mut res: Vec<Box<dyn ChessMove>> = Vec::new();
    let Attacked(set) = &attacked;
    if color == Color::White
        && castles.white_short
        && [pos + Offset(1, 0), pos + Offset(2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
    {
        res.push(Box::new(CastleMove::new(CastleType::WhiteShort)));
    }
    if color == Color::White
        && castles.white_long
        && [pos + Offset(-1, 0), pos + Offset(-2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
        && board.get_square(pos + Offset(-3, 0)).is_none()
    {
        res.push(Box::new(CastleMove::new(CastleType::WhiteLong)));
    }
    if color == Color::Black
        && castles.black_short
        && [pos + Offset(1, 0), pos + Offset(2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
    {
        res.push(Box::new(CastleMove::new(CastleType::BlackShort)));
    }
    if color == Color::Black
        && castles.black_long
        && [pos + Offset(-1, 0), pos + Offset(-2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
        && board.get_square(pos + Offset(-3, 0)).is_none()
    {
        res.push(Box::new(CastleMove::new(CastleType::BlackLong)));
    }
    res
}
