// Author: Cade DuPont
// Date: 29 January 2024
// Description: Main file for ray tracer project

use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const MAX_COLOR: u32 = 255;

fn main() -> io::Result<()> {
    // Map each each pixel to a color, join the pixels' color values into a single string
    let pixels = (0..HEIGHT)
        .cartesian_product(0..WIDTH)
        .progress_count(HEIGHT as u64 * WIDTH as u64)
        .map(|(y, x)| {
            let r = x as f64 / (WIDTH - 1) as f64;
            let g = y as f64 / (HEIGHT - 1) as f64;
            let b = 0.25;

            format!(
                "{} {} {}\n",
                (MAX_COLOR as f64 * r) as u32,
                (MAX_COLOR as f64 * g) as u32,
                (MAX_COLOR as f64 * b) as u32
            )
        })
        .collect::<Vec<String>>()
        .join("");

    // Write the pixels to a PPM file
    fs::write(
        "image.ppm",
        format!(
            "P3\n{} {}\n{}\n{}",
            WIDTH, HEIGHT, MAX_COLOR, pixels
        )
    )
}
