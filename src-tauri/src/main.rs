#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// fn main() {
// 	nuxtor_lib::run();
// }

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_serialplugin::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}