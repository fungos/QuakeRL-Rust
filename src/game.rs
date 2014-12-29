extern crate graphics;
extern crate input;

use self::graphics::*;
use self::input::{Button, Key};

use piston::RenderArgs;
use piston::UpdateArgs;
use event::Window;
use player::Player;
use render::Render;

pub struct Game {
    pub render: Render,
    pub player: Player,
    pub rotation: f64,
}

impl Game {
    pub fn render<W: Window>(&mut self, _: &mut W, _: &RenderArgs) {
        // Clear the screen
        graphics::clear([1.0, ..4], &mut self.render.gl);

        // Draw player actor
        self.render.draw(&self.player.sprite);//, self.rotation);
    }

    pub fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    pub fn press(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up)       => { self.player.sprite.y -= 10.0 },
            Button::Keyboard(Key::Down)     => { self.player.sprite.y += 10.0 },
            Button::Keyboard(Key::Left)     => { self.player.sprite.x -= 10.0 },
            Button::Keyboard(Key::Right)    => { self.player.sprite.x += 10.0 },
            _ => {}
        }
    }
}
