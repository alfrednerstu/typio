import type { FileEntry, FileContent, SearchResult, BacklinkEntry } from './tauri';

interface AppState {
  rootPath: string | null;
  fileTree: FileEntry[];
  currentFile: FileContent | null;
  searchQuery: string;
  searchResults: SearchResult[];
  backlinks: BacklinkEntry[];
  sidebarView: 'files' | 'search';
  expandedDirs: Set<string>;
  showBacklinks: boolean;
}

export const state: AppState = $state({
  rootPath: null,
  fileTree: [],
  currentFile: null,
  searchQuery: '',
  searchResults: [],
  backlinks: [],
  sidebarView: 'files',
  expandedDirs: new Set(),
  showBacklinks: true,
});
