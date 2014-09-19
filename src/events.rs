/// The states an Event can be in
#[deriving(Clone, Show, PartialEq, Eq)]
pub enum Event {
    Pressed(EventType),
    Released(EventType),
    Repeated(EventType)
}

/// The list of Events that a user can trigger through
/// normal action in the game. These get mapped to individual
/// keys and input via input system and config files.
#[deriving(Clone, Show, PartialEq, Eq)]
pub enum EventType {
    Unknown,

    Quit,

    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

pub fn from_string(string : &String) -> EventType {
    match string.as_slice() {
        "Quit"          => Quit,
        "MoveForward"   => MoveForward,
        "MoveBackward"  => MoveBackward,
        "MoveLeft"      => MoveLeft,
        "MoveRight"     => MoveRight,
        _               => {
            println!("No event known with the name {}", string);
            Unknown
        }
    }
}
