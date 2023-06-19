/* Game objects and their structures */

use sdl2::render;
use sdl2::rect;

pub struct Player<'a> {
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
    pub speed: i32
}

pub struct Object<'a> {
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}