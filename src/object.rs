/* Game objects and their structures */

use sdl2::{render, rect};

pub struct Player<'a> {
    /* Player Object */
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
    pub speed: i32
}

pub struct Object<'a> {
    /* Generic Object */
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}