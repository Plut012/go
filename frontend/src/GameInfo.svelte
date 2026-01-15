<script>
  import { createEventDispatcher } from 'svelte';

  export let gameState = null;
  export let myColor = null;

  const dispatch = createEventDispatcher();
</script>

<div class="game-info">
  <div class="info-row">
    <span class="label">You are:</span>
    <span class="value">{myColor || 'Not assigned'}</span>
  </div>

  {#if gameState}
    <div class="info-row">
      <span class="label">Turn:</span>
      <span class="value">{gameState.turn || 'Black'}</span>
    </div>

    <div class="info-row">
      <span class="label">Prisoners:</span>
      <span class="value">
        Black: {gameState.prisoners?.black || 0},
        White: {gameState.prisoners?.white || 0}
      </span>
    </div>
  {/if}

  <div class="buttons">
    <button on:click={() => dispatch('pass')}>Pass</button>
    <button on:click={() => dispatch('reset')}>New Game</button>
  </div>
</div>

<style>
  .game-info {
    margin: 1rem 0;
    padding: 1rem;
    background: #2a2a2a;
    border-radius: 4px;
  }

  .info-row {
    margin: 0.5rem 0;
    display: flex;
    justify-content: center;
    gap: 0.5rem;
  }

  .label {
    font-weight: bold;
  }

  .buttons {
    margin-top: 1rem;
    display: flex;
    justify-content: center;
    gap: 1rem;
  }

  button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    cursor: pointer;
    border: 2px solid #666;
    background: #3a3a3a;
    color: #e0e0e0;
    border-radius: 4px;
    min-height: 44px;
  }

  button:hover {
    background: #4a4a4a;
  }
</style>
