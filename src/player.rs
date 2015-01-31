use cgmath::*;
use graphics::ImageSize;
use opengl_graphics::Texture;
use render::{Render, Draw, draw_texture};
use std::num::Float;

pub struct Player {
    pub image: Texture,
    aabb: Aabb2<f32>,
}

impl Player {

    #[allow(dead_code)]
    pub fn get_pos(&self) -> Vector2<f32> {
        self.aabb.center().to_vec()
    }

    pub fn set_pos(&mut self, pos: Vector2<f32>) {
        let diff = pos - self.get_pos();
        self.add_pos(diff);
    }

    pub fn add_pos(&mut self, pos: Vector2<f32>) {
        self.aabb = self.aabb.add_v(&pos);
    }

    pub fn intersect(&self, other: &Aabb2<f32>) -> bool {
        other.contains(self.aabb.min()) || other.contains(self.aabb.max())
    }

    pub fn from_path(path: &Path) -> Player {
        let texture = Texture::from_path(path).unwrap();
        let (w, h) = texture.get_size();

        Player {
            image: texture,
            aabb: Aabb2::new(Point2::new(0.0, 0.0), Point2::new(w as f32, h as f32)),
        }
    }
}

impl Draw for Player {
    fn draw(&self, at: &Vector2<f32>, render: &mut Render) {
        draw_texture(&self.image, at, render);
        render.draw(&self.aabb, at);
    }
}
