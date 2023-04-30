use crate::{
    board_setup::models::Board,
    move_generator::models::{Color, PieceType, Square},
};

#[derive(Clone, Copy, Debug)]
pub struct Space {
    queenside: (i8, i8),
    central: (i8, i8),
    kingside: (i8, i8),
}

impl Space {
    pub fn get_from_board(board: &Board) -> Self {
        let mut res = [(0, 0); 8];
        for file in 0..8 {
            res[file] = get_space_from_file(board, file as i8);
        }

        Self {
            queenside: (
                res[0].0 + res[1].0 + res[2].0,
                res[0].1 + res[1].1 + res[2].1,
            ),
            central: (res[3].0 + res[4].0, res[3].1 + res[4].1),
            kingside: (
                res[5].0 + res[6].0 + res[7].0,
                res[5].1 + res[6].1 + res[7].1,
            ),
        }
    }

    pub fn evaluate(self, board: &Board) -> i16 {
        let white_king_file = board.king_positions.0 .0;
        let black_king_file = board.king_positions.1 .0;

        let white_kingside_weight = match white_king_file {
            0..=2 => 5,
            3..=4 => 0,
            5..=7 => -5,
            _ => unreachable!(),
        };

        let white_queenside_weight = match white_king_file {
            0..=2 => -5,
            3..=4 => 0,
            5..=7 => 5,
            _ => unreachable!(),
        };

        let black_kingside_weight = match black_king_file {
            0..=2 => 5,
            3..=4 => 0,
            5..=7 => -5,
            _ => unreachable!(),
        };

        let black_queenside_weight = match black_king_file {
            0..=2 => -5,
            3..=4 => 0,
            5..=7 => 5,
            _ => unreachable!(),
        };

        let central_weight = 5;

        let queenside_eval = self.queenside.0 as i16 * white_queenside_weight
            - self.queenside.1 as i16 * black_queenside_weight;
        let central_eval = (self.central.0 as i16 - self.central.1 as i16) * central_weight;
        let kingside_eval = self.kingside.0 as i16 * white_kingside_weight
            - self.kingside.1 as i16 * black_kingside_weight;

        queenside_eval + central_eval + kingside_eval
    }
}

fn get_space_from_file(board: &Board, file: i8) -> (i8, i8) {
    let mut white_res = None;
    let mut black_res = None;
    for rank in 0..8 {
        let Some(piece) = board.get_square(Square(file, rank)) else {
            continue
        };
        if piece.piece_type == PieceType::Pawn {
            if piece.color == Color::White {
                white_res.get_or_insert(rank);
            } else {
                let _ = black_res.insert(7 - rank);
            }
        }
    }

    let white_space = white_res.unwrap_or(7 - black_res.unwrap_or(-1));
    let black_space = black_res.unwrap_or(7 - white_res.unwrap_or(-1));

    (white_space, black_space)
}
