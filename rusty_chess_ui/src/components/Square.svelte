<script lang="ts">
  import {
    board,
    clicked,
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

  function getMovesBySquares(from: number, to: number) {
    const movesToPlay = legalMoves.moves.filter((move) => {
      return (
        move.from[1] * 8 + move.from[0] === from &&
        move.to[1] * 8 + move.to[0] === to
      );
    });

    console.assert(
      movesToPlay.length <= 1 || movesToPlay.length === 4,
      `Expected at most one move to play, got ${movesToPlay}`
    );

    return movesToPlay;
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
    movesToPlay = getMovesBySquares(fromSquareId, toSquareId);

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

  async function handleMoveByClick(from: number, to: number) {
    console.log(`Moved from ${from} to ${to}`);
    await handleMoveByClickInner(from, to);
    legalMoves.moves = [];
  }

  async function handleMoveByClickInner(from: number, to: number) {
    const movesToPlay = getMovesBySquares(from, to);

    if (from === to) return;
    if (movesToPlay.length === 0) return;
    if (movesToPlay.length === 4) {
      console.assert(
        turnState.turn === "black" || turnState.turn == "white",
        "Expected a player's turn during promotion, not bot"
      );
      promotionState.promotionData = movesToPlay;

      return;
    }

    await playMove(movesToPlay[0]);
  }

  async function playMove(move: ChessMove) {
    board.board[move.to[1]][move.to[0]] = JSON.parse(
      JSON.stringify(board.board[move.from[1]][move.to[0]])
    );
    board.board[move.from[1]][move.from[0]] = [];
    await playMoveManually(move);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class={`
    w-10 h-10 sm:w-16 sm:h-16 overflow-hidden ${getSquareStyle()}
    `}
  use:dndzone={{
    items,
    dropTargetStyle: {},
    dragDisabled:
      !(turnState.turn === "white" || turnState.turn === "black") ||
      promotionState.isPromoting,
  }}
  onconsider={(e) => {
    clicked.clicked = { state: "dragged" };
    handleConsider(e.detail);
  }}
  onfinalize={(e) => {
    clicked.clicked = { state: "idle" };
    handleFinalize(e.detail);
  }}
  onclick={async () => {
    if (
      turnState.turn === "blackBot" ||
      turnState.turn === "whiteBot" ||
      promotionState.isPromoting
    )
      return;
    if (clicked.clicked.state === "idle") {
      clicked.clicked = { state: "clicked", squareId: squareId };
      legalMoves.moves = (await getLegalMoves()).filter(
        (move) => move.from[0] === col && move.from[1] === row
      );
    } else if (
      clicked.clicked.state === "clicked" &&
      getMovesBySquares(clicked.clicked.squareId, squareId).length !== 0
    ) {
      handleMoveByClick(clicked.clicked.squareId, squareId);
      clicked.clicked = { state: "idle" };
    } else if (
      clicked.clicked.state === "clicked" &&
      clicked.clicked.squareId !== squareId
    ) {
      clicked.clicked = { state: "clicked", squareId: squareId };
      legalMoves.moves = (await getLegalMoves()).filter(
        (move) => move.from[0] === col && move.from[1] === row
      );
    } else {
      legalMoves.moves = [];
      clicked.clicked = { state: "idle" };
    }
  }}
>
  {#each board.board[row][col] as piece (piece.id)}
    <img
      src={`${pieceToString(piece.piece)}.svg`}
      alt="A chess piece"
      class="w-full h-full"
    />
  {/each}
</div>
