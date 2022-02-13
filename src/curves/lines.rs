use super::{Curve, Point};
use num::ToPrimitive;

struct AngledLine {
    start: Point,
    stop: Point,
    dx: i32,
    dy: i32,
    aa_threshold: i32,
}

impl AngledLine {
    fn new(start: Point, stop: Point) -> Self {
        let dx = stop.x - start.x;
        let dy = stop.y - start.y;
        let aa_threshold: i32 = (dx.pow(2) + dy.pow(2))
            .to_f64()
            .expect("could not cast AA threshold to f64")
            .sqrt()
            .to_i32()
            .expect("could not convert f64 -> i32");

        Self {
            start,
            stop,
            dx,
            dy,
            aa_threshold,
        }
    }
}

impl Curve for AngledLine {
    type T = i32;

    fn start(&self) -> &Point {
        &self.start
    }

    fn stop(&self) -> &Point {
        &self.stop
    }

    fn equation(&self, point: &Point) -> Self::T {
        self.dx * (point.y - self.start.y) - (point.x - self.start.x) * self.dy
    }

    fn antialiased_threshold(&self) -> Self::T {
        self.aa_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curves::{Canvas, Drawable};

    #[test]
    #[ignore = "visual check"]
    fn angled_line() {
        let aline = AngledLine::new(Point::new(0, 0), Point::new(150, 100));
        let mut img = Canvas::new(200, 200);
        aline.draw_antialiased(&mut img);
        img.save("test.bmp");
        dbg!(aline.antialiased_threshold());
    }
}
