#![feature(globs)]

extern crate native;

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate shader_version;

extern crate serialize;

use shader_version::*;
use opengl_graphics::{
    Gl,
    Texture,
};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use piston::{
    AssetStore,
    EventIterator,
    EventSettings,
    WindowSettings,
    Render,
};

use player_controller::PlayerController;
use config::GameConfig;
use input::InputMap;

mod input;
mod events;
mod player_controller;
mod config;

#[cfg(not(test))]
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {

    let asset_store = AssetStore::from_folder("../assets");
    let config_store = AssetStore::from_folder("../config");

    let game_config = GameConfig::new(config_store.path("settings.json").unwrap());

    let input_map = InputMap::new(game_config.input);

    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_3,
        WindowSettings {
            title: "Slartibartfast".to_string(),
            size: [game_config.window.width, game_config.window.height],
            fullscreen: game_config.window.fullscreen,
            samples: 0,
            //exit_on_esc: false,
            exit_on_esc: true,
        }
    );

    //----------------
    let image = asset_store.path("rust-logo.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let event_iter_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    let ref mut gl = Gl::new(opengl::OpenGL_3_3);

    let image_x : f64 = 50.0;
    let image_y : f64 = 50.0;
    //-----------------------------

    for e in EventIterator::new(&mut window, &event_iter_settings) {
        match e {
            piston::Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                c.trans(image_x, image_y).image(&image).draw(gl);
                c.trans(image_x + 10.0, image_y + 10.0).image(&image).draw(gl);
            },
            piston::Input(e) => {
                match input_map.convert(e) {
                    Some(input_event) => {
                        player_controller.handle_event(input_event);
                    },
                    None => {}
                }
            },
            _ => {},
        }
    }
}
