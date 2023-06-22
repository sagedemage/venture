/* Game objects and their structures */

use sdl2::{render, rect};

pub struct Player<'a> {
    /* Player Object */
    texture: &'a render::Texture<'a>,
    speed: i32,
    srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}

impl<'a> Player<'a> {
    pub const fn new(texture: &'a render::Texture<'a>, speed: i32, srcrect: rect::Rect, dstrect: rect::Rect) -> Player<'a> {
        /* Create new player */
        Player {
            texture, 
            speed,
            srcrect,
            dstrect
        }
    }

    pub const fn get_texture(&self) -> &render::Texture<'a> {
        /* Get player's texture */
        self.texture
    }

    pub const fn get_speed(&self) -> i32 {
        /* Get player's speed */
        self.speed
    }

    pub const fn get_srcrect(&self) -> rect::Rect {
        /* Get player's source rectangle */
        self.srcrect
    }
}

pub struct Object<'a> {
    /* Generic Object */
    pub texture: &'a render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}