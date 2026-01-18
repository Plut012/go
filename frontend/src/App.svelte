<script>
  import Board from './Board.svelte';
  import GameInfo from './GameInfo.svelte';
  import { createConnection } from './lib/websocket.js';
  import { loadTheme } from './lib/theme.js';
  import { onMount } from 'svelte';

  let connected = false;
  let gameState = null;
  let myColor = null;
  let ws = null;

  onMount(async () => {
    // Load default theme
    await loadTheme('default');

    // Connect to WebSocket
    ws = createConnection({
      onOpen: () => {
        connected = true;
        console.log('Connected to server');
      },
      onMessage: (data) => {
        handleMessage(data);
      },
      onClose: () => {
        connected = false;
        console.log('Disconnected from server');
      }
    });
  });

  function handleMessage(data) {
    if (data.type === 'state') {
      gameState = data;
    } else if (data.type === 'your_color') {
      myColor = data.color;
    } else if (data.type === 'error') {
      console.error('Server error:', data.message);
      // TODO: Show error to user
    }
  }

  function chooseColor(color) {
    if (ws) {
      ws.send(JSON.stringify({ type: 'choose_color', color }));
    }
  }

  function makeMove(x, y) {
    if (ws && connected) {
      ws.send(JSON.stringify({ type: 'move', x, y }));
    }
  }

  function pass() {
    if (ws && connected) {
      ws.send(JSON.stringify({ type: 'pass' }));
    }
  }

  function reset() {
    if (ws && connected) {
      ws.send(JSON.stringify({ type: 'reset' }));
    }
  }
</script>

<main>
  {#if !connected}
    <div class="center-message">
      <div class="status">Connecting...</div>
    </div>
  {:else if !myColor}
    <div class="center-message">
      <div class="color-selection">
        <h2>Choose your color</h2>
        <button on:click={() => chooseColor('black')}>Play as Black</button>
        <button on:click={() => chooseColor('white')}>Play as White</button>
      </div>
    </div>
  {:else}
    <div class="game-layout">
      <aside class="sidebar">
        <GameInfo {gameState} {myColor} on:pass={pass} on:reset={reset} />
      </aside>
      <div class="board-area">
        <Board {gameState} on:move={(e) => makeMove(e.detail.x, e.detail.y)} />
      </div>
    </div>
  {/if}
</main>

<style>
  main {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .center-message {
    text-align: center;
  }

  .status {
    padding: 2rem;
    font-size: 1.2rem;
  }

  .color-selection {
    padding: 2rem;
  }

  .color-selection h2 {
    margin-bottom: 1rem;
    font-size: 1.2rem;
    font-weight: normal;
  }

  .color-selection button {
    margin: 0.5rem;
    padding: 1rem 2rem;
    font-size: 1rem;
    cursor: pointer;
    border: 2px solid #666;
    background: #2a2a2a;
    color: #e0e0e0;
    border-radius: 4px;
    min-width: 160px;
    min-height: 44px;
  }

  .color-selection button:hover {
    background: #3a3a3a;
  }

  .game-layout {
    position: relative;
    min-height: 100vh;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .sidebar {
    position: fixed;
    left: 0;
    top: 0;
    height: 100vh;
    width: 200px;
    padding: 2rem 1.5rem;
    border-right: 1px solid rgba(255, 255, 255, 0.05);
    z-index: 10;
  }

  .board-area {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100vh;
    padding: 2rem;
  }

  .board-area :global(.board-container) {
    width: min(92vh, 1380px);
    height: min(92vh, 1380px);
  }

  @media (max-width: 900px) {
    .game-layout {
      flex-direction: column;
      height: auto;
    }

    .sidebar {
      position: relative;
      width: 100%;
      height: auto;
      border-right: none;
      border-bottom: 1px solid rgba(255, 255, 255, 0.05);
      padding: 1.5rem;
    }

    .board-area {
      padding: 1rem;
      height: auto;
    }

    .board-area :global(.board-container) {
      width: min(95vw, 600px);
      height: min(95vw, 600px);
    }
  }
</style>
