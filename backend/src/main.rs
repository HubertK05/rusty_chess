use std::{io, collections::{HashSet, HashMap}};

use backend::{Board, Moves, Color, Attacked, CheckSquares, PinSquares, PiecePosition, PieceType, Vec2, move_register::MoveType, PinDir, FenNotation};

fn main() {
    // let mut board = Board::try_from(
    //     [
    //     ["r", "n", "b", ".", "k", "b", "n", "r"],
    //     ["p", "p", "p", "p", "q", "p", "p", "p"],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", "n", ".", "."],
    //     ["P", "P", "P", "P", "P", "P", ".", "P"],
    //     ["R", "N", "B", "Q", "K", "B", "N", "R"],
    //     ]
    // ).unwrap();
    // println!("{board}");
    // println!("White moves: {}", Moves::get_all_moves(&board, Color::White));
    // println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
    // println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
    // println!("Black attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::Black));
    // println!("Black can block white's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::White));
    // println!("White can block black's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::Black));
    // println!("White pins black's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::White));
    // println!("Black pins white's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::Black));

    // let mut board = Board::try_from(
    //     [
    //     ["r", "n", "b", "q", "k", "b", "n", "r"],
    //     ["p", "p", "p", "p", "p", "p", "p", "p"],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     ["P", "P", "P", "P", "P", "P", "P", "P"],
    //     ["R", "N", "B", "Q", "K", "B", "N", "R"],
    //     ]
    // ).unwrap();

    // let mut board = Board::try_from(
    //     [
    //     ["r", ".", ".", ".", "k", ".", ".", "r"],
    //     ["p", ".", "p", "p", "q", "p", "b", "."],
    //     ["b", "n", ".", ".", "p", "n", "p", "."],
    //     [".", ".", ".", "P", "N", ".", ".", "."],
    //     [".", "p", ".", ".", "P", ".", ".", "."],
    //     [".", ".", "N", ".", ".", "Q", ".", "p"],
    //     ["P", "P", "P", "B", "B", "P", "P", "P"],
    //     ["R", ".", ".", ".", "K", ".", ".", "R"],
    //     ]
    // ).unwrap();

    let mut board = Board::try_from(
        [
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", "p", ".", ".", ".", ".", "."],
        [".", ".", ".", "p", ".", ".", ".", "."],
        ["K", "P", ".", ".", ".", ".", ".", "r"],
        [".", "R", ".", ".", ".", "p", ".", "k"],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", "P", ".", "P", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        ]
    ).unwrap();

    // let mut board = Board::try_from(
    //     [
    //     ["r", ".", ".", ".", "k", ".", ".", "r"],
    //     ["P", "p", "p", "p", ".", "p", "p", "p"],
    //     [".", "b", ".", ".", ".", "n", "b", "N"],
    //     ["n", "P", ".", ".", ".", ".", ".", "."],
    //     ["B", "B", "P", ".", "P", ".", ".", "."],
    //     ["q", ".", ".", ".", ".", "N", ".", "."],
    //     ["P", "p", ".", "P", ".", ".", "P", "P"],
    //     ["R", ".", ".", "Q", ".", "R", "K", "."],
    //     ]
    // ).unwrap();

    // let mut board = Board::try_from(
    //     [
    //     ["r", "n", "b", "q", ".", "k", ".", "r"],
    //     ["p", "p", ".", "P", "b", "p", "p", "p"],
    //     [".", ".", "p", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     [".", ".", "B", ".", ".", ".", ".", "."],
    //     [".", ".", ".", ".", ".", ".", ".", "."],
    //     ["P", "P", "P", ".", "N", "n", "P", "P"],
    //     ["R", "N", "B", "Q", "K", ".", ".", "R"],
    //     ]
    // ).unwrap();

    // let mut board = Board::try_from(
    //     [
    //     ["r", ".", ".", ".", ".", "r", "k", "."],
    //     [".", "p", "p", ".", "q", "p", "p", "p"],
    //     ["p", ".", "n", "p", ".", "n", ".", "."],
    //     [".", ".", "b", ".", "p", ".", "B", "."],
    //     [".", ".", "B", ".", "P", ".", "b", "."],
    //     ["P", ".", "N", "P", ".", "N", ".", "."],
    //     [".", "P", "P", ".", "Q", "P", "P", "P"],
    //     ["R", ".", ".", ".", ".", "R", "K", "."],
    //     ]
    // ).unwrap();

    let mut repetition_map: HashMap<String, u8> = HashMap::new();
    repetition_map.insert("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq".into(), 1);
    loop {
        println!("{board}");

        let color = match board.half_move_number % 2 {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!(),
        };

        let opponent_color = match board.half_move_number % 2 {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };

        let moves = Moves::get_all_moves(&board, color);

        if moves.0.is_empty() {
            match CheckSquares::get_all_checked_squares(&board, opponent_color).1 {
                0 => println!("No moves left - draw by stalemate."),
                _ => println!("{} wins by checkmate.", opponent_color.to_string())
            }
            break
        }
        
        println!("Available moves: {} ({})", moves, moves.0.len());
        // println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
        // println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
        // println!("Opponent can block your check on these squares: {}", CheckSquares::get_all_checked_squares(&board, color));

        println!("Please input your move choice.");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim().parse::<usize>().expect("Failed to convert this value to a number");
        let chosen_move = &moves.0[choice];

        chosen_move.register_move(&mut board).expect("wtf is this move??");

        if board.half_move_timer_50 >= 100 {
            println!("{board}");
            println!("Draw by the 50 move rule.");
            break
        }

        if board.white_mating_material < 3 && board.black_mating_material < 3 {
            println!("{board}");
            println!("Draw by insufficient mating material.");
            break
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

        board.half_move_number += 1;

        // println!("debug: move timer: {}, white mating material: {}, black_mating_material: {}", &board.half_move_timer_50, &board.white_mating_material, &board.black_mating_material);
        // println!("debug: fen notation: {:?}", &fen);
    }
    
    // println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
    // println!("Black attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::Black));
    // println!("Black can block white's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::White));
    // println!("White can block black's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::Black));
    // println!("White pins black's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::White));
    // println!("Black pins white's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::Black));

    // for i in 1..6 {
    //     println!("Number of positions to depth {}: {:?}", i, test_count_moves(board.clone(), 0, i));
    // }

    // let board = Board::try_from(
    //     [
    //     ["q", ".", ".", ".", "n", "r", ".", "k"],
    //     [".", ".", ".", ".", "b", "p", "p", "p"],
    //     [".", ".", ".", "p", ".", ".", ".", "."],
    //     ["r", "B", ".", ".", ".", "P", "P", "."],
    //     [".", ".", "n", "B", "P", ".", ".", "P"],
    //     ["N", "p", ".", ".", "Q", ".", ".", "."],
    //     [".", "P", ".", ".", ".", ".", ".", "."],
    //     ["K", ".", ".", "R", ".", ".", ".", "R"],
    //     ]
    // ).unwrap();
    // println!("{board}");
    // println!("White moves: {}", Moves::get_all_moves(&board, Color::White));
    // println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
    // println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
    // println!("Black attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::Black));
    // println!("Black can block white's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::White));
    // println!("White can block black's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::Black));
    // println!("White pins black's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::White));
    // println!("Black pins white's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::Black));

    
}

// fn test_count_moves(board: Board, depth: u8, max_depth: u8) -> (u64, u64) {
//     let (color, attacked) = match depth % 2 {
//         0 => (Color::Black, Attacked::get_attacked_squares(&board, Color::White)),
//         1 => (Color::White, Attacked::get_attacked_squares(&board, Color::Black)),
//         _ => unreachable!(),
//     };
//     if depth == max_depth {
//         // for file in 0..7 {
//         //     for rank in 0..7 {
//         //         match &board.board[file][rank] {
//         //             Some(piece) if piece.piece_type() == PieceType::King && piece.color() == color => {
//         //                 if attacked.0.contains(&PiecePosition::from(Vec2(file as i8, rank as i8))) {
//         //                     println!("Illegal position detected:\n{}", board);
//         //                     return (1, 1);
//         //                 }
//         //             },
//         //             _ => (),
//         //         };
//         //     }
//         // }
//         return (1, 0)
//     };
//     let move_set = match depth % 2 {
//         0 => Moves::get_all_moves(&board, Color::White),
//         1 => Moves::get_all_moves(&board, Color::Black),
//         _ => unreachable!(),
//     };
//     move_set.0.into_iter().map(|test_move| {
//         let mut new_board = board.clone();
//         let _ = test_move.register_move(&mut new_board).expect("wtf is this?");
//         let mut en_passant_detected = false;
//         if depth == max_depth - 1 {
//             if test_move.move_type() == MoveType::EnPassantMove {
//                 en_passant_detected = true;
//             }
//             // println!("En passant detected:\n{}\n{}", &board, &new_board);
//         }
//         let mut res = test_count_moves(new_board, depth + 1, max_depth);
//         if en_passant_detected { res.1 = 1 }
//         res
//     }).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
// }