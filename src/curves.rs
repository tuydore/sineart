pub mod lines;
pub mod sine;

use crate::canvas::XYDrawable;
use num::{Signed, ToPrimitive};
use std::fmt::Display;

/// Anything that is drawable onto a canvas.
pub trait Drawable {
    /// Draw a single, non-antialiased line of thickness 1.
    fn draw(&self, canvas: &mut impl XYDrawable);

    /// Draw a line of thickness `thickness`. The expansion is done on the horizontal axis.
    fn draw_thick(&self, canvas: &mut impl XYDrawable, thickness: u32);
}

/// A line with a fixed gradient and direction, meaning the next possible pixel at each iteration
/// can only be one of three options. E.g. for a curve starting at (0, 0) and ending at (10, 10),
/// that has positive derivative at all points, the next possible options at every step will be
/// (x + 1, y), (x, y + 1) or (x + 1, y + 1). The approach is taken from
/// http://members.chello.at/%7Eeasyfilter/Bresenham.pdf.
pub trait Curve {
    /// Type to use in error functions, returned by equation etc.
    type T: Signed + PartialOrd + ToPrimitive + Display + core::fmt::Debug;

    fn start(&self) -> &Point;
    fn stop(&self) -> &Point;

    /// Implicit equation of curve, f(x, y) = 0.
    fn equation(&self, point: &Point) -> Self::T;
}

/// Potential direction of the curve, a mixture of start and stop ordering and of the derivative
/// value for its entire length.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Slope {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Slope {
    /// Select the next possible three points to be drawn.
    fn next(&self, point: &Point) -> [Point; 3] {
        let Point { x, y } = *point;
        match self {
            Slope::NorthEast => [
                Point::new(x, y + 1),
                Point::new(x + 1, y + 1),
                Point::new(x + 1, y),
            ],
            Slope::SouthEast => [
                Point::new(x + 1, y),
                Point::new(x + 1, y - 1),
                Point::new(x, y - 1),
            ],
            Slope::SouthWest => [
                Point::new(x, y - 1),
                Point::new(x - 1, y - 1),
                Point::new(x - 1, y),
            ],
            Slope::NorthWest => [
                Point::new(x - 1, y),
                Point::new(x - 1, y + 1),
                Point::new(x, y + 1),
            ],
        }
    }

    /// Determines the slope type, assuming the derivative does not change sign.
    fn between(start: &Point, stop: &Point) -> Self {
        if start.x < stop.x {
            if start.y < stop.y {
                Slope::NorthEast
            } else {
                Slope::SouthEast
            }
        } else if start.y < stop.y {
            Slope::NorthWest
        } else {
            Slope::SouthWest
        }
    }
}

impl<C: Curve> Drawable for C {
    fn draw(&self, canvas: &mut impl XYDrawable) {
        let mut current = *self.start();
        let slope = Slope::between(self.start(), self.stop());

        while &current != self.stop() {
            canvas.set_point(&current, 0);
            current = slope
                .next(&current)
                .into_iter()
                .map(|p| (p, self.equation(&p).abs()))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN encountered"))
                .map(|(p, _)| p)
                .expect("no viable next point found");
        }
        canvas.set_point(&current, 0);
    }

    fn draw_thick(&self, canvas: &mut impl XYDrawable, thickness: u32) {
        let mut current = *self.start();
        let slope = Slope::between(self.start(), self.stop());

        while &current != self.stop() {
            canvas.set_horizontal_line(&current, 0, thickness);
            current = slope
                .next(&current)
                .into_iter()
                .map(|p| (p, self.equation(&p).abs()))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN encountered"))
                .map(|(p, _)| p)
                .expect("no viable next point found");
        }
        canvas.set_horizontal_line(&current, 0, thickness);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod slope {
        use super::*;

        #[test]
        fn between() {
            let start = Point::new(10, 10);
            assert_eq!(
                Slope::between(&start, &Point::new(11, 11)),
                Slope::NorthEast
            );
            assert_eq!(Slope::between(&start, &Point::new(9, 11)), Slope::NorthWest);
            assert_eq!(Slope::between(&start, &Point::new(11, 9)), Slope::SouthEast);
            assert_eq!(Slope::between(&start, &Point::new(9, 9)), Slope::SouthWest);
        }

        #[test]
        fn next() {
            let point = Point::new(0, 0);
            let slope = Slope::NorthEast;
            assert_eq!(
                slope.next(&point),
                [Point::new(0, 1), Point::new(1, 1), Point::new(1, 0)]
            )
        }
    }
}
