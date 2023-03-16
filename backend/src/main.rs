use std::{io, collections::HashMap};

use backend::{
    board_setup::models::{Board, FenNotation},
    move_generator::{models::Moves, restrictions::get_checked},
};

fn main() {
    let mut board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).unwrap();
    let mut repetition_map: HashMap<String, u8> = HashMap::new();
    repetition_map.insert("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq".into(), 1);
    loop {
        println!("{board}");
        println!("{:?}", FenNotation::try_from(&board).expect("wrong board"));
        let color = board.turn;

        let mut moves = Moves::get_all_moves(&board, color);

        if moves.0.is_empty() {
            match get_checked(&board, color).checks_amount {
                0 => println!("No moves left - draw by stalemate."),
                _ => println!("{} wins by checkmate.", color.opp().to_string())
            }
            break
        }

        println!("Available moves: {} ({})", moves, moves.0.len());
        println!("Please input your move choice.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice
            .trim()
            .parse::<usize>()
            .expect("Failed to convert this value to a number");
        let chosen_move = moves.0.remove(choice);

        board
            .register_move(&*chosen_move)
            .expect("wtf is this move??");

        if board.half_move_timer_50 >= 100 {
            println!("{board}");
            println!("Draw by the 50 move rule.");
            break;
        }

        if board.mating_material.0 < 3 && board.mating_material.1 < 3 {
            println!("{board}");
            println!("Draw by insufficient mating material.");
            break;
        }

        let fen = FenNotation::from(&board);
        let rep_number = *repetition_map.entry(fen.to_draw_fen())
            .and_modify(|position_num| {
                *position_num += 1;
            })
            .or_insert(1);

        if rep_number >= 3 {
            println!("{board}");
            println!("Draw by threefold repetition.");
            break
        }
    }
}
