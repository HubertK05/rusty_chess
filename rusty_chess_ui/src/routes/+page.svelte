<script lang="ts">
  import Square from "../components/Square.svelte";
  import {
    board,
    promotePawn,
    promotionState,
    restartGameState,
    turnState,
  } from "../lib/shared.svelte";
  import { listen } from "@tauri-apps/api/event";

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

    board.board = newBoard;
  });

  listen<string>("end-game", (event) => {
    turnState.endGame(event.payload);
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

    <div class="grid gap-4 w-64 ml-4 grid-rows-[1fr_1fr_2fr_1fr_1fr]">
      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg px-4 hover:border-gray-400"
        onclick={() => {
          reversed = !reversed;
        }}
      >
        Reverse board
      </button>

      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg px-4 hover:border-gray-400"
        onclick={async () => {
          reversed = false;
          turnState.restartGame();
          promotionState.promotionData = null;
          board.restart();
          restartGameState();
        }}
      >
        Restart game
      </button>

      {#if promotionState.isPromoting}
        <div class="rounded-lg flex items-center justify-center">
          <div>
            <button onclick={async () => await promotePawn("Queen")}>
              <img
                src={`../src/assets/${turnState.color === "White" ? "wQ" : "bQ"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
            <button onclick={async () => await promotePawn("Rook")}>
              <img
                src={`../src/assets/${turnState.color === "White" ? "wR" : "bR"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
            <button onclick={async () => await promotePawn("Bishop")}>
              <img
                src={`../src/assets/${turnState.color === "White" ? "wB" : "bB"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
            <button onclick={async () => await promotePawn("Knight")}>
              <img
                src={`../src/assets/${turnState.color === "White" ? "wN" : "bN"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
          </div>
        </div>
      {:else if (turnState.turn as CurrentPlayer) === "white" || (turnState.turn as CurrentPlayer) === "whiteBot"}
        <div
          class="bg-gray-300 text-black rounded-lg flex items-center justify-center"
        >
          White's turn
        </div>
      {:else if (turnState.turn as CurrentPlayer) === "black" || (turnState.turn as CurrentPlayer) === "blackBot"}
        <div
          class="bg-black text-gray-400 rounded-lg flex items-center justify-center"
        >
          Black's turn
        </div>
      {:else if (turnState.turn as { endgameMsg: string }).endgameMsg}
        <div class="text-gray-400 rounded-lg flex items-center justify-center">
          {(turnState.turn as { endgameMsg: string }).endgameMsg}
        </div>
      {/if}

      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg px-4 hover:border-gray-400 disabled:opacity-50"
        onclick={async () => {
          await turnState.toggleWhiteBot();
        }}
        disabled={promotionState.isPromoting}
      >
        White's bot ({turnState.whiteBotState})
      </button>

      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg px-4 hover:border-gray-400 disabled:opacity-50"
        onclick={async () => {
          await turnState.toggleBlackBot();
        }}
        disabled={promotionState.isPromoting}
      >
        Black's bot ({turnState.blackBotState})
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
