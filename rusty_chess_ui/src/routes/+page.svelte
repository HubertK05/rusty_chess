<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Square from "../components/Square.svelte";
  import { board } from "../lib/shared.svelte";
  import { dndzone } from "svelte-dnd-action";
  import { listen } from "@tauri-apps/api/event";

  type BotState = "on" | "off";
  type Turn = "White" | "Black" | { endgameMsg: string };

  let reversed = false;
  let botState = "off";
  let turn: Turn = "White";

  function generate_series(n: number) {
    return Array.from({ length: n }, (_, i) => i);
  }

  async function getLegalMoves() {
    return await invoke("get_legal_moves");
  }

  async function autoplayMove() {
    const res = await invoke("autoplay_move");
    console.log(res);
  }

  listen<BackendBoard>("update-board", (event) => {
    console.log(
      `Update board may be successful: ${event.payload.board}, ${event.payload.turn}`
    );

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
    turn = { endgameMsg: event.payload };
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
        class="bg-gray-500 border-2 border-gray-700 rounded-lg py-2 px-4 hover:border-gray-400"
        onclick={() => {
          reversed = !reversed;
        }}
      >
        Reverse board
      </button>

      {#if (turn as Turn) === "White"}
        <div
          class="bg-gray-300 text-black rounded-lg flex items-center justify-center py-2"
        >
          White's turn
        </div>
      {:else if (turn as Turn) === "Black"}
        <div
          class="bg-black text-gray-400 rounded-lg flex items-center justify-center py-2"
        >
          Black's turn
        </div>
      {:else}
        <div
          class="text-gray-400 rounded-lg flex items-center justify-center py-2"
        >
          {(turn as { endgameMsg: string }).endgameMsg}
        </div>
      {/if}

      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg py-2 px-4 hover:border-gray-400"
        onclick={async () => {
          botState = botState === "off" ? "on" : "off";
          console.log(await autoplayMove());
        }}
      >
        Toggle bot ({botState})
      </button>
    </div>
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
