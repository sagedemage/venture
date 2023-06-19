/*
texture: Texture<'_>
srcrect: Rect
dstrect: Rect
*/

use sdl2::render;
use sdl2::rect;

pub struct Player<'a> {
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
    pub speed: i32
}