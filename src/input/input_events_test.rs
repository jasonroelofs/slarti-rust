use nalgebra::na::{Vec2};
use nalgebra::na;

use events;
use super::InputEvents;
use super::input_events;

#[test]
fn initializes_an_empty_list() {
    let ie = InputEvents::new();
    assert_eq!(ie.events.len(), 0);
}

#[test]
fn add_an_event_to_the_list() {
    let mut ie = InputEvents::new();
    ie.add(events::Pressed(events::MoveForward));

    assert_eq!(ie.events.len(), 1);
}

#[test]
fn knows_if_the_list_has_events_to_process() {
    let mut ie = InputEvents::new();

    assert!(!ie.has_events());

    ie.add(events::Pressed(events::MoveForward));

    assert!(ie.has_events());
}

#[test]
fn clear_all_events_from_the_list() {
    let mut ie = InputEvents::new();
    ie.add(events::Pressed(events::MoveForward));
    ie.clear();

    assert_eq!(ie.events.len(), 0);
}

#[test]
fn set_current_mouse_position() {
    let mut ie = InputEvents::new();
    ie.mouse_at(4.0, 3.0);

    assert_eq!(ie.mouse, Vec2{x: 4.0, y: 3.0});
}

#[test]
fn keeps_the_latest_mouse_position() {
    let mut ie = InputEvents::new();
    ie.mouse_at(4.0, 3.0);
    ie.mouse_at(1.0, 2.0);
    ie.mouse_at(7.0, 6.0);

    assert_eq!(ie.mouse, Vec2{x: 7.0, y: 6.0});
}
