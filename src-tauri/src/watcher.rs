use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::Mutex;
use tauri::Emitter;

static WATCHER: Mutex<Option<RecommendedWatcher>> = Mutex::new(None);

pub fn start_watching(path: String, app: tauri::AppHandle) -> Result<(), String> {
    // Stop any existing watcher
    stop_watching()?;

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    let paths: Vec<String> = event
                        .paths
                        .iter()
                        .filter(|p| p.extension().map(|e| e == "md").unwrap_or(false))
                        .map(|p| p.to_string_lossy().to_string())
                        .collect();

                    if !paths.is_empty() {
                        let kind = match event.kind {
                            EventKind::Create(_) => "create",
                            EventKind::Modify(_) => "modify",
                            EventKind::Remove(_) => "remove",
                            _ => "unknown",
                        };

                        let _ = app.emit(
                            "file-changed",
                            serde_json::json!({
                                "kind": kind,
                                "paths": paths,
                            }),
                        );
                    }
                }
                _ => {}
            }
        }
    })
    .map_err(|e| e.to_string())?;

    watcher
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .map_err(|e| e.to_string())?;

    let mut guard = WATCHER.lock().map_err(|e| e.to_string())?;
    *guard = Some(watcher);

    Ok(())
}

pub fn stop_watching() -> Result<(), String> {
    let mut guard = WATCHER.lock().map_err(|e| e.to_string())?;
    *guard = None;
    Ok(())
}
