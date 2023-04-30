use anyhow::Context;
use regex::Regex;
use thiserror::Error;

use crate::{
    board_setup::models::{Board, BoardError, FenNotation},
    move_generator::models::{Color, Moves, PieceType, Square},
    move_register::models::{CastleType, ChessMove, MoveType, PromotedPieceType, RawMoveType},
};

pub fn parse_move(fen: FenNotation, san: String) -> Result<ChessMove, MoveParseError> {
    let board = Board::try_from(fen)?;
    let moves = Moves::get_all_moves(&board, board.turn);

    let pawn_move = Regex::new(r"^([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if pawn_move.is_match(&san) {
        let captures = &pawn_move.captures(&san).unwrap();
        let moves = moves
            .search_with_piece_type(PieceType::Pawn)
            .search_with_raw_move_types(&[RawMoveType::Move]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[0]);
        return Ok(filtered_moves[0]);
    }

    let piece_move =
        Regex::new(r"^([BKNQR])([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if piece_move.is_match(&san) {
        let captures = piece_move.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::Move]);
        let moves = filter_moves_with_piece_type(moves, &captures[1]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[2]);
        return Ok(filtered_moves[0]);
    }

    let pawn_capture =
        Regex::new(r"^([a-h])x([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if pawn_capture.is_match(&san) {
        let captures = pawn_capture.captures(&san).unwrap();
        let moves = moves
            .search_with_piece_type(PieceType::Pawn)
            .search_with_raw_move_types(&[RawMoveType::Capture, RawMoveType::EnPassantMove]);
        let moves = filter_moves_with_file_letter(moves, &captures[1]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[2]);
        return Ok(filtered_moves[0]);
    }

    let piece_capture =
        Regex::new(r"^([BKNQR])x([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if piece_capture.is_match(&san) {
        let captures = piece_capture.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::Capture]);
        let moves = filter_moves_with_piece_type(moves, &captures[1]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[2]);
        return Ok(filtered_moves[0]);
    }

    let piece_move =
        Regex::new(r"^([BKNQR])([a-h])([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if piece_move.is_match(&san) {
        let captures = piece_move.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::Move]);
        let moves = filter_moves_with_piece_type(moves, &captures[1]);
        let moves = filter_moves_with_file_letter(moves, &captures[2]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[3]);
        return Ok(filtered_moves[0]);
    }

    let piece_move =
        Regex::new(r"^([BKNQR])([1-8])([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if piece_move.is_match(&san) {
        let captures = piece_move.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::Move]);
        let moves = filter_moves_with_piece_type(moves, &captures[1]);
        let moves = filter_moves_with_rank_number(moves, &captures[2]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[3]);
        return Ok(filtered_moves[0]);
    }

    let piece_capture =
        Regex::new(r"^([BKNQR])([a-h])x([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if piece_capture.is_match(&san) {
        let captures = piece_capture.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::Capture]);
        let moves = filter_moves_with_piece_type(moves, &captures[1]);
        let moves = filter_moves_with_file_letter(moves, &captures[2]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[3]);
        return Ok(filtered_moves[0]);
    }

    let piece_capture =
        Regex::new(r"^([BKNQR])([1-8])x([a-h][1-8])\+?#?$").context("Regex creation failed")?;
    if piece_capture.is_match(&san) {
        let captures = piece_capture.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::Capture]);
        let moves = filter_moves_with_piece_type(moves, &captures[1]);
        let moves = filter_moves_with_rank_number(moves, &captures[2]);
        let Moves(filtered_moves) = filter_moves_with_to_square(moves, &captures[3]);
        return Ok(filtered_moves[0]);
    }

    let promotion = Regex::new(r"^([a-h][1-8])=([BNQR])\+?#?$").context("Regex creation failed")?;
    if promotion.is_match(&san) {
        let captures = promotion.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::PromotionMove]);
        let moves = filter_moves_with_to_square(moves, &captures[1]);
        let Moves(filtered_moves) = filter_moves_with_promoted_piece_type(moves, &captures[2]);
        return Ok(filtered_moves[0]);
    }

    let promotion_capture =
        Regex::new(r"^([a-h])x([a-h][1-8])=([BNQR])\+?#?$").context("Regex creation failed")?;
    if promotion_capture.is_match(&san) {
        let captures = promotion_capture.captures(&san).unwrap();
        let moves = moves.search_with_raw_move_types(&[RawMoveType::PromotionCapture]);
        let moves = filter_moves_with_file_letter(moves, &captures[1]);
        let moves = filter_moves_with_to_square(moves, &captures[2]);
        let Moves(filtered_moves) = filter_moves_with_promoted_piece_type(moves, &captures[3]);
        return Ok(filtered_moves[0]);
    }

    if &san == "O-O" {
        return Ok(match board.turn {
            Color::White => ChessMove {
                move_type: MoveType::CastleMove(CastleType::WhiteShort),
                from: Square(4, 0),
                to: Square(6, 0),
            },
            Color::Black => ChessMove {
                move_type: MoveType::CastleMove(CastleType::BlackShort),
                from: Square(4, 7),
                to: Square(6, 7),
            },
        });
    }

    if &san == "O-O-O" {
        return Ok(match board.turn {
            Color::White => ChessMove {
                move_type: MoveType::CastleMove(CastleType::WhiteLong),
                from: Square(4, 0),
                to: Square(2, 0),
            },
            Color::Black => ChessMove {
                move_type: MoveType::CastleMove(CastleType::BlackLong),
                from: Square(4, 7),
                to: Square(2, 7),
            },
        });
    }

    Err(MoveParseError::InvalidMove)
}

fn filter_moves_with_to_square(moves: Moves, to: &str) -> Moves {
    let to_square = Square::try_from(to).unwrap();
    moves.search_with_to(to_square)
}

fn filter_moves_with_file_letter(moves: Moves, file_letter: &str) -> Moves {
    let from_file = (file_letter.chars().next().unwrap() as u8 - 97) as i8;
    moves.search_with_from_file(from_file)
}

fn filter_moves_with_rank_number(moves: Moves, rank_number: &str) -> Moves {
    let from_rank = rank_number.parse::<i8>().unwrap() - 1;
    moves.search_with_from_rank(from_rank)
}

fn filter_moves_with_piece_type(moves: Moves, pt: &str) -> Moves {
    let piece_type = PieceType::try_from(pt).unwrap();
    moves.search_with_piece_type(piece_type)
}

fn filter_moves_with_promoted_piece_type(moves: Moves, ppt: &str) -> Moves {
    let promoted_piece_type = PromotedPieceType::try_from(ppt).unwrap();
    moves.search_with_promoted_piece_type(promoted_piece_type)
}

#[derive(Error, Debug)]
pub enum MoveParseError {
    #[error("Invalid move")]
    InvalidMove,
    #[error("Board error")]
    BoardError(#[from] BoardError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::parse_move;
    use crate::{
        board_setup::models::FenNotation,
        move_generator::models::{PieceType, Square},
        move_register::models::{CastleType, ChessMove, MoveType, PromotedPieceType},
    };

    #[test]
    fn pawn_move_test() {
        let res = parse_move(
            FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()),
            "e4".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::Move(PieceType::Pawn),
                from: Square(4, 1),
                to: Square(4, 3),
            }
        )
    }

    #[test]
    fn piece_move_test() {
        let res = parse_move(
            FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()),
            "Nf3".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::Move(PieceType::Knight),
                from: Square(6, 0),
                to: Square(5, 2),
            }
        )
    }

    #[test]
    fn pawn_capture_test() {
        let res = parse_move(
            FenNotation("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2".into()),
            "exd5".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::Capture(PieceType::Pawn),
                from: Square(4, 3),
                to: Square(3, 4),
            }
        )
    }

    #[test]
    fn en_passant_test() {
        let res = parse_move(
            FenNotation("rnbqkbnr/pp1ppppp/8/8/2pPP3/5N2/PPP2PPP/RNBQKB1R b KQkq d3 0 3".into()),
            "cxd3".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::EnPassantMove,
                from: Square(2, 3),
                to: Square(3, 2),
            }
        )
    }

    #[test]
    fn piece_move_with_file_test() {
        let res = parse_move(
            FenNotation(
                "r4rk1/pppq1ppp/2npbn2/2b1p3/2B1P3/2NPBN2/PPPQ1PPP/R4RK1 w - - 10 9".into(),
            ),
            "Rad1".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::Move(PieceType::Rook),
                from: Square(0, 0),
                to: Square(3, 0),
            }
        )
    }

    #[test]
    fn piece_move_with_rank_test() {
        let res = parse_move(
            FenNotation("1r4k1/p1pq1ppp/Brnpbn2/4p3/4P3/bRNPBN2/P1PQ1PPP/1R4K1 w - - 8 15".into()),
            "R3b2".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::Move(PieceType::Rook),
                from: Square(1, 2),
                to: Square(1, 1),
            }
        )
    }

    #[test]
    fn piece_capture_with_file_test() {
        let res = parse_move(
            FenNotation("r1B2rk1/p1pq1ppp/2npbn2/4p3/4P3/2NPBN2/PbPQ1PPP/R4RK1 b - - 1 11".into()),
            "Rfxc8".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::Capture(PieceType::Rook),
                from: Square(5, 7),
                to: Square(2, 7),
            }
        )
    }

    #[test]
    fn piece_capture_with_rank_test() {
        let res = parse_move(
            FenNotation("1r4k1/pBpq1ppp/1rnpbn2/4p3/4P3/bRNPBN2/P1PQ1PPP/1R4K1 b - - 9 15".into()),
            "R8xb7".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::Capture(PieceType::Rook),
                from: Square(1, 7),
                to: Square(1, 6),
            }
        )
    }

    #[test]
    fn promotion_test() {
        let res = parse_move(
            FenNotation("r1bqkb1r/pPpp2pp/2n2n2/4pp2/8/8/PP1PPPPP/RNBQKBNR w KQkq - 1 5".into()),
            "b8=Q".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::PromotionMove(PromotedPieceType::Queen),
                from: Square(1, 6),
                to: Square(1, 7),
            }
        )
    }

    #[test]
    fn promotion_capture_test() {
        let res = parse_move(
            FenNotation("r1bqkb1r/pPpp2pp/2n2n2/4pp2/8/8/PP1PPPPP/RNBQKBNR w KQkq - 1 5".into()),
            "bxc8=Q".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::PromotionCapture(PromotedPieceType::Queen),
                from: Square(1, 6),
                to: Square(2, 7),
            }
        )
    }

    #[test]
    fn castle_test() {
        let res = parse_move(
            FenNotation(
                "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4".into(),
            ),
            "O-O".to_string(),
        );
        assert_eq!(
            res.unwrap(),
            ChessMove {
                move_type: MoveType::CastleMove(CastleType::WhiteShort),
                from: Square(4, 0),
                to: Square(6, 0),
            }
        )
    }
}
