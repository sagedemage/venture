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
    pub fn new(player_texture: render::Texture<'_>, player_speed: i32, player_srcrect: rect::Rect, player_dstrect: rect::Rect) -> Player<'_> {
        /* Create new player */
        Player {
            texture: player_texture,
            speed: player_speed,
            srcrect: player_srcrect,
            dstrect: player_dstrect
        }
    }

    pub const fn get_texture(&self) -> &render::Texture<'_> {
        /* Get player's texture */
        &self.texture
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
    pub texture: render::Texture<'a>,
    pub srcrect: rect::Rect,
    pub dstrect: rect::Rect,
}