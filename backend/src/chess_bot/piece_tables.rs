use crate::{
    board_setup::models::Board,
    move_generator::models::{ChessPiece, Color, Offset, PieceType},
    move_register::models::{CastleType, ChessMove, MoveType, PromotedPieceType},
};

use super::{MATERIAL_VALUE, evaluation::Evaluation};

pub const KING_TABLE: [[i16; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [20, 20, 0, -25, -25, -25, 20, 20],
    [20, 30, 10, -25, 0, -25, 30, 20],
];

pub const KING_ENDGAME_TABLE: [[i16; 8]; 8] = [
    [-30, -20, -10, 0, 0, -10, -20, -30],
    [-20, -10, 0, 10, 10, 0, -10, -20],
    [-10, 0, 10, 20, 20, 10, 0, -10],
    [0, 10, 20, 30, 30, 20, 10, 0],
    [0, 10, 20, 30, 30, 20, 10, 0],
    [-10, 0, 10, 20, 20, 10, 0, -10],
    [-20, -10, 0, 10, 10, 0, -10, -20],
    [-30, -20, -10, 0, 0, -10, -20, -30],
];

pub const QUEEN_TABLE: [[i16; 8]; 8] = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20],
];

pub const ROOK_TABLE: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [0, 0, 0, 5, 5, 0, 0, 0],
];

pub const BISHOP_TABLE: [[i16; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-30, -20, -20, -20, -20, -20, -20, -30],
];

pub const KNIGHT_TABLE: [[i16; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 5, 5, 15, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-60, -40, -40, -40, -40, -40, -40, -60],
];

pub const PAWN_TABLE: [[i16; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [100, 100, 100, 100, 100, 100, 100, 100],
    [40, 40, 50, 60, 60, 50, 40, 40],
    [15, 15, 20, 25, 25, 20, 15, 15],
    [0, 0, 0, 20, 20, 0, 0, 0],
    [5, -5, -10, 0, 0, -10, -5, 5],
    [5, 10, 10, -20, -20, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub fn evaluate_chg(board: &Board, mov: ChessMove, is_endgame: bool) -> Evaluation {
    let from = board
        .get_square(mov.from)
        .expect("no piece found where it should be");

    let (mut material, mut pst) = piece_value_chg(from, mov, is_endgame);
    match mov.move_type {
        MoveType::Move(_) => (),
        MoveType::Capture(_) => {
            let to = board
                .get_square(mov.to)
                .expect("no piece found where it should be");
            let (material_chg, pst_chg) = piece_value(to, is_endgame);
            material -= material_chg;
            pst -= pst_chg;
        }
        MoveType::EnPassantMove => {
            let pawn_sq = board
                .en_passant_square
                .expect("no en passant target square found")
                + match board.turn {
                    Color::White => Offset(0, -1),
                    Color::Black => Offset(0, 1),
                };

            let captured_piece = board
                .get_square(pawn_sq)
                .expect("no pawn next to en passant target square");
            let (material_chg, pst_chg) = piece_value(captured_piece, is_endgame);
            material -= material_chg;
            pst -= pst_chg;
        }
        MoveType::CastleMove(castle_type) => pst += castle_value_chg_for_rook(castle_type),
        MoveType::PromotionMove(ppt) => material += promoted_material_value_chg(ppt),
        MoveType::PromotionCapture(ppt) => {
            let to = board
                .get_square(mov.to)
                .expect("no piece found where it should be");
            let (material_chg, pst_chg) = piece_value(to, is_endgame);
            material += promoted_material_value_chg(ppt);
            material -= material_chg;
            pst -= pst_chg;
        }
    };
    
    Evaluation {
        material,
        pst,
        pawn_structure: 0,
        space: 0,
        king_dist: 0,
    }
}

fn castle_value_chg_for_rook(castle_type: CastleType) -> i16 {
    match castle_type {
        CastleType::WhiteShort => ROOK_TABLE[7][5] - ROOK_TABLE[7][7],
        CastleType::WhiteLong => ROOK_TABLE[7][3] - ROOK_TABLE[7][0],
        CastleType::BlackShort => ROOK_TABLE[0][5] - ROOK_TABLE[0][7],
        CastleType::BlackLong => ROOK_TABLE[0][3] - ROOK_TABLE[0][0],
    }
}

fn promoted_material_value_chg(promoted_piece_type: PromotedPieceType) -> i16 {
    material_value(promoted_piece_type) - material_value(PieceType::Pawn)
}

fn piece_value_chg(piece: ChessPiece, played_move: ChessMove, is_endgame: bool) -> (i16, i16) {
    let (new_material, new_pst) = piece_value(
        ChessPiece {
            piece_type: piece.piece_type,
            color: piece.color,
            position: played_move.to,
        },
        is_endgame,
    );
    let (old_material, old_pst) = piece_value(
        ChessPiece {
            piece_type: piece.piece_type,
            color: piece.color,
            position: played_move.from,
        },
        is_endgame,
    );

    (new_material - old_material, new_pst - old_pst)
}

pub fn piece_value(piece: ChessPiece, is_endgame: bool) -> (i16, i16) {
    let material = if MATERIAL_VALUE {
        material_value(piece.piece_type)
    } else {
        0
    };

    let pst = positional_value(piece, is_endgame);

    match piece.color {
        Color::White => (material, pst),
        Color::Black => (-material, -pst),
    }
}

fn material_value(piece_type: impl Into<PieceType>) -> i16 {
    match piece_type.into() {
        PieceType::Pawn => 100,
        PieceType::Knight => 300,
        PieceType::Bishop => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 25000,
    }
}

fn positional_value(piece: ChessPiece, is_endgame: bool) -> i16 {
    let rank = match piece.color {
        Color::White => 7 - piece.position.1,
        Color::Black => piece.position.1,
    } as usize;

    let file = piece.position.0 as usize;

    match (piece.piece_type, is_endgame) {
        (PieceType::Pawn, _) => PAWN_TABLE[rank][file],
        (PieceType::Knight, _) => KNIGHT_TABLE[rank][file],
        (PieceType::Bishop, _) => BISHOP_TABLE[rank][file],
        (PieceType::Rook, _) => ROOK_TABLE[rank][file],
        (PieceType::Queen, _) => QUEEN_TABLE[rank][file],
        (PieceType::King, false) => KING_TABLE[rank][file],
        (PieceType::King, true) => KING_ENDGAME_TABLE[rank][file],
    }
    // println!("{:?} {}", piece, res);
}
