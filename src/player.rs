use std::num::Float;

use cgmath::*;
use graphics::ImageSize;
use opengl_graphics::Texture;

use render::{Render, Draw, draw_texture};

pub struct Player {
    pub image: Texture,
    aabb: Aabb2<f32>,
}

impl Player {

    #[allow(dead_code)]
    #[inline]
    pub fn get_pos(&self) -> Vector2<f32> {
        self.aabb.center().to_vec()
    }

    #[inline]
    pub fn set_pos(&mut self, pos: Vector2<f32>) {
        let diff = pos - self.get_pos();
        self.add_pos(diff);
    }

    #[inline]
    pub fn add_pos(&mut self, pos: Vector2<f32>) {
        self.aabb = self.aabb.add_v(&pos);
    }

    #[inline]
    pub fn intersect(&self, other: &Aabb2<f32>) -> bool {
        let min = self.aabb.min();
        let max = self.aabb.max();

        other.contains(min) || other.contains(max) || other.contains(&Point2::new(min.x, max.y)) || other.contains(&Point2::new(max.x, min.y))
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
    #[inline]
    fn draw(&self, at: &Vector2<f32>, render: &mut Render) {
        draw_texture(&self.image, at, render);
        render.draw(&self.aabb, at);
    }
}
