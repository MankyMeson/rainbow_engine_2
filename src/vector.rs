
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

/* Vector2 defines a 2D vector with methods and functions specifically for use
 * within the rainbow_engine_2 package. Methods such as move_towards() and
 * normalise() are defined for these purposes.
 * */


impl Vector2 {

    pub fn origin(&self) -> Vector2 {
        Vector2 { x: 0_f64 , y: 0_f64 }
    }


    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x: x, y: y}
    }


    pub fn scalar_mult(&self, f: f64) -> Vector2 {
        Vector2 { x: f*self.x, y: f*self.y }
    }


    pub fn normalise(&self) -> Vector2 {
        let x1 = self.x;
        let y1 = self.y;
        let norm_factor = 1.0_f64 / ((x1*x1 + y1*y1).sqrt() as f64);        // Could be improved with quake inv_sqrt implementation
        Vector2 { x: norm_factor*x1, y: norm_factor*y1 }
    }


    pub fn dot(a: Vector2, b: Vector2) -> f64 {
        a.x*b.x + a.y*b.y
    }


    pub fn magnitude(&self) -> f64 {
        Vector2::dot(*self,*self).sqrt()
    }


    pub fn offset(&self, x_offset: f64, y_offset: f64) -> Vector2 {
        Vector2 { x: self.x + x_offset, y: self.y + y_offset }
    }


    pub fn move_towards(&self, x: f64, y: f64, amount: f64) -> Vector2 {
        let sepx = self.x - x;
        let sepy = self.y - y;
        let mag = (sepy*sepy + sepx*sepx).sqrt();

        if amount >= mag {
            Vector2 { x: x, y: y }
        } else {

            let xout = if sepx == 0. {
                self.x
            } else if sepx < 0. {
                self.x + (amount / ((sepy/sepx).powf(2.) + 1.).sqrt())
            } else {
                self.x - (amount / ((sepy/sepx).powf(2.) + 1.).sqrt())
            };

            let yout = if sepy == 0. {
                self.y
            } else if sepy < 0. {
                self.y + (amount*amount - (self.x - xout).powf(2.)).sqrt()
            } else {
                self.y - (amount*amount - (self.x - xout).powf(2.)).sqrt()
            };

            Vector2 { x: xout, y: yout }
        }
    }


    pub fn move_towards_vec(&self, v: Vector2, amount: f64) -> Vector2 {
        self.move_towards(v.x, v.y, amount)
    }

}

