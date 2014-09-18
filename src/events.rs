/// The list of Events that a user can trigger through
/// normal action in the game. These get mapped to individual
/// keys and input via input system and config files.
#[deriving(Clone, Show, PartialEq, Eq)]
pub enum Event {
    Unknown,

    Quit,

    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

pub fn from_string(string : &String) -> Event {
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
