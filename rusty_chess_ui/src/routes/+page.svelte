<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type BotState = "on" | "off";

  let reversed = false;
  let botState = "off";

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

<main>
  <div class="board">
    {#each reversed ? generate_series(8) : generate_series(8).reverse() as row}
      <div class="row">
        {#each reversed ? generate_series(8).reverse() : generate_series(8) as col}
          <div class="square">
            <div class={(row + col) % 2 == 0 ? "dark-square" : "light-square"}>
              {#if board[row][col] !== ""}
                <img
                  src={`../src/assets/${board[row][col]}.svg`}
                  alt="A chess piece"
                />
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/each}
  </div>
  <div class="game-options">
    <button
      on:click={() => {
        reversed = !reversed;
      }}>Reverse board</button
    >
    <button
      on:click={() => {
        botState = botState === "off" ? "on" : "off";
      }}>Toggle bot ({botState})</button
    >
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .row {
    display: flex;
    flex-direction: row;
  }

  .light-square {
    background-color: rgb(245, 235, 155);
    width: 4rem;
    height: 4rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dark-square {
    background-color: rgb(120, 64, 0);
    width: 4rem;
    height: 4rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
