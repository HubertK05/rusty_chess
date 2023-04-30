pub mod bitmasks;
pub mod evaluation;
pub mod pawn_structure;
pub mod piece_tables;
pub mod space_eval;
pub mod zobrist;

use rand::{seq::SliceRandom, thread_rng};
use std::{cmp::Ordering, collections::BTreeMap};

use self::{
    evaluation::Evaluation,
    pawn_structure::evaluate_pawn_weaknesses,
    piece_tables::{evaluate_chg, piece_value},
    space_eval::Space,
    zobrist::hash_with_move,
};
use crate::{
    board_setup::models::{Board, FenNotation},
    move_generator::{
        models::{ChessPiece, Color, Moves, Offset, PieceType, Square},
        restrictions::get_checked,
    },
    move_register::models::{ChessMove, MoveType},
    opening_book::{move_parser::parse_move, OpeningBook},
};

const EVAL_PRINT: bool = true;
const PRUNING: bool = true;
const POSITIONAL_VALUE_FACTOR: i32 = 60;
const MATERIAL_VALUE: bool = true;
const SEARCH_DEPTH: u8 = 6;

pub const MVV_LVA_TABLE: [[i16; 6]; 6] = [
    //K   Q   R   B   N   P
    [0, 0, 0, 0, 0, 0],       // K
    [12, 15, 12, 11, 11, 10], // Q
    [13, 20, 15, 13, 13, 12], // R
    [14, 26, 25, 15, 14, 13], // B
    [14, 32, 31, 30, 15, 13], // N
    [39, 38, 37, 36, 35, 15], // P
];

pub const fn is_forcing(test_move: ChessMove) -> bool {
    match test_move.move_type {
        MoveType::Move(_) | MoveType::CastleMove(_) => false,
        _ => true,
    }
}

#[derive(Clone)]
pub struct MovePayload {
    played_move: Option<ChessMove>,
    eval: Evaluation,
    line: Vec<ChessMove>,
}

impl MovePayload {
    fn new(played_move: Option<ChessMove>, eval: Evaluation, line: Vec<ChessMove>) -> Self {
        Self {
            played_move,
            eval,
            line,
        }
    }

    fn with_turn(turn: Color) -> Self {
        Self {
            played_move: None,
            eval: match turn {
                Color::White => Evaluation::MIN,
                Color::Black => Evaluation::MAX,
            },
            line: Vec::new(),
        }
    }

    fn better_move(self, other: Self, turn: Color) -> Self {
        match (turn, self.eval.cmp(&other.eval)) {
            (Color::White, Ordering::Greater) | (Color::Black, Ordering::Less) => self,
            _ => other,
        }
    }
}

pub fn choose_move(
    board: &Board,
    mut rep_map: BTreeMap<u64, u8>,
    book: &OpeningBook,
) -> Option<ChessMove> {
    let limit = match board.turn {
        Color::White => i16::MAX,
        Color::Black => i16::MIN,
    };

    let fen = FenNotation::try_from(board).expect("failed to get fen from board");

    if let Some(move_vec) = book.0.get(&fen.to_draw_fen()) {
        let mut rng = thread_rng();
        let san = move_vec
            .choose_weighted(&mut rng, |(_, popularity)| *popularity)
            .unwrap();
        let res = parse_move(fen, san.0.clone()).expect("cannot parse move");
        println!("played book move");

        Some(res)
    } else {
        let hash = board.hash_board();

        let (payload, pos_count) = if SEARCH_DEPTH == 1 {
            search_game_tree(board, 0, SEARCH_DEPTH, limit as i32, hash, &mut rep_map)
        } else {
            search_game_tree(board, 0, SEARCH_DEPTH, limit as i32, hash, &mut rep_map)
        };
        println!(
            "eval: {}\nthe number of positions tested: {pos_count}",
            payload.eval
        );
        payload.played_move
    }
}

fn is_in_check(board: &Board) -> bool {
    get_checked(board, board.turn).checks_amount != 0
}

pub fn is_endgame(board: &Board) -> bool {
    if board.mating_material.0 <= 12 && board.mating_material.1 <= 12 {
        return true;
    };
    for rank in 0..8 {
        for file in 0..8 {
            let piece = board.get_square(Square(file, rank));
            if let Some(p) = piece {
                if p.piece_type == PieceType::Queen {
                    return false;
                }
            }
        }
    }
    return true;
}

pub fn get_ordered_moves(board: &Board) -> Vec<ChessMove> {
    let Moves(mut move_set) = Moves::get_all_moves(&board, board.turn);
    move_set.sort_by_cached_key(|mov| match mov.move_type {
        MoveType::Move(pt) => {
            let base_eval_chg = evaluate_chg(board, *mov, is_endgame(board));
            let eval_chg = match board.turn {
                Color::White => -base_eval_chg.total(),
                Color::Black => base_eval_chg.total(),
            };

            if is_attacked_by_pawn(board, mov.to) && pt != PieceType::Pawn {
                2000 + eval_chg as i16
            } else {
                1000 + eval_chg as i16
            }
        }
        MoveType::Capture(attacker) => {
            let victim = board
                .get_square(mov.to)
                .expect("no piece found where it should be")
                .piece_type;
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
        }
        MoveType::EnPassantMove => 100,
        MoveType::CastleMove(_) => 101,
        MoveType::PromotionMove(_) => 1,
        MoveType::PromotionCapture(_) => 0,
    });
    move_set
}

pub fn search_game_tree(
    board: &Board,
    depth: u8,
    max_depth: u8,
    limit: i32,
    hash: u64,
    rep_map: &mut BTreeMap<u64, u8>,
) -> (MovePayload, u64) {
    let move_set = get_ordered_moves(board);
    let base_eval = evaluate_position(board).with_positional_factor(POSITIONAL_VALUE_FACTOR);
    let is_endgame = is_endgame(board);

    if move_set.len() == 0 {
        if is_in_check(board) {
            let eval = match board.turn {
                Color::White => -25000 + depth as i16 * 100,
                Color::Black => 25000 - depth as i16 * 100,
            };
            let mut res = Evaluation::new();
            res.material = eval;
            return (MovePayload::new(None, res, Vec::new()), 1);
        } else {
            return (MovePayload::new(None, Evaluation::new(), Vec::new()), 1);
        }
    }

    let mut payload = MovePayload::with_turn(board.turn);
    let mut position_count = 0;

    for test_move in move_set.into_iter() {
        let mut new_board = *board;
        let new_hash = hash_with_move(hash, board, test_move);
        (&mut new_board)
            .register_move(test_move)
            .expect("oops, failed to register move during game search");

        let rep_num = *rep_map.entry(new_hash).and_modify(|x| *x += 1).or_insert(1);
        let (branch_payload, branch_pos_count) = if rep_num >= 3 {
            (MovePayload::new(None, Evaluation::new(), Vec::new()), 1)
        } else {
            let (mut branch_payload, branch_pos_count) =
                if depth < max_depth - 1 || (depth == max_depth - 1 && is_forcing(test_move)) {
                    search_game_tree(
                        &new_board,
                        depth + 1,
                        max_depth,
                        payload.eval.total() as i32,
                        new_hash,
                        rep_map,
                    )
                } else {
                    (
                        MovePayload::new(
                            Some(test_move),
                                base_eval
                                + evaluate_chg(board, test_move, is_endgame)
                                .with_positional_factor(POSITIONAL_VALUE_FACTOR),
                            Vec::new(),
                        ),
                        1,
                    )
                };
            if is_endgame && depth == 0 && branch_payload.played_move.is_some() {
                add_king_dist(&mut branch_payload.eval, &new_board);
            }

            (branch_payload, branch_pos_count)
        };

        payload = payload.better_move(
            MovePayload {
                played_move: Some(test_move),
                eval: branch_payload.eval,
                line: branch_payload.line.clone(),
            },
            board.turn,
        );
        rep_map.entry(new_hash).and_modify(|x| *x -= 1);

        if depth == 0 && EVAL_PRINT {
            if branch_payload.played_move.is_some() {
                print_eval(new_board, test_move, branch_payload.eval, &branch_payload.line);
            } else {
                println!("evaluation of the move {test_move}: pruned/no legal move");
            }
        }

        position_count += branch_pos_count;

        if PRUNING {
            match board.turn {
                Color::White => {
                    if branch_payload.eval.total() >= limit {
                        return (
                            MovePayload::new(None, Evaluation::MAX, Vec::new()),
                            position_count,
                        );
                    }
                }
                Color::Black => {
                    if branch_payload.eval.total() <= limit {
                        return (
                            MovePayload::new(None, Evaluation::MIN, Vec::new()),
                            position_count,
                        );
                    }
                }
            };
        }
    }

    payload.line.extend(payload.played_move);
    (payload, position_count)
}

fn evaluate_position(board: &Board) -> Evaluation {
    let is_endgame = is_endgame(board);
    let mut res = Evaluation::new();
    for rank in 0..8 {
        for file in 0..8 {
            let piece = board.get_square(Square(file, rank));
            if let Some(p) = piece {
                let (material, pst) = piece_value(p, is_endgame);
                res.material += material;
                res.pst += pst;
            }
        }
    }

    let pawn_weakness_score = evaluate_pawn_weaknesses(board);

    let space_eval = if !is_endgame {
        let space = Space::get_from_board(board);
        space.evaluate(board)
    } else {
        0
    };

    res.pawn_structure += pawn_weakness_score;
    res.space += space_eval;
    res
}

pub fn is_attacked_by_pawn(board: &Board, sq: Square) -> bool {
    match board.turn {
        Color::White => {
            if let Some(ChessPiece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
                ..
            }) = board.get_square(sq + Offset(-1, 1))
            {
                true
            } else if let Some(ChessPiece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
                ..
            }) = board.get_square(sq + Offset(1, 1))
            {
                true
            } else {
                false
            }
        }
        Color::Black => {
            if let Some(ChessPiece {
                piece_type: PieceType::Pawn,
                color: Color::White,
                ..
            }) = board.get_square(sq + Offset(-1, -1))
            {
                true
            } else if let Some(ChessPiece {
                piece_type: PieceType::Pawn,
                color: Color::White,
                ..
            }) = board.get_square(sq + Offset(1, -1))
            {
                true
            } else {
                false
            }
        }
    }
}

fn add_king_dist(eval: &mut Evaluation, board: &Board) {
    let new_offset = board.king_positions.0 - board.king_positions.1;
    let king_dist_chg = match board.turn {
        Color::White => -(new_offset.0.abs() + new_offset.1.abs()) * 2,
        Color::Black => (new_offset.0.abs() + new_offset.1.abs()) * 2,
    } as i16;
    eval.king_dist += king_dist_chg;
}

fn print_eval(mut board: Board, played_move: ChessMove, eval: Evaluation, line: &[ChessMove]) {
    println!("evaluation of the move {played_move}: {}", eval);
    print!("with line: {}, ", played_move);
    line.iter().rev().for_each(|&mov| {
        print!("{mov}, ");
        let _ = board.register_move(mov);
    });
    println!("which results in board:\n{board}");
}
