use backend::{Board, Moves, Color, Attacked, CheckSquares, PinSquares};

fn main() {
    let board = Board::try_from(
        [
        ["r", ".", "b", ".", "k", "b", ".", "r"],
        ["p", "p", "p", ".", "q", "p", "p", "p"],
        ["n", ".", ".", "p", ".", "n", ".", "."],
        [".", ".", ".", ".", "P", ".", ".", "B"],
        ["Q", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", "N", ".", ".", "N", ".", "."],
        ["P", "P", "P", "P", ".", "P", "P", "P"],
        ["R", ".", "B", ".", "K", ".", ".", "R"],
        ]
    ).unwrap();
    println!("{board}");
    println!("White moves: {}", Moves::get_all_moves(&board, Color::White));
    println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
    println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
    println!("Black attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::Black));
    println!("Black can block white's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::White));
    println!("White can block black's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::Black));
    println!("White pins black's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::White));
    println!("Black pins white's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::Black));

    let board = Board::try_from(
        [
        ["q", ".", ".", ".", "n", "r", ".", "k"],
        [".", ".", ".", ".", "b", "p", "p", "p"],
        [".", ".", ".", "p", ".", ".", ".", "."],
        ["r", "B", ".", ".", ".", "P", "P", "."],
        [".", ".", "n", "B", "P", ".", ".", "P"],
        ["N", "p", ".", ".", "Q", ".", ".", "."],
        [".", "P", ".", ".", ".", ".", ".", "."],
        ["K", ".", ".", "R", ".", ".", ".", "R"],
        ]
    ).unwrap();
    println!("{board}");
    println!("White moves: {}", Moves::get_all_moves(&board, Color::White));
    println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
    println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
    println!("Black attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::Black));
    println!("Black can block white's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::White));
    println!("White can block black's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::Black));
    println!("White pins black's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::White));
    println!("Black pins white's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::Black));

    let board = Board::try_from(
        [
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        ["k", "p", "P", ".", ".", ".", ".", "R"],
        [".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", ".", ".", ".", "K", ".", "."],
        [".", ".", ".", ".", ".", ".", ".", "."],
        ]
    ).unwrap();
    println!("{board}");
    println!("White moves: {}", Moves::get_all_moves(&board, Color::White));
    println!("Black moves: {}", Moves::get_all_moves(&board, Color::Black));
    println!("White attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::White));
    println!("Black attacks these squares: {}", Attacked::get_attacked_squares(&board, Color::Black));
    println!("Black can block white's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::White));
    println!("White can block black's check on these squares: {}", CheckSquares::get_all_checked_squares(&board, Color::Black));
    println!("White pins black's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::White));
    println!("Black pins white's pieces to these squares: {}", PinSquares::get_all_pin_squares(&board, Color::Black));
}
