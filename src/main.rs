mod canvas;
mod curves;
mod plotter;

use canvas::XYDrawable;
use clap::Parser;
use plotter::Plotter;
use std::path::PathBuf;

/// Plots images as sinewave art, inspired by /u/tfoust10's Reddit posts.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Source image.
    input: PathBuf,

    /// Number of rows of sine waves.
    #[clap(short = 'i', default_value = "50")]
    vcells: u32,

    /// Number of sine oscillations.
    #[clap(short = 'j', default_value = "50")]
    hcells: u32,

    /// Percentage scaling of image resolution.
    #[clap(short = 's', long = "scale", default_value = "100")]
    scale: u32,

    /// Thickness of line in pixels.
    #[clap(short = 't', long = "thickness", default_value = "4")]
    thickness: u32,

    /// Output image path. Defaults to $INPUT_sine.jpg.
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let output = args.output.unwrap_or_else(|| {
        args.input.with_file_name(format!(
            "{}_sine.jpg",
            args.input
                .file_stem()
                .expect("could not read input file stem")
                .to_str()
                .expect("input file stem is not valid unicode")
        ))
    });
    let mut plotter = Plotter::new(args.hcells, args.vcells, args.input, args.scale);
    plotter.draw(args.thickness);
    plotter.canvas.save(output);
}
