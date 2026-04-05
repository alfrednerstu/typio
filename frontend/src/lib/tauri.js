import { invoke } from '@tauri-apps/api/core';

export async function openFolder(path) {
  return invoke('open_folder', { path });
}

export async function scanFolder(path) {
  return invoke('scan_folder', { path });
}

export async function readFile(path) {
  return invoke('read_file', { path });
}

export async function writeFile(path, content) {
  return invoke('write_file', { path, content });
}

export async function searchFiles(rootPath, query) {
  return invoke('search_files', { rootPath, query });
}

export async function getBacklinks(rootPath, pageName) {
  return invoke('get_backlinks', { rootPath, pageName });
}

export async function getRootPath() {
  return invoke('get_root_path');
}

export async function watchFolder(path) {
  return invoke('watch_folder', { path });
}

export async function unwatchFolder() {
  return invoke('unwatch_folder');
}
