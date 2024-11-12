interface GameState {
    board: (ChessPiece | undefined)[][],
    turn: Color,
}

interface ChessPiece {
    piece_type: PieceType,
    color: Color,
}

type PieceType = "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King";
type Color = "White" | "Black";
