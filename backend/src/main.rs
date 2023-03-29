use backend::{
    board_setup::models::{Board, FenNotation}, move_generator::{models::{Moves, Color, PieceType, Square, ChessPiece, MoveRestrictionData}, restrictions::{get_attacked, get_checked, get_pins}},
};
use easybench::bench;

fn main() {
    let board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).unwrap();

    println!("attacked - {}", bench(|| get_attacked(&board, Color::White)));
    println!("checked - {}", bench(|| get_checked(&board, Color::White)));
    println!("pins - {}", bench(|| get_pins(&board, Color::White)));
    let restrictions = MoveRestrictionData::get(&board, Color::White);

    println!("all restrictions - {}", bench(|| MoveRestrictionData::get(&board, Color::White)));

    let pawn = ChessPiece { piece_type: PieceType::Pawn, position: Square(4, 1), color: Color::White};
    println!("pawn moves (x8) - {}", bench(|| pawn.get_moves(&board, &restrictions)));

    let knight = ChessPiece { piece_type: PieceType::Knight, position: Square(1, 0), color: Color::White};
    println!("knight moves (x2) - {}", bench(|| knight.get_moves(&board, &restrictions)));

    let bishop = ChessPiece { piece_type: PieceType::Bishop, position: Square(2, 0), color: Color::White};
    println!("bishop moves (x2) - {}", bench(|| bishop.get_moves(&board, &restrictions)));

    let rook = ChessPiece { piece_type: PieceType::Rook, position: Square(0, 0), color: Color::White};
    println!("rook moves (x2) - {}", bench(|| rook.get_moves(&board, &restrictions)));

    let queen = ChessPiece { piece_type: PieceType::Queen, position: Square(3, 0), color: Color::White};
    println!("queen moves - {}", bench(|| queen.get_moves(&board, &restrictions)));

    let king = ChessPiece { piece_type: PieceType::King, position: Square(4, 0), color: Color::White};
    println!("king moves - {}", bench(|| king.get_moves(&board, &restrictions)));  

    println!("sum - {}", bench(|| Moves::get_all_moves(&board, Color::White)));
}
