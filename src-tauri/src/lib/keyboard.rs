use crate::types::keycode_wrapper::MyKeycode;
use keybind::{Keybind, Keycode};
use std::thread;

#[tauri::command]
pub fn map_key(input: MyKeycode) {
    thread::spawn(move || {
        let user_keycode: Keycode = input.keycode().clone();
        let mut keybind = Keybind::new(&[user_keycode]);

        keybind.on_trigger(|| {
            println!("This will be printed when you press da buttun");
        });

        keybind.wait();
    });
}
