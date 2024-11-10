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

export let board: { id: number; content: string; isDndShadowItem?: boolean }[][][] = $state(
    boardPre.map((x, row) => {
        return x.map((elem, col) => {
            return elem === "" ? [] : [{ id: row * 8 + col, content: elem }]
        })
    })
)

console.log($state.snapshot(board))
