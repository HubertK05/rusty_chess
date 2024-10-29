<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  function generate_series(n: number) {
    return Array.from({ length: n }, (_, i) => i);
  }
</script>

<main>
  {#each generate_series(8).reverse() as row}
    <div class="row">
      {#each generate_series(8) as col}
        <div class="square">
          <div class={(row + col) % 2 == 0 ? "light-square" : "dark-square"}>
            {row + col}
          </div>
        </div>
      {/each}
    </div>
  {/each}
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
