use crate::markdown;
use crate::watcher;
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tauri::State;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub name: String,
    pub raw: String,
    pub frontmatter: HashMap<String, serde_json::Value>,
    pub wikilinks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub name: String,
    pub line_number: usize,
    pub line_content: String,
    pub context_before: String,
    pub context_after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklinkEntry {
    pub source_path: String,
    pub source_name: String,
    pub context: String,
}

/// Open a folder and set it as root path
#[tauri::command]
pub fn open_folder(path: String, state: State<AppState>) -> Result<String, String> {
    let p = Path::new(&path);
    if !p.exists() || !p.is_dir() {
        return Err("Invalid directory path".to_string());
    }
    let mut root = state.root_path.lock().map_err(|e| e.to_string())?;
    *root = Some(path.clone());
    Ok(path)
}

/// Get the current root path
#[tauri::command]
pub fn get_root_path(state: State<AppState>) -> Result<Option<String>, String> {
    let root = state.root_path.lock().map_err(|e| e.to_string())?;
    Ok(root.clone())
}

/// Recursively scan a folder and build a file tree of .md files
#[tauri::command]
pub fn scan_folder(path: String) -> Result<Vec<FileEntry>, String> {
    let root = Path::new(&path);
    if !root.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    fn build_tree(dir: &Path, root: &Path) -> Vec<FileEntry> {
        let mut entries: Vec<FileEntry> = Vec::new();

        let mut items: Vec<_> = match std::fs::read_dir(dir) {
            Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
            Err(_) => return entries,
        };

        items.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            b_is_dir
                .cmp(&a_is_dir)
                .then(a.file_name().cmp(&b.file_name()))
        });

        for item in items {
            let item_path = item.path();
            let name = item.file_name().to_string_lossy().to_string();

            // Skip hidden files/dirs
            if name.starts_with('.') {
                continue;
            }

            if item_path.is_dir() {
                let children = build_tree(&item_path, root);
                // Only include dirs that contain .md files (directly or nested)
                if !children.is_empty() {
                    entries.push(FileEntry {
                        path: item_path.to_string_lossy().to_string(),
                        name,
                        is_dir: true,
                        children,
                    });
                }
            } else if item_path.extension().map(|e| e == "md").unwrap_or(false) {
                entries.push(FileEntry {
                    path: item_path.to_string_lossy().to_string(),
                    name,
                    is_dir: false,
                    children: vec![],
                });
            }
        }

        entries
    }

    Ok(build_tree(root, root))
}

/// Read a markdown file and parse it
#[tauri::command]
pub fn read_file(path: String) -> Result<FileContent, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    let raw = std::fs::read_to_string(p).map_err(|e| e.to_string())?;
    let parsed = markdown::parse_markdown(&raw);
    let name = p
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    Ok(FileContent {
        path,
        name,
        raw,
        frontmatter: parsed.frontmatter,
        wikilinks: parsed.wikilinks,
    })
}

/// Write content to a markdown file
#[tauri::command]
pub fn write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

/// Full-text search across all .md files in the root folder
#[tauri::command]
pub fn search_files(root_path: String, query: String) -> Result<Vec<SearchResult>, String> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().map(|e| e == "md").unwrap_or(false) {
            if let Ok(content) = std::fs::read_to_string(path) {
                let lines: Vec<&str> = content.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    if line.to_lowercase().contains(&query_lower) {
                        let context_before = if i > 0 {
                            lines[i - 1].to_string()
                        } else {
                            String::new()
                        };
                        let context_after = if i + 1 < lines.len() {
                            lines[i + 1].to_string()
                        } else {
                            String::new()
                        };

                        results.push(SearchResult {
                            path: path.to_string_lossy().to_string(),
                            name: path
                                .file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            line_number: i + 1,
                            line_content: line.to_string(),
                            context_before,
                            context_after,
                        });
                    }
                }
            }
        }
    }

    // Limit results
    results.truncate(200);
    Ok(results)
}

/// Find all files that link to a given page via wikilinks
#[tauri::command]
pub fn get_backlinks(root_path: String, page_name: String) -> Result<Vec<BacklinkEntry>, String> {
    let page_lower = page_name.to_lowercase();
    let mut backlinks = Vec::new();

    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() || path.extension().map(|e| e != "md").unwrap_or(true) {
            continue;
        }

        if let Ok(content) = std::fs::read_to_string(path) {
            let wikilinks = markdown::extract_wikilinks(&content);
            for link in &wikilinks {
                if link.to_lowercase() == page_lower {
                    // Find context around the link
                    let search = format!("[[{}]]", link);
                    let alt_search = format!("[[{}|", link);
                    let context = content
                        .lines()
                        .find(|l| l.contains(&search) || l.contains(&alt_search))
                        .unwrap_or("")
                        .to_string();

                    backlinks.push(BacklinkEntry {
                        source_path: path.to_string_lossy().to_string(),
                        source_name: path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                        context,
                    });
                    break; // One backlink per file
                }
            }
        }
    }

    Ok(backlinks)
}

/// Start watching a folder for changes
#[tauri::command]
pub fn watch_folder(path: String, app: tauri::AppHandle) -> Result<(), String> {
    watcher::start_watching(path, app)
}

/// Stop watching
#[tauri::command]
pub fn unwatch_folder() -> Result<(), String> {
    watcher::stop_watching()
}
