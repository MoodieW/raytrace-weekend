mod vector;

use image::{ImageBuffer, Rgb};
use std::io::{self, Write};
use vector::Vec3;

struct Image {
    height: u32,
    width: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = Image {
        height: 256,
        width: 256,
    };

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut img_buffer = ImageBuffer::new(img.width, img.height);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        eprint!("\rScanlines remaining: {} ", img.height - y);
        io::stderr().flush()?;

        let r = x as f64 / (img.width - 1) as f64;
        let g = y as f64 / (img.height - 1) as f64;
        let b = 0.0;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    // Save the image as "output.png", the format is deduced from the path
    img_buffer.save("output.png")?;

    eprintln!("\nDone.");
    Ok(())
}
