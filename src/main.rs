#![windows_subsystem = "console"]

use inputbot::{
    KeybdKey::{self, *},
    MouseButton::*,
};
use sonny::{Macro, MacroState};
use std::{sync::{Arc, Mutex}, os::windows};
use std::thread::sleep;
use std::time::Duration;

mod sonny;

fn main() {
    
    let og = Arc::new(Mutex::new(Macro::new()));
    let val = og.clone();

    // Numrow1Key.bind(move || {
    //     let v = val.as_ref();
    //     v.lock().unwrap().key_pressed(Numrow1Key);
    // });
    //
    // let val = og.clone();
    // Numpad1Key.bind(move || {
    //     let v = val.as_ref();
    //     v.lock().unwrap().key_pressed(Numpad1Key);
    // });
    KeybdKey::bind_all(move |e| {
        val.lock().unwrap().key_pressed(e);
    });

    // Bind your caps lock key to a function that starts an autoclicker.
    CapsLockKey.bind(move || {
        while CapsLockKey.is_toggled() {
            LeftButton.press();
            LeftButton.release();

            sleep(Duration::from_millis(30));
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
