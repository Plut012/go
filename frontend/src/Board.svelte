<script>
  import { createEventDispatcher } from 'svelte';

  export let gameState = null;

  const dispatch = createEventDispatcher();
  const BOARD_SIZE = 19;

  function handleClick(x, y) {
    dispatch('move', { x, y });
  }
</script>

<div class="board-container">
  <svg viewBox="0 0 800 800" class="board">
    <!-- Board grid -->
    {#each Array(BOARD_SIZE) as _, i}
      <line
        x1={40} y1={40 + i * 40}
        x2={760} y2={40 + i * 40}
        stroke="#8b7355"
        stroke-width="1"
      />
      <line
        x1={40 + i * 40} y1={40}
        x2={40 + i * 40} y2={760}
        stroke="#8b7355"
        stroke-width="1"
      />
    {/each}

    <!-- Star points (hoshi) -->
    {#each [[3,3], [3,9], [3,15], [9,3], [9,9], [9,15], [15,3], [15,9], [15,15]] as [x, y]}
      <circle
        cx={40 + x * 40}
        cy={40 + y * 40}
        r="4"
        fill="#8b7355"
      />
    {/each}

    <!-- Stones -->
    {#if gameState && gameState.board}
      {#each gameState.board as row, y}
        {#each row as stone, x}
          {#if stone}
            <circle
              cx={40 + x * 40}
              cy={40 + y * 40}
              r="18"
              fill={stone === 'black' ? '#000' : '#fff'}
              stroke={stone === 'white' ? '#000' : 'none'}
              stroke-width="1"
              class="stone"
            />
          {/if}
        {/each}
      {/each}
    {/if}

    <!-- Clickable intersections -->
    {#each Array(BOARD_SIZE) as _, y}
      {#each Array(BOARD_SIZE) as _, x}
        <circle
          cx={40 + x * 40}
          cy={40 + y * 40}
          r="18"
          fill="transparent"
          class="intersection"
          on:click={() => handleClick(x, y)}
        />
      {/each}
    {/each}
  </svg>
</div>

<style>
  .board-container {
    width: 100%;
    aspect-ratio: 1;
  }

  .board {
    width: 100%;
    height: 100%;
    background: #dcb; /* goban color */
    border-radius: 4px;
  }

  .intersection {
    cursor: pointer;
  }

  .intersection:hover {
    fill: rgba(0, 0, 0, 0.1);
  }

  .stone {
    pointer-events: none;
  }
</style>
