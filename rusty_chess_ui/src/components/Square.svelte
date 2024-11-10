<script lang="ts">
  import { board } from "$lib/shared.svelte";
  import { dndzone } from "svelte-dnd-action";

  let { row, col }: { row: number; col: number } = $props();

  let items: { id: number; content: string }[] = $derived(board[row][col]);

  function handleConsider(e: {
    detail: {
      items: { id: number; content: string; isDndShadowItem?: boolean }[];
    };
  }) {
    board[row][col] = e.detail.items;
  }

  function handleFinalize(e: {
    detail: {
      items: { id: number; content: string; isDndShadowItem?: boolean }[];
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
    w-16 h-16
    ${(row + col) % 2 === 0 ? "bg-yellow-300" : "bg-orange-800"}
    `}
  use:dndzone={{ items }}
  onconsider={(e) => {
    handleConsider(e);
  }}
  onfinalize={(e) => {
    handleFinalize(e);
  }}
>
  {#each board[row][col] as piece (piece.id)}
    <img
      src={`../src/assets/${piece.content}.svg`}
      alt="A chess piece"
      class="w-full h-full"
    />
  {/each}
</div>
