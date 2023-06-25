/* Game objects and their structures */

use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Player<'a> {
    /* Player Object */
    texture: &'a Texture<'a>,
    speed: i32,
    srcrect: Rect,
    pub dstrect: Rect,
}

impl<'a> Player<'a> {
    pub const fn new(
        texture: &'a Texture<'a>,
        speed: i32,
        srcrect: Rect,
        dstrect: Rect,
    ) -> Player<'a> {
        /* Create new player */
        Player {
            texture,
            speed,
            srcrect,
            dstrect,
        }
    }

    pub const fn get_texture(&self) -> &Texture<'a> {
        /* Get player's texture */
        self.texture
    }

    pub const fn get_speed(&self) -> i32 {
        /* Get player's speed */
        self.speed
    }

    pub const fn get_srcrect(&self) -> Rect {
        /* Get player's source rectangle */
        self.srcrect
    }
}

pub struct Object<'a> {
    /* Generic Object */
    pub texture: &'a Texture<'a>,
    pub srcrect: Rect,
    pub dstrect: Rect,
}
