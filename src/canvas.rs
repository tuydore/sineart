use crate::curves::Point;
use image::{GrayImage, Luma};
use std::path::Path;

pub trait XYDrawable {
    /// Set cartesian (X, Y) coordinates: X == J and Y == -I.
    fn set_xy(&mut self, x: u32, y: u32, value: u8);

    /// Set a point in cartesian coordinates.
    fn set_point(&mut self, point: &Point, value: u8) {
        self.set_xy(point.x, point.y, value);
    }

    fn set_vertical_line(&mut self, point: &Point, value: u8, extent: u32) {
        for y in point.y.saturating_sub(extent)..=point.y + extent {
            self.set_xy(point.x, y, value)
        }
    }

    fn set_horizontal_line(&mut self, point: &Point, value: u8, extent: u32) {
        for x in point.x.saturating_sub(extent)..=point.x + extent {
            self.set_xy(x, point.y, value)
        }
    }

    /// Save the drawable to disk as an image.
    fn save<P: AsRef<Path>>(&self, path: P);
}

#[derive(Debug, Clone)]
pub struct Canvas {
    /// Full width of image, in pixels.
    pub fw: u32,
    /// Full height of image, in pixels.
    pub fh: u32,
    /// Inner width of image, in pixels.
    pub iw: u32,
    /// Inner height of image, in pixels.
    pub ih: u32,
    /// Plotting offset width, when asking to set P(x, y), this must be in the inner image.
    pub ow: u32,
    /// Plotting offset height, when asking to set P(x, y), this must be in the inner image.
    pub oh: u32,
    /// Image buffer.
    image: GrayImage,
}

impl Canvas {
    pub fn new(full_hw: [u32; 2], inner_hw: [u32; 2]) -> Self {
        Self {
            fh: full_hw[0],
            fw: full_hw[1],
            ih: inner_hw[0],
            iw: inner_hw[1],
            oh: (full_hw[0] - inner_hw[0]) / 2,
            ow: (full_hw[1] - inner_hw[1]) / 2,
            image: GrayImage::from_pixel(full_hw[1], full_hw[0], Luma([255])),
        }
    }
}

impl XYDrawable for Canvas {
    fn set_xy(&mut self, x: u32, y: u32, value: u8) {
        self.image
            .put_pixel(x + self.ow, self.fh - 1 - y - self.oh, Luma([value]));
    }

    fn save<P: AsRef<Path>>(&self, path: P) {
        self.image.save(path).expect("failed to save image");
    }
}
