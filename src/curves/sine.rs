use crate::canvas::XYDrawable;

use super::{Curve, Drawable, Point};
use num::ToPrimitive;
use std::f64::consts::PI;

/// Quadrant of sine wave, travelling towards +X.
#[derive(Debug, Clone, Copy)]
enum SineQuadrant {
    /// Quadrant between [0, PI/2].
    First,

    /// Quadrant between [PI/2, PI].
    Second,

    /// Quadrant between [PI, 3*PI/2].
    Third,

    /// Quadrant between [3*PI/2, 2*PI].
    Fourth,
}

struct QuarterSine {
    start: Point,
    stop: Point,
    quadrant: SineQuadrant,
    amplitude: f64,
    quarter_wavelength: f64,
}

struct Sine {
    start: Point,
    amplitude: u32,
    quarter_wavelength: u32,
}

struct SineWave {
    start: Point,
    amplitude: u32,
    quarter_wavelength: u32,
    num_oscillations: usize,
}

impl SineWave {
    pub fn new(
        start: Point,
        amplitude: u32,
        quarter_wavelength: u32,
        num_oscillations: usize,
    ) -> Self {
        Self {
            start,
            amplitude,
            quarter_wavelength,
            num_oscillations,
        }
    }
}

impl Drawable for SineWave {
    fn draw(&self, canvas: &mut impl XYDrawable) {
        let mut sine = Sine::new(self.start, self.amplitude, self.quarter_wavelength);
        sine.draw(canvas);
        for _ in 0..self.num_oscillations {
            sine = sine.next();
            sine.draw(canvas);
        }
    }

    fn draw_antialiased(&self, canvas: &mut impl XYDrawable) {
        let mut sine = Sine::new(self.start, self.amplitude, self.quarter_wavelength);
        sine.draw_antialiased(canvas);
        for _ in 0..self.num_oscillations {
            sine = sine.next();
            sine.draw_antialiased(canvas);
        }
    }
}

impl Sine {
    fn new(start: Point, amplitude: u32, quarter_wavelength: u32) -> Self {
        Self {
            start,
            amplitude,
            quarter_wavelength,
        }
    }

    /// Stopping point of the sine.
    fn stop(&self) -> Point {
        Point::new(self.start.x + 4 * self.quarter_wavelength, self.start.y)
    }

    /// Creates the next sine.
    fn next(&self) -> Self {
        Self::new(self.stop(), self.amplitude, self.quarter_wavelength)
    }

    /// Return the four comprising quarters.
    fn quarters(&self) -> [QuarterSine; 4] {
        let q1 = QuarterSine::new(
            self.start,
            SineQuadrant::First,
            self.amplitude,
            self.quarter_wavelength,
        );
        let q2 = QuarterSine::new(
            q1.stop,
            SineQuadrant::Second,
            self.amplitude,
            self.quarter_wavelength,
        );
        let q3 = QuarterSine::new(
            q2.stop,
            SineQuadrant::Third,
            self.amplitude,
            self.quarter_wavelength,
        );
        let q4 = QuarterSine::new(
            q3.stop,
            SineQuadrant::Fourth,
            self.amplitude,
            self.quarter_wavelength,
        );
        // QUESTION: perhaps turn into iterator, avoid some duplication?
        [q1, q2, q3, q4]
    }
}

impl Drawable for Sine {
    fn draw(&self, canvas: &mut impl XYDrawable) {
        for quarter in self.quarters().iter() {
            quarter.draw(canvas);
        }
    }

    fn draw_antialiased(&self, canvas: &mut impl XYDrawable) {
        for quarter in self.quarters().iter() {
            quarter.draw_antialiased(canvas);
        }
    }
}

impl SineQuadrant {
    fn stop(&self, start: &Point, quarter_wavelength: u32, amplitude: u32) -> Point {
        let mut dy = match self {
            SineQuadrant::First => amplitude as i32,
            SineQuadrant::Second => -(amplitude as i32),
            SineQuadrant::Third => -(amplitude as i32),
            SineQuadrant::Fourth => amplitude as i32,
        };

        Point::new(start.x + quarter_wavelength, (start.y as i32 + dy) as u32)
    }
}

impl QuarterSine {
    /// Creates a new quarter sine, with cached stop point and amplitude and quarter wavelength pre-converted to f64.
    fn new(start: Point, quadrant: SineQuadrant, amplitude: u32, quarter_wavelength: u32) -> Self {
        let stop = quadrant.stop(&start, quarter_wavelength, amplitude);

        Self {
            start,
            quadrant,
            amplitude: amplitude as f64,
            quarter_wavelength: quarter_wavelength as f64,
            stop,
        }
    }

    /// Auxiliary equation for centering start of quadrant equation at current point.
    fn equation_aux(&self, x: i32, y: i32) -> f64 {
        let x = x.to_f64().expect("could not convert to f64");
        let y = y.to_f64().expect("could not convert to f64");

        match self.quadrant {
            SineQuadrant::First => {
                y - self.amplitude * (x * PI / (2.0 * self.quarter_wavelength)).sin()
            }
            SineQuadrant::Second => {
                y - self.amplitude * ((x * PI / (2.0 * self.quarter_wavelength)).cos() - 1.0)
            }
            SineQuadrant::Third => {
                y + self.amplitude * (x * PI / (2.0 * self.quarter_wavelength)).sin()
            }
            SineQuadrant::Fourth => {
                y + self.amplitude * ((x * PI / (2.0 * self.quarter_wavelength)).cos() - 1.0)
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
        self.equation_aux(
            point.x as i32 - self.start.x as i32,
            point.y as i32 - self.start.y as i32,
        )
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
    fn sine() {
        let sinewave = SineWave::new(Point::new(0, 100), 50, 10, 8);
        let mut img = Canvas::new([600; 2], [400; 2]);
        sinewave.draw(&mut img);
        img.save("test.bmp");
    }
}
