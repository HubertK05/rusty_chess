<script lang="ts">
  import {
    autoplayMove,
    board,
    legalMoves,
    pieceFromString,
    pieceToString,
    playMoveManually,
  } from "$lib/shared.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { dndzone } from "svelte-dnd-action";

  let { row, col }: { row: number; col: number } = $props();

  let items: DraggableChessPiece[] = $derived(board[row][col]);

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

    board[row][col] = e.detail.items;
  }

  async function handleFinalize(e: {
    detail: {
      items: DraggableChessPiece[];
    };
  }) {
    const pieceWasMoved = $state
      .snapshot(board[row][col])
      .filter((x) => x.isDndShadowItem && x.id !== row * 8 + col);

    let moveToPlay: ChessMove | undefined;
    if (pieceWasMoved.length !== 0) {
      const from = pieceWasMoved[0].id;
      const to = row * 8 + col;
      const movesToPlay = legalMoves.moves.filter(
        (move) =>
          move.from[1] * 8 + move.from[0] === from &&
          move.to[1] * 8 + move.to[0] === to
      );
      console.assert(
        movesToPlay.length <= 1,
        `Expected at most one move to play, got ${movesToPlay}`
      );
      if (movesToPlay[0]) moveToPlay = movesToPlay[0];
    }

    legalMoves.moves = [];

    if (!moveToPlay) {
      if (e.detail.items.length === 0) return;

      e.detail.items.forEach((item) => {
        board[(item.id / 8) >> 0][item.id % 8 >> 0] = [item];
      });
      const filteredItems = e.detail.items.filter(
        (item) => item.id === row * 8 + col
      );
      board[row][col] = filteredItems;
      return;
    }

    if (board[row][col].length >= 2) {
      e.detail.items.forEach((incoming) =>
        board[row][col].find((current) => current.id === incoming.id)
      );
      board[row][col] = e.detail.items.filter(
        (incoming) =>
          board[row][col].find((current) => current.id === incoming.id)!
            .isDndShadowItem === true
      );
    } else {
      board[row][col] = e.detail.items;
    }
    await playMoveManually(moveToPlay);

    if (pieceWasMoved.length !== 0) {
      await autoplayMove();
    }
  }

  function advanceTurn() {
    throw new Error("Function not implemented.");
  }
</script>

<div
  class={`
    w-16 h-16 overflow-hidden ${getSquareStyle()}
    `}
  use:dndzone={{ items, dropTargetStyle: {} }}
  onconsider={(e) => {
    handleConsider(e);
  }}
  onfinalize={(e) => {
    handleFinalize(e);
  }}
>
  {#each board[row][col] as piece (piece.id)}
    <img
      src={`../src/assets/${pieceToString(piece.piece)}.svg`}
      alt="A chess piece"
      class="w-full h-full"
    />
  {/each}
</div>

<!--

Events that change the state:
- playing move manually (changes turn to opposite color - human or bot, depending on the bot state),
- autoplaying move (same as playing move manually),
- turning the bot on (which can result in autoplaying move)
- turning the bot off (which can result in canceling move)

-->
