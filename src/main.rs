#![feature(globs)]

extern crate native;

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate nalgebra;

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

use nalgebra::na::{Vec2};
use nalgebra::na;

use player_controller::PlayerController;
use config::GameConfig;
use input::{InputMap, InputEvents};
use camera::Camera;
use events::*;

mod input;
mod events;
mod player_controller;
mod config;
mod camera;
mod render;

struct Colonist {
    position : Vec2<f64>
}

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

    let mut camera = Camera::new();

    let mut colonist = Colonist{ position: Vec2{ x: 200f64, y: 200f64} };

    let player_controller = PlayerController::new(camera);

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

    //let render = behaviors::Render::new(window);

    //----------------------------
    let image = asset_store.path("rust-logo.png").unwrap();
    let image = Texture::from_path(&image).unwrap();
    let ref mut gl = Gl::new(opengl::OpenGL_3_3);

    let image_x : f64 = 50.0;
    let image_y : f64 = 50.0;
    //-----------------------------

    let event_iter_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    let mut input_events = InputEvents::new();

    for e in EventIterator::new(&mut window, &event_iter_settings) {
        match e {
            piston::Update(args) => {
                if input_events.has_events() {
                    player_controller.process_input(input_events.clone());
                    input_events.clear();
                }
            },
            piston::Render(args) => {
                //renderer.render(camera, render::RenderFrame{width: args.width, height: args.height});

                //------------------------------------------------------
                gl.viewport(0, 0, args.width as i32, args.height as i32);

                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                c.trans(colonist.position.x, colonist.position.y).image(&image).draw(gl);
                //------------------------------------------------------
            },
            piston::Input(piston::input::Move(piston::input::MouseCursor(x, y))) => {
                input_events.mouse_at(x, y);
            },
            piston::Input(event) => {
                match input_map.convert(event) {
                    Some(input_event) => {
                        input_events.add(input_event);
                    },
                    None => {}
                }
            }
        }
    }
}
