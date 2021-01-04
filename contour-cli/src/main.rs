extern crate contour;
extern crate clap;

use contour::isoline_from_tiff;
use clap::Clap;

#[derive(Clap)]
struct Opts {
    /// Input tiff file path
    #[clap(short, long)]
    input: String,
    /// Output svg file path
    #[clap(short, long)]
    output: String,
    /// Number of equally spaced isolines to draw
    #[clap(short, long)]
    num_lines: u32
}

use std::fs;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let opts: Opts = Opts::parse();

    let img_bytes = fs::read(opts.input).expect("Issue reading input file");
    let thresholds = (10..100).step_by(10).map(|x| x as f32).collect::<Vec<f32>>();

    let svg = isoline_from_tiff(&img_bytes, &thresholds);

    let svg_string = format!("{}", svg);

    let mut file = match File::create(&opts.output) {
        Err(why) => panic!("couldn't create {}: {}", opts.output, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(svg_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", opts.output, why),
        Ok(_) => println!("successfully wrote to {}", opts.output),
    }
}
