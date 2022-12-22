use backend::{Board, Moves, Color};

fn main() {
    let board = Board::try_from(
        [
        ["r", ".", ".", ".", "k", "b", "n", "r"],
        ["p", "p", "p", "b", ".", "p", ".", "p"],
        [".", ".", "n", ".", ".", "q", "p", "."],
        [".", ".", ".", ".", "p", ".", ".", "Q"],
        [".", ".", "B", "p", "P", ".", ".", "."],
        [".", "P", ".", "P", ".", "N", ".", "."],
        [".", "P", "P", ".", ".", "P", "P", "P"],
        ["R", "N", "B", ".", "K", ".", ".", "R"],
        ]
    ).unwrap();
    println!("{board}");
    println!("White moves: {}", Moves::get_all_moves(&board, Color::White));
    println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
}
