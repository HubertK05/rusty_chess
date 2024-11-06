<script lang="ts">
  import { board } from "$lib/shared.svelte";
  import { dndzone } from "svelte-dnd-action";

  let { row, col }: { row: number; col: number } = $props();

  let items: { id: number; content: string }[] = $derived(
    board[row][col] ? [{ id: row * 8 + col, content: board[row][col] }] : []
  );

  let nextItem = [];

  function handleConsider(e) {
    nextItem = e.detail.items.filter(
      (x) => items.findIndex((elem) => elem === x) === -1
    );
  }

  function handleFinalize(e) {
    // TODO: fix edge case when dragging destination is the same as source
    board[row][col] = nextItem.length ? nextItem[0].content : "";
  }
</script>

<div
  class={`
        w-16 h-16 flex items-center justify-center
        ${(row + col) % 2 === 0 ? "bg-yellow-300" : "bg-orange-800"}
        `}
  use:dndzone={{ items }}
  onconsider={handleConsider}
  onfinalize={handleFinalize}
>
  {#if items.length !== 0}
    <img
      src={`../src/assets/${items[0].content}.svg`}
      alt="A chess piece"
      class="w-full h-full"
    />
  {:else}
    {row + col}
  {/if}
</div>
