use sdl2::rect::{Point, Rect};

use crate::vector::Vector2;

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Entity<T> {
    pub position: Point,
    pub kind: T
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Sprite { img: Rect }

impl<T> Entity<T> {
    pub fn new(x: i32, y: i32, k: T) -> Entity<T> {
        Entity { position: Point::new(x,y), kind: k }
    }
}

//impl Entity<Sprite> {
//
//}

impl Entity<Kinetic> {

    pub fn smooth_move_topdown(
        entity: &mut Entity<Kinetic>,
        max_speed: i32,
        friction: i32,
        accn: i32
    ) -> Entity<Kinetic> {

        let kinetic_out = entity.kind.smooth_topdown_speed_update(max_speed, friction, accn);

        Entity::<Kinetic> {
            position: entity.position.offset(
                (entity.kind.speed.x.round()) as i32,
                (entity.kind.speed.y.round()) as i32
            ),
            kind: kinetic_out
        }
    }

}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Kinetic {
    pub speed: Vector2,
    pub direction: Vector2
}

impl Kinetic {
    pub fn smooth_topdown_speed_update(
        &self,
        max_speed: i32,
        friction: i32,
        accn: i32
    ) -> Kinetic {
        let spdout = if self.direction.x != 0. || self.direction.y != 0. {
            let max_spd_vec = self.direction
                .normalise()
                .scalar_mult(max_speed as f64);
            self.speed
                .move_towards(max_spd_vec.x, max_spd_vec.y, accn as f64)
        } else {
            self.speed.move_towards(0.,0.,friction as f64)
        };

        Kinetic {
            speed: spdout,
            direction: self.direction
        }
    }
}


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Collision { pub shape: Rect }

