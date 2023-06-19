use sdl2::render;
use sdl2::rect;

pub struct Object<'a> {
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}