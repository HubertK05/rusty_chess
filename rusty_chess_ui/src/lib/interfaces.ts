interface BackendBoard {
    board: (ChessPiece | undefined)[][],
    turn: Color,
}

type Board = DraggableChessPiece[][][];

interface ChessPiece {
    piece_type: PieceType,
    color: Color,
}

interface DraggableChessPiece {
    id: number,
    piece: ChessPiece,
    isDndShadowItem?: boolean,
}

type PieceType = "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King";
type Color = "White" | "Black";
