#[macro_use]
extern crate impl_ops;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
// use std::time::Duration;

pub mod camera;
pub mod vector;

use crate::camera::Camera;
use crate::vector::Vector;

const SCREEN_WIDTH: u32 = 160 * 2;
const SCREEN_HEIGHT: u32 = 100 * 2;

const SCREEN_SCALE: u32 = 3;

const MAX_D: f64 = 1024.0;
const LOD_FACTOR: u32 = 4;
const DETAIL: u32 = 1;

const FOG_COLOR: RGB = RGB {
    r: 98,
    g: 192,
    b: 255,
};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "earf-rs",
            SCREEN_WIDTH * SCREEN_SCALE,
            SCREEN_HEIGHT * SCREEN_SCALE,
        )
        .position_centered()
        .build()
        .unwrap();

    let _image_context = sdl2::image::init(sdl2::image::InitFlag::JPG);
    // Why do I need to provide type annotation here?
    use sdl2::image::LoadSurface;
    let heightmap_surface: sdl2::surface::Surface =
        LoadSurface::from_file(Path::new("heightmap.jpg")).unwrap();
    let heightmap = Map::new(&heightmap_surface);
    let colormap_surface: sdl2::surface::Surface =
        LoadSurface::from_file(Path::new("colormap.jpg")).unwrap();
    let colormap = Map::new(&colormap_surface);

    let mut cam = Camera::new(
        Vector {
            x: 127.0,
            y: 90.0,
            z: 127.0,
        },
        25.0,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    );
    cam.set_angle(-std::f64::consts::PI);

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut screen_texture = texture_creator
        .create_texture_streaming(
            texture_creator.default_pixel_format(),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .unwrap();
    screen_texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.set_draw_color(FOG_COLOR);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut angle: f64 = 0.0;
    'running: loop {
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

        // cam.set_angle(angle);
        cam.eye.y = 84.0 - 20.0 * (angle * 2.0).sin();
        cam.eye.x = 127.0 + 512.0 * (angle).sin();
        cam.eye.z = 127.0 + 512.0 * (angle * 0.8).cos();
        angle += 0.01;

        canvas.clear();
        screen_texture
            .with_lock(None, |mut screen, _size| {
                cast(&cam, &heightmap, &colormap, &mut screen);
                // let mut x = 0;
                // while x < colormap.width && x < SCREEN_WIDTH {
                //     let mut y = 0;
                //     while y < colormap.height && y < SCREEN_HEIGHT {
                //         let color = colormap.pixel_at(x, y);
                //         let pos = y as usize * size + (x * 4) as usize;
                //         screen[pos] = color.b;
                //         screen[pos + 1] = color.g;
                //         screen[pos + 2] = color.r;
                //         screen[pos + 3] = 255;
                //         y += 1;
                //     }
                //     x += 1;
                // }
            })
            .unwrap();
        canvas.copy(&screen_texture, None, None).unwrap();
        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn cast(cam: &Camera, heightmap: &Map, colormap: &Map, screen: &mut [u8]) {
    let mut x = 0;
    let screen_height = cam.screen_height;
    while x < cam.screen_width {
        let mut max_y: i32 = (cam.screen_height - 1) as i32;
        let ray = cam.get_ray_from_uv(x, 0);
        let mut d: u32 = 15;
        let mut lod = 1;
        while lod < LOD_FACTOR {
            let maxd = (MAX_D as f64) / ((LOD_FACTOR as f64) - (lod as f64));
            while (d as f64) < maxd {
                let cx = (cam.eye.x + ray.x * d as f64).floor() as u32;
                let cz = (cam.eye.z + ray.z * d as f64).floor() as u32;
                let r = heightmap.pixel_at(cx, cz).r;
                let h = r as f64 * 0.25;
                let y = ((screen_height as f64)
                    - (((h - cam.eye.y) * (screen_height as f64 * 2.0)) / (d as f64)
                        + (screen_height as f64)))
                    .floor() as i32;

                if y >= 0 {
                    if y < max_y {
                        let mut current_y: i32 = max_y;
                        let fog = 1.0 - ((d as f64) - 100.0) / (MAX_D - 100.0);
                        let color = colormap.pixel_at(cx, cz);
                        while current_y > y && current_y < (cam.screen_height as i32) {
                            let index: usize = current_y as usize * (cam.screen_width as usize * 4)
                                + (x * 4) as usize;
                            screen[index] = color.b;
                            screen[index + 1] = color.g;
                            screen[index + 2] = color.r;
                            screen[index + 3] = (fog * 255.0).floor() as u8;
                            current_y -= 1;
                        }
                        max_y = y;
                    }
                }

                d += DETAIL * lod;
            }
            lod += 1;
        }
        x += 1;
    }
}

struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Into<Color> for RGB {
    fn into(self) -> Color {
        Color::RGB(self.r, self.g, self.b)
    }
}

struct Map {
    pub width: u32,
    pub height: u32,
    data: Vec<Vec<RGB>>,
}

impl Map {
    pub fn new(surface: &sdl2::surface::Surface) -> Map {
        surface.with_lock(|pixels| {
            let width = surface.width();
            let height = surface.height();
            let pitch = surface.pitch();
            let mut data: Vec<Vec<RGB>> = Vec::with_capacity(height as usize);
            for y in 0..height {
                let mut row: Vec<RGB> = Vec::with_capacity(width as usize);
                for x in 0..width {
                    let pixel_pos = (y * pitch + (x * 3)) as usize;
                    row.push(RGB {
                        r: pixels[pixel_pos],
                        g: pixels[pixel_pos + 1],
                        b: pixels[pixel_pos + 2],
                    });
                }
                data.push(row);
            }

            Map {
                width,
                height,
                data,
            }
        })
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> &RGB {
        &self.data[(y % self.height) as usize][(x % self.width) as usize]
    }
}
