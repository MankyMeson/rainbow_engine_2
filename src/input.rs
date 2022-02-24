use sdl2::event::Event;
use sdl2::keyboard::{Keycode,Scancode,KeyboardState};
use crate::vector::Vector2;

pub fn is_pressed(e: &sdl2::EventPump, k: Scancode) -> bool {
    e.keyboard_state().is_scancode_pressed(k)
}

pub fn input_dir(e: &sdl2::EventPump) -> Vector2 {
    Vector2 {
        x: (is_pressed(e,Scancode::D) as i32 - is_pressed(e,Scancode::A) as i32) as f64,
        y: (is_pressed(e,Scancode::S) as i32 - is_pressed(e,Scancode::W) as i32) as f64,
    }
}
