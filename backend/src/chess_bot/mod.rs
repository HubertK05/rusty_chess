use std::cmp::{min, max};

use crate::{move_register::models::{ChessMove, MoveType, PromotedPieceType}, board_setup::models::Board, move_generator::{models::{Moves, Square, Color, PieceType}, restrictions::get_checked}};

pub fn choose_move(board: &Board) -> Option<ChessMove> {
    let limit = match board.turn {
        Color::White => i16::MAX,
        Color::Black => i16::MIN,
    };

    let res = search_game_tree(board, 0, 4, limit);
    println!("{}", res.1);
    res.0
}

fn search_game_tree(board: &Board, depth: u8, max_depth: u8, limit: i16) -> (Option<ChessMove>, i16, i128, i128) {
    // limit passed in to the function is the max for white and the min for black
    // when the limit is exceeded or at least approached, the search in the same function will discontinue and will return inf for white and -inf for black

    let Moves(mut move_set) = Moves::get_all_moves(&board, board.turn);
    move_set.sort_by_key(|mov| {
        match mov.move_type {
            MoveType::Move(_) => 4,
            MoveType::Capture(_) => 2,
            MoveType::EnPassantMove => 3,
            MoveType::CastleMove(_) => 4,
            MoveType::PromotionMove(_) => 1,
            MoveType::PromotionCapture(_) => 0,
        }
    });

    if depth == max_depth - 1 {
        let base_eval = evaluate_position(board);
        let move_set_len = move_set.len();
        let move_iter = move_set.into_iter();
        let mut eval_sum: i128 = 0;
        let mut move_iter_2 = Vec::new();
        for test_move in move_iter {
            let mut new_board = *board;
            (&mut new_board).register_move(test_move).expect("oops, failed to register move during game search");
            
            let chg = evaluate_chg(board, test_move);
            let res = base_eval + chg;

            match board.turn {
                Color::White => {
                    if res > limit {
                        return (None, i16::MAX, 0, 0);
                    }
                },
                Color::Black => {
                    if res < limit {
                        return (None, i16::MIN, 0, 0);
                    }
                },
            };

            // println!("normal at depth 3 {res}");
            eval_sum += res as i128;
            move_iter_2.push((Some(test_move), res));
        };

        let max_eval = match board.turn {
            Color::White => move_iter_2.into_iter().max_by_key(|node| node.1),
            Color::Black => move_iter_2.into_iter().min_by_key(|node| node.1),
        };
    
        return match max_eval {
            Some(eval) => (eval.0, eval.1, eval_sum, move_set_len as i128),
            None => if get_checked(board, board.turn).checks_amount != 0 {
                let eval = match board.turn {
                    Color::White => -25000 + depth as i16,
                    Color::Black => 25000 - depth as i16,
                };
                (None, eval, eval as i128, 1)
            } else {
                (None, 0, 0, 1)
            }
        }
    };

    let mut new_limit = match board.turn {
        Color::White => i16::MIN,
        Color::Black => i16::MAX,
    };

    let mut eval_sum: i128 = 0;
    let mut position_num: i128 = 0;

    let move_iter = move_set.into_iter();
    let mut move_iter_2 = Vec::new();
    for test_move in move_iter {
        let mut new_board = *board;
        (&mut new_board).register_move(test_move).expect("oops, failed to register move during game search");
        
        let (_, res, ev_sum, pos_num) = search_game_tree(&new_board, depth + 1, max_depth, new_limit);

        new_limit = match board.turn {
            Color::White => max(new_limit, res),
            Color::Black => min(new_limit, res),
        };

        match board.turn {
            Color::White => {
                if res > limit {
                    return (None, i16::MAX, 0, 0);
                }
            },
            Color::Black => {
                if res < limit {
                    return (None, i16::MIN, 0, 0);
                }
            },
        };

        move_iter_2.push((Some(test_move), res, ev_sum, pos_num));
        // println!("eval sum = {} depth = {}", ev_sum, depth);
        eval_sum += ev_sum;
        position_num += pos_num;
    };

    let max_eval = if depth == 0 {
        match board.turn {
            Color::White => move_iter_2.into_iter().max_by_key(|node| (node.1 as f64 + if node.3 == 0 {
                0f64
            } else {
                node.2 as f64 / node.3 as f64
            }).round() as i64 / 10),
            Color::Black => move_iter_2.into_iter().min_by_key(|node| (node.1 as f64 + if node.3 == 0 {
                0f64
            } else {
                node.2 as f64 / node.3 as f64
            }).round() as i64 / 10),
        }
    } else {
        match board.turn {
            Color::White => move_iter_2.into_iter().max_by_key(|node| node.1),
            Color::Black => move_iter_2.into_iter().min_by_key(|node| node.1),
        }
    };

    match max_eval {
        Some(eval) => (eval.0, eval.1, eval_sum, position_num),
        None => if get_checked(board, board.turn).checks_amount != 0 {
            let eval = match board.turn {
                Color::White => -25000 + depth as i16,
                Color::Black => 25000 - depth as i16,
            };
            (None, eval, eval as i128, 1)
        } else {
            (None, 0, 0, 1)
        }
    }
}

fn evaluate_position(board: &Board) -> i16 {
    let mut res = 0;
    for rank in 0..8 {
        for file in 0..8 {
            let piece = board.get_square(Square(file, rank));
            if let Some(p) = piece {
                res += match p.color {
                    Color::White => piece_value(p.piece_type),
                    Color::Black => -piece_value(p.piece_type),
                };
            }
        }
    }

    res
}

fn evaluate_chg(board: &Board, mov: ChessMove) -> i16 {
    let chg = match mov.move_type {
        MoveType::Move(_) => 0,
        MoveType::Capture(_) => {
            let square = board.get_square(mov.to);
            match square {
                Some(piece) => piece_value(piece.piece_type),
                None => 0,
            }
        },
        MoveType::EnPassantMove => 100,
        MoveType::CastleMove(_) => 0,
        MoveType::PromotionMove(ppt) => {
            match ppt {
                PromotedPieceType::Queen => 800,
                PromotedPieceType::Knight => 200,
                PromotedPieceType::Bishop => 200,
                PromotedPieceType::Rook => 400,
            }
        },
        MoveType::PromotionCapture(ppt) => {
            let square = board.get_square(mov.to);
            let cap_chg = match square {
                Some(piece) => piece_value(piece.piece_type),
                None => 0,
            };
            
            cap_chg + match ppt {
                PromotedPieceType::Queen => 800,
                PromotedPieceType::Knight => 200,
                PromotedPieceType::Bishop => 200,
                PromotedPieceType::Rook => 400,
            }
        },
    };

    match board.turn {
        Color::White => chg,
        Color::Black => -chg,
    }
}

fn piece_value(piece_type: PieceType) -> i16 {
    match piece_type {
        PieceType::Pawn => 100,
        PieceType::Knight => 300,
        PieceType::Bishop => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 25000,
    }
}
