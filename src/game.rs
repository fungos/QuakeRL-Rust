use cgmath::*;
use render::{Render, RenderState};
use input::{Input, Signal};
use player::Player;
use tilemap::TileMap;

pub struct Game {
    pub render: Render,
    pub input: Input,
    pub player: Player,
    pub tilemap: TileMap,
    pub timestamp: f64,
    pub top_wall: Aabb2<f32>,
    pub bottom_wall: Aabb2<f32>,
    pub left_wall: Aabb2<f32>,
    pub right_wall: Aabb2<f32>
}

impl Game {
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
        self.render.draw(&self.player, &self.player.get_pos());
        self.render.state_pop();
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
        ];
        for t in walls.iter() {
            let &(w, _/*n*/) = t;
            if self.player.intersect(w) {
                //println!("collided with {} - {} - {}", n, dir, self.player.get_pos());
                self.player.add_pos(-dir);
            }
        }

    }
}
