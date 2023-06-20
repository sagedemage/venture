/* Game objects and their structures */

use sdl2::{render, rect};

pub struct Player<'a> {
    /* Player Object */
    texture: render::Texture<'a>,
    speed: i32,
    srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}

impl Player<'_> {
    pub fn new(texture: render::Texture<'_>, speed: i32, srcrect: rect::Rect, dstrect: rect::Rect) -> Player<'_> {
        Player {
            texture: texture,
            speed: speed,
            srcrect: srcrect,
            dstrect: dstrect
        }
    }

    pub fn get_texture(&self) -> &render::Texture<'_> {
        &self.texture
    }

    pub fn get_speed(&self) -> i32 {
        self.speed
    }

    pub fn get_srcrect(&self) -> rect::Rect {
        self.srcrect
    }
}

pub struct Object<'a> {
    /* Generic Object */
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}