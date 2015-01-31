extern crate event;
extern crate quack;
extern crate window;
extern crate "input" as input_lib;
use self::input_lib::{Button, Key};
use cgmath::*;
use sdl2_window::Sdl2Window as Window;
//use self::glfw_window::GlfwWindow as Window;
use settings::Settings;

// Represents the App
pub struct App {
    config: Settings
}

impl App {
    // Returns a app struct
    pub fn new(config: Settings) -> App {
        // Return a new App
        App {config: config}
    }

    // Run the app
    pub fn run(&mut self) {
        use std::cell::RefCell;
        use cgmath::{Aabb2, Point2};

        use render::Render;
        use input::Input;
        use player::Player;
        use tilemap::TileMap;
        use game::Game;

        let w = self.config.window_width as f32;
        let h = self.config.window_height as f32;

        // Create the window
        let window = RefCell::new(self.window());
        let render = Render::new(w, h);
        let input = Input::new();

        // Create the player
        let mut player = Player::from_path(&Path::new("./assets/ranger_avatar.png"));
        player.set_pos(Vector2::new(80.0, 80.0 as f32));

        // Create the map
        let mut tilemap = TileMap::from_tileset_path(&Path::new("./assets/tileset.png"));
        tilemap.build_procedural_map(20, 15);

        // Create a new game and run it.
        let mut game = Game {
            render: render,
            input: input,
            player: player,
            tilemap: tilemap,
            timestamp: 0.0,
            top_wall:    Aabb2::new(Point2::new(0.0, 0.0),      Point2::new(w as f32, 40.0)),
            bottom_wall: Aabb2::new(Point2::new(0.0, h as f32 - 40.0), Point2::new(w as f32, h as f32)),
            left_wall:   Aabb2::new(Point2::new(0.0, 0.0),      Point2::new(40.0, h as f32)),
            right_wall:  Aabb2::new(Point2::new(w as f32 - 40.0, 0.0), Point2::new(w as f32, h as f32)),
        };

        // Iterate the main game loop
        let mut dt : f32 = 0.0;
        for e in self::event::events(&window) {
            use self::event::{Event, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent };

            let e: Event<input_lib::Input> = e;
            e.render (|args|   game.render());
            e.update (|args| { game.update(args.dt); dt = args.dt as f32; });
            e.press  (|args|   game.input.press(args, dt));
            e.release(|args|   game.input.release(args, dt));
        }
    }

    // Returns a window.
    fn window(&self) -> Window {
        use shader_version::opengl::OpenGL;

        // Values for Window Creation
        let window_settings = window::WindowSettings {
            title: self.config.title.to_string(),
            size: [self.config.window_width, self.config.window_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        };

        // Create a window
        Window::new(OpenGL::_3_2, window_settings)
    }
}
