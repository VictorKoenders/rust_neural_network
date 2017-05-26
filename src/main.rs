extern crate sdl2;
extern crate rand;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::{Rect};

mod nn;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut rng = rand::thread_rng();
    let mut simulation = nn::Simulation::new(&mut rng);

    let window = video_subsystem.window("Mouse", 800, 600)
        .position_centered()
        .allow_highdpi()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .unwrap();        
    
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();

    canvas.with_texture_canvas(&mut texture, |texture_canvas| {
        texture_canvas.clear();
        texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        texture_canvas.fill_rect(Rect::new(0, 0, 100, 100)).unwrap();
    }).unwrap();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space) , .. } => simulation = nn::Simulation::new(&mut rng),
                _ => {}
            }
        }

        simulation.update(&mut rng);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for source in &simulation.energy_nodes {
            canvas.filled_circle(source.0 as i16, source.1 as i16, 25, Color::RGB(0, 150, 0)).unwrap();
        }

        for node in &simulation.networks {
            canvas.filled_circle(node.x as i16, node.y as i16, 25, Color::RGB(if node.is_charging { 255 } else { 150 }, 0, 0)).unwrap();
            let start = (node.x as i32, node.y as i32);
            let diff = (node.facing.cos(), node.facing.sin());
            let end = (
                (node.x + diff.0 * 30f32) as i32,
                (node.y + diff.1 * 30f32) as i32
            );

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_line(start, end).unwrap();
        }

        canvas.present();
    }
}