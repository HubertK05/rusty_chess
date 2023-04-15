use std::collections::BTreeMap;

use backend::{
    board_setup::models::{Board, FenNotation}, move_generator::{models::{Moves, Color, PieceType, Square}}, move_register::models::{MoveType, ChessMove}, chess_bot::{search_game_tree, search_game_tree_base_case, get_ordered_moves, evaluate_chg, is_endgame, is_attacked_by_pawn},
};
use easybench::bench;

fn main() {
    let board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).unwrap();
    // let board = Board::try_from(FenNotation("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".into())).unwrap();
    let mut rep_map = BTreeMap::new();
    let Moves(moves) = Moves::get_all_moves(&board, board.turn);
    let test_move = ChessMove { move_type: MoveType::Move(PieceType::Pawn), from: Square(4, 1), to: Square(4, 3) };

    println!("eval chg - {}", bench(|| evaluate_chg(&board, test_move, is_endgame(&board))));
    println!("score move - {}", bench(|| {
        let mov = &test_move;

        let positional_chg = match board.turn {
            Color::White => -evaluate_chg(&board, *mov, is_endgame(&board)),
            Color::Black => evaluate_chg(&board, *mov, is_endgame(&board)),
        };
    
        if is_attacked_by_pawn(&board, test_move.to) {
            2000 + positional_chg
        } else {
            1000 + positional_chg
        }
    }));
    println!("score and order moves - {}", bench(|| {
        let mut move_set = moves.clone();
        move_set.sort_by_cached_key(|mov| {
            let positional_chg = match board.turn {
                Color::White => -evaluate_chg(&board, *mov, is_endgame(&board)),
                Color::Black => evaluate_chg(&board, *mov, is_endgame(&board)),
            };

            if is_attacked_by_pawn(&board, mov.to) {
                2000 + positional_chg
            } else {
                1000 + positional_chg
            }
        });
        move_set
    }));
    println!("search to depth 2 - {}", bench(|| search_game_tree(&board, 0, 2, i16::MIN, board.hash_board(), &mut rep_map)));
    println!("last depth search - {}", bench(|| search_game_tree_base_case(moves.clone(), &board, i16::MIN, board.hash_board(), &mut rep_map)));
    println!("get unordered moves - {}", bench(|| Moves::get_all_moves(&board, board.turn)));
    println!("get ordered moves - {}", bench(|| get_ordered_moves(&board)));
}
