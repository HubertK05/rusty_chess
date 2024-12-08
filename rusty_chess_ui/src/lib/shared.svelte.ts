import { invoke } from "@tauri-apps/api/core"
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow"
import { setDebugMode } from "svelte-dnd-action"

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

class Transition {
    constructor(public newPlayer: () => Promise<void> | void, public newOtherBotState: BotState) {}
}

function isEndgameMsg(currentPlayer: CurrentPlayer): currentPlayer is { endgameMsg: string } {
    return typeof currentPlayer === "object" && "endgameMsg" in currentPlayer;
}

export class TurnStateMachine {
    #turn: CurrentPlayer = $state("white");
    #otherBotState: BotState = $state("off");

    setWhite = () => {
        this.#turn = "white"
        cancelMove();
    }

    setWhiteBot = async () => {
        this.#turn = "whiteBot"
        await autoplayMove();
    }

    setBlack = () => {
        this.#turn = "black"
        cancelMove();
    }

    setBlackBot = async () => {
        this.#turn = "blackBot"
        await autoplayMove();
    }

    endGame(msg: string) {
        this.#turn = { endgameMsg: msg }
        cancelMove();
    }

    restartGame() {
        this.setWhite()
        this.#otherBotState = "off"
    }

    get turn(): CurrentPlayer {
        return this.#turn
    }

    get color(): Color | null {
        if (this.#turn === "white" || this.#turn === "whiteBot" ) return "White"
        else if (this.#turn === "black" || this.#turn === "blackBot") return "Black"
        else return null
    }

    get whiteBotState(): BotState {
        if (this.#turn === "white") return "off"
        else if (this.#turn === "whiteBot") return "on"
        else if (this.#turn === "black" || this.#turn === "blackBot") return this.#otherBotState
        else return "off"
    }

    get blackBotState(): BotState {
        if (this.#turn === "black") return "off"
        else if (this.#turn === "blackBot") return "on"
        else if (this.#turn === "white" || this.#turn === "whiteBot") return this.#otherBotState
        else return "off"
    }

    async transformState(transitions: Record<Player, Record<BotState, Transition>>) {
        if (isEndgameMsg(this.#turn)) return;
        const transition = transitions[this.#turn as Player][this.#otherBotState]
        if (!transition) return;
        this.#otherBotState = transition.newOtherBotState
        await transition.newPlayer()
    }

    async toggleWhiteBot() {
        this.transformState({
            "white": {
                "off": new Transition(this.setWhiteBot, "off"),
                "on": new Transition(this.setWhiteBot, "on"),
            },
            "whiteBot": {
                "off": new Transition(this.setWhite, "off"),
                "on": new Transition(this.setWhite, "on"),
            },
            "black": {
                "off": new Transition(this.setBlack, "on"),
                "on": new Transition(this.setBlack, "off"),
            },
            "blackBot": {
                "off": new Transition(this.setBlackBot, "on"),
                "on": new Transition(this.setBlackBot, "off"),
            }
        })
    }

    async toggleBlackBot() {
        this.transformState({
            "white": {
                "off": new Transition(this.setWhite, "on"),
                "on": new Transition(this.setWhite, "off"),
            },
            "whiteBot": {
                "off": new Transition(this.setWhiteBot, "on"),
                "on": new Transition(this.setWhiteBot, "off"),
            },
            "black": {
                "off": new Transition(this.setBlackBot, "off"),
                "on": new Transition(this.setBlackBot, "on"),
            },
            "blackBot": {
                "off": new Transition(this.setBlack, "off"),
                "on": new Transition(this.setBlack, "on"),
            }
        })
    }

    async advanceTurn() {
        this.transformState({
            "white": {
                "off": new Transition(this.setBlack, "off"),
                "on": new Transition(this.setBlackBot, "off"),
            },
            "whiteBot": {
                "off": new Transition(this.setBlack, "on"),
                "on": new Transition(this.setBlackBot, "on"),
            },
            "black": {
                "off": new Transition(this.setWhite, "off"),
                "on": new Transition(this.setWhiteBot, "off"),
            },
            "blackBot": {
                "off": new Transition(this.setWhite, "on"),
                "on": new Transition(this.setWhiteBot, "on"),
            }
        })
    }
}

export class PromotionState {
    promotionData: ChessMove[] | null = $state(null)

    get isPromoting() {
        return !(this.promotionData === null)
    }
}

export let turnState = new TurnStateMachine();
export let promotionState = new PromotionState();
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

export async function promotePawn(
    option: PromotedPieceType,
) {
    const options = promotionState.promotionData
    console.assert(options !== null, "Expected to be promoting");
    if (options === null) return;

    const playedMove = options.filter(
        (move) =>
            (move.move_type as { PromotionMove: PromotedPieceType })
        .PromotionMove === option ||
        (move.move_type as { PromotionCapture: PromotedPieceType })
        .PromotionCapture === option
    );

    console.assert(
        playedMove.length === 1,
        `Expected one promotion move option to play, got ${playedMove}`
    );

    promotionState.promotionData = null;
    await playMoveManually(playedMove[0]);
}
