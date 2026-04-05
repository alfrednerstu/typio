<script>
  let { backlinks = [], onSelectFile } = $props();
</script>

<aside class="backlinks-panel">
  <div class="backlinks-header">
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      <path d="M6.5 3H3.5C2.67 3 2 3.67 2 4.5V6.5M9.5 13H12.5C13.33 13 14 12.33 14 11.5V9.5M5 8L2 11M11 8L14 5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
    </svg>
    <span>Backlinks</span>
    <span class="count">{backlinks.length}</span>
  </div>
  <div class="backlinks-list">
    {#each backlinks as backlink}
      <button class="backlink-item" onclick={() => onSelectFile(backlink.source_path)}>
        <span class="backlink-name">{backlink.source_name}</span>
        {#if backlink.context}
          <span class="backlink-context">{backlink.context}</span>
        {/if}
      </button>
    {/each}
  </div>
</aside>

<style>
  .backlinks-panel {
    width: var(--backlinks-width);
    min-width: var(--backlinks-width);
    border-left: 1px solid var(--border);
    background: var(--bg);
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .backlinks-header {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 16px 16px 12px;
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .count {
    background: var(--accent-light);
    color: var(--accent);
    padding: 1px 7px;
    border-radius: 10px;
    font-size: 11px;
  }

  .backlinks-list {
    overflow-y: auto;
    flex: 1;
    padding: 0 8px 8px;
  }

  .backlink-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
    width: 100%;
    padding: 8px 10px;
    border-radius: 6px;
    text-align: left;
    transition: background 0.1s;
  }

  .backlink-item:hover {
    background: var(--bg-hover);
  }

  .backlink-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--accent);
  }

  .backlink-context {
    font-size: 12px;
    color: var(--text-secondary);
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
