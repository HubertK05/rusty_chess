<script lang="ts">
  import { board, pieceFromString, pieceToString } from "$lib/shared.svelte";
  import { dndzone } from "svelte-dnd-action";

  let { row, col }: { row: number; col: number } = $props();

  let items: DraggableChessPiece[] = $derived(board[row][col]);

  function handleConsider(e: {
    detail: {
      items: DraggableChessPiece[];
    };
  }) {
    board[row][col] = e.detail.items;
  }

  function handleFinalize(e: {
    detail: {
      items: DraggableChessPiece[];
    };
  }) {
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
  }
</script>

<div
  class={`
    w-16 h-16 overflow-hidden
    ${(row + col) % 2 === 0 ? "bg-yellow-300" : "bg-orange-800"}
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
