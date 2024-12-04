<script lang="ts">
  import {
    autoplayMove,
    board,
    legalMoves,
    pieceFromString,
    pieceToString,
    playMoveManually,
    turnState,
  } from "$lib/shared.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { dndzone } from "svelte-dnd-action";

  let { row, col }: { row: number; col: number } = $props();

  let items: DraggableChessPiece[] = $derived(board.board[row][col]);

  function getSquareStyle() {
    if (
      legalMoves.moves.find((move) => move.to[0] === col && move.to[1] == row)
    ) {
      return "bg-red-500";
    }
    return (row + col) % 2 === 0 ? "bg-yellow-300" : "bg-orange-800";
  }

  async function getLegalMoves(): Promise<ChessMove[]> {
    return await invoke("get_legal_moves");
  }

  async function handleConsider(e: {
    detail: {
      items: DraggableChessPiece[];
    };
  }) {
    const draggedItems = e.detail.items.filter((x) => x.isDndShadowItem);
    if (draggedItems.find((x) => x.id === row * 8 + col)) {
      legalMoves.moves = (await getLegalMoves()).filter(
        (move) => move.from[0] === col && move.from[1] === row
      );
    }

    board.board[row][col] = e.detail.items;
  }

  async function handleFinalize(e: {
    detail: {
      items: DraggableChessPiece[];
    };
  }) {
    const pieceWasMoved = $state
      .snapshot(board.board[row][col])
      .filter((x) => x.isDndShadowItem && x.id !== row * 8 + col);

    let movesToPlay: ChessMove[] = [];
    if (pieceWasMoved.length !== 0) {
      const from = pieceWasMoved[0].id;
      const to = row * 8 + col;
      movesToPlay = legalMoves.moves.filter(
        (move) =>
          move.from[1] * 8 + move.from[0] === from &&
          move.to[1] * 8 + move.to[0] === to
      );
      console.assert(
        movesToPlay.length <= 1 || movesToPlay.length === 4,
        `Expected at most one move to play, got ${movesToPlay}`
      );
    }

    legalMoves.moves = [];

    if (movesToPlay.length === 0 || movesToPlay.length === 4) {
      if (e.detail.items.length === 0) return;

      e.detail.items.forEach((item) => {
        board.board[(item.id / 8) >> 0][item.id % 8 >> 0] = [item];
      });
      const filteredItems = e.detail.items.filter(
        (item) => item.id === row * 8 + col
      );
      board.board[row][col] = filteredItems;

      if (movesToPlay.length === 4) {
        console.assert(
          turnState.turn === "black" || turnState.turn == "white",
          "Expected a player's turn during promotion, not bot"
        );
        // const currentTurn: Color =
        //   turnState.turn === "white" ? "White" : "Black";
        // turnState.turn = {
        //   promotionOptions: movesToPlay,
        //   color: currentTurn,
        // };
      }

      return;
    }

    const moveToPlay = movesToPlay[0];
    if (board.board[row][col].length >= 2) {
      e.detail.items.forEach((incoming) =>
        board.board[row][col].find((current) => current.id === incoming.id)
      );
      board.board[row][col] = e.detail.items.filter(
        (incoming) =>
          board.board[row][col].find((current) => current.id === incoming.id)!
            .isDndShadowItem === true
      );
    } else {
      board.board[row][col] = e.detail.items;
    }
    await playMoveManually(moveToPlay);
  }
</script>

<div
  class={`
    w-16 h-16 overflow-hidden ${getSquareStyle()}
    `}
  use:dndzone={{
    items,
    dropTargetStyle: {},
    dragDisabled: !(turnState.turn === "white" || turnState.turn === "black"),
  }}
  onconsider={(e) => {
    handleConsider(e);
  }}
  onfinalize={(e) => {
    handleFinalize(e);
  }}
>
  {#each board.board[row][col] as piece (piece.id)}
    <img
      src={`../src/assets/${pieceToString(piece.piece)}.svg`}
      alt="A chess piece"
      class="w-full h-full"
    />
  {/each}
</div>
