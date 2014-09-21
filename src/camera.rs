use nalgebra::na::{Vec2};
use nalgebra::na;

pub struct Camera {
    speed    : uint,
    position : Vec2<f64>
}

impl Camera {
    pub fn new() -> Camera {
        Camera { position: na::zero(), speed: 10 }
    }

    pub fn move_forward(&mut self) {
    }

    pub fn move_backward(&mut self) {
    }

    pub fn move_left(&mut self) {
    }

    pub fn move_right(&mut self) {
    }
}
