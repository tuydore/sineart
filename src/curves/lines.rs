//! Implemented only for debug purposes.

use super::{Curve, Point};

struct AngledLine {
    start: Point,
    stop: Point,
    dx: i32,
    dy: i32,
}

impl AngledLine {
    #[allow(dead_code)]
    fn new(start: Point, stop: Point) -> Self {
        let dx = stop.x as i32 - start.x as i32;
        let dy = stop.y as i32 - start.y as i32;

        Self {
            start,
            stop,
            dx,
            dy,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        canvas::{Canvas, XYDrawable},
        curves::Drawable,
    };

    #[test]
    #[ignore = "visual check"]
    fn angled_line() {
        let aline = AngledLine::new(Point::new(0, 0), Point::new(549, 549));
        let mut img = Canvas::new([600; 2], [550; 2]);
        aline.draw(&mut img);
        img.save("tests/test.bmp");
    }
}
