extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::{event, image, keyboard, mixer, pixels, rect, render, ttf, video};
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
    let font_path = path::Path::new("/usr/share/fonts/truetype/freefont/FreeSans.ttf");

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

    // ttf loader
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    /* Font Message */
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(ttf::FontStyle::BOLD);

    let message_surface = font
        .render("Hello Rust")
        .blended(pixels::Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;

    let message_texture = texture_creator
        .create_texture_from_surface(&message_surface)
        .map_err(|e| e.to_string())?;

    let messege_target = rect::Rect::new(10, 10, 10, 10);

    /* Player */
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
    let first_top_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 8 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let second_top_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 4 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let third_top_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 8 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let fourth_top_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 2 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let fifth_top_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 5 / 8 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let sixth_top_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 4 - tree_width / 2,
            level_height / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let seventh_top_tree: object::Object<'_> = object::Object {
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
    let first_bottom_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let second_bottom_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 4 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let third_bottom_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let fourth_bottom_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width / 2 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let fifth_bottom_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 5 / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let sixth_bottom_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 3 / 4 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let seventh_bottom_tree: object::Object<'_> = object::Object {
        texture: &tree_texture,
        srcrect: rect::Rect::new(0, 0, TREE_WIDTH, TREE_HEIGHT),
        dstrect: rect::Rect::new(
            level_width * 7 / 8 - tree_width / 2,
            level_height * 3 / 4 - tree_height / 2,
            TREE_WIDTH,
            TREE_HEIGHT,
        ),
    };

    let trees: Vec<object::Object> = vec![
        first_top_tree,
        second_top_tree,
        third_top_tree,
        fourth_top_tree,
        fifth_top_tree,
        sixth_top_tree,
        seventh_top_tree,
        first_bottom_tree,
        second_bottom_tree,
        third_bottom_tree,
        fourth_bottom_tree,
        fifth_bottom_tree,
        sixth_bottom_tree,
        seventh_bottom_tree,
    ];

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
        physics::activate_collision(&mut player, &trees[0]);
        physics::activate_collision(&mut player, &trees[1]);
        physics::activate_collision(&mut player, &trees[2]);
        physics::activate_collision(&mut player, &trees[3]);
        physics::activate_collision(&mut player, &trees[4]);
        physics::activate_collision(&mut player, &trees[5]);
        physics::activate_collision(&mut player, &trees[6]);
        physics::activate_collision(&mut player, &trees[7]);
        physics::activate_collision(&mut player, &trees[8]);
        physics::activate_collision(&mut player, &trees[9]);
        physics::activate_collision(&mut player, &trees[10]);
        physics::activate_collision(&mut player, &trees[11]);
        physics::activate_collision(&mut player, &trees[12]);
        physics::activate_collision(&mut player, &trees[13]);

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
        canvas.copy(trees[0].texture, trees[0].srcrect, trees[0].dstrect)?;
        canvas.copy(trees[1].texture, trees[1].srcrect, trees[1].dstrect)?;
        canvas.copy(trees[2].texture, trees[2].srcrect, trees[2].dstrect)?;
        canvas.copy(trees[3].texture, trees[3].srcrect, trees[3].dstrect)?;
        canvas.copy(trees[4].texture, trees[4].srcrect, trees[4].dstrect)?;
        canvas.copy(trees[5].texture, trees[5].srcrect, trees[5].dstrect)?;
        canvas.copy(trees[6].texture, trees[6].srcrect, trees[6].dstrect)?;
        canvas.copy(trees[7].texture, trees[7].srcrect, trees[7].dstrect)?;
        canvas.copy(trees[8].texture, trees[8].srcrect, trees[8].dstrect)?;
        canvas.copy(trees[9].texture, trees[9].srcrect, trees[9].dstrect)?;
        canvas.copy(trees[10].texture, trees[10].srcrect, trees[10].dstrect)?;
        canvas.copy(trees[11].texture, trees[11].srcrect, trees[11].dstrect)?;
        canvas.copy(trees[12].texture, trees[12].srcrect, trees[12].dstrect)?;
        canvas.copy(trees[13].texture, trees[13].srcrect, trees[13].dstrect)?;
        canvas.copy(&message_texture, None, Some(messege_target))?;
        canvas.present();
        std::thread::sleep(time::Duration::new(0, 1_000_000_000u32 / fps));
    }

    Ok(())
}
