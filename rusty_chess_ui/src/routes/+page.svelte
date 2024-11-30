<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Square from "../components/Square.svelte";
  import {
    autoplayMove,
    blackBotState,
    board,
    CurrentPlayerState,
    legalMoves,
    toggleBot,
    turn,
    whiteBotState,
  } from "../lib/shared.svelte";
  import { dndzone } from "svelte-dnd-action";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  let reversed = $state(false);

  function generate_series(n: number) {
    return Array.from({ length: n }, (_, i) => i);
  }

  listen<BackendBoard>("update-board", (event) => {
    const newBoard: Board = event.payload.board.map((row, rowNumber) =>
      row.map((piece, colNumber) =>
        piece
          ? [
              {
                id: rowNumber * 8 + colNumber,
                piece,
              },
            ]
          : []
      )
    );

    newBoard.forEach((row, rowNumber) => {
      board[rowNumber] = row;
    });
  });

  listen<string>("end-game", (event) => {
    turn.turn = { endgameMsg: event.payload };
  });
</script>

<main class="flex justify-center items-center h-screen">
  <div class="flex">
    <div>
      {#each reversed ? generate_series(8) : generate_series(8).reverse() as row}
        <div class="flex flex-row">
          {#each reversed ? generate_series(8).reverse() : generate_series(8) as col}
            <Square {row} {col} />
          {/each}
        </div>
      {/each}
    </div>

    <div class="grid gap-4 w-64 ml-4">
      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg py-2 px-4 hover:border-gray-400 row-span-2"
        onclick={() => {
          reversed = !reversed;
        }}
      >
        Reverse board
      </button>

      {#if (turn.turn as CurrentPlayer) === "white" || (turn.turn as CurrentPlayer) === "whiteBot"}
        <div
          class="bg-gray-300 text-black rounded-lg flex items-center justify-center py-2 row-span-2"
        >
          White's turn
        </div>
      {:else if (turn.turn as CurrentPlayer) === "black" || (turn.turn as CurrentPlayer) === "blackBot"}
        <div
          class="bg-black text-gray-400 rounded-lg flex items-center justify-center py-2 row-span-2"
        >
          Black's turn
        </div>
      {:else}
        <div
          class="text-gray-400 rounded-lg flex items-center justify-center py-2 row-span-2"
        >
          {(turn.turn as { endgameMsg: string }).endgameMsg}
        </div>
      {/if}

      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg py-2 px-4 hover:border-gray-400"
        onclick={async () => {
          toggleBot("White");
        }}
      >
        White's bot ({whiteBotState.state})
      </button>

      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg py-2 px-4 hover:border-gray-400"
        onclick={async () => {
          toggleBot("Black");
        }}
      >
        Black's bot ({blackBotState.state})
      </button>
    </div>
    Turn: {turn.turn}
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #222;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
</style>
