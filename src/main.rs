extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use std::path::Path;

pub fn run(scene_image_file: &Path, bean_image_file: &Path) -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("sdl2-rust-experiment", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    //canvas.clear();
    //canvas.present();

    // load textures
    let texture_creator = canvas.texture_creator();
    let scene_texture = texture_creator.load_texture(scene_image_file)?;
    let bean_texture = texture_creator.load_texture(bean_image_file)?;

    // rectangles
    // x, y, width, height
    let scene_srcrect = Rect::new(0, 0, 1754, 1240);
    let scene_dstrect = Rect::new(0, 0, 1754, 1240);

    let bean_srcrect = Rect::new(0, 0, 51, 85);
    let bean_dstrect = Rect::new(0, 0, 51, 85);

    // render texture
    // &mut self, texture: &Texture<'_>, src: R1, dst: R2

    canvas.copy(&scene_texture, scene_srcrect, scene_dstrect)?;
    canvas.copy(&bean_texture, bean_srcrect, bean_dstrect)?;
    canvas.present();

    /* Game loop */
    'running: loop {
        //canvas.clear();

        // Keybindings
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { 
                    keycode: Option::Some(Keycode::Escape),
                    .. 
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn main() -> Result<(), String> {
    // image path
    let scene_image_file = Path::new("assets/scene.png");
    let bean_image_file = Path::new("assets/bean.png");
    run(scene_image_file, bean_image_file)?;
    Ok(())
}
