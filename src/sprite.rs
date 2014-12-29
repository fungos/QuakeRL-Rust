extern crate graphics;

use opengl_graphics::Texture;
use render::{Render, Draw};

use opengl_graphics::Gl;
use piston::graphics::*;

// abstract this to an actor
pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub image: Texture,
}

impl Sprite {
    pub fn from_path(path: &Path) -> Sprite {
        let image = Texture::from_path(path).unwrap();
        Sprite {
            x: 0.0,
            y: 0.0,
            image: image,
        }
    }
}

pub impl Draw for Sprite {
    //pub fn draw(&mut self, sprite: &Sprite, rotation: f64) {
    pub fn draw(&self, render: &mut Render) {
        // find a way to get w/h from texture private width/height
        let w = 40f64;
        let h = 40f64;
        let hw = w / 2.0f64;
        let hh = h / 2.0f64;

        // Draw the player
        let sprite_context = &render.ctx
            .trans(self.x - hw, self.y - hh)
            //.rot_rad(rotation)
            .trans(-hw, -hh)
        ;

        if cfg!(feature="debug_sprite") {
            // add border to sprite so we can debug it as we do not have a nice bg yet
            Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([-1.0, -1.0, w + 2.0, h + 2.0], sprite_context, &mut render.gl);
        }

        graphics::image(&self.image, sprite_context, &mut render.gl);

        if cfg!(feature="debug_sprite") {
            let sprite_context = &render.ctx.trans(self.x - hw, self.y - hh);
            Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-2.0, -2.0, 5.0, 5.0], sprite_context, &mut render.gl);
        }
    }
}
