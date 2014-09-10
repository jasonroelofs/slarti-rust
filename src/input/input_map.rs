use std::collections::HashMap;
use std::vec::Vec;

use piston;
use piston::input;

use events;

pub struct InputMap {
    key_maps : HashMap<piston::input::keyboard::Key, events::Event>
}

impl InputMap {
    pub fn new() -> InputMap {
        let mut key_map = HashMap::new();
        key_map.insert(piston::input::keyboard::E, events::MoveForward);
        key_map.insert(piston::input::keyboard::S, events::MoveLeft);
        key_map.insert(piston::input::keyboard::D, events::MoveBackward);
        key_map.insert(piston::input::keyboard::F, events::MoveRight);

        InputMap { key_maps: key_map }
    }

    pub fn convert(&self, piston_event : piston::input::InputEvent) -> Option<events::Event> {
        println!("Checking convert: {}", piston_event);
        match piston_event  {
            piston::input::Press(piston::input::Keyboard(key)) => {
                self.key_maps.find_copy(&key)
            },
            _ => { None },
        }
    }
}
