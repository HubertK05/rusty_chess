<script lang="ts">
  import {
    board,
    legalMoves,
    pieceFromString,
    pieceToString,
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

  async function autoplayMove() {
    const res = await invoke("autoplay_move");
    console.log(res);
  }

  async function playMoveManually(moveToPlay: ChessMove) {
    await invoke("play_move_manually", { moveToPlay });
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

    if (pieceWasMoved.length !== 0) {
      //   console.log(pieceWasMoved);
      const from = pieceWasMoved[0].id;
      const to = row * 8 + col;
      //   console.log(from, to);
      const moveToPlay = legalMoves.moves.filter(
        (move) =>
          move.from[1] * 8 + move.from[0] === from &&
          move.to[1] * 8 + move.to[0] === to
      );
      console.log("Moves to play: ", moveToPlay);
      console.assert(
        moveToPlay.length === 1,
        `Expected one move to play, got ${moveToPlay}`
      );
      await playMoveManually(moveToPlay[0]);
    }
    console.log(row * 8 + col, $state.snapshot(board[row][col]), pieceWasMoved);

    legalMoves.moves = [];
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

    if (pieceWasMoved.length !== 0) {
      await autoplayMove();
    }
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
