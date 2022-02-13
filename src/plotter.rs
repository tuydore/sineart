use crate::{
    canvas::Canvas,
    curves::{sine::Sine, Drawable, Point},
};
use image::{imageops::FilterType, io::Reader as ImageReader, GrayImage};
use std::path::Path;

/// Core crate component, takes a source image, resizes it to a number of cells, and plots those
/// cells to the canvas using sine waves.
pub struct Plotter {
    source: GrayImage,
    pub canvas: Canvas,
}

impl Plotter {
    pub fn new<P: AsRef<Path>>(nw: u32, nh: u32, source: P, scale: u32) -> Self {
        let source = ImageReader::open(source)
            .expect("could not open source image")
            .decode()
            .expect("could not decode source image");

        let nw_scale = nw * 4;

        let target_width = (source.width() * scale / 100 / nw_scale + 1) * nw_scale + 1;
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

    fn cell_to_sine_start_y(&self, cell_y: u32) -> u32 {
        (self.canvas.ih / 2 + self.canvas.ih * (self.source.height() - cell_y - 1))
            / self.source.height()
    }

    pub fn draw(&mut self, thickness: u32) {
        let cw = self.cell_width();
        let qwave = self.quarter_wavelength();
        let amax = self.max_amplitude();
        let mut x: u32;
        let mut y: u32;
        let mut a: u32;
        let mut sine: Sine;

        for cell_y in 0..self.source.height() {
            for cell_x in 0..self.source.width() {
                x = cw * cell_x;

                // calculate every time to avoid period falling behind
                y = self.cell_to_sine_start_y(cell_y);
                a = amax - amax * self.source.get_pixel(cell_x, cell_y).0[0] as u32 / 255;
                sine = Sine::new(Point::new(x, y), a, qwave);
                sine.draw_thick(&mut self.canvas, thickness)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::XYDrawable;

    #[test]
    #[ignore = "visual check"]
    fn logo() {
        let mut plotter = Plotter::new(50, 50, "tests/lincoln.jpeg", 100);
        plotter.draw(4);
        plotter.canvas.save("tests/lincoln_sine.jpg");
    }
}
