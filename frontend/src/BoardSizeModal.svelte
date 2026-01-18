<script>
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  function selectSize(size) {
    dispatch('select', { size });
  }

  function close() {
    dispatch('close');
  }
</script>

<div class="modal-overlay" on:click={close}>
  <div class="modal" on:click|stopPropagation>
    <h2>Select board size</h2>
    <div class="size-options">
      <button on:click={() => selectSize(9)}>
        <div class="size-label">9×9</div>
        <div class="size-description">Quick game</div>
      </button>
      <button on:click={() => selectSize(13)}>
        <div class="size-label">13×13</div>
        <div class="size-description">Medium game</div>
      </button>
      <button on:click={() => selectSize(19)}>
        <div class="size-label">19×19</div>
        <div class="size-description">Standard</div>
      </button>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: #1a1a1a;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 2rem;
    min-width: 300px;
  }

  h2 {
    margin: 0 0 1.5rem 0;
    font-size: 1rem;
    font-weight: normal;
    text-align: center;
    opacity: 0.9;
  }

  .size-options {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  button {
    padding: 1rem 1.5rem;
    background: transparent;
    border: 1px solid #444;
    color: #e0e0e0;
    cursor: pointer;
    border-radius: 2px;
    transition: all 0.15s ease;
    text-align: left;
  }

  button:hover {
    border-color: #666;
    background: rgba(255, 255, 255, 0.05);
  }

  .size-label {
    font-size: 1.1rem;
    margin-bottom: 0.25rem;
  }

  .size-description {
    font-size: 0.8rem;
    opacity: 0.6;
  }
</style>
