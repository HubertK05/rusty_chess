use crate::{
    board_setup::models::{AvailableCastles, Board},
    move_register::{
        models::{
            Capture, CastleMove, CastleType, EnPassantMove, Move, MoveType, PromotedPieceType,
            PromotionCapture, PromotionMove, ChessMove,
        },
    },
};

use self::{
    models::{
        Attacked, Bishop, CheckedAdd, Color, King, Knight, MoveDir, MoveRestrictionData, Offset,
        Pawn, PieceType, PinDir, Queen, Rook, Square, ChessPiece,
    },
    restrictions::{filter_with_checked, filter_with_pins},
};

pub mod models;
pub mod restrictions;

const WHITE_PAWN_DOUBLE_MOVE_RANK: i8 = 1;
const BLACK_PAWN_DOUBLE_MOVE_RANK: i8 = 6;
const WHITE_PROMOTION_RANK: i8 = 7;
const BLACK_PROMOTION_RANK: i8 = 0;
const MAX_MOVES_IN_A_SERIES: usize = 7;

const WHITE_PAWN_MOVE: MoveDir = MoveDir::Up;
const WHITE_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::UpLeft, MoveDir::UpRight];

const BLACK_PAWN_MOVE: MoveDir = MoveDir::Down;
const BLACK_PAWN_CAPTURES: [MoveDir; 2] = [MoveDir::DownLeft, MoveDir::DownRight];

const PROMOTED_PIECE_TYPES: [PromotedPieceType; 4] = [
    PromotedPieceType::Queen,
    PromotedPieceType::Rook,
    PromotedPieceType::Bishop,
    PromotedPieceType::Knight,
];

const KNIGHT_MOVES: [Offset; 8] = [
    Offset(2, 1),
    Offset(1, 2),
    Offset(-1, 2),
    Offset(-2, 1),
    Offset(-2, -1),
    Offset(-1, -2),
    Offset(1, -2),
    Offset(2, -1),
];

const KING_MOVES: [Offset; 8] = [
    Offset(1, 0),
    Offset(1, 1),
    Offset(0, 1),
    Offset(-1, 1),
    Offset(-1, 0),
    Offset(-1, -1),
    Offset(0, -1),
    Offset(1, -1),
];

const ROOK_MOVES: [MoveDir; 4] = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

const BISHOP_MOVES: [MoveDir; 4] = [
    MoveDir::UpLeft,
    MoveDir::DownLeft,
    MoveDir::UpRight,
    MoveDir::DownRight,
];

const QUEEN_MOVES: [MoveDir; 8] = [
    MoveDir::Up,
    MoveDir::Down,
    MoveDir::Left,
    MoveDir::Right,
    MoveDir::UpLeft,
    MoveDir::DownLeft,
    MoveDir::UpRight,
    MoveDir::DownRight,
];

fn pawn_get_moves(
    pawn: &Pawn,
    board: &Board,
    restriction: &MoveRestrictionData,
) -> Vec<ChessMove> {
    let mut all_moves = get_pawn_moves(board, pawn.position, pawn.color);
    all_moves.extend(get_pawn_captures(board, pawn.position, pawn.color));

    let en_passant = get_en_passant(board, pawn.position, pawn.color);
    if let Some(ep) = &en_passant {
        if restriction.pin_squares.0.get(&ep.from()) != Some(&PinDir::EnPassantBlock) {
            all_moves.extend(en_passant);
        }
    };

    let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
    let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
    all_moves
}

fn rook_get_moves(
    rook: &Rook,
    board: &Board,
    restriction: &MoveRestrictionData,
) -> Vec<ChessMove> {
    let mut all_moves: Vec<ChessMove> = Vec::new();
    for dir in ROOK_MOVES {
        all_moves.extend(get_move_series(
            board,
            rook.position,
            dir,
            MAX_MOVES_IN_A_SERIES as u8,
            ChessPiece::Rook(*rook),
        ));
    }
    let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
    let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
    all_moves
}

fn bishop_get_moves(
    bishop: &Bishop,
    board: &Board,
    restriction: &MoveRestrictionData,
) -> Vec<ChessMove> {
    let mut all_moves: Vec<ChessMove> = Vec::new();
    for dir in BISHOP_MOVES {
        all_moves.extend(get_move_series(
            board,
            bishop.position,
            dir,
            MAX_MOVES_IN_A_SERIES as u8,
            ChessPiece::Bishop(*bishop),
        ));
    }
    let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
    let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
    all_moves
}

fn queen_get_moves(
    queen: &Queen,
    board: &Board,
    restriction: &MoveRestrictionData,
) -> Vec<ChessMove> {
    let mut all_moves: Vec<ChessMove> = Vec::new();
    for dir in QUEEN_MOVES {
        all_moves.extend(get_move_series(
            board,
            queen.position,
            dir,
            MAX_MOVES_IN_A_SERIES as u8,
            ChessPiece::Queen(*queen),
        ));
    }
    let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
    let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
    all_moves
}

fn knight_get_moves(
    knight: &Knight,
    board: &Board,
    restriction: &MoveRestrictionData,
) -> Vec<ChessMove> {
    let mut all_moves: Vec<ChessMove> = Vec::new();
    for offset in KNIGHT_MOVES {
        all_moves.extend(get_move(board, knight.position, offset, ChessPiece::Knight(*knight)));
    }
    let all_moves = filter_with_pins(all_moves, &restriction.pin_squares);
    let all_moves = filter_with_checked(all_moves, &restriction.check_squares);
    all_moves
}

fn king_get_moves(
    king: &King,
    board: &Board,
    restriction: &MoveRestrictionData,
) -> Vec<ChessMove> {
    let mut all_moves: Vec<ChessMove> = Vec::new();
    for offset in KING_MOVES {
        if let Some(m) = get_move(board, king.position, offset, ChessPiece::King(*king)) {
            if !restriction.attacked.0.contains(&m.to()) {
                all_moves.push(m);
            }
        }
    }
    if restriction.check_squares.checks_amount == 0 {
        all_moves.extend(get_castles(
            board,
            king.position,
            &board.castling,
            &restriction.attacked,
            king.color,
        ));
    }

    all_moves
}

fn get_move_series(
    board: &Board,
    start: Square,
    dir: MoveDir,
    count: u8,
    piece: ChessPiece,
) -> Vec<ChessMove> {
    let translation = Offset::from(dir);
    let mut res: Vec<ChessMove> = Vec::new();
    for i in 1..=count {
        let m = get_move(board, start, translation * i as i8, piece);
        let end = match m {
            Some(ref x) if (*x).move_type() == MoveType::Move => false,
            _ => true,
        };
        res.extend(m);

        if end {
            break;
        }
    }

    res
}

fn get_pawn_moves(board: &Board, start: Square, color: Color) -> Vec<ChessMove> {
    let mut res: Vec<ChessMove> = Vec::new();
    let (translation, double_move_rank, promotion_rank) = match color {
        Color::White => (
            Offset::from(WHITE_PAWN_MOVE),
            WHITE_PAWN_DOUBLE_MOVE_RANK,
            WHITE_PROMOTION_RANK,
        ),
        Color::Black => (
            Offset::from(BLACK_PAWN_MOVE),
            BLACK_PAWN_DOUBLE_MOVE_RANK,
            BLACK_PROMOTION_RANK,
        ),
    };

    let count = if start.1 == double_move_rank { 2 } else { 1 };

    for i in 1..=count {
        let Some(sq) = start.c_add(translation * i as i8) else {
            break
        };

        if board.get_square(sq).is_some() {
            break;
        }

        if sq.1 == promotion_rank {
            for pp in PROMOTED_PIECE_TYPES {
                res.push(ChessMove::PromotionMove(PromotionMove::new(start, sq, pp, color)));
            }
            break;
        } else {
            res.push(ChessMove::Move(Move::new(PieceType::Pawn, start, sq)));
        }
    }

    res
}

fn get_pawn_captures(board: &Board, start: Square, color: Color) -> Vec<ChessMove> {
    let mut res: Vec<ChessMove> = Vec::new();
    let (pawn_captures, promotion_rank) = match color {
        Color::White => (WHITE_PAWN_CAPTURES, WHITE_PROMOTION_RANK),
        Color::Black => (BLACK_PAWN_CAPTURES, BLACK_PROMOTION_RANK),
    };

    for move_dir in pawn_captures {
        let Some(sq) = start.c_add(Offset::from(move_dir)) else {
            continue
        };

        let Some(p) = board.get_square(sq) else {
            continue
        };

        if p.color() != color && sq.1 == promotion_rank {
            for pp in PROMOTED_PIECE_TYPES {
                res.push(ChessMove::PromotionCapture(PromotionCapture::new(start, sq, pp, color)));
            }
        } else if p.color() != color {
            res.push(ChessMove::Capture(Capture::new(PieceType::Pawn, start, sq)));
        }
    }

    res
}

fn get_en_passant(board: &Board, start: Square, color: Color) -> Option<ChessMove> {
    let target_sq = board.en_passant_square?;
    if target_sq.1 - start.1 != 0 || (target_sq.0 - start.0).abs() != 1 {
        return None;
    }
    if board.get_square(target_sq)?.color() != color {
        let res = match color {
            Color::White => Square(target_sq.0, target_sq.1 + 1),
            Color::Black => Square(target_sq.0, target_sq.1 - 1),
        };
        Some(ChessMove::EnPassantMove(EnPassantMove::new(start, res)))
    } else {
        None
    }
}

fn get_move(
    board: &Board,
    start: Square,
    offset: Offset,
    piece: ChessPiece,
) -> Option<ChessMove> {
    let sq = start.c_add(offset)?;
    match board.get_square(sq) {
        Some(p) if p.color() == piece.color() => None,
        Some(_) => Some(ChessMove::Capture(Capture::new(piece.piece_type(), start, sq))),
        None => Some(ChessMove::Move(Move::new(piece.piece_type(), start, sq))),
    }
}

fn get_castles(
    board: &Board,
    pos: Square,
    castles: &AvailableCastles,
    attacked: &Attacked,
    color: Color,
) -> Vec<ChessMove> {
    let mut res: Vec<ChessMove> = Vec::new();
    let Attacked(set) = &attacked;
    if color == Color::White
        && castles.white_short
        && [pos + Offset(1, 0), pos + Offset(2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
    {
        res.push(ChessMove::CastleMove(CastleMove::new(CastleType::WhiteShort)));
    }
    if color == Color::White
        && castles.white_long
        && [pos + Offset(-1, 0), pos + Offset(-2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
        && board.get_square(pos + Offset(-3, 0)).is_none()
    {
        res.push(ChessMove::CastleMove(CastleMove::new(CastleType::WhiteLong)));
    }
    if color == Color::Black
        && castles.black_short
        && [pos + Offset(1, 0), pos + Offset(2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
    {
        res.push(ChessMove::CastleMove(CastleMove::new(CastleType::BlackShort)));
    }
    if color == Color::Black
        && castles.black_long
        && [pos + Offset(-1, 0), pos + Offset(-2, 0)]
            .into_iter()
            .all(|x| !set.contains(&x) && board.get_square(x).is_none())
        && board.get_square(pos + Offset(-3, 0)).is_none()
    {
        res.push(ChessMove::CastleMove(CastleMove::new(CastleType::BlackLong)));
    }
    res
}
