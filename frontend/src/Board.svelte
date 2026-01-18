<script>
  import { createEventDispatcher } from 'svelte';

  export let gameState = null;

  const dispatch = createEventDispatcher();

  $: boardSize = gameState?.board_size || 19;

  // Calculate offset to center the board in the SVG viewBox
  $: offset = (800 - (boardSize - 1) * 40) / 2;

  // Star points (hoshi) for different board sizes
  $: starPoints = getStarPoints(boardSize);

  function getStarPoints(size) {
    if (size === 19) {
      return [[3,3], [3,9], [3,15], [9,3], [9,9], [9,15], [15,3], [15,9], [15,15]];
    } else if (size === 13) {
      return [[3,3], [3,9], [6,6], [9,3], [9,9]];
    } else if (size === 9) {
      return [[2,2], [2,6], [4,4], [6,2], [6,6]];
    }
    return [];
  }

  function handleClick(x, y) {
    dispatch('move', { x, y });
  }
</script>

<div class="board-container">
  <svg viewBox="0 0 800 800" class="board">
    <!-- Board grid -->
    {#each Array(boardSize) as _, i}
      <line
        x1={offset} y1={offset + i * 40}
        x2={offset + (boardSize - 1) * 40} y2={offset + i * 40}
        stroke="#8b7355"
        stroke-width="1"
      />
      <line
        x1={offset + i * 40} y1={offset}
        x2={offset + i * 40} y2={offset + (boardSize - 1) * 40}
        stroke="#8b7355"
        stroke-width="1"
      />
    {/each}

    <!-- Star points (hoshi) -->
    {#each starPoints as [x, y]}
      <circle
        cx={offset + x * 40}
        cy={offset + y * 40}
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
              cx={offset + x * 40}
              cy={offset + y * 40}
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
    {#each Array(boardSize) as _, y}
      {#each Array(boardSize) as _, x}
        <circle
          cx={offset + x * 40}
          cy={offset + y * 40}
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
