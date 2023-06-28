use enigo::{Enigo, MouseButton, MouseControllable};

#[tauri::command]
pub fn click_at(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    enigo.mouse_click(MouseButton::Left);
}

#[tauri::command]
pub fn move_to(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
}

#[tauri::command]
pub fn get_position() -> (i32, i32) {
    let enigo = Enigo::new();
    enigo.mouse_location()
}
