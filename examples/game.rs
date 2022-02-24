/* This File is for testing the capabilities of
 * the sdl2 crate for now.
 * */
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Point, Rect};
use std::time::Duration;
use rainbow_engine_2::{self, consts, Direction};
use rainbow_engine_2::vector::Vector2;
use rainbow_engine_2::entity::{Entity,Kinetic};
use rainbow_engine_2::input;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("rainbow-example", 1280, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("/home/clio/Pictures/Wallpapers/alien.png")?;

//  let mut player = rainbow_engine_2::Player {
//      position: Point::new(0,0),
//      sprite: Rect::new(0,0,26,36),
//      speed: Vector2::new(0.,0.),
//      direction: Vector2::new(0.,0.)
//  };

    let mut player = rainbow_engine_2::Player_New {
        position: Point::new(0,0),
        sprite: Entity<Sprite> { position: Point::new(0,0), kind: Sprite { img: Rect::new(0,0,26,36) } },
        kinematics: Entity<Kinetic> {
            position: Point::new(0,0),
            kind: Kinetic {
                speed: Vector2::new(0.,0.),
                direction: Vector2::new(0.,0.) 
            }
        }
    };


    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        // Handle events
        rainbow_engine_2::update_player_direction(&mut player, &event_pump);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
            println!("player.direction: [{},{}]",player.direction.x,player.direction.y);
        }

        // Update
        i = ( i + 1 ) % 255;
        rainbow_engine_2::update_player(&mut player);

        // Debug
        if i%30 == 0 {
            println!("player.speed: [{},{}]",player.speed.x,player.direction.y);
        }

        // Render
        rainbow_engine_2::render(&mut canvas, Color::RGB(i,64,255-i), &texture, &player)?;

        // Time management
        ::std::thread::sleep(Duration::new(0,1_000_000_000u32 / consts::FRAMERATE as u32));
    }

    Ok(())
}
