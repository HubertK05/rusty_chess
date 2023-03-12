use std::{io, collections::HashMap};

fn main() {
    // let mut board = Board::try_from(FenNotation("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".into())).unwrap();
    // // let mut board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).unwrap();
    // // println!("{}, {:?}", &board, &board);

    // let mut repetition_map: HashMap<String, u8> = HashMap::new();
    // repetition_map.insert("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq".into(), 1);
    // loop {
    //     println!("{board}");
    //     println!("{:?}", &board);
    //     let color = board.turn;

    //     let opponent_color = color.opp();

    //     let moves = Moves::get_all_moves(&board, color);

    //     if moves.0.is_empty() {
    //         match CheckSquares::get_all_checked_squares(&board, opponent_color).1 {
    //             0 => println!("No moves left - draw by stalemate."),
    //             _ => println!("{} wins by checkmate.", opponent_color.to_string())
    //         }
    //         break
    //     }
        
    //     println!("Available moves: {} ({})", moves, moves.0.len());
    //     // println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
    //     // println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
    //     // println!("Opponent can block your check on these squares: {}", CheckSquares::get_all_checked_squares(&board, color));

    //     println!("Please input your move choice.");

    //     let mut choice = String::new();

    //     io::stdin()
    //         .read_line(&mut choice)
    //         .expect("Failed to read line");

    //     let choice = choice.trim().parse::<usize>().expect("Failed to convert this value to a number");
    //     let chosen_move = &moves.0[choice];

    //     chosen_move.register_move(&mut board).expect("wtf is this move??");

    //     if board.half_move_timer_50 >= 100 {
    //         println!("{board}");
    //         println!("Draw by the 50 move rule.");
    //         break
    //     }

    //     if board.mating_material.0 < 3 && board.mating_material.1 < 3 {
    //         println!("{board}");
    //         println!("Draw by insufficient mating material.");
    //         break
    //     }
        
    //     let fen = FenNotation::from(&board);
    //     let rep_number = *repetition_map.entry(fen.to_draw_fen())
    //         .and_modify(|position_num| {
    //             *position_num += 1;
    //         })
    //         .or_insert(1);

    //     if rep_number >= 3 {
    //         println!("{board}");
    //         println!("Draw by threefold repetition.");
    //         break
    //     }

    //     if color == Color::Black { board.full_move_number += 1 }
    //     board.turn = board.turn.opp();

    //     // println!("debug: move timer: {}, white mating material: {}, black_mating_material: {}", &board.half_move_timer_50, &board.white_mating_material, &board.black_mating_material);
    //     // println!("debug: fen notation: {:?}", &fen);
    //     // println!("debug: castling rights: {:?}", &board.castling);
    // }
}
