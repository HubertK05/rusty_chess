use std::{collections::{BTreeMap, HashMap}, fs::File, io::{Write, Read}};

use backend::{
    board_setup::models::{Board, FenNotation}, move_generator::models::{Moves, Color, PieceType, Square}, move_register::models::{MoveType, ChessMove}, chess_bot::{search_game_tree, search_game_tree_base_case, get_ordered_moves, evaluate_chg, is_endgame, is_attacked_by_pawn}, opening_book::{get_opening_book, OpeningBook},
};
use easybench::bench;

const DO_BENCHMARKS: bool = false;
const CREATE_BOOK: bool = false;

#[tokio::main]
async fn main() {
    if DO_BENCHMARKS {
        do_benchmarks();
    }
    if CREATE_BOOK {
        // create_opening_book().await;
    }
}

fn do_benchmarks() {
    let board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).unwrap();
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

async fn create_opening_book() {
    let mut file = File::create("opening_book.txt").unwrap();
    let mut book = OpeningBook(HashMap::new());
    get_opening_book(&mut book, FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).await.unwrap();
    // println!("{:#?}", book);

    let book_json = serde_json::to_string(&book).unwrap();
    file.write(book_json.as_bytes()).unwrap();
}
