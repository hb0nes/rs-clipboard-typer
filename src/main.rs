use std::thread::sleep;
use std::time::Duration;
use copypasta::{ClipboardContext, ClipboardProvider};
use inputbot::{KeySequence, KeybdKey::*, MouseButton::*, KeybdKey, handle_input_events};

fn main() {
    F12Key.bind(|| {
        if !LShiftKey.is_pressed() {
            return;
        }
        let mut ctx = ClipboardContext::new().unwrap();
        let clipboard_result = ctx.get_contents();
        let contents = match clipboard_result {
            Ok(contents) => {
                contents
            }
            Err(why) => {
                println!("No usable text found in clipboard: {why}");
                return;
            }
        };
        println!("Found '{contents}' in clipboard. Typing it after 500ms delay.");
        sleep(Duration::from_millis(500));
        for char in contents.chars() {
            if char == '\n' {
                EnterKey.press();
                EnterKey.release();
                continue
            }
            let key = inputbot::get_keybd_key(char);
            if let Some(key) = key {
                if char.is_uppercase() || [
                    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|',
                    ':', '"', '<', '>', '?', '~',
                ].contains(&char) {
                    LShiftKey.press();
                }
                key.press();
                key.release();
                LShiftKey.release();
            }
        }
    });
    println!("Write current clipboard contents character by character, as if you were typing it yourself.");
    println!("Keybind: shift+f12");
    handle_input_events();
}