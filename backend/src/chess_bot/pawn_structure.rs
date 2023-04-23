use crate::{board_setup::models::Board, move_generator::models::{PieceType, Color, Square}};

pub struct PawnStructure {
    pub white: [u8; 8],
    pub black: [u8; 8],
}

impl PawnStructure {
    pub fn new() -> Self {
        Self {
            white: [0; 8],
            black: [0; 8],
        }
    }

    pub fn get_from_board(board: &Board) -> Self {
        let mut res = PawnStructure::new();
        for rank in 1..=6 {
            for file in 0..=7 {
                if let Some(piece) = board.get_square(Square(file as i8, rank as i8)) {
                    if piece.piece_type == PieceType::Pawn && piece.color == Color::White {
                        res.white[file] += 1;
                    } else if piece.piece_type == PieceType::Pawn && piece.color == Color::Black {
                        res.black[file] += 1;
                    }
                }
            }
        }
        res
    }

    pub fn count_doubled_pawns(&self) -> (u8, u8) {
        (self.white.iter().filter(|&&x| x >= 2).count() as u8, self.black.iter().filter(|&&x| x >= 2).count() as u8)
    }

    pub fn count_isolated_pawns(&self) -> (u8, u8) {
        (isolated_pawns_for_side(self.white), isolated_pawns_for_side(self.black))
    }
}

fn isolated_pawns_for_side(structure: [u8; 8]) -> u8 {
    let mut ext_structure = [0; 10];
    structure.into_iter().enumerate().for_each(|(i, val)| ext_structure[i + 1] = val);
    ext_structure.windows(3).filter(|&x| x[0] == 0 && x[1] != 0 && x[2] == 0).count() as u8
}

pub fn get_pawn_weaknesses_from_board(board: &Board) -> (u8, u8, u8, u8) {
    let structure = PawnStructure::get_from_board(board);
    let doubled = structure.count_doubled_pawns();
    let isolated = structure.count_isolated_pawns();
    (doubled.0, doubled.1, isolated.0, isolated.1)
}

pub fn evaluate_pawn_weaknesses(board: &Board) -> i16 {
    let structure = PawnStructure::get_from_board(board);
    let doubled = structure.count_doubled_pawns();
    let isolated = structure.count_isolated_pawns();
    (doubled.1 as i16 - doubled.0 as i16 + isolated.1 as i16 - isolated.0 as i16) * 50
}
