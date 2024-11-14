interface BackendBoard {
    board: (ChessPiece | undefined)[][],
    turn: Color,
}

interface Board {
    board: DraggableChessPiece[][][],
    turn: Color,
}

interface ChessPiece {
    piece_type: PieceType,
    color: Color,
}

interface DraggableChessPiece {
    id: number,
    piece: ChessPiece,
}

type PieceType = "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King";
type Color = "White" | "Black";
