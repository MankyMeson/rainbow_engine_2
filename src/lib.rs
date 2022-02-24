use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod consts;
pub mod vector;
pub mod entity;
pub mod input;

use crate::entity::*;
use vector::Vector2;

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
    pub speed: vector::Vector2,
    pub direction: vector::Vector2
}

#[derive(Debug)]
pub struct Player_new {
    pub position: Point,
    pub sprite: Entity<Sprite>,
    pub kinematics: Entity<Kinetic>,
    pub collider: Entity<Collision>
}

pub fn update_player_new(player: &mut Player_new) {
    // Movement Update
    player.kinematics.kind.speed = if player.kinematics.kind.direction.magnitude() == 0. {
        player.kinematics.kind.speed.move_towards(0.,0.,consts::PLAYER_FRICTION as f64)
    } else {
        let max_spd_vec = player.kinematics.kind.direction
            .normalise()
            .scalar_mult(consts::PLAYER_SPEED as f64);
        player.kinematics.kind.speed
            .move_towards_vec(max_spd_vec, consts::PLAYER_ACCN as f64)
    };

    let dx = player.kinematics.kind.speed.x.round() as i32;
    let dy = player.kinematics.kind.speed.y.round() as i32;

    player.position             = player.position.offset(dx, dy);
    player.sprite.position      = player.sprite.position.offset(dx, dy);
    player.kinematics.position  = player.kinematics.position.offset(dx, dy);
    player.collider.position    = player.collider.position.offset(dx, dy);

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

pub fn update_player_direction(player: &mut Player, event_pump: &sdl2::EventPump) {
    let dir = input::input_dir(event_pump);
    player.kinematics.kind.direction = Vector2::new(dir.x,dir.y);
}
