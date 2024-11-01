<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type BotState = "on" | "off";
  type Turn = "White" | "Black";

  let reversed = false;
  let botState = "off";
  let turn: Turn = "Black";

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
  <div class="game-container">
    <div class="board">
      {#each reversed ? generate_series(8) : generate_series(8).reverse() as row}
        <div class="row">
          {#each reversed ? generate_series(8).reverse() : generate_series(8) as col}
            <div class="square">
              <div
                class={(row + col) % 2 == 0 ? "dark-square" : "light-square"}
              >
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
        class="game-option"
        on:click={() => {
          reversed = !reversed;
        }}>Reverse board</button
      >
      {#if turn as Turn === "White"}
        <div class="white-turn-board">White's turn</div>
      {:else}
        <div class="black-turn-board">Black's turn</div>
      {/if}
      <button
        class="game-option"
        on:click={() => {
          botState = botState === "off" ? "on" : "off";
        }}>Toggle bot ({botState})</button
      >
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

  main {
    display: flex;
    justify-content: center;
    align-items: center;

    height: 100vh;
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

  .game-container {
    display: flex;
  }

  .game-options {
    display: grid;
    width: 16rem;
  }

  .game-option {
    background-color: #777;
    border: 0.25rem solid;
    border-radius: 0.5rem;
  }

  .game-option:hover {
    background-color: #777;
    border: 0.25rem solid;
    border-color: #aaa;
    border-radius: 0.5rem;
  }

  .white-turn-board {
    border: 0 solid;
    border-radius: 0.5rem;
    background-color: #ddd;

    display: flex;
    align-items: center;
    justify-content: center;
  }

  .black-turn-board {
    border: 0 solid;
    border-radius: 0.5rem;
    background-color: #000;
    color: #aaa;

    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
