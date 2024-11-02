<script lang="ts">
  type BotState = "on" | "off";
  type Turn = "White" | "Black";

  let reversed = false;
  let botState = "off";
  let turn: Turn = "White";

  let board = [
    ["wR", "wN", "wB", "wQ", "wK", "wB", "wN", "wR"],
    ["wP", "wP", "wP", "wP", "wP", "wP", "wP", "wP"],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", ""],
    ["bP", "bP", "bP", "bP", "bP", "bP", "bP", "bP"],
    ["bR", "bN", "bB", "bQ", "bK", "bB", "bN", "bR"],
  ];

  function generate_series(n: number) {
    return Array.from({ length: n }, (_, i) => i);
  }
</script>

<main class="flex justify-center items-center h-screen">
  <div class="flex">
    <div class="">
      {#each reversed ? generate_series(8) : generate_series(8).reverse() as row}
        <div class="flex flex-row">
          {#each reversed ? generate_series(8).reverse() : generate_series(8) as col}
            <div
              class={`
                  w-16 h-16 flex items-center justify-center
                  ${(row + col) % 2 === 0 ? "bg-yellow-300" : "bg-orange-800"}
                `}
            >
              {#if board[row][col] !== ""}
                <img
                  src={`../src/assets/${board[row][col]}.svg`}
                  alt="A chess piece"
                  class="w-full h-full"
                />
              {/if}
            </div>
          {/each}
        </div>
      {/each}
    </div>

    <div class="grid gap-4 w-64 ml-4">
      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg py-2 px-4 hover:border-gray-400"
        on:click={() => {
          reversed = !reversed;
        }}
      >
        Reverse board
      </button>

      {#if turn as Turn === "White"}
        <div
          class="bg-gray-300 text-black rounded-lg flex items-center justify-center py-2"
        >
          White's turn
        </div>
      {:else}
        <div
          class="bg-black text-gray-400 rounded-lg flex items-center justify-center py-2"
        >
          Black's turn
        </div>
      {/if}

      <button
        class="bg-gray-500 border-2 border-gray-700 rounded-lg py-2 px-4 hover:border-gray-400"
        on:click={() => {
          botState = botState === "off" ? "on" : "off";
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
