extern crate sdl2;

use sdl2::{event, keyboard, pixels, rect, render, video, mixer};
use sdl2::image::{self, LoadTexture};
use std::{time, path};

mod object;
mod physics;

const LEVEL_WIDTH: i32 = 600;
const LEVEL_HEIGHT: i32 = 400;
const PLAYER_WIDTH: i32 = 20;
const PLAYER_HEIGHT: i32 = 20;
const TREE_WIDTH: i32 = 40;
const TREE_HEIGHT: i32 = 40;

fn main() -> Result<(), String> {
    /* Run the Game */
    // Variables
    let chunksize: i32 = 1024;
    let volume: i32 = 64; // 128 is max
    let fps: u32 = 60;
    let player_image_path: &path::Path = path::Path::new("assets/art/player.png");
    let tree_image_path: &path::Path = path::Path::new("assets/art/tree.png");
    let theme_music_path: &path::Path = path::Path::new("assets/music/cool.ogg");

    let sdl: sdl2::Sdl = sdl2::init()?;
    let _audio: sdl2::AudioSubsystem = sdl.audio()?;

    let video_subsystem: sdl2::VideoSubsystem = sdl.video()?;
    let _image_context: image::Sdl2ImageContext = image::init(image::InitFlag::PNG)?;

    mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, chunksize)?;

    let _mixer_contennt: mixer::Sdl2MixerContext = mixer::init(mixer::InitFlag::OGG)?;

    mixer::allocate_channels(4);
    mixer::Music::set_volume(volume);

    let window: video::Window = video_subsystem
        .window("venture", LEVEL_WIDTH as u32, LEVEL_HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: render::Canvas<video::Window> = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    // load textures
    let texture_creator: render::TextureCreator<sdl2::video::WindowContext> = canvas.texture_creator();

    let mut player: object::Player<'_> = object::Player {
        texture: texture_creator.load_texture(player_image_path)?,
        srcrect: rect::Rect::new(0, 0, PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32),
        dstrect: rect::Rect::new(LEVEL_WIDTH / 4 - PLAYER_WIDTH / 2, LEVEL_HEIGHT / 2 - PLAYER_HEIGHT / 2, PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32),
        speed: 2,
    };

    /* Tree Objects */
    // trees near the top of the window
    let tree1: object::Object<'_> = object::Object {
        texture: texture_creator.load_texture(tree_image_path)?,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH as u32, TREE_HEIGHT as u32),
        dstrect: rect::Rect::new(LEVEL_WIDTH / 4 - TREE_WIDTH / 2, LEVEL_HEIGHT / 4 - TREE_HEIGHT / 2, TREE_WIDTH as u32, TREE_HEIGHT as u32),
    };

    let tree2: object::Object<'_> = object::Object {
        texture: texture_creator.load_texture(tree_image_path)?,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH as u32, TREE_HEIGHT as u32),
        dstrect: rect::Rect::new(LEVEL_WIDTH / 2 - TREE_WIDTH / 2, LEVEL_HEIGHT / 4 - TREE_HEIGHT / 2, TREE_WIDTH as u32, TREE_HEIGHT as u32),
    };

    let tree3: object::Object<'_> = object::Object {
        texture: texture_creator.load_texture(tree_image_path)?,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH as u32, TREE_HEIGHT as u32),
        dstrect: rect::Rect::new(LEVEL_WIDTH * 3 / 4 - TREE_WIDTH / 2, LEVEL_HEIGHT / 4 - TREE_HEIGHT / 2, TREE_WIDTH as u32, TREE_HEIGHT as u32),
    };

    // trees near the bottom of the window
    let tree4: object::Object<'_> = object::Object {
        texture: texture_creator.load_texture(tree_image_path)?,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH as u32, TREE_HEIGHT as u32),
        dstrect: rect::Rect::new(LEVEL_WIDTH / 4 -TREE_WIDTH / 2, LEVEL_HEIGHT * 3 / 4 - TREE_HEIGHT / 2, TREE_WIDTH as u32, TREE_HEIGHT as u32),
    };

    let tree5: object::Object<'_> = object::Object {
        texture: texture_creator.load_texture(tree_image_path)?,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH as u32, TREE_HEIGHT as u32),
        dstrect: rect::Rect::new(LEVEL_WIDTH / 2 - TREE_WIDTH / 2, LEVEL_HEIGHT * 3 / 4 - TREE_HEIGHT / 2, TREE_WIDTH as u32, TREE_HEIGHT as u32),
    };

    let tree6: object::Object<'_> = object::Object {
        texture: texture_creator.load_texture(tree_image_path)?,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH as u32, TREE_HEIGHT as u32),
        dstrect: rect::Rect::new(LEVEL_WIDTH * 3 / 4 - TREE_WIDTH / 2, LEVEL_HEIGHT * 3 / 4 - TREE_HEIGHT / 2, TREE_WIDTH as u32, TREE_HEIGHT as u32),
    };

    /* Plays the music theme forever */
    let music: mixer::Music<'_> = mixer::Music::from_file(theme_music_path)?;
    music.play(-1)?;

    /* Game loop */
    'running: loop {
        /* Click Keybindings */
        for event in sdl.event_pump()?.poll_iter() {
            match event {
                event::Event::Quit {..} 
                | event::Event::KeyDown { 
                    keycode: Option::Some(keyboard::Keycode::Escape),
                    .. 
                } => break 'running,
                _ => {}
            }
        }

        /* Hold Keybindings */
        let event: sdl2::EventPump = sdl.event_pump()?;

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

        /* Object collision */
        physics::activate_collision(&mut player, &tree1);
        physics::activate_collision(&mut player, &tree2);
        physics::activate_collision(&mut player, &tree3);
        physics::activate_collision(&mut player, &tree4);
        physics::activate_collision(&mut player, &tree5);
        physics::activate_collision(&mut player, &tree6);

        /* Player boundaries */
        if player.dstrect.x < 0 {
            // left boundary
            player.dstrect.x = 0;
        }
        if player.dstrect.x + player.dstrect.w > LEVEL_WIDTH {
            // right boundary
            player.dstrect.x = LEVEL_WIDTH - player.dstrect.w;
        }
        if player.dstrect.y + player.dstrect.h > LEVEL_HEIGHT {
            // bottom boundary
            player.dstrect.y = LEVEL_HEIGHT - player.dstrect.h;
        }
        if player.dstrect.y < 0 {
            // top boundary
            player.dstrect.y = 0;
        }

        /* Canvas renders the textures and background */
        canvas.set_draw_color(pixels::Color::RGB(134, 191, 255));
        canvas.clear();
        canvas.copy(&player.texture, player.srcrect, player.dstrect)?;
        canvas.copy(&tree1.texture, tree1.srcrect, tree1.dstrect)?;
        canvas.copy(&tree2.texture, tree2.srcrect, tree2.dstrect)?;
        canvas.copy(&tree3.texture, tree3.srcrect, tree3.dstrect)?;
        canvas.copy(&tree4.texture, tree4.srcrect, tree4.dstrect)?;
        canvas.copy(&tree5.texture, tree5.srcrect, tree5.dstrect)?;
        canvas.copy(&tree6.texture, tree6.srcrect, tree6.dstrect)?;
        canvas.present();
        std::thread::sleep(time::Duration::new(0, 1_000_000_000u32 / fps));
    }

    Ok(())
}
