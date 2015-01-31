#![cfg_attr(test, allow(warnings))]
#![feature(collections)]
#![feature(path)]

//extern crate glfw_window;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;

extern crate texture;
extern crate cgmath;
extern crate graphics;

mod app;
mod settings;
mod render;
mod game;
mod player;
mod tilemap;
mod input;

fn main() {
    // Set the config for the game
    let config = settings::Settings {
        title: "QuakeRL".to_string(),
        window_width: 800,
        window_height: 600,
    };

    // Create and run the game
    let mut app = app::App::new(config);
    app.run();
}
