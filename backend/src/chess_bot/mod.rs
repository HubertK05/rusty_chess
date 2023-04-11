pub mod piece_tables;
use std::cmp::Ordering;
use crate::{move_register::models::{ChessMove, MoveType, CastleType, PromotedPieceType}, board_setup::models::Board, move_generator::{models::{Moves, Square, Color, PieceType, ChessPiece}, restrictions::get_checked}};
use self::piece_tables::{PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE, ROOK_TABLE, QUEEN_TABLE, KING_TABLE, KING_ENDGAME_TABLE};

const PRUNING: bool = true;
const POSITIONAL_VALUE: bool = true;
const MATERIAL_VALUE: bool = true;
const SEARCH_DEPTH: u8 = 6;

const MVV_LVA_TABLE: [[u8; 6]; 6] = [
    //K   Q   R   B   N   P
    [ 0,  0,  0,  0,  0,  0], // K
    [12, 15, 12, 11, 11, 10], // Q
    [13, 20, 15, 13, 13, 12], // R
    [14, 26, 25, 15, 14, 13], // B
    [14, 32, 31, 30, 15, 13], // N
    [39, 38, 37, 36, 35, 15], // P
];

pub fn choose_move(board: &Board) -> Option<ChessMove> {
    let limit = match board.turn {
        Color::White => i16::MAX,
        Color::Black => i16::MIN,
    };

    let res = search_game_tree(board, 0, SEARCH_DEPTH, limit);
    println!("eval: {}\nthe number of positions tested: {}", res.1, res.2);
    res.0
}

fn is_in_check(board: &Board) -> bool {
    get_checked(board, board.turn).checks_amount != 0
}

fn is_endgame(board: &Board) -> bool {
    for rank in 0..8 {
        for file in 0..8 {
            let piece = board.get_square(Square(file, rank));
            if let Some(p) = piece {
                if p.piece_type == PieceType::Queen {
                    return false
                }
            }
        }
    }
    return true
}

fn get_ordered_moves(board: &Board) -> Vec<ChessMove> {
    let Moves(mut move_set) = Moves::get_all_moves(&board, board.turn);
    move_set.sort_by_key(|mov| {
        match mov.move_type {
            MoveType::Move(_) => 101,
            MoveType::Capture(attacker) => {
                let victim = board.get_square(mov.to).expect("no piece found where it should be").piece_type;
                let victim_idx = match victim {
                    PieceType::Pawn => 5,
                    PieceType::Knight => 4,
                    PieceType::Bishop => 3,
                    PieceType::Rook => 2,
                    PieceType::Queen => 1,
                    PieceType::King => 0,
                };
                let attacker_idx = match attacker {
                    PieceType::Pawn => 5,
                    PieceType::Knight => 4,
                    PieceType::Bishop => 3,
                    PieceType::Rook => 2,
                    PieceType::Queen => 1,
                    PieceType::King => 0,
                };

                MVV_LVA_TABLE[victim_idx][attacker_idx]
            },
            MoveType::EnPassantMove => 100,
            MoveType::CastleMove(_) => 101,
            MoveType::PromotionMove(_) => 1,
            MoveType::PromotionCapture(_) => 0,
        }
    });
    move_set
}

fn new_best_move(best_move: Option<ChessMove>, best_eval: i16, curr_move: ChessMove, curr_eval: i16, turn: Color) -> (Option<ChessMove>, i16) {
    match (turn, best_eval.cmp(&curr_eval)) {
        (Color::White, Ordering::Less) => (Some(curr_move), curr_eval),
        (Color::White, Ordering::Greater) => (best_move, best_eval),
        (Color::Black, Ordering::Less) => (best_move, best_eval),
        (Color::Black, Ordering::Greater) => (Some(curr_move), curr_eval),
        (_, Ordering::Equal) => (Some(curr_move), curr_eval)
    }
}

fn search_game_tree_base_case(move_set: Vec<ChessMove>, board: &Board, limit: i16) -> (Option<ChessMove>, i16, u64) {
    let is_endgame = is_endgame(board);
    let base_eval = evaluate_position(board, is_endgame);
    let move_iter = move_set.into_iter();
    let mut position_num = 0;

    let mut best_move: Option<ChessMove> = None;
    let mut best_eval = match board.turn {
        Color::White => i16::MIN,
        Color::Black => i16::MAX,
    };

    for test_move in move_iter {
        let chg = evaluate_chg(board, test_move, is_endgame);
        let res = base_eval + chg;
        (best_move, best_eval) = new_best_move(best_move, best_eval, test_move, res, board.turn);

        position_num += 1;

        if PRUNING {
            match board.turn {
                Color::White => if res >= limit {
                    return (None, i16::MAX, position_num);
                },
                Color::Black => if res <= limit {
                    return (None, i16::MIN, position_num);
                },
            };
        }

        let mut new_board = *board;
        let _ = (&mut new_board).register_move(test_move);
    };

    (best_move, best_eval, position_num)
}

fn search_game_tree(board: &Board, depth: u8, max_depth: u8, limit: i16) -> (Option<ChessMove>, i16, u64) {
    let move_set = get_ordered_moves(board);
    if move_set.len() == 0 {
        if is_in_check(board) {
            let eval = match board.turn {
                Color::White => -25000 + depth as i16,
                Color::Black => 25000 - depth as i16,
            };
            return (None, eval, 1)
        } else {
            return (None, 0, 1)
        }
    }

    if depth == max_depth - 1 {
        return search_game_tree_base_case(move_set, board, limit)
    };

    let mut best_move: Option<ChessMove> = None;
    let mut best_eval = match board.turn {
        Color::White => i16::MIN,
        Color::Black => i16::MAX,
    };

    let mut position_num = 0;

    for test_move in move_set.into_iter() {
        let mut new_board = *board;
        (&mut new_board).register_move(test_move).expect("oops, failed to register move during game search");

        let (_, res, lower_pos_num) = search_game_tree(&new_board, depth + 1, max_depth, best_eval);
        
        (best_move, best_eval) = new_best_move(best_move, best_eval, test_move, res, board.turn);

        if depth == 0 {
            match board.turn {
                Color::White => println!("the move {test_move} is evaluated to {res}"),
                Color::Black => println!("the move {test_move} is evaluated to {res}"),
            }
        }

        position_num += lower_pos_num;

        if PRUNING {
            match board.turn {
                Color::White => if res >= limit {
                    return (None, i16::MAX, position_num);
                },
                Color::Black => if res <= limit {
                    return (None, i16::MIN, position_num);
                },
            };
        }
    };

    (best_move, best_eval, position_num)
}

fn evaluate_position(board: &Board, is_endgame: bool) -> i16 {
    let mut res = 0;
    for rank in 0..8 {
        for file in 0..8 {
            let piece = board.get_square(Square(file, rank));
            if let Some(p) = piece {
                res += piece_value(p, is_endgame)
            }
        }
    }

    res
}

fn evaluate_chg(board: &Board, mov: ChessMove, is_endgame: bool) -> i16 {
    let from = board.get_square(mov.from).expect("no piece found where it should be");    

    piece_value_chg(from, mov, is_endgame) + match mov.move_type {
        MoveType::Move(_) => 0,
        MoveType::Capture(_) => {
            let to = board.get_square(mov.to).expect("no piece found where it should be");
            -piece_value(to, is_endgame)
        },
        MoveType::EnPassantMove => {
            let captured_piece = board.en_passant_square.and_then(|sq| board.get_square(sq))
                .expect("no pawn is the en passant target square");
            -piece_value(captured_piece, is_endgame)
        },
        MoveType::CastleMove(castle_type) => castle_value_chg_for_rook(castle_type),
        MoveType::PromotionMove(ppt) => promoted_material_value_chg(ppt),
        MoveType::PromotionCapture(ppt) => {
            let to = board.get_square(mov.to).expect("no piece found where it should be");
            -piece_value(to, is_endgame) + promoted_material_value_chg(ppt)
        }
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

fn piece_value_chg(piece: ChessPiece, played_move: ChessMove, is_endgame: bool) -> i16 {
    piece_value(ChessPiece {
        piece_type: piece.piece_type,
        color: piece.color,
        position: played_move.to,
    }, is_endgame) - piece_value(ChessPiece {
        piece_type: piece.piece_type,
        color: piece.color,
        position: played_move.from,
    }, is_endgame)
}
 
fn piece_value(piece: ChessPiece, is_endgame: bool) -> i16 {
    let mut res = 0;
    if MATERIAL_VALUE {
        res += material_value(piece.piece_type)
    }
    if POSITIONAL_VALUE {
        res += positional_value(piece, is_endgame)
    }
    match piece.color {
        Color::White => res,
        Color::Black => -res,
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
    
    let res = match (piece.piece_type, is_endgame) {
        (PieceType::Pawn, _) => PAWN_TABLE[rank][file],
        (PieceType::Knight, _) => KNIGHT_TABLE[rank][file],
        (PieceType::Bishop, _) => BISHOP_TABLE[rank][file],
        (PieceType::Rook, _) => ROOK_TABLE[rank][file],
        (PieceType::Queen, _) => QUEEN_TABLE[rank][file],
        (PieceType::King, false) => KING_TABLE[rank][file],
        (PieceType::King, true) => KING_ENDGAME_TABLE[rank][file],
    };
    // println!("{:?} {}", piece, res);
    res
}

#[cfg(test)] 
mod tests {
    use crate::{move_generator::models::{ChessPiece, PieceType, Square, Color}, chess_bot::{piece_value, MATERIAL_VALUE, POSITIONAL_VALUE}};

    #[test]
    fn piece_value_eval_test_1() {
        let piece = ChessPiece {
            piece_type: PieceType::Pawn,
            color: Color::White,
            position: Square(3, 6),
        };

        assert_eq!(
            piece_value(piece, false),
            match (MATERIAL_VALUE, POSITIONAL_VALUE) {
                (true, true) => 150,
                (true, false) => 100,
                (false, true) => 50,
                (false, false) => 0,
            }
        )
    }

    #[test]
    fn piece_value_eval_test_2() {
        let piece = ChessPiece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
            position: Square(3, 6),
        };

        assert_eq!(
            piece_value(piece, false),
            match (MATERIAL_VALUE, POSITIONAL_VALUE) {
                (true, true) => -80,
                (true, false) => -100,
                (false, true) => 20,
                (false, false) => 0,
            }
        )
    }
}
