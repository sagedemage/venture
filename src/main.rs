extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::image::{self, LoadTexture};
use std::time::Duration;
use std::path::Path;
use sdl2::mixer;

const LEVEL_WIDTH: u32 = 750;
const LEVEL_HEIGHT: u32 = 500;

pub fn run(player_image_path: &Path, theme_music_path: &Path) -> Result<(), String> {
    let sdl = sdl2::init()?;
    let _audio = sdl.audio();

    let video_subsystem = sdl.video()?;
    let _image_context = image::init(image::InitFlag::PNG)?;

    let chunksize: i32 = 1024;
    let volume: i32 = 64; // 128 is max

    mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, mixer::DEFAULT_CHANNELS, chunksize)?;

    let _mixer_contennt = mixer::init(mixer::InitFlag::OGG)?;

    mixer::allocate_channels(4);
    mixer::Music::set_volume(volume);

    let window = video_subsystem
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
    let texture_creator = canvas.texture_creator();
    let player_texture = texture_creator.load_texture(player_image_path)?;

    let player_srcrect = Rect::new(0, 0, 50, 50);
    let mut player_dstrect = Rect::new(0, 0, 50, 50);

    let music = mixer::Music::from_file(theme_music_path)?;
    music.play(-1)?;

    /* Game loop */
    'running: loop {
        // Click Keybindings
        for event in sdl.event_pump()?.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { 
                    keycode: Option::Some(Keycode::Escape),
                    .. 
                } => break 'running,
                _ => {}
            }
        }

        /* Hold Keybindings */
        let event = sdl.event_pump().unwrap();

        if event.keyboard_state().is_scancode_pressed(Scancode::Right) {
            player_dstrect.x += 2;
        }
        if event.keyboard_state().is_scancode_pressed(Scancode::Left) {
            player_dstrect.x -= 2;
        }
        if event.keyboard_state().is_scancode_pressed(Scancode::Up) {
            player_dstrect.y -= 2;
        }
        if event.keyboard_state().is_scancode_pressed(Scancode::Down) {
            player_dstrect.y += 2;
        }

        /* Player boundaries */
        if player_dstrect.x < 0 {
            // left boundary
            player_dstrect.x = 0;
        }
        if player_dstrect.x + player_dstrect.w > LEVEL_WIDTH as i32 {
            // right boundary
            player_dstrect.x = LEVEL_WIDTH as i32 - player_dstrect.w;
        }
        if player_dstrect.y + player_dstrect.h > LEVEL_HEIGHT as i32 {
            // bottom boundary
            player_dstrect.y = LEVEL_HEIGHT as i32 - player_dstrect.h;
        }
        if player_dstrect.y < 0 {
            // top boundary
            player_dstrect.y = 0;
        }

        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(134, 191, 255));
        canvas.clear();
        canvas.copy(&player_texture, player_srcrect, player_dstrect)?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    // image path
    let player_image_path: &Path = Path::new("assets/art/player.png");
    let theme_music_path: &Path = Path::new("assets/music/cool.ogg");
    run(player_image_path, theme_music_path)?;
    Ok(())
}
