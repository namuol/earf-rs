extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::JPG);

    // Why do I need to provide type annotation here?
    let heightmap_surface: sdl2::surface::Surface =
        sdl2::image::LoadSurface::from_file(Path::new("heightmap.jpg")).unwrap();
    let colormap_surface: sdl2::surface::Surface =
        sdl2::image::LoadSurface::from_file(Path::new("colormap.jpg")).unwrap();

    let window = video_subsystem
        .window("earf-rs", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let heightmap = texture_creator
        .create_texture_from_surface(heightmap_surface)
        .unwrap();
    let colormap = texture_creator
        .create_texture_from_surface(colormap_surface)
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.copy(&colormap, None, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
