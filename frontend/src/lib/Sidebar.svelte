<script>
  let {
    fileTree = [],
    currentFilePath = null,
    searchQuery = '',
    searchResults = [],
    sidebarView = 'files',
    expandedDirs = new Set(),
    onOpenFolder,
    onSelectFile,
    onSearch,
    onToggleView,
    onToggleDir,
    hasFolder = false,
  } = $props();

  let searchInput = $state('');
  let searchTimeout;

  function handleSearchInput(e) {
    searchInput = e.target.value;
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      onSearch(searchInput);
    }, 200);
  }
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <div class="sidebar-title">
      <span class="logo-text">Typio</span>
      <button class="open-btn" onclick={onOpenFolder} title="Open folder">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M1 3.5C1 2.67 1.67 2 2.5 2H6l1.5 1.5H13.5C14.33 3.5 15 4.17 15 5V12.5C15 13.33 14.33 14 13.5 14H2.5C1.67 14 1 13.33 1 12.5V3.5Z" stroke="currentColor" stroke-width="1.2" fill="none"/>
        </svg>
      </button>
    </div>

    {#if hasFolder}
      <div class="sidebar-tabs">
        <button
          class="tab"
          class:active={sidebarView === 'files'}
          onclick={() => onToggleView('files')}
        >Files</button>
        <button
          class="tab"
          class:active={sidebarView === 'search'}
          onclick={() => onToggleView('search')}
        >Search</button>
      </div>
    {/if}
  </div>

  {#if hasFolder && sidebarView === 'search'}
    <div class="search-box">
      <svg class="search-icon" width="14" height="14" viewBox="0 0 16 16" fill="none">
        <circle cx="7" cy="7" r="5.5" stroke="currentColor" stroke-width="1.2"/>
        <path d="M11 11L14.5 14.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
      </svg>
      <input
        type="text"
        placeholder="Search all files..."
        value={searchInput}
        oninput={handleSearchInput}
      />
    </div>

    <div class="search-results">
      {#each searchResults as result}
        <button
          class="search-result"
          onclick={() => onSelectFile(result.path)}
        >
          <span class="result-name">{result.name}</span>
          <span class="result-line">Line {result.line_number}</span>
          <span class="result-context">{result.line_content}</span>
        </button>
      {/each}
      {#if searchInput && searchResults.length === 0}
        <div class="no-results">No results found</div>
      {/if}
    </div>
  {:else if hasFolder}
    <div class="file-tree">
      {#each fileTree as entry}
        {@render fileEntry(entry, 0)}
      {/each}
    </div>
  {/if}
</aside>

{#snippet fileEntry(entry, depth)}
  {#if entry.is_dir}
    <button
      class="tree-item dir"
      style="padding-left: {0.75 + depth * 1}rem"
      onclick={() => onToggleDir(entry.path)}
    >
      <svg class="tree-icon" class:expanded={expandedDirs.has(entry.path)} width="12" height="12" viewBox="0 0 12 12">
        <path d="M4 2L8 6L4 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/>
      </svg>
      <span class="tree-name">{entry.display_name}</span>
    </button>
    {#if expandedDirs.has(entry.path)}
      {#each entry.children as child}
        {@render fileEntry(child, depth + 1)}
      {/each}
    {/if}
  {:else}
    <button
      class="tree-item file"
      class:active={currentFilePath === entry.path}
      style="padding-left: {0.75 + depth * 1 + 1}rem"
      onclick={() => onSelectFile(entry.path)}
    >
      <span class="tree-name">{entry.display_name}</span>
    </button>
  {/if}
{/snippet}

<style>
  .sidebar {
    width: var(--sidebar-width);
    min-width: var(--sidebar-width);
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .sidebar-header {
    padding: 1rem 0.75rem 0;
    flex-shrink: 0;
  }

  .sidebar-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .logo-text {
    font-weight: 700;
    font-size: var(--text-xl);
    letter-spacing: -0.02em;
    color: var(--accent);
  }

  .open-btn {
    width: 1.75rem;
    height: 1.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.375rem;
    color: var(--text-secondary);
    transition: background 0.15s, color 0.15s;
  }

  .open-btn:hover {
    background: var(--bg-hover);
    color: var(--text);
  }

  .sidebar-tabs {
    display: flex;
    gap: 0.125rem;
    background: var(--bg-hover);
    border-radius: 0.375rem;
    padding: 0.125rem;
    margin-bottom: 0.5rem;
  }

  .tab {
    flex: 1;
    padding: 0.3125rem 0.5rem;
    border-radius: 0.25rem;
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--text-secondary);
    transition: all 0.15s;
  }

  .tab.active {
    background: var(--bg);
    color: var(--text);
    box-shadow: 0 0.0625rem 0.125rem rgba(0,0,0,0.06);
  }

  .search-box {
    padding: 0 0.75rem 0.5rem;
    position: relative;
    flex-shrink: 0;
  }

  .search-icon {
    position: absolute;
    left: 1.375rem;
    top: 50%;
    transform: translateY(-calc(50% + 0.25rem));
    color: var(--text-muted);
    pointer-events: none;
    margin-top: -0.25rem;
  }

  .search-box input {
    width: 100%;
    padding: 0.4375rem 0.5rem 0.4375rem 1.875rem;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 0.375rem;
    font-size: var(--text-base);
  }

  .search-box input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 0.125rem var(--accent-light);
  }

  .search-results {
    overflow-y: auto;
    flex: 1;
    padding: 0 0.25rem;
  }

  .search-result {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    width: 100%;
    padding: 0.5rem 0.625rem;
    border-radius: 0.25rem;
    text-align: left;
    gap: 0.125rem;
    transition: background 0.1s;
  }

  .search-result:hover {
    background: var(--bg-hover);
  }

  .result-name {
    font-weight: 500;
    font-size: var(--text-base);
    color: var(--text);
  }

  .result-line {
    font-size: var(--text-xs);
    color: var(--text-muted);
  }

  .result-context {
    font-size: var(--text-sm);
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .no-results {
    padding: 1.5rem 0.75rem;
    text-align: center;
    color: var(--text-muted);
    font-size: var(--text-base);
  }

  .file-tree {
    overflow-y: auto;
    flex: 1;
    padding: 0.25rem;
  }

  .tree-item {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    width: 100%;
    padding: 0.25rem 0.75rem;
    border-radius: 0.25rem;
    font-size: var(--text-base);
    text-align: left;
    transition: background 0.1s;
    white-space: nowrap;
    overflow: hidden;
  }

  .tree-item:hover {
    background: var(--bg-hover);
  }

  .tree-item.active {
    background: var(--accent-light);
    color: var(--accent);
    font-weight: 500;
  }

  .tree-item.dir {
    font-weight: 500;
    color: var(--text-secondary);
  }

  .tree-icon {
    flex-shrink: 0;
    transition: transform 0.15s;
  }

  .tree-icon.expanded {
    transform: rotate(90deg);
  }

  .tree-name {
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
