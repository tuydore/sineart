pub mod lines;
pub mod sine;

use num::{Signed, ToPrimitive};
use std::fmt::Display;

use crate::canvas::{Canvas, XYDrawable};

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

pub trait Drawable {
    fn draw(&self, canvas: &mut impl XYDrawable);

    fn draw_antialiased(&self, canvas: &mut impl XYDrawable);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Slope {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
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

trait Curve {
    /// Type to use in error functions, returned by equation etc.
    type T: Signed + PartialOrd + ToPrimitive + Display + core::fmt::Debug;

    fn start(&self) -> &Point;
    fn stop(&self) -> &Point;

    /// Implicit equation of curve, f(x, y) = 0.
    fn equation(&self, point: &Point) -> Self::T;

    /// Threshold for anti-aliasing, will set lines to 255 * equation(p) / threshold.
    fn antialiased_threshold(&self) -> Self::T;

    /// Value to set pixel to when using anti-aliasing.
    fn antialiased_value(&self, point: &Point) -> u8 {
        let value = self.equation(point).abs();
        let threshold = self.antialiased_threshold();
        if value > threshold {
            255
        } else {
            (value.to_u32().expect("could not convert value to u32") * 255
                / threshold
                    .to_u32()
                    .expect("could not convert threshold to u32")) as u8
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

    fn draw_antialiased(&self, canvas: &mut impl XYDrawable) {
        let mut current = *self.start();
        let slope = Slope::between(self.start(), self.stop());

        canvas.set_point(&current, self.antialiased_value(&current));

        while &current != self.stop() {
            let next = slope.next(&current);

            // println!(
            //     "{:?}",
            //     next.iter().map(|p| self.equation(p)).collect::<Vec<_>>()
            // );

            for p in next.iter() {
                canvas.set_point(p, self.antialiased_value(p));
            }
            current = next
                .into_iter()
                .map(|p| (p, self.equation(&p).abs()))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN encountered"))
                .map(|(p, _)| p)
                .expect("no viable next point found");
        }
        // println!("{}", self.antialiased_threshold());
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
