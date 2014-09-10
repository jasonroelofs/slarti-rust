/// All config-related options handled here

use std::collections::HashMap;
use std::path::posix::Path;
use std::io::File;
use serialize::json;

use std::str;

#[deriving(Decodable, Encodable, Show)]
pub struct GameConfig {
    pub input  : HashMap<String, String>,
    pub window : WindowOptions
}

#[deriving(Decodable, Encodable, Show)]
struct WindowOptions {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

impl GameConfig {
    pub fn new(json_file: Path) -> GameConfig {
        let file_contents = File::open(&json_file).read_to_end().unwrap();
        let json_vector = file_contents.as_slice();
        let json_contents = str::from_utf8(json_vector).unwrap();
        let config : GameConfig = json::decode(json_contents).unwrap();
        config
    }
}
