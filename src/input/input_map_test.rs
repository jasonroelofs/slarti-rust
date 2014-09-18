use std::collections::HashMap;

use piston;

use events;
use super::InputMap;
use super::input_map;

fn esdf_events() -> HashMap<String, String> {
    let mut known_events = HashMap::new();
    known_events.insert(String::from_str("MoveForward"), String::from_str("E"));
    known_events.insert(String::from_str("MoveBackward"), String::from_str("D"));
    known_events.insert(String::from_str("MoveLeft"), String::from_str("S"));
    known_events.insert(String::from_str("MoveRight"), String::from_str("F"));
    known_events
}

fn press_keyboard(key : piston::input::keyboard::Key) -> piston::input::InputEvent {
    piston::input::Press(piston::input::Keyboard(key))
}

#[test]
fn convert_piston_event_to_internal_event() {
    let map = InputMap::new(esdf_events());

    assert_eq!(events::MoveForward, map.convert(press_keyboard(piston::input::keyboard::E)).unwrap());
    assert_eq!(events::MoveBackward, map.convert(press_keyboard(piston::input::keyboard::D)).unwrap());
    assert_eq!(events::MoveLeft, map.convert(press_keyboard(piston::input::keyboard::S)).unwrap());
    assert_eq!(events::MoveRight, map.convert(press_keyboard(piston::input::keyboard::F)).unwrap());
}

#[test]
fn returns_none_if_no_match_vound() {
    let map = InputMap::new(esdf_events());

    assert_eq!(None, map.convert(press_keyboard(piston::input::keyboard::I)));
}
