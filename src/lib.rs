use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod consts;
pub mod vector_2;

use vector_2::Vector2;

pub fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
    canvas.copy(texture, player.sprite, screen_rect)?;

    canvas.present();

    Ok(())
}

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: vector_2::Vector2,
    pub direction: vector_2::Vector2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,Down,Left,Right
}

pub fn update_player(player: &mut Player) {
    let mut max_spd_vec = Vector2::new(0.,0.);

    player.speed = if
        player.direction.x != 0. || player.direction.y != 0.
    {
        max_spd_vec = player.direction
            .normalise().scalar_mult(consts::PLAYER_SPEED as f64);
        player.speed.move_towards(
            max_spd_vec.x,
            max_spd_vec.y,
            consts::PLAYER_ACCN as f64
        )
    } else {
        player.speed.move_towards(
            max_spd_vec.x,
            max_spd_vec.y,
            consts::PLAYER_FRICTION as f64
        )
    };

    player.position = player.position.offset(
            (player.speed.x.round()) as i32,
            (player.speed.y.round()) as i32
        );
}

pub fn get_move_inpt(player: &mut Player, event: Event) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
            player.direction = player.direction.offset(-1.0,0.0);
        },
        Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
            player.direction = player.direction.offset(1.0,0.0);
        },
        Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
            player.direction = player.direction.offset(0.0,-1.0);
        },
        Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
            player.direction = player.direction.offset(0.0,1.0);
        },
        Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
            player.direction = player.direction.offset(1.0,0.0);
        },
        Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
            player.direction = player.direction.offset(-1.0,0.0);
        },
        Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
            player.direction = player.direction.offset(0.0,1.0);
        },
        Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
            player.direction = player.direction.offset(0.0,-1.0);
        },
        _ => {}
    }
}
