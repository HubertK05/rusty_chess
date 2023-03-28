use backend::{
    board_setup::models::{Board, FenNotation}, move_generator::models::{Moves, Color, PieceType, Square}, move_register::models::{Move, ChessMove},
};
use easybench::bench;

fn main() {
    let board = Board::try_from(FenNotation("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into())).unwrap();

    println!("{}", bench(|| Moves::get_all_moves(&board, Color::White)));
    println!("{}", bench(|| board.clone().register_move(ChessMove::Move(Move { piece: PieceType::Pawn, from: Square(4, 1), to: Square(4, 3) }))))
}
