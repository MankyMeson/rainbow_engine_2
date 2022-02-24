use rainbow_engine_2::{self,vector::Vector2};

fn main() {
    let a: Vector2 = Vector2::new(1.,5.);
    let b: Vector2 = a.origin();
    let c: Vector2 = a.normalise();
    let d: f64 = Vector2::dot(a,c);
    let e: f64 = a.magnitude();

    let mut f: Vector2 = Vector2::new(40.,12.);
    println!("f_init: [{},{}]",f.x,f.y);
    loop {
        f = f.move_towards(a.x,a.y,0.5);
        println!("f: [{},{}]",f.x,f.y);
        if f.x == a.x && f.y == a.y { break; }
    }

    println!("a: [{},{}]\nb: [{},{}]\nc: [{},{}]\nd: {}\ne: {}",a.x,a.y,b.x,b.y,c.x,c.y,d,e);
}
