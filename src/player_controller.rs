use events;

pub struct PlayerController {
    id : i32
}

impl PlayerController {
    pub fn new() -> PlayerController {
        PlayerController{ id: 4 }
    }

    pub fn handle_event(&self, game_event : events::Event) {
        println!("Handling event {}", game_event);
    }
}
