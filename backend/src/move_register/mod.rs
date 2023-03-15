use dyn_clone::DynClone;

use crate::{board_setup::models::Board, move_generator::{models::{Square, PieceType, Offset, Color, Queen, Knight, Bishop, Rook}, ChessPiece}};

use self::models::{MoveError, MoveType, Move, Capture, EnPassantMove, PromotionMove, PromotionCapture, CastleType, CastleMove, PromotedPieceType};

pub mod models;

pub trait ChessMove: std::fmt::Debug + std::fmt::Display + DynClone {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError>;
    fn from(&self) -> Square;
    fn to(&self) -> Square;
    fn move_type(&self) -> MoveType;
}

dyn_clone::clone_trait_object!(ChessMove);

impl ChessMove for Move {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let p = board.take_piece(self.from)?;
        if p.piece_type() == PieceType::King {
            board.set_king_position(self.to, p.color())
        }

        board.set_castling(self);

        if p.piece_type() == PieceType::Pawn {
            if (self.to - self.from).1.abs() == 2 {
                board.set_ep_target_square(Some(self.to));
            }
            board.reset_half_move_timer();
        }

        board.place_piece(p, self.to)?;

        Ok(())
    }

    fn from(&self) -> Square {
        self.from
    }

    fn to(&self) -> Square {
        self.to
    }

    fn move_type(&self) -> MoveType {
        MoveType::Move
    }
}

impl ChessMove for Capture {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let p = board.take_piece(self.from)?;
        let cap_p = board.take_piece(self.to)?;

        if p.piece_type() == PieceType::King {
            board.set_king_position(self.to, p.color())
        }

        board.set_castling(self);

        board.change_mating_material(cap_p.color(), -(cap_p.mating_material_points() as i8));
        board.place_piece(p, self.to)?;

        Ok(())
    }

    fn from(&self) -> Square {
        self.from
    }

    fn to(&self) -> Square {
        self.to
    }

    fn move_type(&self) -> MoveType {
        MoveType::Capture
    }
}

impl ChessMove for EnPassantMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let p = board.take_piece(self.from)?;

        let offset = match p.color() {
            Color::White => Offset(0, -1),
            Color::Black => Offset(0, 1),
        };

        let cap_p = board.take_piece(self.to + offset)?;

        board.change_mating_material(cap_p.color(), -3);
        board.place_piece(p, self.to)?;

        Ok(())
    }

    fn from(&self) -> Square {
        self.from
    }

    fn to(&self) -> Square {
        self.to
    }

    fn move_type(&self) -> MoveType {
        MoveType::EnPassantMove
    }
}

impl ChessMove for PromotionMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let p = board.take_piece(self.from)?;
        let new_p = promote_piece(self.to_piece, self.to, p.color());

        board.change_mating_material(new_p.color(), new_p.mating_material_points() as i8 - 3);
        board.place_piece(new_p, self.to)?;

        Ok(())
    }

    fn from(&self) -> Square {
        self.from
    }

    fn to(&self) -> Square {
        self.to
    }

    fn move_type(&self) -> MoveType {
        MoveType::PromotionMove
    }
}

impl ChessMove for PromotionCapture {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        let p = board.take_piece(self.from)?;
        let cap_p = board.take_piece(self.to)?;
        let new_p = promote_piece(self.to_piece, self.to, p.color());

        board.change_mating_material(cap_p.color(), -(cap_p.mating_material_points() as i8));
        board.change_mating_material(new_p.color(), new_p.mating_material_points() as i8 - 3);
        board.place_piece(new_p, self.to)?;

        Ok(())
    }

    fn from(&self) -> Square {
        self.from
    }

    fn to(&self) -> Square {
        self.to
    }

    fn move_type(&self) -> MoveType {
        MoveType::PromotionCapture
    }
}

impl ChessMove for CastleMove {
    fn register_move(&self, board: &mut Board) -> Result<(), MoveError> {
        board.set_castling(self);

        let (king_pos, rook_pos, target_king_pos, target_rook_pos) = match self.castle_type {
            CastleType::WhiteShort => (Square(4, 0), Square(7, 0), Square(6, 0), Square(5, 0)),
            CastleType::WhiteLong => (Square(4, 0), Square(0, 0), Square(2, 0), Square(3, 0)),
            CastleType::BlackShort => (Square(4, 7), Square(7, 7), Square(6, 7), Square(5, 7)),
            CastleType::BlackLong => (Square(4, 7), Square(0, 7), Square(2, 7), Square(3, 7)),
        };

        let k = board.take_piece(king_pos)?;
        let r = board.take_piece(rook_pos)?;
        board.place_piece(k, target_king_pos)?;
        board.place_piece(r, target_rook_pos)?;

        Ok(())
    }

    fn from(&self) -> Square {
        match self.castle_type {
            CastleType::WhiteShort | CastleType::WhiteLong => Square(4, 0),
            CastleType::BlackShort | CastleType::BlackLong => Square(4, 7),
        }
    }

    fn to(&self) -> Square {
        match self.castle_type {
            CastleType::WhiteShort => Square(6, 0),
            CastleType::WhiteLong => Square(2, 0),
            CastleType::BlackShort => Square(6, 7),
            CastleType::BlackLong => Square(2, 7),
        }
    }

    fn move_type(&self) -> MoveType {
        MoveType::CastleMove
    }
}

fn promote_piece(piece_type: PromotedPieceType, position: Square, color: Color) -> Box<dyn ChessPiece> {
    match piece_type {
        PromotedPieceType::Queen => Box::new(Queen { color, position }),
        PromotedPieceType::Knight => Box::new(Knight { color, position }),
        PromotedPieceType::Bishop => Box::new(Bishop { color, position }),
        PromotedPieceType::Rook => Box::new(Rook { color, position }),
    }
}
