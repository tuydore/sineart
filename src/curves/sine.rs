use super::{Curve, Drawable, Point};
use num::ToPrimitive;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
enum SineQuadrant {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, Copy)]
enum SineDirection {
    Forward,
    Backward,
}

struct QuarterSine {
    start: Point,
    quadrant: SineQuadrant,
    amplitude: f64,
    wavelength: f64,
    stop: Point,
}

struct Sine {
    quarters: [QuarterSine; 4],
    direction: SineDirection,
    amplitude: i32,
    quarter_wavelength: i32,
}

impl Sine {
    fn new(
        start: Point,
        direction: SineDirection,
        amplitude: i32,
        quarter_wavelength: i32,
    ) -> Self {
        let sine1 = QuarterSine::new(
            start,
            SineQuadrant::First,
            direction,
            amplitude,
            quarter_wavelength,
        );
        let sine2 = QuarterSine::new(
            sine1.stop,
            SineQuadrant::Second,
            direction,
            amplitude,
            quarter_wavelength,
        );
        let sine3 = QuarterSine::new(
            sine2.stop,
            SineQuadrant::Third,
            direction,
            amplitude,
            quarter_wavelength,
        );
        let sine4 = QuarterSine::new(
            sine3.stop,
            SineQuadrant::Fourth,
            direction,
            amplitude,
            quarter_wavelength,
        );
        Self {
            quarters: [sine1, sine2, sine3, sine4],
            direction,
            amplitude,
            quarter_wavelength,
        }
    }

    fn stop(&self) -> Point {
        self.quarters[3].stop
    }

    fn next(&self) -> Self {
        Self::new(
            self.stop(),
            self.direction,
            self.amplitude,
            self.quarter_wavelength,
        )
    }
}

impl Drawable for Sine {
    fn draw(&self, canvas: &mut super::Canvas) {
        for quarter in self.quarters.iter() {
            quarter.draw(canvas);
        }
    }

    fn draw_antialiased(&self, canvas: &mut super::Canvas) {
        for quarter in self.quarters.iter() {
            quarter.draw_antialiased(canvas);
        }
    }
}

impl SineQuadrant {
    fn stop(
        &self,
        direction: SineDirection,
        start: &Point,
        wavelength: i32,
        amplitude: i32,
    ) -> Point {
        let dx = match direction {
            SineDirection::Forward => wavelength,
            SineDirection::Backward => -wavelength,
        };

        let mut dy = match self {
            SineQuadrant::First => amplitude,
            SineQuadrant::Second => -amplitude,
            SineQuadrant::Third => -amplitude,
            SineQuadrant::Fourth => amplitude,
        };

        if matches!(direction, SineDirection::Backward) {
            dy *= -1;
        }

        Point::new(start.x + dx, start.y + dy)
    }
}

impl QuarterSine {
    fn new(
        start: Point,
        quadrant: SineQuadrant,
        direction: SineDirection,
        amplitude: i32,
        wavelength: i32,
    ) -> Self {
        let stop = quadrant.stop(direction, &start, wavelength, amplitude);

        Self {
            start,
            quadrant,
            amplitude: amplitude as f64,
            wavelength: wavelength as f64,
            stop,
        }
    }

    fn equation_aux(&self, x: i32, y: i32) -> f64 {
        let x = x.to_f64().expect("could not convert to f64");
        let y = y.to_f64().expect("could not convert to f64");

        match self.quadrant {
            SineQuadrant::First => y - self.amplitude * (x * PI / (2.0 * self.wavelength)).sin(),
            SineQuadrant::Second => {
                y - self.amplitude * ((x * PI / (2.0 * self.wavelength)).cos() - 1.0)
            }
            SineQuadrant::Third => y + self.amplitude * (x * PI / (2.0 * self.wavelength)).sin(),
            SineQuadrant::Fourth => {
                y + self.amplitude * ((x * PI / (2.0 * self.wavelength)).cos() - 1.0)
            }
        }
    }
}

impl Curve for QuarterSine {
    type T = f64;

    fn start(&self) -> &Point {
        &self.start
    }

    fn stop(&self) -> &Point {
        &self.stop
    }

    fn antialiased_threshold(&self) -> Self::T {
        PI
    }

    fn equation(&self, point: &Point) -> Self::T {
        self.equation_aux(point.x - self.start.x, point.y - self.start.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curves::{Canvas, Drawable};

    #[test]
    #[ignore = "visual check"]
    fn sine() {
        let mut sine = Sine::new(Point::new(0, 100), SineDirection::Forward, 50, 10);
        let mut img = Canvas::new(402, 200);
        for _ in 0..8 {
            sine.draw(&mut img);
            sine = sine.next();
        }
        img.save("test.bmp");
    }
}
