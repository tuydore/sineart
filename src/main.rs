use canvas::Canvas;
use curves::{sine::Sine, Drawable, Point};
use image::{io::Reader as ImageReader, GrayImage};
use std::path::Path;

mod canvas;
mod curves;

struct SineArt {
    source: GrayImage,
    canvas: Canvas,
}

impl SineArt {
    fn new<P: AsRef<Path>>(nwidth: u32, nheight: u32, source_image: P, canvas: Canvas) -> Self {
        let source = ImageReader::open(source_image)
            .expect("could not open source image")
            .decode()
            .expect("could not decode source image")
            .resize_exact(nwidth, nheight, image::imageops::FilterType::Gaussian)
            .into_luma8();
        Self { source, canvas }
    }

    fn from_source_and_border<P: AsRef<Path>>(
        nwidth: u32,
        nheight: u32,
        source_image: P,
        border_percent: u32,
    ) -> Self {
        let source = ImageReader::open(source_image)
            .expect("could not open source image")
            .decode()
            .expect("could not decode source image");

        let full_width = source.width() * (100 + border_percent) / 100;
        let full_height = source.height() * (100 + border_percent) / 100;
        let canvas = Canvas::new([full_height, full_width], [source.height(), source.width()]);
        Self {
            source: source
                .resize_exact(nwidth, nheight, image::imageops::FilterType::Gaussian)
                .into_luma8(),
            canvas,
        }
    }

    fn cell_height(&self) -> u32 {
        self.canvas.ih / self.source.height()
    }

    fn cell_width(&self) -> u32 {
        self.canvas.iw / self.source.width()
    }

    /// Return the max amplitude a sine wave can have. A_max = 0.9 x cell_height / 2.
    fn max_amplitude(&self) -> u32 {
        self.cell_height() * 9 / 20
    }

    fn quarter_wavelength(&self) -> u32 {
        // TODO: add division checks
        self.cell_width() / 4
    }

    fn draw_on_canvas(&mut self) {
        let ch = self.cell_height();
        let cw = self.cell_width();
        let qwave = self.quarter_wavelength();
        let amax = self.max_amplitude();

        for img_y in 0..self.source.height() {
            for img_x in 0..self.source.width() {
                let x = cw * img_x;
                let y = ch / 2 + ch * (self.source.height() - img_y - 1);
                let a = amax - amax * self.source.get_pixel(img_x, img_y).0[0] as u32 / 255;
                let sine = Sine::new(Point::new(x, y), a, qwave);
                sine.draw(&mut self.canvas)
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
        let mut art = SineArt::from_source_and_border(70, 35, "tests/lincoln.jpeg", 5);
        art.draw_on_canvas();
        art.canvas.save("tests/lincoln_sine.jpg");
    }
}
