import { invoke } from "@tauri-apps/api/core"
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"

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

export class BoardState {
    _board: Board = $state([]);

    constructor() {
        // TODO: Can I keep DRY without using constructors that rely on other methods?
        this.restart()
    }

    restart() {
        const boardPre = [
            ["wR", "wN", "wB", "wQ", "wK", "wB", "wN", "wR"],
            ["wP", "wP", "wP", "wP", "wP", "wP", "wP", "wP"],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["bP", "bP", "bP", "bP", "bP", "bP", "bP", "bP"],
            ["bR", "bN", "bB", "bQ", "bK", "bB", "bN", "bR"],
        ];

        this._board = boardPre.map((x, row) => {
            return x.map((elem, col) => {
                return elem === "" ? [] : [{ id: row * 8 + col, piece: pieceFromString(elem) }]
            })
        })
    }

    get board() {
        return this._board;
    }

    set board(newBoard: Board) {
        this._board = newBoard;
    }
}

export class CurrentBotState {
    _state: BotState = $state("off")
    
    get state() {
        return this._state
    }
    
    set state(newState: BotState) {
        this._state = newState
    }

    toggle() {
        console.log(this._state, "off", this._state === "off");
        if (this._state === "off") {
            this._state = "on"
        } else {
            this._state = "off"
        }
        console.log(this._state)
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

export let board = new BoardState();

export class TurnStateMachine {
    _turn: CurrentPlayer = $state("white");
    _otherBotState: BotState = $state("off");

    setWhite = () => {
        this._turn = "white"
        cancelMove();
    }

    setWhiteBot = async () => {
        this._turn = "whiteBot"
        await autoplayMove();
    }

    setBlack = () => {
        this._turn = "black"
        cancelMove();
    }

    setBlackBot = async () => {
        this._turn = "blackBot"
        await autoplayMove();
    }

    endGame(msg: string) {
        this._turn = { endgameMsg: msg }
        cancelMove();
    }

    restartGame() {
        this.setWhite()
        this._otherBotState = "off"
    }

    get turn() {
        return this._turn
    }

    get whiteBotState() {
        if (this._turn === "white") return "off"
        else if (this._turn === "whiteBot") return "on"
        else if (this._turn === "black" || this._turn === "blackBot") return this._otherBotState
        else return "off"
    }

    get blackBotState() {
        if (this._turn === "black") return "off"
        else if (this._turn === "blackBot") return "on"
        else if (this._turn === "white" || this._turn === "whiteBot") return this._otherBotState
        else return "off"
    }

    // State described as turn/otherBotState
    async toggleWhiteBot() {
        // white/- => whiteBot/-
        // whiteBot/- => white/-
        // black/off => black/on
        // black/on => black/off
        // blackBot/off => blackBot/on
        // blackBot/on => blackBot/off
        const transitions: Record<("white" | "whiteBot" | "black" | "blackBot"), Record<BotState, any[]>> = {
            "white": {
                "off": [this.setWhiteBot, "off"],
                "on": [this.setWhiteBot, "on"],
            },
            "whiteBot": {
                "off": [this.setWhite, "off"],
                "on": [this.setWhite, "on"],
            },
            "black": {
                "off": [this.setBlack, "on"],
                "on": [this.setBlack, "off"],
            },
            "blackBot": {
                "off": [this.setBlackBot, "on"],
                "on": [this.setBlackBot, "off"],
            }
        }

        const transition = transitions[this._turn as ("white" | "whiteBot" | "black" | "blackBot")][this._otherBotState]
        this._otherBotState = transition[1]
        await transition[0]()
    }

    async toggleBlackBot() {
        // black/- => blackBot/-
        // blackBot/- => black/-
        // white/off => white/on
        // white/on => white/off
        // whiteBot/off => whiteBot/on
        // whiteBot/on => whiteBot/off
        const transitions: Record<("white" | "whiteBot" | "black" | "blackBot"), Record<BotState, any[]>> = {
            "white": {
                "off": [this.setWhite, "on"],
                "on": [this.setWhite, "off"],
            },
            "whiteBot": {
                "off": [this.setWhiteBot, "on"],
                "on": [this.setWhiteBot, "off"],
            },
            "black": {
                "off": [this.setBlackBot, "off"],
                "on": [this.setBlackBot, "on"],
            },
            "blackBot": {
                "off": [this.setBlack, "off"],
                "on": [this.setBlack, "on"],
            }
        }

        const transition = transitions[this._turn as ("white" | "whiteBot" | "black" | "blackBot")][this._otherBotState]
        this._otherBotState = transition[1]
        await transition[0]()
    }

    async advanceTurn() {
        // white/off => black/off
        // white/on => blackBot/off
        // whiteBot/off => black/on
        // whiteBot/on => blackBot/on
        // black/off => white/off
        // black/on => whiteBot/off
        // blackBot/off => white/on
        // blackBot/on => whiteBot/on
        const transitions: Record<("white" | "whiteBot" | "black" | "blackBot"), Record<BotState, any[]>> = {
            "white": {
                "off": [this.setBlack, "off"],
                "on": [this.setBlackBot, "off"],
            },
            "whiteBot": {
                "off": [this.setBlack, "on"],
                "on": [this.setBlackBot, "on"],
            },
            "black": {
                "off": [this.setWhite, "off"],
                "on": [this.setWhiteBot, "off"],
            },
            "blackBot": {
                "off": [this.setWhite, "on"],
                "on": [this.setWhiteBot, "on"],
            }
        }

        const transition = transitions[this._turn as ("white" | "whiteBot" | "black" | "blackBot")][this._otherBotState]
        this._otherBotState = transition[1]
        await transition[0]()
    }
}

export let turnState = new TurnStateMachine();
const appWebview = getCurrentWebviewWindow();

export function cancelMove() {
    appWebview.emit("cancel-move");
}

export async function autoplayMove() {
    try {
        await invoke("autoplay_move");
        await turnState.advanceTurn();
    } catch (e) {
        // event canceled
    }
}

export async function playMoveManually(moveToPlay: ChessMove) {
    await invoke("play_move_manually", { moveToPlay });
    await turnState.advanceTurn();
}

export async function restartGameState() {
    await invoke("restart_game");
}
