<script>
  import { state } from './lib/store.svelte.js';
  import { openFolder, scanFolder, readFile, searchFiles, getBacklinks, watchFolder } from './lib/tauri.js';
  import { listen } from '@tauri-apps/api/event';
  import { open } from '@tauri-apps/plugin-dialog';
  import Sidebar from './lib/Sidebar.svelte';
  import Reader from './lib/Reader.svelte';
  import BacklinksPanel from './lib/BacklinksPanel.svelte';
  import Welcome from './lib/Welcome.svelte';

  async function handleOpenFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      await openFolder(selected);
      state.rootPath = selected;
      state.fileTree = await scanFolder(selected);
      state.currentFile = null;
      state.backlinks = [];
      state.expandedDirs = new Set();

      // Start file watching
      try {
        await watchFolder(selected);
      } catch (e) {
        console.warn('Could not start file watcher:', e);
      }
    }
  }

  async function handleSelectFile(path) {
    try {
      state.currentFile = await readFile(path);
      // Fetch backlinks
      if (state.rootPath && state.currentFile) {
        state.backlinks = await getBacklinks(state.rootPath, state.currentFile.name);
      }
    } catch (e) {
      console.error('Failed to read file:', e);
    }
  }

  async function handleNavigateWikilink(target) {
    if (!state.rootPath) return;

    // Search file tree for matching file
    function findFile(entries, target) {
      const targetLower = target.toLowerCase();
      for (const entry of entries) {
        if (!entry.is_dir) {
          const name = entry.name.replace(/\.md$/, '');
          if (name.toLowerCase() === targetLower) {
            return entry.path;
          }
        }
        if (entry.children) {
          const found = findFile(entry.children, target);
          if (found) return found;
        }
      }
      return null;
    }

    const filePath = findFile(state.fileTree, target);
    if (filePath) {
      await handleSelectFile(filePath);
    }
  }

  async function handleSearch(query) {
    if (!state.rootPath || !query.trim()) {
      state.searchResults = [];
      return;
    }
    try {
      state.searchResults = await searchFiles(state.rootPath, query);
    } catch (e) {
      console.error('Search failed:', e);
      state.searchResults = [];
    }
  }

  // Listen for file change events from Tauri backend
  $effect(() => {
    let unlisten;
    if (state.rootPath) {
      listen('file-changed', async (event) => {
        const { kind, paths } = event.payload;

        // Re-scan file tree
        state.fileTree = await scanFolder(state.rootPath);

        // If current file was modified, reload it
        if (state.currentFile && paths.includes(state.currentFile.path)) {
          if (kind === 'modify') {
            state.currentFile = await readFile(state.currentFile.path);
            if (state.rootPath) {
              state.backlinks = await getBacklinks(state.rootPath, state.currentFile.name);
            }
          } else if (kind === 'remove') {
            state.currentFile = null;
            state.backlinks = [];
          }
        }
      }).then(fn => { unlisten = fn; });
    }

    return () => {
      if (unlisten) unlisten();
    };
  });
</script>

<div class="app-layout">
  <Sidebar
    fileTree={state.fileTree}
    currentFilePath={state.currentFile?.path}
    searchQuery={state.searchQuery}
    searchResults={state.searchResults}
    sidebarView={state.sidebarView}
    expandedDirs={state.expandedDirs}
    onOpenFolder={handleOpenFolder}
    onSelectFile={handleSelectFile}
    onSearch={handleSearch}
    onToggleView={(view) => state.sidebarView = view}
    onToggleDir={(path) => {
      if (state.expandedDirs.has(path)) {
        state.expandedDirs.delete(path);
      } else {
        state.expandedDirs.add(path);
      }
      state.expandedDirs = new Set(state.expandedDirs);
    }}
    hasFolder={!!state.rootPath}
  />

  <main class="main-content">
    {#if state.currentFile}
      <Reader
        file={state.currentFile}
        onNavigateWikilink={handleNavigateWikilink}
      />
    {:else}
      <Welcome onOpenFolder={handleOpenFolder} hasFolder={!!state.rootPath} />
    {/if}
  </main>

  {#if state.currentFile && state.backlinks.length > 0}
    <BacklinksPanel
      backlinks={state.backlinks}
      onSelectFile={handleSelectFile}
    />
  {/if}
</div>

<style>
  .app-layout {
    display: flex;
    height: 100%;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    min-width: 0;
  }
</style>
