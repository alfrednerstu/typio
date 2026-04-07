<script>
  let { onOpenFolder, hasFolder = false } = $props();
</script>

<div class="welcome">
  <div class="welcome-content">
    <h1 class="welcome-title">Typio</h1>
    <p class="welcome-subtitle">A fast, local markdown knowledge base</p>

    {#if !hasFolder}
      <button class="open-folder-btn" onclick={onOpenFolder}>
        <svg width="20" height="20" viewBox="0 0 16 16" fill="none">
          <path d="M1 3.5C1 2.67 1.67 2 2.5 2H6l1.5 1.5H13.5C14.33 3.5 15 4.17 15 5V12.5C15 13.33 14.33 14 13.5 14H2.5C1.67 14 1 13.33 1 12.5V3.5Z" stroke="currentColor" stroke-width="1.2" fill="none"/>
        </svg>
        Open a folder
      </button>
      <p class="welcome-hint">Point Typio at a folder of .md files to get started</p>
    {:else}
      <p class="welcome-hint">Select a file from the sidebar to start reading</p>
    {/if}
  </div>
</div>

<style>
  .welcome {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 3rem;
  }

  .welcome-content {
    text-align: center;
  }

  .welcome-title {
    font-family: var(--font-serif);
    font-size: var(--text-6xl);
    font-weight: 700;
    color: var(--accent);
    letter-spacing: -0.03em;
    margin-bottom: 0.5rem;
  }

  .welcome-subtitle {
    font-size: var(--text-2xl);
    color: var(--text-secondary);
    margin-bottom: 2rem;
  }

  .open-folder-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.75rem 1.75rem;
    background: var(--accent);
    color: white;
    border-radius: 0.625rem;
    font-size: var(--text-lg);
    font-weight: 500;
    transition: opacity 0.15s, transform 0.15s;
  }

  .open-folder-btn:hover {
    opacity: 0.9;
    transform: translateY(-0.0625rem);
  }

  .open-folder-btn:active {
    transform: translateY(0);
  }

  .welcome-hint {
    font-size: var(--text-md);
    color: var(--text-muted);
    margin-top: 1rem;
  }
</style>
