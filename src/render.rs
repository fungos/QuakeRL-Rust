extern crate graphics;

use shader_version::opengl::OpenGL;
use opengl_graphics::{Gl, Texture};
use texture::ImageSize;
use graphics::{internal, Context, RelativeTransform};
use cgmath::*;

pub struct RenderState {
    pub enable_alpha: bool,
    pub clear: Option<internal::Color>,
}

pub struct Render {
    pub gl: Gl,
    pub ctx: Context, // this context is per object or global?
    pub states: Vec<RenderState>,
}

pub trait Draw {
    fn draw(&self, at: &Vector2<f32>, render: &mut Render);
}

impl Render {
    pub fn new(w: f32, h: f32) -> Render {
        let default = RenderState {
            enable_alpha: false,
            clear: None,
        };

        let mut v = Vec::with_capacity(5);
        v.push(default);

        Render {
            gl: Gl::new(OpenGL::_3_2),
            ctx: Context::abs(w as f64, h as f64),
            states: v,
        }
    }

    #[inline]
    pub fn draw<T: Draw>(&mut self, obj: &T, at: &Vector2<f32>) {
        obj.draw(at, self);
    }

    #[inline]
    pub fn state_push(&mut self, state: RenderState) {
        self.state_apply(&state);
        self.states.push(state);
    }

    #[inline]
    pub fn state_pop(&mut self) -> RenderState {
        assert!(self.states.len() > 1, "Unbalanced push<>pop.");
        let ret = self.states.pop().unwrap();
        let state = self.states.pop().unwrap();

        self.state_apply(&state);
        self.states.push(state);

        ret
    }

    fn state_apply(&mut self, state: &RenderState) {
        match state.enable_alpha {
            true  => self.gl.enable_alpha_blend(),
            false => self.gl.disable_alpha_blend(),
        }

        match state.clear {
            Some(color) => graphics::clear(color, &mut self.gl),
            None => (),
        }
    }
}

pub fn draw_texture(tex: &Texture, at: &Vector2<f32>, render: &mut Render) {
    let (w, h) = tex.get_size();
    let hw = w as f64 / 2.0;
    let hh = h as f64 / 2.0;

    // Draw the player
    let sprite_context = &render.ctx
        .trans(at.x as f64, at.y as f64)
        //.rot_rad(0.0)
        .trans(-hw, -hh)
    ;

    if cfg!(feature="debug_sprite") {
        // add border to sprite so we can debug it as we do not have a nice bg yet
        graphics::Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-1.0, -1.0, w as f64 + 2.0, h as f64 + 2.0], sprite_context, &mut render.gl);
    }

    graphics::image(tex, sprite_context, &mut render.gl);

    if cfg!(feature="debug_sprite") {
        let sprite_context = &render.ctx.trans(at.x as f64, at.y as f64);
        graphics::Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-2.0, -2.0, 5.0, 5.0], sprite_context, &mut render.gl);
    }
}

impl Draw for Aabb2<f32> {
    fn draw(&self, at: &Vector2<f32>, render: &mut Render) {
        use graphics::Line;

        if cfg!(feature="debug_volume") {
            let x = at.x as f64;
            let y = at.y as f64;
            let d = self.dim();
            let w = d.x as f64 / 2.0;
            let h = d.y as f64 / 2.0;

            // Get the context
            let collider_ctx = &render.ctx.trans(x, y);

            // Add border to collider
            let width = 1.0;
            let color = [0.0, 1.0, 0.0, 1.0];
            Line::new(color, width).draw(
                [-w, h, w, h], collider_ctx, &mut render.gl
            );

            Line::new(color, width).draw(
                [-w, -h, w, -h], collider_ctx, &mut render.gl
            );

            Line::new(color, width).draw(
                [w, -h, w, h], collider_ctx, &mut render.gl
            );

            Line::new(color, width).draw(
                [-w, -h, -w, h], collider_ctx, &mut render.gl
            );
        }
    }
}
