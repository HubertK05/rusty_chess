<script lang="ts">
  import {
    board,
    getLegalMoves,
    legalMoves,
    pieceToString,
    playMoveManually,
    promotionState,
    turnState,
  } from "$lib/shared.svelte";
  import { dndzone, TRIGGERS, type DndEvent } from "svelte-dnd-action";

  let { row, col }: { row: number; col: number } = $props();

  let items: DraggableChessPiece[] = $derived(board.board[row][col]);
  const squareId: number = $derived(row * 8 + col);

  function getSquareStyle() {
    if (
      legalMoves.moves.find((move) => move.to[0] === col && move.to[1] == row)
    ) {
      return "bg-red-500";
    }
    return (row + col) % 2 === 0 ? "bg-yellow-300" : "bg-orange-800";
  }

  async function handleConsider(e: DndEvent<DraggableChessPiece>) {
    board.board[row][col] = e.items;
    if (e.info.trigger === TRIGGERS.DRAG_STARTED) {
      legalMoves.moves = (await getLegalMoves()).filter(
        (move) => move.from[0] === col && move.from[1] === row
      );
    }
  }

  async function handleFinalize(e: DndEvent<DraggableChessPiece>) {
    await handleFinalizeInner(e);
    legalMoves.moves = [];
  }

  async function handleFinalizeInner(e: DndEvent<DraggableChessPiece>) {
    if (e.info.trigger !== TRIGGERS.DROPPED_INTO_ZONE) {
      return;
    }

    if (squareId === +e.info.id) {
      board.board[row][col] = e.items;
      return;
    }

    const fromSquareId = +e.info.id;
    const toSquareId = squareId;

    let movesToPlay: ChessMove[] = [];
    movesToPlay = legalMoves.moves.filter((move) => {
      return (
        move.from[1] * 8 + move.from[0] === fromSquareId &&
        move.to[1] * 8 + move.to[0] === toSquareId
      );
    });
    console.assert(
      movesToPlay.length <= 1 || movesToPlay.length === 4,
      `Expected at most one move to play, got ${movesToPlay}`
    );

    if (movesToPlay.length === 0 || movesToPlay.length === 4) {
      e.items.forEach((item) => {
        board.board[(item.id / 8) >> 0][item.id % 8 >> 0] = [item];
      });
      board.board[row][col] = e.items.filter((item) => item.id === squareId);

      if (movesToPlay.length === 4) {
        console.assert(
          turnState.turn === "black" || turnState.turn == "white",
          "Expected a player's turn during promotion, not bot"
        );
        promotionState.promotionData = movesToPlay;
      }

      return;
    }

    const moveToPlay = movesToPlay[0];
    if (board.board[row][col].length >= 2) {
      board.board[row][col] = e.items.filter(
        (piece) => piece.id === +e.info.id
      );
    } else {
      board.board[row][col] = e.items;
    }
    await playMoveManually(moveToPlay);
  }
</script>

<div
  class={`
    w-10 h-10 sm:w-16 sm:h-16 overflow-hidden ${getSquareStyle()}
    `}
  use:dndzone={{
    items,
    dropTargetStyle: {},
    dragDisabled: !(turnState.turn === "white" || turnState.turn === "black"),
  }}
  onconsider={(e) => {
    handleConsider(e.detail);
  }}
  onfinalize={(e) => {
    handleFinalize(e.detail);
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
