mod lines;
mod sine;

use image::{GrayImage, Luma};
use num::{Signed, ToPrimitive};
use std::{fmt::Display, path::Path};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Canvas(GrayImage);

impl Canvas {
    fn new(width: u32, height: u32) -> Self {
        let mut img = GrayImage::new(width, height);
        img.fill(255);
        Self(img)
    }

    fn set(&mut self, x: u32, y: u32, value: u8) {
        self.0.put_pixel(x, self.0.height() - y - 1, Luma([value]));
    }

    fn save<P: AsRef<Path>>(&self, path: P) {
        self.0.save(path).expect("failed to save image");
    }
}

trait Drawable {
    fn draw(&self, canvas: &mut Canvas);

    fn draw_antialiased(&self, canvas: &mut Canvas);
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
    fn draw(&self, canvas: &mut Canvas) {
        let mut current = *self.start();
        let slope = Slope::between(self.start(), self.stop());

        while &current != self.stop() {
            canvas.set(current.x as u32, current.y as u32, 0);
            current = slope
                .next(&current)
                .into_iter()
                .map(|p| (p, self.equation(&p).abs()))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN encountered"))
                .map(|(p, _)| p)
                .expect("no viable next point found");
        }
        canvas.set(current.x as u32, current.y as u32, 0);
    }

    fn draw_antialiased(&self, canvas: &mut Canvas) {
        let mut current = *self.start();
        let slope = Slope::between(self.start(), self.stop());

        canvas.set(
            current.x as u32,
            current.y as u32,
            self.antialiased_value(&current),
        );

        while &current != self.stop() {
            let next = slope.next(&current);

            // println!(
            //     "{:?}",
            //     next.iter().map(|p| self.equation(p)).collect::<Vec<_>>()
            // );

            for p in next.iter() {
                canvas.set(p.x as u32, p.y as u32, self.antialiased_value(p));
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
