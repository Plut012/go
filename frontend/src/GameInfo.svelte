<script>
  import { createEventDispatcher } from 'svelte';

  export let gameState = null;
  export let myColor = null;

  const dispatch = createEventDispatcher();
</script>

<div class="game-info">
  <div class="info-row">
    <span class="label">You are:</span>
    <span class="value" class:highlight={myColor}>{myColor || 'Not assigned'}</span>
  </div>

  {#if gameState}
    <div class="info-row">
      <span class="label">Players:</span>
      <span class="value">
        Black {gameState.players?.black ? '✓' : '◯'} |
        White {gameState.players?.white ? '✓' : '◯'}
      </span>
    </div>

    <div class="info-row">
      <span class="label">Turn:</span>
      <span class="value turn-indicator" class:active={gameState.turn === myColor}>
        {gameState.turn || 'Black'}
        {#if gameState.turn === myColor}
          (Your turn!)
        {/if}
      </span>
    </div>

    <div class="info-row">
      <span class="label">Prisoners:</span>
      <span class="value">
        Black: {gameState.prisoners?.black || 0} |
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
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .info-row {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .label {
    font-size: 0.65rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity: 0.5;
  }

  .value {
    font-size: 0.85rem;
    line-height: 1.3;
  }

  .value.highlight {
    color: #4a9eff;
  }

  .turn-indicator.active {
    color: #4aff9e;
  }

  .buttons {
    margin-top: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
  }

  button {
    padding: 0.6rem 1rem;
    font-size: 0.75rem;
    cursor: pointer;
    border: 1px solid #444;
    background: transparent;
    color: #e0e0e0;
    border-radius: 2px;
    min-height: 36px;
    transition: all 0.15s ease;
  }

  button:hover {
    border-color: #666;
    background: rgba(255, 255, 255, 0.05);
  }
</style>
