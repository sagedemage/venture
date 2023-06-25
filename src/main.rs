extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::{event, image, keyboard, mixer, pixels, rect, render, video};
use std::{path, time};

mod object;
mod physics;

const LEVEL_WIDTH: u32 = 1280;
const LEVEL_HEIGHT: u32 = 720;
const PLAYER_WIDTH: u32 = 20;
const PLAYER_HEIGHT: u32 = 20;
const TREE_WIDTH: u32 = 40;
const TREE_HEIGHT: u32 = 40;

fn main() -> Result<(), String> {
    /* Run the Game */
    // Variables
    let level_width: i32 = LEVEL_WIDTH as i32;
    let level_height: i32 = LEVEL_HEIGHT as i32;
    let player_width: i32 = PLAYER_WIDTH as i32;
    let player_height: i32 = PLAYER_HEIGHT as i32;
    let tree_width: i32 = TREE_WIDTH as i32;
    let tree_height: i32 = TREE_HEIGHT as i32;

    let chunksize: i32 = 1024;
    let volume: i32 = 64; // 128 is max
    let fps: u32 = 60;
    let player_speed: i32 = 2;
    let player_image_path: &path::Path = path::Path::new("assets/art/player.png");
    let tree_image_path: &path::Path = path::Path::new("assets/art/tree.png");
    let theme_music_path: &path::Path = path::Path::new("assets/music/cool.ogg");

    let sdl: sdl2::Sdl = sdl2::init()?;
    let _audio: sdl2::AudioSubsystem = sdl.audio()?;

    let video_subsystem: sdl2::VideoSubsystem = sdl.video()?;
    let _image_context: image::Sdl2ImageContext = image::init(image::InitFlag::PNG)?;

    mixer::open_audio(
        mixer::DEFAULT_FREQUENCY,
        mixer::DEFAULT_FORMAT,
        mixer::DEFAULT_CHANNELS,
        chunksize,
    )?;

    let _mixer_contennt: mixer::Sdl2MixerContext = mixer::init(mixer::InitFlag::OGG)?;

    mixer::allocate_channels(4);
    mixer::Music::set_volume(volume);

    let window: video::Window = video_subsystem
        .window("venture", LEVEL_WIDTH, LEVEL_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    /* Set the renderer to use hardware acceleration.
     * This improves the game's performance (FPS) */
    let mut canvas: render::Canvas<video::Window> = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    // texture loader
    let texture_creator: render::TextureCreator<sdl2::video::WindowContext> =
        canvas.texture_creator();
    let player_texture: render::Texture<'_> = texture_creator.load_texture(player_image_path)?;

    let player_srcrect: rect::Rect = rect::Rect::new(0, 0, PLAYER_WIDTH, PLAYER_HEIGHT);
    let player_dstrect: rect::Rect = rect::Rect::new(
        level_width / 4 - player_width / 2,
        level_height / 2 - player_height / 2,
        PLAYER_WIDTH,
        PLAYER_HEIGHT,
    );

    let mut player: object::Player<'_> = object::Player::new(
        &player_texture,
        player_speed,
        player_srcrect,
        player_dstrect,
    );

    /* Tree Objects */
    let tree_texture: render::Texture<'_> = texture_creator.load_texture(tree_image_path)?;

    // trees near the top of the window
    let tree1: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 8 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree2: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 4 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree3: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 8 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree4: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 2 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree5: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 5 / 8 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree6: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 4 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree7: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 7 / 8 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    // trees near the bottom of the window
    let tree8: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree9: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 4 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree10: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree11: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 2 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree12: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 5 / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree13: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 4 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let tree14: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 7 / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    /* Plays the music theme forever */
    let music: mixer::Music<'_> = mixer::Music::from_file(theme_music_path)?;
    music.play(-1)?;

    /* Game loop */
    'running: loop {
        /* Click Keybindings */
        for event in sdl.event_pump()?.poll_iter() {
            match event {
                event::Event::Quit { .. }
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
        if event
            .keyboard_state()
            .is_scancode_pressed(keyboard::Scancode::Right)
        {
            // player moves right
            player.dstrect.x += player.get_speed();
        }
        if event
            .keyboard_state()
            .is_scancode_pressed(keyboard::Scancode::Left)
        {
            // player moves left
            player.dstrect.x -= player.get_speed();
        }
        if event
            .keyboard_state()
            .is_scancode_pressed(keyboard::Scancode::Up)
        {
            // player moves up
            player.dstrect.y -= player.get_speed();
        }
        if event
            .keyboard_state()
            .is_scancode_pressed(keyboard::Scancode::Down)
        {
            // player moves down
            player.dstrect.y += player.get_speed();
        }

        /* Object collision */
        physics::activate_collision(&mut player, &tree1);
        physics::activate_collision(&mut player, &tree2);
        physics::activate_collision(&mut player, &tree3);
        physics::activate_collision(&mut player, &tree4);
        physics::activate_collision(&mut player, &tree5);
        physics::activate_collision(&mut player, &tree6);
        physics::activate_collision(&mut player, &tree7);
        physics::activate_collision(&mut player, &tree8);
        physics::activate_collision(&mut player, &tree9);
        physics::activate_collision(&mut player, &tree10);
        physics::activate_collision(&mut player, &tree11);
        physics::activate_collision(&mut player, &tree12);
        physics::activate_collision(&mut player, &tree13);
        physics::activate_collision(&mut player, &tree14);

        /* Player boundaries */
        if player.dstrect.x < 0 {
            // left boundary
            player.dstrect.x = 0;
        }
        if player.dstrect.x + player.dstrect.w > level_width {
            // right boundary
            player.dstrect.x = level_width - player.dstrect.w;
        }
        if player.dstrect.y + player.dstrect.h > level_height {
            // bottom boundary
            player.dstrect.y = level_height - player.dstrect.h;
        }
        if player.dstrect.y < 0 {
            // top boundary
            player.dstrect.y = 0;
        }

        /* Canvas renders the textures and background */
        canvas.set_draw_color(pixels::Color::RGB(134, 191, 255));
        canvas.clear();
        canvas.copy(player.get_texture(), player.get_srcrect(), player.dstrect)?;
        canvas.copy(tree1.texture, tree1.srcrect, tree1.dstrect)?;
        canvas.copy(tree2.texture, tree2.srcrect, tree2.dstrect)?;
        canvas.copy(tree3.texture, tree3.srcrect, tree3.dstrect)?;
        canvas.copy(tree4.texture, tree4.srcrect, tree4.dstrect)?;
        canvas.copy(tree5.texture, tree5.srcrect, tree5.dstrect)?;
        canvas.copy(tree6.texture, tree6.srcrect, tree6.dstrect)?;
        canvas.copy(tree7.texture, tree7.srcrect, tree7.dstrect)?;
        canvas.copy(tree8.texture, tree8.srcrect, tree8.dstrect)?;
        canvas.copy(tree9.texture, tree9.srcrect, tree9.dstrect)?;
        canvas.copy(tree10.texture, tree10.srcrect, tree10.dstrect)?;
        canvas.copy(tree11.texture, tree11.srcrect, tree11.dstrect)?;
        canvas.copy(tree12.texture, tree12.srcrect, tree12.dstrect)?;
        canvas.copy(tree13.texture, tree13.srcrect, tree13.dstrect)?;
        canvas.copy(tree14.texture, tree14.srcrect, tree14.dstrect)?;
        canvas.present();
        std::thread::sleep(time::Duration::new(0, 1_000_000_000u32 / fps));
    }

    Ok(())
}
