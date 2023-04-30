use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Write,
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use backend::{
    board_setup::models::{Board, FenNotation},
    chess_bot::{
        choose_move, get_ordered_moves, is_endgame,
        piece_tables::evaluate_chg, search_game_tree,
    },
    move_generator::models::{MoveRestrictionData, Moves, PieceType, Square},
    move_register::models::{ChessMove, MoveType},
    opening_book::{get_opening_book, OpeningBook},
};
use easybench::bench;

const DO_BENCHMARKS: bool = true;
const CREATE_BOOK: bool = false;

fn main() {
    let mut board = Board::new_game();
    let mut game_counter = 1;
    let mut rep_map = BTreeMap::new();
    let book = OpeningBook::from_file("opening_book.txt");

    struct OptionThread {
        thread: Option<JoinHandle<ChessMove>>,
    }

    let mut bot = OptionThread { thread: None };

    loop {
        if bot.thread.is_none() {
            let new_board = *&board;
            let new_rep_map = rep_map.clone();
            let new_book = book.clone();
            let x = thread::spawn(move || {
                choose_move(&new_board, new_rep_map, &new_book).expect("no move chosen")
            });
            bot.thread = Some(x);
        }
        let thread = bot.thread.take();
        if let Some(th) = thread {
            if th.is_finished() {
                let played_move = th.join().expect("join error");
                board
                    .register_move(played_move)
                    .expect("failed to register the move");
                println!("Board state (game {game_counter}):\n{}", board);
                if is_completed(&board, &mut rep_map) {
                    board = Board::new_game();
                    rep_map = BTreeMap::new();
                    game_counter += 1;
                    println!("NEW GAME ({game_counter})");
                    println!("{}", board);
                    sleep(Duration::from_secs(1));
                }
            } else {
                bot.thread = Some(th);
            }
        }
    }

    if DO_BENCHMARKS {
        do_benchmarks();
    }
    if CREATE_BOOK {
        // create_opening_book().await;
    }
}

fn is_completed(board: &Board, rep_map: &mut BTreeMap<u64, u8>) -> bool {
    let legal_moves = Moves::get_all_moves(board, board.turn);
    let position_count = *rep_map
        .entry(board.hash_board())
        .and_modify(|x| *x += 1)
        .or_insert(1);
    if board.half_move_timer_50 > 100 {
        return true;
    } else if board.mating_material.0 < 3 && board.mating_material.1 < 3 {
        return true;
    } else if legal_moves.0.is_empty() {
        if MoveRestrictionData::get(board, board.turn)
            .check_squares
            .checks_amount
            != 0
        {
            return true;
        } else {
            return true;
        }
    } else if position_count >= 3 {
        return true;
    } else {
        false
    }
}

fn do_benchmarks() {
    let board = Board::try_from(FenNotation(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into(),
    ))
    .unwrap();
    let mut rep_map = BTreeMap::new();
    let Moves(_moves) = Moves::get_all_moves(&board, board.turn);
    let test_move = ChessMove {
        move_type: MoveType::Move(PieceType::Pawn),
        from: Square(4, 1),
        to: Square(4, 3),
    };

    println!(
        "eval chg - {}",
        bench(|| evaluate_chg(&board, test_move, is_endgame(&board)))
    );
    println!(
        "search to depth 2 - {}",
        bench(|| search_game_tree(&board, 0, 2, i32::MIN, board.hash_board(), &mut rep_map))
    );
    println!(
        "last depth search - {}",
        bench(|| search_game_tree(&board, 0, 1, i32::MIN, board.hash_board(), &mut rep_map))
    );
    println!(
        "get unordered moves - {}",
        bench(|| Moves::get_all_moves(&board, board.turn))
    );
    println!(
        "get ordered moves - {}",
        bench(|| get_ordered_moves(&board))
    );
}

async fn create_opening_book() {
    let mut file = File::create("opening_book.txt").unwrap();
    let mut book = OpeningBook(HashMap::new());
    get_opening_book(
        &mut book,
        FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()),
    )
    .await
    .unwrap();
    // println!("{:#?}", book);

    let book_json = serde_json::to_string(&book).unwrap();
    file.write(book_json.as_bytes()).unwrap();
}
