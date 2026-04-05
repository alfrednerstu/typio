// Reactive state store using Svelte 5 runes

export const state = $state({
  rootPath: null,
  fileTree: [],
  currentFile: null,
  searchQuery: '',
  searchResults: [],
  backlinks: [],
  sidebarView: 'files', // 'files' | 'search'
  expandedDirs: new Set(),
  showBacklinks: true,
});
