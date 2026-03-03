<script>
  export let ownership = null;
  export let boardSize = 19;
  export let offset = 0;
  export let cellSize = 40;

  // Convert ownership value to color overlay
  function getOverlayColor(value) {
    if (!value || Math.abs(value) < 0.1) return 'transparent';

    // Subtle heat map: -1 = black territory, +1 = white territory
    const intensity = Math.abs(value) * 0.25; // Max 25% opacity
    const color = value < 0 ? '0,0,0' : '255,255,255';
    return `rgba(${color},${intensity})`;
  }
</script>

{#if ownership && ownership.length > 0}
  <g class="ownership-overlay">
    {#each ownership as row, y}
      {#each row as value, x}
        {@const color = getOverlayColor(value)}
        {#if color !== 'transparent'}
          <rect
            x={offset + x * cellSize - cellSize/2}
            y={offset + y * cellSize - cellSize/2}
            width={cellSize}
            height={cellSize}
            fill={color}
            class="ownership-cell"
          />
        {/if}
      {/each}
    {/each}
  </g>
{/if}

<style>
  .ownership-cell {
    pointer-events: none;
  }
</style>
