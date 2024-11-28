import { invoke } from "@tauri-apps/api/core"

export const pieceFromString: (x: string) => ChessPiece = (pieceString) => {
    const pieceTable: Record<string, ChessPiece> = {
        "wR": { piece_type: "Rook", color: "White" },
        "wN": { piece_type: "Knight", color: "White" },
        "wB": { piece_type: "Bishop", color: "White" },
        "wQ": { piece_type: "Queen", color: "White" },
        "wK": { piece_type: "King", color: "White" },
        "wP": { piece_type: "Pawn", color: "White" },
        "bR": { piece_type: "Rook", color: "Black" },
        "bN": { piece_type: "Knight", color: "Black" },
        "bB": { piece_type: "Bishop", color: "Black" },
        "bQ": { piece_type: "Queen", color: "Black" },
        "bK": { piece_type: "King", color: "Black" },
        "bP": { piece_type: "Pawn", color: "Black" },
    }

    return pieceTable[pieceString] ?? undefined
}

export const pieceToString: (x: ChessPiece) => String = (piece) => {
    if (piece.piece_type == "Knight") {
        return `${piece.color[0].toLowerCase()}N`
    } else {
        return `${piece.color[0].toLowerCase()}${piece.piece_type[0].toUpperCase()}`
    }
}


let boardPre = [
    ["wR", "wN", "wB", "wQ", "wK", "wB", "wN", "wR"],
    ["wP", "wP", "wP", "wP", "wP", "wP", "wP", "wP"],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["bP", "bP", "bP", "bP", "bP", "bP", "bP", "bP"],
    ["bR", "bN", "bB", "bQ", "bK", "bB", "bN", "bR"],
];

export let board: Board = $state(
    boardPre.map((x, row) => {
        return x.map((elem, col) => {
            return elem === "" ? [] : [{ id: row * 8 + col, piece: pieceFromString(elem) }]
        })
    }),
)


export class CurrentBotState {
    _state: BotState = $state("off")
    
    get state() {
        return this._state
    }
    
    set state(newState: BotState) {
        this._state = newState
    }
}


export class CurrentPlayerState {
    _turn: CurrentPlayer = $state("white")
    
    get turn() {
        return this._turn;
    }
    
    set turn(newTurn: CurrentPlayer) {
        this._turn = newTurn;
    }
}

export let legalMoves: { moves: ChessMove[] } = $state({
    moves: []
})

export async function autoplayMove() {
    try {
        await invoke("autoplay_move");
        advanceTurn();
    } catch (e) {
        // event canceled
    }
}

export async function playMoveManually(moveToPlay: ChessMove) {
    await invoke("play_move_manually", { moveToPlay });
    advanceTurn();
}

export let whiteBotState = new CurrentBotState();
export let blackBotState = new CurrentBotState();
export let turn = new CurrentPlayerState();

function advanceTurn() {
    console.log(turn.turn);
    if (
        (turn.turn as CurrentPlayer) == "white" ||
        (turn.turn as CurrentPlayer) == "whiteBot"
    ) {
        if (blackBotState.state == "on") {
        turn.turn = "blackBot";
        // yes. recursive call. TODO: upgrade state machine such that it doesn't rely on recursion
        autoplayMove();
        } else {
        turn.turn = "black";
        }
    } else if (
        (turn.turn as CurrentPlayer) == "black" ||
        (turn.turn as CurrentPlayer) == "blackBot"
    ) {
        if (whiteBotState.state == "on") {
        turn.turn = "whiteBot";
        autoplayMove();
        } else {
        turn.turn = "white";
        }
    }
}
