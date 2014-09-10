use std::collections::HashMap;

use piston;

use events;
use super::InputMap;
use super::input_map;

fn esdf_events() -> HashMap<piston::input::keyboard::Key, events::Event> {
    let mut known_events = HashMap::new();
    known_events.insert(piston::input::keyboard::E, events::MoveForward);
    known_events.insert(piston::input::keyboard::D, events::MoveBackward);
    known_events.insert(piston::input::keyboard::S, events::MoveLeft);
    known_events.insert(piston::input::keyboard::F, events::MoveRight);
    known_events
}

#[test]
fn convert_piston_event_to_internal_event() {
    let event_list = esdf_events();
    let map = InputMap::new();

    for (key, event) in event_list.iter() {
        let event_type = map.convert(piston::input::Press(piston::input::Keyboard(*key)));
        assert_eq!(event_type, Some(*event));
    }
}

#[test]
fn returns_none_if_no_match_vound() {
    let map = InputMap::new();
    let event_type = map.convert(piston::input::Press(piston::input::Keyboard(piston::input::keyboard::O)));
    assert_eq!(event_type, None);
}
