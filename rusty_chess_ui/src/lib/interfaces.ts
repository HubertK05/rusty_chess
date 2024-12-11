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

type CastleType = "WhiteShort" | "WhiteLong" | "BlackShort" | "BlackLong";
type PromotedPieceType = "Queen" | "Knight" | "Bishop" | "Rook";

type MoveType = { Move: PieceType } | { Capture: PieceType } | "EnPassantMove" | { CastleMove: CastleType } | { PromotionMove: PromotedPieceType } | { PromotionCapture: PromotedPieceType };

interface ChessMove {
    move_type: MoveType,
    from: number[],
    to: number[],
}

type Player = "white" | "whiteBot" | "black" | "blackBot"

type CurrentPlayer =
    Player | { endgameMsg: string }

type BotState = "off" | "on"

interface AppSettings {
    eval_print: boolean,
    pruning: boolean,
    positional_value_factor: number,
    search_depth: number,
}
