// Author: Cade DuPont
// Date: 29 January 2024
// Description: Main file for ray tracer project

fn main() {
    // Declare image width and height
    let width: i32 = 256;
    let height: i32 = 256;

    // Print header for PPM image
    println!("P3\n{} {}\n255\n", width, height);

    // Loop through each pixel and print color
    for j in (0..height).rev() {
        for i in 0..width {
            // Calculate color for each pixel from 0.0 to 1.0
            let r: f32 = (i as f32) / ((width - 1) as f32);
            let g: f32 = (j as f32) / ((height - 1) as f32);
            let b: f32 = 0.25;

            // Convert color to 0-255 range for printing to PPM image
            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            // Print color to PPM image
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
