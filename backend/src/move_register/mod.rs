use crate::{
    board_setup::models::Board,
    move_generator::{
        models::{Bishop, Color, Knight, Offset, PieceType, Queen, Rook, Square, ChessPiece},
    },
};

use self::models::{
    Capture, CastleMove, CastleType, EnPassantMove, Move, MoveError, PromotedPieceType,
    PromotionCapture, PromotionMove,
};

pub mod models;

fn move_register_move(m: Move, board: &mut Board) -> Result<(), MoveError> {
    let p = board.take_piece(m.from)?;
    if p.piece_type() == PieceType::King {
        board.set_king_position(m.to, p.color())
    }

    if p.piece_type() == PieceType::Pawn {
        if (m.to - m.from).1.abs() == 2 {
            board.set_ep_target_square(Some(m.to));
        }
        board.reset_half_move_timer();
    }

    board.place_piece(p, m.to)?;

    Ok(())
}

fn capture_register_move(cap: Capture, board: &mut Board) -> Result<(), MoveError> {
    let p = board.take_piece(cap.from)?;
    let cap_p = board.take_piece(cap.to)?;

    if p.piece_type() == PieceType::King {
        board.set_king_position(cap.to, p.color())
    }

    board.change_mating_material(cap_p.color(), -(cap_p.mating_material_points() as i8));
    board.place_piece(p, cap.to)?;

    Ok(())
}

fn en_passant_register_move(ep: EnPassantMove, board: &mut Board) -> Result<(), MoveError> {
    let p = board.take_piece(ep.from)?;

    let offset = match p.color() {
        Color::White => Offset(0, -1),
        Color::Black => Offset(0, 1),
    };

    let cap_p = board.take_piece(ep.to + offset)?;

    board.change_mating_material(cap_p.color(), -3);
    board.place_piece(p, ep.to)?;

    Ok(())
}

fn promotion_register_move(pr: PromotionMove, board: &mut Board) -> Result<(), MoveError> {
    let p = board.take_piece(pr.from)?;
    let new_p = promote_piece(pr.to_piece, pr.to, p.color());

    board.change_mating_material(new_p.color(), new_p.mating_material_points() as i8 - 3);
    board.place_piece(new_p, pr.to)?;

    Ok(())
}

fn promotion_capture_register_move(pr: PromotionCapture, board: &mut Board) -> Result<(), MoveError> {
    let p = board.take_piece(pr.from)?;
    let cap_p = board.take_piece(pr.to)?;
    let new_p = promote_piece(pr.to_piece, pr.to, p.color());

    board.change_mating_material(cap_p.color(), -(cap_p.mating_material_points() as i8));
    board.change_mating_material(new_p.color(), new_p.mating_material_points() as i8 - 3);
    board.place_piece(new_p, pr.to)?;

    Ok(())
}

fn castle_move_register_move(castle: CastleMove, board: &mut Board) -> Result<(), MoveError> {
    let (king_pos, rook_pos, target_king_pos, target_rook_pos) = match castle.castle_type {
        CastleType::WhiteShort => (Square(4, 0), Square(7, 0), Square(6, 0), Square(5, 0)),
        CastleType::WhiteLong => (Square(4, 0), Square(0, 0), Square(2, 0), Square(3, 0)),
        CastleType::BlackShort => (Square(4, 7), Square(7, 7), Square(6, 7), Square(5, 7)),
        CastleType::BlackLong => (Square(4, 7), Square(0, 7), Square(2, 7), Square(3, 7)),
    };

    let k = board.take_piece(king_pos)?;
    board.set_king_position(target_king_pos, k.color());
    let r = board.take_piece(rook_pos)?;
    board.place_piece(k, target_king_pos)?;
    board.place_piece(r, target_rook_pos)?;

    Ok(())
}

fn promote_piece(
    piece_type: PromotedPieceType,
    position: Square,
    color: Color,
) -> ChessPiece {
    match piece_type {
        PromotedPieceType::Queen => ChessPiece::Queen(Queen { color, position }),
        PromotedPieceType::Knight => ChessPiece::Knight(Knight { color, position }),
        PromotedPieceType::Bishop => ChessPiece::Bishop(Bishop { color, position }),
        PromotedPieceType::Rook => ChessPiece::Rook(Rook { color, position }),
    }
}
