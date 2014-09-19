use std::collections::HashMap;
use std::vec::Vec;

use piston;
use piston::input;
use piston::input::keyboard::Key;

use events;

pub struct InputMap {
    key_maps : HashMap<piston::input::keyboard::Key, events::EventType>
}

impl InputMap {
    pub fn new(key_bindings : HashMap<String,String>) -> InputMap {
        let mut key_event_map = HashMap::new();

        for (event_name, key_name) in key_bindings.iter() {
            // Piston allows mapping of the lower-case ascii character codes
            // back to the piston::input::keyboard::Key variant, so we need to
            // make sure we convert using the lower case code
            let first_byte : char = key_name.as_slice().char_at(0).to_lowercase();
            let piston_key : piston::input::keyboard::Key = FromPrimitive::from_u64(first_byte as u64).unwrap();
            println!("{} => {} // {}", key_name, first_byte as u8, piston_key);

            let event = events::from_string(event_name);

            key_event_map.insert(piston_key, event);
        }

        InputMap { key_maps: key_event_map }
    }

    pub fn convert(&self, piston_event : piston::input::InputEvent) -> Option<events::Event> {
        println!("Checking convert: {}", piston_event);
        match piston_event  {
            piston::input::Press(piston::input::Keyboard(key)) => {
                match self.key_maps.find_copy(&key) {
                    Some(e) => Some(events::Pressed(e)),
                    None => None
                }
            },
            piston::input::Release(piston::input::Keyboard(key)) => {
                match self.key_maps.find_copy(&key) {
                    Some(e) => Some(events::Released(e)),
                    None => None
                }
            },
            _ => { None },
        }
    }
}
