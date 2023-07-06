#![windows_subsystem = "console"]

use async_std::{self, task};
use inputbot::{
    KeybdKey::{self, *},
    MouseButton::*,
};
use sonny::{Macro, MacroState};
use std::sync::{Arc, Mutex};
// use async_std::sync::Mutex;
// use async_std::sync::Arc;

mod sonny;
mod window;
// mod macros;
// use macros::Macro;

fn main() {
    // let mut sonny = Macro::new();
    let og = Arc::new(Mutex::new(Macro::new()));
    let val = og.clone();

    KeybdKey::bind_all(move |e| {
        // *val.lock().unwrap() = Some(e);
        val.lock().unwrap().key_pressed(e);
    });
    inputbot::handle_input_events();
}
