extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::image::{self, LoadTexture};
use sdl2::render;
use sdl2::video;
use std::time::Duration;
use std::path::Path;
use sdl2::mixer;

mod player;
mod object;
use player::Player;
use object::Object;

const LEVEL_WIDTH: u32 = 750;
const LEVEL_HEIGHT: u32 = 500;

fn activate_collision(player: &mut Player<'_>, object: &Object<'_>) {
    /* Object collision */
    let vertex_gap: i32 = 2;
    if player.dstrect.y < object.dstrect.y + 50 - vertex_gap && player.dstrect.y + 50 > object.dstrect.y + vertex_gap {
        if player.dstrect.x + 50 > object.dstrect.x && player.dstrect.x < object.dstrect.x + 25 {
            // left collision
            player.dstrect.x = object.dstrect.x - 50; 
        }
        else if player.dstrect.x < object.dstrect.x + 50 && player.dstrect.x > object.dstrect.x + 25 {
            // right collision
            player.dstrect.x = object.dstrect.x + 50; 
        }
    }

    else if player.dstrect.x < object.dstrect.x - vertex_gap + 50 && player.dstrect.x + 50 > object.dstrect.x + vertex_gap {
        if player.dstrect.y + 50 > object.dstrect.y && player.dstrect.y < object.dstrect.y + 25 {
            // top collision
            player.dstrect.y = object.dstrect.y - 50;
        }
        else if player.dstrect.y < object.dstrect.y + 50 && player.dstrect.y > object.dstrect.y + 25 {
            // bottom collision
            player.dstrect.y = object.dstrect.y + 50;
        }    
    }
    //return player.dstrect;

}

pub fn run(player_image_path: &Path, tree_image_path: &Path, theme_music_path: &Path) -> Result<(), String> {
    let sdl: sdl2::Sdl = sdl2::init()?;
    let _audio: sdl2::AudioSubsystem = sdl.audio()?;

    let video_subsystem: sdl2::VideoSubsystem = sdl.video()?;
    let _image_context: image::Sdl2ImageContext = image::init(image::InitFlag::PNG)?;

    let chunksize: i32 = 1024;
    let volume: i32 = 64; // 128 is max

    mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, chunksize)?;

    let _mixer_contennt: mixer::Sdl2MixerContext = mixer::init(mixer::InitFlag::OGG)?;

    mixer::allocate_channels(4);
    mixer::Music::set_volume(volume);

    let window: video::Window = video_subsystem
        .window("venture", LEVEL_WIDTH, LEVEL_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    // load textures
    let texture_creator: render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();

    let mut player: Player<'_> = Player {
        texture: texture_creator.load_texture(player_image_path)?,
        srcrect: rect::Rect::new(0, 0, 50, 50),
        dstrect: rect::Rect::new(25, LEVEL_HEIGHT as i32 / 2 - 25, 50, 50),
        speed: 2,
    };

    let tree: Object<'_> = Object {
        texture: texture_creator.load_texture(tree_image_path)?,
        srcrect: rect::Rect::new(0, 0, 50, 50),
        dstrect: rect::Rect::new(LEVEL_WIDTH as i32 / 2 - 25, LEVEL_HEIGHT as i32 / 2 - 25, 50, 50),
    };

    let music: mixer::Music<'_> = mixer::Music::from_file(theme_music_path)?;
    music.play(-1)?;

    /* Game loop */
    'running: loop {
        // Click Keybindings
        for event in sdl.event_pump()?.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { 
                    keycode: Option::Some(keyboard::Keycode::Escape),
                    .. 
                } => break 'running,
                _ => {}
            }
        }

        /* Hold Keybindings */
        let event: sdl2::EventPump = sdl.event_pump().unwrap();

        /* Player Movement Keybindings */
        if event.keyboard_state().is_scancode_pressed(keyboard::Scancode::Right) {
            // player moves right
            player.dstrect.x += player.speed;
        }
        if event.keyboard_state().is_scancode_pressed(keyboard::Scancode::Left) {
            // player moves left
            player.dstrect.x -= player.speed;
        }
        if event.keyboard_state().is_scancode_pressed(keyboard::Scancode::Up) {
            // player moves up
            player.dstrect.y -= player.speed;
        }
        if event.keyboard_state().is_scancode_pressed(keyboard::Scancode::Down) {
            // player moves down
            player.dstrect.y += player.speed;
        }

        //let vertex_gap: i32 = 2;

        /* Object collision */
        /*
        if player.dstrect.y < tree.dstrect.y + 50 - vertex_gap && player.dstrect.y + 50 > tree.dstrect.y + vertex_gap {
            if player.dstrect.x + 50 > tree.dstrect.x && player.dstrect.x < tree.dstrect.x + 25 {
                // left collision
                player.dstrect.x = tree.dstrect.x - 50; 
            }
            else if player.dstrect.x < tree.dstrect.x + 50 && player.dstrect.x > tree.dstrect.x + 25 {
                // right collision
                player.dstrect.x = tree.dstrect.x + 50; 
            }
        }

        else if player.dstrect.x < tree.dstrect.x - vertex_gap + 50 && player.dstrect.x + 50 > tree.dstrect.x + vertex_gap {
            if player.dstrect.y + 50 > tree.dstrect.y && player.dstrect.y < tree.dstrect.y + 25 {
                // top collision
                player.dstrect.y = tree.dstrect.y - 50;
            }
            else if player.dstrect.y < tree.dstrect.y + 50 && player.dstrect.y > tree.dstrect.y + 25 {
                // bottom collision
                player.dstrect.y = tree.dstrect.y + 50;
            }    
        }
        */

        activate_collision(&mut player, &tree);

        /* Player boundaries */
        if player.dstrect.x < 0 {
            // left boundary
            player.dstrect.x = 0;
        }
        if player.dstrect.x + player.dstrect.w > LEVEL_WIDTH as i32 {
            // right boundary
            player.dstrect.x = LEVEL_WIDTH as i32 - player.dstrect.w;
        }
        if player.dstrect.y + player.dstrect.h > LEVEL_HEIGHT as i32 {
            // bottom boundary
            player.dstrect.y = LEVEL_HEIGHT as i32 - player.dstrect.h;
        }
        if player.dstrect.y < 0 {
            // top boundary
            player.dstrect.y = 0;
        }

        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(134, 191, 255));
        canvas.clear();
        canvas.copy(&player.texture, player.srcrect, player.dstrect)?;
        canvas.copy(&tree.texture, tree.srcrect, tree.dstrect)?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    // image path
    let player_image_path: &Path = Path::new("assets/art/player.png");
    let tree_image_path: &Path = Path::new("assets/art/tree.png");
    let theme_music_path: &Path = Path::new("assets/music/cool.ogg");
    run(player_image_path, tree_image_path, theme_music_path)?;
    Ok(())
}
