use input::InputEvents;
use camera::Camera;

pub struct PlayerController {
    camera: Camera
}

impl PlayerController {
    pub fn new(camera : Camera) -> PlayerController {
        PlayerController{ camera: camera }
    }

    pub fn process_input(&self, events : InputEvents) {
        println!("Handling events {}", events);
    }

}
