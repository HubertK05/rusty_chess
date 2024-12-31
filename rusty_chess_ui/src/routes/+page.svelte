<script lang="ts">
  import { onMount } from "svelte";
  import Square from "../components/Square.svelte";
  import {
    board,
    clicked,
    clickOutside,
    getAppSettings,
    legalMoves,
    promotePawn,
    promotionState,
    restartGameState,
    turnState,
    updateAppSettings,
  } from "../lib/shared.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { Button, DarkMode, Label, Modal, Range } from "flowbite-svelte";

  let reversed = $state(false);
  let settings: AppSettings | null = $state(null);

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

<header class="flex justify-end absolute w-full">
  <DarkMode />
  <button
    type="button"
    class="text-white rounded-lg px-2 py-2 hover:bg-gray-100 dark:hover:bg-gray-700"
    color="none"
    aria-label="Settings"
    onclick={async () => {
      settings = await getAppSettings();
    }}
    ><svg
      class="w-6 h-6 text-gray-800 dark:text-white"
      aria-hidden="true"
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      fill="none"
      viewBox="0 0 24 24"
    >
      <path
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="1"
        d="M21 13v-2a1 1 0 0 0-1-1h-.757l-.707-1.707.535-.536a1 1 0 0 0 0-1.414l-1.414-1.414a1 1 0 0 0-1.414 0l-.536.535L14 4.757V4a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v.757l-1.707.707-.536-.535a1 1 0 0 0-1.414 0L4.929 6.343a1 1 0 0 0 0 1.414l.536.536L4.757 10H4a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h.757l.707 1.707-.535.536a1 1 0 0 0 0 1.414l1.414 1.414a1 1 0 0 0 1.414 0l.536-.535 1.707.707V20a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-.757l1.707-.708.536.536a1 1 0 0 0 1.414 0l1.414-1.414a1 1 0 0 0 0-1.414l-.535-.536.707-1.707H20a1 1 0 0 0 1-1Z"
      />
      <path
        stroke="currentColor"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="1"
        d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"
      />
    </svg>
  </button>
</header>

{#if settings}
  <Modal
    open={settings !== null}
    outsideclose
    on:close={() => {
      settings = null;
    }}
  >
    <div class="mb-6">
      <Label for="default-input" class="block mb-2">Search depth</Label>
      <Range
        id="search-depth"
        min="1"
        max="10"
        bind:value={settings.search_depth}
      />
      <p>Value: {settings.search_depth}</p>
    </div>
    <div class="mb-6">
      <Label for="small-input" class="block mb-2"
        >Positional value factor (lower = more materialistic)</Label
      >
      <Range
        id="pos-val-factor"
        min="0"
        max="100"
        bind:value={settings.positional_value_factor}
      />
      <p>Value: {settings.positional_value_factor}</p>
    </div>
    <Button
      onclick={async () => {
        console.assert(
          settings !== null,
          "Expected present settings when saving them"
        );
        if (settings) await updateAppSettings(settings);
        settings = null;
      }}>Save</Button
    >
  </Modal>
{/if}

<main
  class="bg-gray-300 dark:bg-[#222] flex justify-center items-center h-screen"
>
  <div class="flex flex-col lg:flex-row">
    <div
      class="shadow-xl dark:shadow-none mb-4 lg:mb-0 lg:mr-4"
      use:clickOutside
      onoutclick={() => {
        clicked.clicked = { state: "idle" };
        console.log("Clicked outside");
      }}
    >
      {#each reversed ? generate_series(8) : generate_series(8).reverse() as row}
        <div class="flex flex-row">
          {#each reversed ? generate_series(8).reverse() : generate_series(8) as col}
            <Square {row} {col} />
          {/each}
        </div>
      {/each}
    </div>

    <div
      class="grid gap-2 sm:gap-4 w-full lg:w-64 grid-rows-[1fr_1fr_2fr_1fr_1fr] sm:grid-rows-[1fr_1fr_2fr_1fr_1fr]"
    >
      <Button
        color="alternative"
        onclick={() => {
          reversed = !reversed;
        }}
      >
        Reverse board
      </Button>

      <Button
        color="alternative"
        onclick={async () => {
          reversed = false;
          turnState.restartGame();
          promotionState.promotionData = null;
          board.restart();
          legalMoves.moves = [];
          clicked.clicked = { state: "idle" };
          restartGameState();
        }}
      >
        Restart game
      </Button>

      {#if promotionState.isPromoting}
        <div class="rounded-lg flex items-center justify-center">
          <div>
            <button onclick={async () => await promotePawn("Queen")}>
              <img
                src={`${turnState.color === "White" ? "wQ" : "bQ"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
            <button onclick={async () => await promotePawn("Rook")}>
              <img
                src={`${turnState.color === "White" ? "wR" : "bR"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
            <button onclick={async () => await promotePawn("Bishop")}>
              <img
                src={`${turnState.color === "White" ? "wB" : "bB"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
            <button onclick={async () => await promotePawn("Knight")}>
              <img
                src={`${turnState.color === "White" ? "wN" : "bN"}.svg`}
                alt="A chess piece"
                class="w-full h-full"
              />
            </button>
          </div>
        </div>
      {:else if (turnState.turn as CurrentPlayer) === "white" || (turnState.turn as CurrentPlayer) === "whiteBot"}
        <div
          class="bg-white text-black rounded-lg flex items-center justify-center"
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

      <Button
        color="alternative"
        onclick={async () => {
          await turnState.toggleWhiteBot();
        }}
        disabled={promotionState.isPromoting}
      >
        White's bot ({turnState.whiteBotState})
      </Button>

      <Button
        color="alternative"
        onclick={async () => {
          await turnState.toggleBlackBot();
        }}
        disabled={promotionState.isPromoting}
      >
        Black's bot ({turnState.blackBotState})
      </Button>
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
