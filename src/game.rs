use cgmath::*;
use fps_counter::*;

use render::{Render, RenderState};
use input::{Input, Signal};
use player::Player;
use tilemap::TileMap;

pub struct Game {
    pub render_fps: FPSCounter,
    pub update_fps: FPSCounter,
    pub render: Render,
    pub input: Input,
    pub player: Player,
    pub tilemap: TileMap,
    pub timestamp: f64,
    pub top_wall: Aabb2<f32>,
    pub bottom_wall: Aabb2<f32>,
    pub left_wall: Aabb2<f32>,
    pub right_wall: Aabb2<f32>,
    pub block: Aabb2<f32>,
}

impl Game {
    // Fix this
    pub fn new(w: f32, h: f32, render: Render, input: Input) -> Self {
        // Create the player
        let mut player = Player::from_path(&Path::new("./assets/ranger_avatar.png"));
        player.set_pos(Vector2::new(80.0, 80.0 as f32));

        // Create the map
        let mut tilemap = TileMap::from_tileset_path(&Path::new("./assets/tileset.png"));
        tilemap.build_procedural_map(20, 15);

        Game {
            render_fps: FPSCounter::new(),
            update_fps: FPSCounter::new(),
            render: render,
            input: input,
            player: player,
            tilemap: tilemap,
            timestamp: 0.0,
            top_wall:    Aabb2::new(Point2::new(0.0, 0.0),      Point2::new(w as f32, 40.0)),
            bottom_wall: Aabb2::new(Point2::new(0.0, h as f32 - 40.0), Point2::new(w as f32, h as f32)),
            left_wall:   Aabb2::new(Point2::new(0.0, 0.0),      Point2::new(40.0, h as f32)),
            right_wall:  Aabb2::new(Point2::new(w as f32 - 40.0, 0.0), Point2::new(w as f32, h as f32)),
            block:       Aabb2::new(Point2::new(300.0, 200.0), Point2::new(500.0, 400.0)),
        }
    }

    pub fn render(&mut self) {
        let state = RenderState {
            enable_alpha: true,
            clear: Some([0.0; 4]),
        };

        let zero = Vector2::new(0.0, 0.0);

        self.render.state_push(state);
        self.render.draw(&self.tilemap, &zero);
        self.render.draw(&self.top_wall, &self.top_wall.center().to_vec());
        self.render.draw(&self.bottom_wall, &self.bottom_wall.center().to_vec());
        self.render.draw(&self.left_wall, &self.left_wall.center().to_vec());
        self.render.draw(&self.right_wall, &self.right_wall.center().to_vec());
        self.render.draw(&self.block, &self.block.center().to_vec());
        self.render.draw(&self.player, &self.player.get_pos());
        self.render.state_pop();
        //println!("Render: {}", self.render_fps.tick());
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f64) {
        // Rotate 2 radians per second.
        //self.player.add_rotation(0.0 * dt);

        let x = self.input.get_signal(Signal::AxisX);
        let y = self.input.get_signal(Signal::AxisY);

        let dir = if x == 0.0 && y == 0.0 {
            Vector2::new(0.0, 0.0)
        } else {
            Vector2::new(x, y).normalize()
        };

        self.player.add_pos(dir);

        // TODO: create something to manage this. a world?
        // TODO: add name to things for debugging?
        // TODO: do a "physics" update elsewhere with fixed dt?
        let walls = [
            (&self.top_wall, "top"),
            (&self.bottom_wall, "bottom"),
            (&self.left_wall, "left"),
            (&self.right_wall, "right"),
            (&self.block, "block"),
        ];

        for t in walls.iter() {
            let &(w, _/*n*/) = t;
            if self.player.intersect(w) {
                //println!("collided with {} - {} - {}", n, dir, self.player.get_pos());
                self.player.add_pos(-dir);
            }
        }
        //println!("Update: {}", self.update_fps.tick());
    }
}
