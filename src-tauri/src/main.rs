#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib {
    pub mod keyboard;
    pub mod mouse;
}
mod types;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            lib::mouse::click_at,
            lib::mouse::move_to,
            lib::mouse::get_position,
            lib::keyboard::map_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
