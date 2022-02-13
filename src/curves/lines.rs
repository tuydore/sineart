use super::{Curve, Drawable, Point};
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
        let dx = stop.x as i32 - start.x as i32;
        let dy = stop.y as i32 - start.y as i32;
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
        self.dx * (point.y as i32 - self.start.y as i32)
            - (point.x as i32 - self.start.x as i32) * self.dy
    }

    fn antialiased_threshold(&self) -> Self::T {
        self.aa_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        canvas::XYDrawable,
        curves::{Canvas, Drawable},
    };

    #[test]
    #[ignore = "visual check"]
    fn angled_line() {
        let aline = AngledLine::new(Point::new(0, 0), Point::new(549, 549));
        let mut img = Canvas::new([600; 2], [550; 2]);
        aline.draw_antialiased(&mut img);
        img.save("tests/test.bmp");
        dbg!(aline.antialiased_threshold());
    }
}
