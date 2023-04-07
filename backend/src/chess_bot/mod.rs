use crate::{move_register::models::{ChessMove, MoveType, PromotedPieceType}, board_setup::models::Board, move_generator::{models::{Moves, Square, Color, PieceType}, restrictions::get_checked}};

pub fn choose_move(board: &Board) -> Option<ChessMove> {
    let res = search_game_tree(board, 0, 4);
    println!("{}", res.1);
    res.0
}

fn search_game_tree(board: &Board, depth: u8, max_depth: u8) -> (Option<ChessMove>, i16) {
    let Moves(move_set) = Moves::get_all_moves(&board, board.turn);
    if depth == max_depth - 1 {
        let base_eval = evaluate_position(board);
        let move_iter = move_set
            .into_iter()
            .map(|test_move| {
                let mut new_board = *board;
                (&mut new_board).register_move(test_move).expect("oops, failed to register move during game search");
                
                let chg = evaluate_chg(board, test_move);
                (test_move, base_eval + chg)
            });

        let max_eval = match board.turn {
            Color::White => move_iter.max_by_key(|node| node.1),
            Color::Black => move_iter.min_by_key(|node| node.1),
        };
    
        return match max_eval {
            Some(eval) => (Some(eval.0), eval.1),
            None => if get_checked(board, board.turn).checks_amount != 0 {
                (None, match board.turn {
                    Color::White => -25000 + depth as i16,
                    Color::Black => 25000 - depth as i16,
                })
            } else {
                (None, 0)
            }
        }
    };

    let move_iter = move_set
        .into_iter()
        .map(|test_move| {
            let mut new_board = *board;
            (&mut new_board).register_move(test_move).expect("oops, failed to register move during game search");
            (test_move, search_game_tree(&new_board, depth + 1, max_depth).1)
        });

    let max_eval = match board.turn {
        Color::White => move_iter.max_by_key(|node| node.1),
        Color::Black => move_iter.min_by_key(|node| node.1),
    };

    match max_eval {
        Some(eval) => (Some(eval.0), eval.1),
        None => if get_checked(board, board.turn).checks_amount != 0 {
            (None, match board.turn {
                Color::White => -25000 + depth as i16,
                Color::Black => 25000 - depth as i16,
            })
        } else {
            (None, 0)
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
