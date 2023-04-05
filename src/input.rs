use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use fltk::{
    app::event_key,
    enums::{Event, Key},
    prelude::WidgetBase,
};

use crate::Emulator;

pub fn handle_events(window: &mut fltk::window::Window, keyboard: Arc<Mutex<VecDeque<char>>>) {
    window.handle(move |_, event| {
        if event != Event::KeyDown && event != Event::KeyUp {
            return false;
        }

        let mut key_presses = keyboard
            .lock()
            .expect("Could not get a lock on the keyboard events");

        let key = event_key();
        let ascii_key = key.bits() as u8 as char;

        match event {
            Event::KeyDown => {
                if key == Key::Escape {
                    fltk::app::quit();
                }

                if !key_presses.contains(&ascii_key) {
                    key_presses.push_front(ascii_key);
                }

                true
            }
            Event::KeyUp => {
                if let Some(index) = key_presses.iter().position(|k| *k == ascii_key) {
                    key_presses.remove(index);
                }

                true
            }
            _ => unreachable!(),
        }
    });
}

pub fn handle_keyboard(emulator: &mut Emulator, keyboard: &Arc<Mutex<VecDeque<char>>>) {
    let mut keyboard_event = keyboard
        .lock()
        .expect("Could not get a lock on the keyboard events");

    emulator.keyboard = [false; 16];

    let key = if emulator.waiting_for_keypress {
        keyboard_event.pop_front()
    } else {
        keyboard_event.get(0).copied()
    };

    if let Some(key) = key {
        match key.to_ascii_uppercase() {
            /*  (Hex)        (KBD)
               1 2 3 C  =>  1 2 3 4
               4 5 6 D  =>  Q W E R
               7 8 9 E  =>  A S D F
               A 0 B F  =>  Z X C V
            */
            '1' => emulator.keyboard[0x01] = true,
            '2' => emulator.keyboard[0x02] = true,
            '3' => emulator.keyboard[0x03] = true,
            '4' => emulator.keyboard[0x0C] = true,

            'Q' => emulator.keyboard[0x04] = true,
            'W' => emulator.keyboard[0x05] = true,
            'E' => emulator.keyboard[0x06] = true,
            'R' => emulator.keyboard[0x0D] = true,

            'A' => emulator.keyboard[0x07] = true,
            'S' => emulator.keyboard[0x08] = true,
            'D' => emulator.keyboard[0x09] = true,
            'F' => emulator.keyboard[0x0E] = true,

            'Z' => emulator.keyboard[0x0A] = true,
            'X' => emulator.keyboard[0x00] = true,
            'C' => emulator.keyboard[0x0B] = true,
            'V' => emulator.keyboard[0x0F] = true,
            _ => {}
        }
    }
}
