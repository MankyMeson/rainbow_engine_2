use sdl2::event::Event;
use sdl2::keyboard::{Keycode,Scancode,KeyboardState};
use sdl2::image::{self,LoadTexture,InitFlag};
use sdl2::pixels::Color;
use std::time::Duration;
use rainbow_engine_2::consts;
use rainbow_engine_2::vector::Vector2;

fn is_pressed(e: &sdl2::EventPump, key: Scancode) -> bool {
    e.keyboard_state().is_scancode_pressed(key)
}

fn inpt_dir(e: &sdl2::EventPump) -> Vector2 {
    let mut dir = Vector2::new(0.,0.);
    dir.x = (is_pressed(e,Scancode::D) as i32 - is_pressed(e,Scancode::A) as i32) as f64;
    dir.y = (is_pressed(e,Scancode::S) as i32 - is_pressed(e,Scancode::W) as i32) as f64;
    dir
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("keyboard_example", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialise video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    let mut i = 0;
    'running: loop {
        let direction = inpt_dir(&event_pump);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
                _ => {}
            }
        }
        i += 1;
        println!("Input Direction : [{},{}]",direction.x,direction.y);
        canvas.set_draw_color(Color::RGB(20,20,70));
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0,1_000_000_000_u32 / 14));
    }
    Ok(())
}
