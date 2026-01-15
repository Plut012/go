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
  <h1>Go</h1>

  {#if !connected}
    <div class="status">Connecting...</div>
  {:else if !myColor}
    <div class="color-selection">
      <h2>Choose your color</h2>
      <button on:click={() => chooseColor('black')}>Play as Black</button>
      <button on:click={() => chooseColor('white')}>Play as White</button>
    </div>
  {:else}
    <GameInfo {gameState} {myColor} on:pass={pass} on:reset={reset} />
    <Board {gameState} on:move={(e) => makeMove(e.detail.x, e.detail.y)} />
  {/if}
</main>

<style>
  main {
    text-align: center;
  }

  h1 {
    font-size: 2rem;
    margin-bottom: 1rem;
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
</style>
