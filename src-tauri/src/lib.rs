mod commands;
mod markdown;
mod watcher;

use std::sync::Mutex;
pub struct AppState {
    pub root_path: Mutex<Option<String>>,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            root_path: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::open_folder,
            commands::scan_folder,
            commands::read_file,
            commands::write_file,
            commands::search_files,
            commands::get_backlinks,
            commands::get_root_path,
            commands::watch_folder,
            commands::unwatch_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
