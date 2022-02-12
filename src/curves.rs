use image::{GrayImage, Luma};

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

trait Drawable {
    fn draw(&self, image: &mut GrayImage);

    fn draw_antialiased(&self, image: &mut GrayImage);
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
    fn start(&self) -> &Point;
    fn stop(&self) -> &Point;
    fn equation(&self, point: &Point) -> f64;
    fn antialiased_threshold(&self) -> f64;
    fn antialiased_value(&self, point: &Point) -> u8 {
        let value = self.equation(point).abs();
        let threshold = self.antialiased_threshold();
        if value > threshold {
            255
        } else {
            (value * 255.0 / threshold) as u8
        }
    }
}

impl<C: Curve> Drawable for C {
    fn draw(&self, image: &mut GrayImage) {
        let mut current = *self.start();
        let slope = Slope::between(self.start(), self.stop());

        while &current != self.stop() {
            image.put_pixel(current.x as u32, current.y as u32, Luma([0]));
            current = slope
                .next(&current)
                .into_iter()
                .map(|p| (p, self.equation(&p).abs()))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN encountered"))
                .map(|(p, _)| p)
                .expect("no viable next point found");
        }
        image.put_pixel(current.x as u32, current.y as u32, Luma([0]));
    }

    fn draw_antialiased(&self, image: &mut GrayImage) {
        let mut current = *self.start();
        let slope = Slope::between(self.start(), self.stop());

        image.put_pixel(
            current.x as u32,
            current.y as u32,
            Luma([self.antialiased_value(&current)]),
        );

        while &current != self.stop() {
            let next = slope.next(&current);

            println!(
                "{:?}",
                next.iter()
                    .map(|p| self.antialiased_value(p))
                    .collect::<Vec<u8>>()
            );

            for p in next.iter() {
                image.put_pixel(p.x as u32, p.y as u32, Luma([self.antialiased_value(p)]));
            }
            current = next
                .into_iter()
                .map(|p| (p, self.equation(&p).abs()))
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN encountered"))
                .map(|(p, _)| p)
                .expect("no viable next point found");
        }
    }
}

struct AngledLine {
    start: Point,
    stop: Point,
    dx: i32,
    dy: i32,
}

impl AngledLine {
    fn new(start: Point, stop: Point) -> Self {
        Self {
            dx: stop.x - start.x,
            dy: stop.y - start.y,
            start,
            stop,
        }
    }
}

impl Curve for AngledLine {
    fn start(&self) -> &Point {
        &self.start
    }

    fn stop(&self) -> &Point {
        &self.stop
    }

    fn equation(&self, point: &Point) -> f64 {
        (self.dx * (point.y - self.start.y) - (point.x - self.start.x) * self.dy) as f64
    }

    fn antialiased_threshold(&self) -> f64 {
        ((self.dx.pow(2) + self.dy.pow(2)) as f64).sqrt()
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

    mod drawing {
        use super::*;

        #[test]
        #[ignore = "visual check"]
        fn angled_line() {
            let aline = AngledLine::new(Point::new(0, 0), Point::new(150, 100));
            let mut img = GrayImage::new(200, 200);
            img.fill(255);
            aline.draw_antialiased(&mut img);
            img.save("test.bmp").unwrap();
            dbg!(aline.antialiased_threshold());
        }
    }
}
