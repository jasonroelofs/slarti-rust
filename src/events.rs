/// The list of Events that a user can trigger through
/// normal action in the game. These get mapped to individual
/// keys and input via input system and config files.
#[deriving(Clone, Show, PartialEq, Eq)]
pub enum Event {
    Quit,

    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
}

