extern crate graphics;
extern crate input;
extern crate vecmath;

use piston::RenderArgs;
use piston::UpdateArgs;
use player::Player;
use self::vecmath::*;
use collider::AABB;

use render::{Render, RenderState};
use input::{Input, Signal};

pub struct Game {
    pub render: Render,
    pub input: Input,
    pub player: Player,
    pub timestamp: f64,
    pub top_wall: AABB,
    pub bottom_wall: AABB,
    pub left_wall: AABB,
    pub right_wall: AABB
}

impl Game {
    pub fn render(&mut self, _: &RenderArgs) {
        let state = RenderState {
            enable_alpha: true,
            clear: Some([0.0, ..4]),
        };

        self.render.state_push(state);
        self.render.draw(&self.top_wall);
        self.render.draw(&self.bottom_wall);
        self.render.draw(&self.left_wall);
        self.render.draw(&self.right_wall);
        self.render.draw(&self.player);
        self.render.state_pop();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.add_rotation(0.0 * args.dt);

        let x = self.input.get_signal(Signal::AxisX);
        let y = self.input.get_signal(Signal::AxisY);

        let dir = if x == 0.0 && y == 0.0 {
            [0.0, 0.0]
        } else {
            vec2_normalized([x, y])
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
                self.player.add_pos([-dir[0], -dir[1]]);
            }
        }

    }
}
