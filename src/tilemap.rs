extern crate graphics;
extern crate quack;

use std::vec::Vec;

use self::quack::Set;
use cgmath::*;
use opengl_graphics::Texture;
use graphics::{Image, SrcRect, RelativeTransform};

use render::{Render, Draw};

#[allow(dead_code)]
enum Tiles {
    TileBrickFloor,
    TileBrickRoomFloor,
    TileStoneWall,
    TileMetalFloor,
    TileCorridor,
    TileDoor,
    TileUpStairs,
    TileDownStairs,
    TileChest
}

pub struct TileMap {
    pub tileset: Texture,
    pub map: Vec<u32>,
    pub tiles: Vec<Image>,

    x_size: u32,
    y_size: u32,
}

impl TileMap {
    pub fn from_tileset_path(path: &Path) -> TileMap {
        TileMap {
            // Initialize the tileset
            tileset: Texture::from_path(path).unwrap(),

            // Initialize the tiles
            tiles: Vec::new(),

            // Initialize common values
            x_size: 0,
            y_size: 0,

            // Initialize the vector of map
            map: Vec::new(),
        }
    }

    pub fn build_procedural_map(&mut self, width: u32, height: u32) -> Vec<u32>
    {
        // Create the map
        self.x_size = width;
        self.y_size = height;

        // Ugly, try to use an iterator rather then line/column loop algorithm
        // Create the borders and fill the midle with dirt
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                //Making the borders of unwalkable walls
                if y == 0 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                else if y == self.y_size - 1 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                else if x == 0 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                else if x == self.x_size - 1 {
                    self.map.push(Tiles::TileStoneWall as u32);
                }
                //Fill the rest with floor or dirt
                else {
                    self.map.push(Tiles::TileBrickFloor as u32);
                }
            }
        }

        // Ugly, try to return a reference instead
        self.map.clone()
    }

}

impl Draw for TileMap {
    fn draw(&self, at: &Vector2<f32>, render: &mut Render) {

        // Ugly, try to use an iterator rather then line/column loop algorithm
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let px = at.x + (x * 40) as f32;
                let py = at.y + (y * 40) as f32;
                let sprite_context = &render.ctx.trans(px as f64, py as f64);
                Image::new().set(SrcRect([0, (self.map[(x + self.x_size * y) as usize] as i32) * 40, 40, 40])).draw(&self.tileset, sprite_context, &mut render.gl);
            }
        }
    }
}

