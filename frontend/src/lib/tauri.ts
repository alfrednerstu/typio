import { invoke } from '@tauri-apps/api/core';

export interface FileEntry {
  path: string;
  name: string;
  display_name: string;
  is_dir: boolean;
  children: FileEntry[];
}

export interface FileContent {
  path: string;
  name: string;
  raw: string;
  frontmatter: Record<string, unknown>;
  wikilinks: string[];
}

export interface SearchResult {
  path: string;
  name: string;
  line_number: number;
  line_content: string;
  context_before: string;
  context_after: string;
}

export interface BacklinkEntry {
  source_path: string;
  source_name: string;
  context: string;
}

export async function openFolder(path: string): Promise<string> {
  return invoke('open_folder', { path });
}

export async function scanFolder(path: string): Promise<FileEntry[]> {
  return invoke('scan_folder', { path });
}

export async function readFile(path: string): Promise<FileContent> {
  return invoke('read_file', { path });
}

export async function writeFile(path: string, content: string): Promise<void> {
  return invoke('write_file', { path, content });
}

export async function searchFiles(rootPath: string, query: string): Promise<SearchResult[]> {
  return invoke('search_files', { rootPath, query });
}

export async function getBacklinks(rootPath: string, pageName: string): Promise<BacklinkEntry[]> {
  return invoke('get_backlinks', { rootPath, pageName });
}

export async function getRootPath(): Promise<string | null> {
  return invoke('get_root_path');
}

export async function watchFolder(path: string): Promise<void> {
  return invoke('watch_folder', { path });
}

export async function unwatchFolder(): Promise<void> {
  return invoke('unwatch_folder');
}
