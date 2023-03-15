use std::collections::{HashSet, HashMap};

use crate::{board_setup::models::Board, move_register::ChessMove};

use super::{models::{Square, Color, MoveDir, CheckedAdd, Offset, PieceType, CheckSquares, PinSquares, PinDir, Attacked, Moves}, MAX_MOVES_IN_A_SERIES, ChessPiece, KNIGHT_MOVES, QUEEN_MOVES};

const POTENTIALLY_ATTACKED_SQUARES: [Offset; 13] = [Offset(0, 1), Offset(0, -1), Offset(-1, 0), Offset(1, 0), Offset(-1, 1), Offset(-1, -1), Offset(1, 1), Offset(1, -1), Offset(1, 0), Offset(2, 0), Offset(-1, 0), Offset(-2, 0), Offset(-3, 0)];

pub fn get_attacked(board: &Board, color: Color) -> Attacked {
    let sq = match color {
        Color::White => board.king_positions.0,
        Color::Black => board.king_positions.1,
    };
    
    let mut res = HashSet::new();
    for dir in POTENTIALLY_ATTACKED_SQUARES {
        let Some(target_sq) = sq.c_add(Offset::from(dir)) else {
            continue
        };

        if is_attacked(board, target_sq, color) {
            res.insert(target_sq);
        }
    }

    Attacked(res)
}

pub fn is_attacked(board: &Board, sq: Square, color: Color) -> bool {
    for dir in QUEEN_MOVES {
        for i in 1..=MAX_MOVES_IN_A_SERIES {
            let offset = Offset::from(dir) * i as i8;
            let Some(target_sq) = sq.c_add(offset) else {
                break
            };

            let Some(p) = board.get_square(target_sq) else {
                continue
            };

            if p.piece_type() == PieceType::King && p.color() == color {
                continue
            };

            if attack_condition(offset, color, p.piece_type()) && p.color() != color {
                return true
            } else {
                break
            };
        }
    }

    for offset in KNIGHT_MOVES {
        let Some(target_sq) = sq.c_add(offset) else {
            continue
        };

        let Some(p) = board.get_square(target_sq) else {
            continue
        };

        if attack_condition(offset, color, p.piece_type()) && p.color() != color {
            return true
        };
    }

    return false
}

pub fn get_checked(board: &Board, color: Color) -> CheckSquares {
    let sq = match color {
        Color::White => board.king_positions.0,
        Color::Black => board.king_positions.1,
    };

    let mut res = CheckSquares {
        squares: HashSet::new(),
        checks_amount: 0,
    };

    for dir in QUEEN_MOVES {
        let mut squares = HashSet::new();
        for i in 1..=MAX_MOVES_IN_A_SERIES {
            let offset = Offset::from(dir) * i as i8;
            let Some(target_sq) = sq.c_add(offset) else {
                break
            };

            squares.insert(target_sq);

            let Some(p) = board.get_square(target_sq) else {
                continue
            };

            if attack_condition(offset, color, p.piece_type()) && p.color() != color {
                res.squares.extend(squares);
                res.checks_amount += 1;
                break
            };
        }
    }

    for offset in KNIGHT_MOVES {
        let Some(target_sq) = sq.c_add(offset) else {
            continue
        };

        let Some(p) = board.get_square(target_sq) else {
            continue
        };

        if attack_condition(offset, color, p.piece_type()) && p.color() != color {
            res.squares.insert(target_sq);
            res.checks_amount += 1;
            continue
        };
    }

    res
}

pub fn get_pins(board: &Board, color: Color) -> PinSquares {
    let sq = match color {
        Color::White => board.king_positions.0,
        Color::Black => board.king_positions.1,
    };

    let mut res = HashMap::new();

    for dir in QUEEN_MOVES {
        let mut pin_sq: Option<Square> = None;
        for i in 1..=MAX_MOVES_IN_A_SERIES {
            let offset = Offset::from(dir) * i as i8;
            let Some(target_sq) = sq.c_add(offset) else {
                break
            };

            let Some(p) = board.get_square(target_sq) else {
                continue
            };

            if p.color() == color && pin_sq.is_none() {
                pin_sq = Some(target_sq);
            } else if attack_condition(offset, color, p.piece_type()) && p.color() != color {
                if let Some(s) = pin_sq {
                    res.insert(s, PinDir::from(dir));
                }
                break
            } else {
                break
            };
        }
    }

    if let Some(en_passant_sq) = board.en_passant_square {
        let mut your_pawn_count = 0;
        for dir in [MoveDir::Left, MoveDir::Right] {
            let Some(p) = board.get_square(en_passant_sq + Offset::from(dir)) else {
                continue
            };
            if p.piece_type() == PieceType::Pawn && p.color() == color {
                your_pawn_count += 1;
            }
        }
        if your_pawn_count != 1 {
            return PinSquares(res);
        }

        for dir in [MoveDir::Left, MoveDir::Right] {
            let mut pin_sq: Option<Square> = None;
            for i in 1..=MAX_MOVES_IN_A_SERIES {
                let offset = Offset::from(dir) * i as i8;
                let Some(target_sq) = sq.c_add(offset) else {
                    break
                };
    
                if target_sq == en_passant_sq {
                    continue
                }

                let Some(p) = board.get_square(target_sq) else {
                    continue
                };
    
                if p.color() == color && p.piece_type() == PieceType::Pawn && pin_sq.is_none() {
                    pin_sq = Some(target_sq);
                } else if attack_condition(offset, color, p.piece_type()) && p.color() != color {
                    if let Some(s) = pin_sq {
                        res.insert(s, PinDir::EnPassantBlock);
                    }
                    break
                } else {
                    break
                };
            }
        }
    }

    PinSquares(res)
}

fn attack_condition(offset: Offset, color: Color, piece: PieceType) -> bool {
    match piece {
        PieceType::Pawn => {
            match color {
                Color::White => offset == Offset(-1, 1) || offset == Offset(1, 1),
                Color::Black => offset == Offset(-1, -1) || offset == Offset(1, -1),
            }
        },
        PieceType::Knight => (offset.0.abs() == 2 && offset.1.abs() == 1) || (offset.0.abs() == 1 && offset.1.abs() == 2),
        PieceType::Bishop => offset.0.abs() == offset.1.abs(),
        PieceType::Rook => offset.0.abs() == 0 || offset.1.abs() == 0,
        PieceType::Queen => offset.0.abs() == 0 || offset.1.abs() == 0 || offset.0.abs() == offset.1.abs(),
        PieceType::King => offset.0.abs() <= 1 || offset.1.abs() <= 1,
    }
}

pub fn filter_with_checked(moves: Vec<Box<dyn ChessMove>>, checked: &CheckSquares) -> Vec<Box<dyn ChessMove>> {
    match checked.checks_amount {
        0 => moves,
        1 => moves.into_iter().filter(|x| checked.squares.contains(&x.to())).collect(),
        2 => Vec::new(),
        _ => unreachable!(),
    }
}

pub fn filter_with_pins(moves: Vec<Box<dyn ChessMove>>, pins: &PinSquares) -> Vec<Box<dyn ChessMove>> {
    moves.into_iter().filter(|x| {
        if let Some(dir) = pins.0.get(&x.from()) {
            pin_condition(x.to() - x.from(), *dir)
        } else {
            true
        }
    }).collect()
}

fn pin_condition(offset: Offset, pin_dir: PinDir) -> bool {
    match pin_dir {
        PinDir::Vertical => offset.0 == 0,
        PinDir::Horizontal => offset.1 == 0,
        PinDir::LeftDiagonal => offset.0 == -offset.1,
        PinDir::RightDiagonal => offset.0 == offset.1,
        PinDir::EnPassantBlock => true,
    }
}
