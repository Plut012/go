<script>
  import Board from './Board.svelte';
  import GameInfo from './GameInfo.svelte';
  import BoardSizeModal from './BoardSizeModal.svelte';
  import { createConnection } from './lib/websocket.js';
  import { loadTheme } from './lib/theme.js';
  import { onMount } from 'svelte';

  let connected = false;
  let gameState = null;
  let myColor = null;
  let ws = null;
  let showSizeModal = false;
  let aiMode = false;
  let aiColor = null;

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
    } else if (data.type === 'ownership_update') {
      // Update ownership data separately without replacing entire state
      if (gameState) {
        gameState = { ...gameState, ownership: data.ownership };
      }
    } else if (data.type === 'your_color') {
      myColor = data.color;
    } else if (data.type === 'error') {
      console.error('Server error:', data.message);
      // TODO: Show error to user
    }
  }

  function chooseColor(color) {
    if (ws) {
      // If AI mode is enabled, set AI to play opposite color
      if (aiMode) {
        aiColor = color === 'black' ? 'white' : 'black';
      }

      ws.send(JSON.stringify({
        type: 'choose_color',
        color,
        ai_mode: aiMode
      }));
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

  function requestReset() {
    showSizeModal = true;
  }

  function handleSizeSelect(event) {
    const size = event.detail.size;
    showSizeModal = false;
    if (ws && connected) {
      ws.send(JSON.stringify({ type: 'reset', board_size: size }));
    }
  }

  function closeSizeModal() {
    showSizeModal = false;
  }

  function requestAIMove() {
    if (ws && connected && aiMode) {
      ws.send(JSON.stringify({ type: 'request_ai_move' }));
    }
  }

  // Trigger AI move after human moves in AI mode
  $: if (aiMode && gameState && gameState.turn === aiColor && connected && myColor) {
    // Small delay to let UI update
    setTimeout(requestAIMove, 100);
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

        {#if gameState?.katago_available}
          <div class="ai-option">
            <label class="ai-checkbox">
              <input type="checkbox" bind:checked={aiMode} />
              <span>vs AI</span>
            </label>
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <div class="game-layout">
      <aside class="sidebar">
        <GameInfo {gameState} {myColor} {aiMode} {aiColor} on:pass={pass} on:reset={requestReset} />
      </aside>
      <div class="board-area">
        <Board {gameState} on:move={(e) => makeMove(e.detail.x, e.detail.y)} />
      </div>
    </div>
    {#if showSizeModal}
      <BoardSizeModal on:select={handleSizeSelect} on:close={closeSizeModal} />
    {/if}
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

  .ai-option {
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .ai-checkbox {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    cursor: pointer;
    user-select: none;
    opacity: 0.8;
  }

  .ai-checkbox:hover {
    opacity: 1;
  }

  .ai-checkbox input[type="checkbox"] {
    cursor: pointer;
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
