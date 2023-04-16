use rand::{thread_rng, Rng};

use crate::{board_setup::models::Board, move_generator::models::{Square, PieceType, Color, Offset}, move_register::models::{CastleType, ChessMove, MoveType}};

use super::bitmasks::INIT_ZOBRIST_BITMASKS;

#[derive(Clone, Copy, Debug)]
pub struct ZobristBitmasks {
    pub square: [[u64; 13]; 64],
    pub additional: [u64; 5],
}

#[derive(Clone, Copy, PartialEq)]
pub enum HashedData {
    Square(Square, PieceType, Color),
    Castling(CastleType),
    EnPassant(Square),
    Turn,
}

pub const fn to_hash_idx(piece_type: PieceType, color: Color) -> usize {
    match (piece_type, color) {
        (PieceType::Pawn, Color::White) => 0,
        (PieceType::Pawn, Color::Black) => 1,
        (PieceType::Knight, Color::White) => 2,
        (PieceType::Knight, Color::Black) => 3,
        (PieceType::Bishop, Color::White) => 4,
        (PieceType::Bishop, Color::Black) => 5,
        (PieceType::Rook, Color::White) => 6,
        (PieceType::Rook, Color::Black) => 7,
        (PieceType::Queen, Color::White) => 8,
        (PieceType::Queen, Color::Black) => 9,
        (PieceType::King, Color::White) => 10,
        (PieceType::King, Color::Black) => 11,
    }
}

pub const fn castle_to_hash_idx(castle_type: CastleType) -> usize {
    match castle_type {
        CastleType::WhiteShort => 0,
        CastleType::WhiteLong => 1,
        CastleType::BlackShort => 2,
        CastleType::BlackLong => 3,
    }
}

impl ZobristBitmasks {
    pub fn new() -> ZobristBitmasks {
        let mut rng = thread_rng();
        let mut square_bitmasks = [[0; 13]; 64];
        for i in 0..64 {
            for j in 0..13 {
                square_bitmasks[i][j] = rng.gen();
            }
        }
        let additional_bitmasks = rng.gen();
    
        Self {
            square: square_bitmasks,
            additional: additional_bitmasks,
        }
    }
}

pub fn zobrist_hash(board: &Board) -> u64 {
    let mut res = 0;

    for rank in 0..8 as i8 {
        for file in 0..8 as i8 {
            if let Some(piece) = board.get_square(Square(file, rank)) {
                res = res.with(HashedData::Square(Square(file, rank), piece.piece_type, piece.color));
            }
        }
    }

    if board.castling.white_short {
        res = res.with(HashedData::Castling(CastleType::WhiteShort));
    }
    if board.castling.white_long {
        res = res.with(HashedData::Castling(CastleType::WhiteLong));
    }
    if board.castling.black_short {
        res = res.with(HashedData::Castling(CastleType::BlackShort));
    }
    if board.castling.black_long {
        res = res.with(HashedData::Castling(CastleType::BlackLong));
    }
    if board.turn == Color::Black {
        res = res.with(HashedData::Turn);
    }

    if let Some(sq) = board.en_passant_square {
        res = res.with(HashedData::EnPassant(sq));
    }

    res
}

pub trait ZobristHash {
    fn with(self, data: HashedData) -> Self;
}

impl ZobristHash for u64 {
    fn with(self, data: HashedData) -> Self {
        self ^ match data {
            HashedData::Square(sq, p_type, color) => INIT_ZOBRIST_BITMASKS.square[(sq.1 * 8 + sq.0) as usize][to_hash_idx(p_type, color)],
            HashedData::Castling(c_type) => INIT_ZOBRIST_BITMASKS.additional[castle_to_hash_idx(c_type)],
            HashedData::EnPassant(sq) => INIT_ZOBRIST_BITMASKS.square[(sq.1 * 8 + sq.0) as usize][12],
            HashedData::Turn => INIT_ZOBRIST_BITMASKS.additional[4],
        }
    }
}

pub fn hash_with_move(mut hash: u64, board: &Board, played_move: ChessMove) -> u64 {
    let moved_piece = board.get_square(played_move.from).expect("no piece found where it should be");
    hash = hash.with(HashedData::Square(played_move.from, moved_piece.piece_type, moved_piece.color));
    if let Some(sq) = board.en_passant_square {
        hash = hash.with(HashedData::EnPassant(sq));
    };
    match played_move.move_type {
        MoveType::Move(_) => {
            hash = hash.with(HashedData::Square(played_move.to, moved_piece.piece_type, moved_piece.color));
            if moved_piece.piece_type == PieceType::Pawn && (played_move.to - played_move.from).1.abs() == 2 {
                hash = hash.with(HashedData::EnPassant(played_move.to));
            };
        },
        MoveType::Capture(_) => {
            let captured_piece = board.get_square(played_move.to).expect("no piece found where it should be");
            hash = hash.with(HashedData::Square(played_move.to, moved_piece.piece_type, moved_piece.color))
                .with(HashedData::Square(played_move.to, captured_piece.piece_type, captured_piece.color));
        },
        MoveType::EnPassantMove => {
            let ep_target_sq = board.en_passant_square.expect("no en passant target square found");
            let pawn_sq = ep_target_sq + match board.turn {
                Color::White => Offset(0, -1),
                Color::Black => Offset(0, 1),
            };

            hash = hash.with(HashedData::Square(played_move.to, moved_piece.piece_type, moved_piece.color))
                .with(HashedData::Square(pawn_sq, PieceType::Pawn, moved_piece.color.opp()));
        },
        MoveType::CastleMove(castle_type) => {
            let (rook_from, rook_to) = match castle_type {
                CastleType::WhiteShort => (Square(7, 0), Square(5, 0)),
                CastleType::WhiteLong => (Square(0, 0), Square(3, 0)),
                CastleType::BlackShort => (Square(7, 7), Square(5, 7)),
                CastleType::BlackLong => (Square(0, 7), Square(3, 7)),
            };
            hash = hash.with(HashedData::Square(played_move.to, moved_piece.piece_type, moved_piece.color))
                .with(HashedData::Square(rook_from, PieceType::Rook, moved_piece.color))
                .with(HashedData::Square(rook_to, PieceType::Rook, moved_piece.color));
        },
        MoveType::PromotionMove(ppt) => {
            hash = hash.with(HashedData::Square(played_move.to, ppt.into(), moved_piece.color));
        },
        MoveType::PromotionCapture(ppt) => {
            let captured_piece = board.get_square(played_move.to).expect("no piece found where it should be");
            hash = hash.with(HashedData::Square(played_move.to, captured_piece.piece_type, captured_piece.color))
                .with(HashedData::Square(played_move.to, ppt.into(), moved_piece.color));
        },
    };
    hash = hash_with_castling(hash, board, played_move);
    hash.with(HashedData::Turn)
}

fn hash_with_castling(mut hash: u64, board: &Board, played_move: ChessMove) -> u64 {
    if board.castling.white_long && (played_move.from == Square(0, 0) || played_move.to == Square(0, 0) || played_move.from == Square(4, 0)) {
        hash = hash.with(HashedData::Castling(CastleType::WhiteLong));
    }
    if board.castling.white_short && (played_move.from == Square(7, 0) || played_move.to == Square(7, 0) || played_move.from == Square(4, 0)) {
        hash = hash.with(HashedData::Castling(CastleType::WhiteShort));
    }
    if board.castling.black_long && (played_move.from == Square(0, 7) || played_move.to == Square(0, 7) || played_move.from == Square(4, 7)) {
        hash = hash.with(HashedData::Castling(CastleType::BlackLong));
    }
    if board.castling.black_short && (played_move.from == Square(7, 7) || played_move.to == Square(7, 7) || played_move.from == Square(4, 7)) {
        hash = hash.with(HashedData::Castling(CastleType::BlackShort));
    }
    hash
}

#[cfg(test)]
mod tests {
    use crate::{board_setup::models::Board, move_register::models::{ChessMove, MoveType}, move_generator::models::{Square, PieceType}};

    use super::hash_with_move;

    #[test]
    fn hash_with_test() {
        let mut board = Board::new_game();
        let played_move = ChessMove {
            move_type: MoveType::Move(PieceType::Pawn),
            from: Square(0, 6),
            to: Square(0, 5),
        };
        let hash_1 = board.hash_board(); 
        let hash_3 = hash_with_move(hash_1, &board, played_move);
        board.register_move(played_move).unwrap();
        let hash_2 = board.hash_board();
        assert_eq!(hash_2, hash_3);
    }
}
