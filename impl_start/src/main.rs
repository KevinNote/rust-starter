struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn get_distance(&self, p: &Point) -> f32 {
        ((p.x - &self.x).powf(2.0) + (p.y - &self.y).powf(2.0)).powf(0.5)
    }
}

trait IShape {
    fn is_right(&self) -> bool;
}

struct Triangle {
    x: Point,
    y: Point,
    z: Point,
}

impl Triangle {
    fn get_x2y(&self) -> f32 {
        self.x.get_distance(&self.y)
    }
    fn get_x2z(&self) -> f32 {
        self.x.get_distance(&self.z)
    }
    fn get_y2z(&self) -> f32 {
        self.y.get_distance(&self.z)
    }
}

impl IShape for Triangle {
    fn is_right(&self) -> bool {
        let x2y = self.get_x2y();
        let x2z = self.get_x2z();
        let y2z = self.get_y2z();
        let x2y = x2y.powf(2.0);
        let x2z = x2z.powf(2.0);
        let y2z = y2z.powf(2.0);
        if (x2y + x2z == y2z) || (x2y + y2z == x2z) || (y2z + x2z == x2y) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let r = Triangle {
        x: Point { x: 0.0, y: 3.0 },
        y: Point { x: 4.0, y: 0.0 },
        z: Point { x: 0.0, y: 0.0 },
    };
    println!("R is a right triangle: {}", r.is_right());
}
