use std::vec::Vec;

use nalgebra::na::{Vec2};
use nalgebra::na;

use events;

/// A per-frame list of all input events received
/// as well as other information such as mouse position
/// for each update pass.
#[deriving(Show, Clone)]
pub struct InputEvents {
    pub events : Vec<events::Event>,
    pub mouse  : Vec2<f64>
}

impl InputEvents {
    pub fn new() -> InputEvents {
        InputEvents{
            events: Vec::new(),
            mouse : na::zero()
        }
    }

    pub fn add(&mut self, event: events::Event) {
        self.events.push(event)
    }

    pub fn has_events(&self) -> bool {
        self.events.len() > 0
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn mouse_at(&mut self, x: f64, y: f64) {
        self.mouse = Vec2{x: x, y: y};
    }
}
