use canvas::Canvas;
use curves::{sine::Sine, Drawable, Point};
use image::{imageops::FilterType, io::Reader as ImageReader, GrayImage};
use std::{cmp::max, path::Path};

mod canvas;
mod curves;

struct SineArt {
    source: GrayImage,
    canvas: Canvas,
}

impl SineArt {
    fn new<P: AsRef<Path>>(nw: u32, nh: u32, source: P, downscale: u32) -> Self {
        let mut source = ImageReader::open(source)
            .expect("could not open source image")
            .decode()
            .expect("could not decode source image");

        let nw_scale = nw * 4;

        let target_width = (source.width() * downscale / 100 / nw_scale + 1) * nw_scale + 1;
        let target_height = (source.height() * target_width) / source.width();

        let canvas = Canvas::new(
            [target_height * 105 / 100, target_width * 105 / 100],
            [target_height, target_width],
        );

        Self {
            source: source
                .resize_exact(nw, nh, FilterType::Triangle)
                .into_luma8(),
            canvas,
        }
    }

    fn cell_height(&self) -> u32 {
        self.canvas.ih / self.source.height()
    }

    fn cell_width(&self) -> u32 {
        (self.canvas.iw - 1) / self.source.width()
    }

    /// Return the max amplitude a sine wave can have. A_max = 0.9 x cell_height / 2.
    fn max_amplitude(&self) -> u32 {
        self.cell_height() * 9 / 20
    }

    fn quarter_wavelength(&self) -> u32 {
        self.cell_width() / 4
    }

    fn draw_on_canvas(&mut self) {
        let cw = self.cell_width();
        let qwave = self.quarter_wavelength();
        let amax = self.max_amplitude();

        for img_y in 0..self.source.height() {
            for img_x in 0..self.source.width() {
                let x = cw * img_x;

                // calculate every time to avoid period falling behind
                let y = (self.canvas.ih / 2 + self.canvas.ih * (self.source.height() - img_y - 1))
                    / self.source.height();
                let a = amax - amax * self.source.get_pixel(img_x, img_y).0[0] as u32 / 255;
                let sine = Sine::new(Point::new(x, y), a, qwave);
                sine.draw_antialiased(&mut self.canvas)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::canvas::XYDrawable;

    use super::*;

    #[test]
    #[ignore = "visual check"]
    fn logo() {
        let mut art = SineArt::new(70, 35, "tests/lincoln.jpeg", 25);
        dbg!(
            art.quarter_wavelength() * 4,
            art.cell_width(),
            art.canvas.iw
        );
        art.draw_on_canvas();
        art.canvas.save("tests/lincoln_sine.jpg");
    }
}
