mod ray;
mod vector;

use image::{ImageBuffer, Rgb};
use ray::Ray;
use std::io::{self, Write};
use vector::Vec3;

use crate::vector::Point3;

struct Image {
    height: u32,
    width: u32,
}
fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400 as u32;

    // calculate the hight and width
    let image_height = image_width / aspect_ratio as u32;
    let image_height = if image_height > 1 { image_height } else { 1 };

    // camera
    let focal_legnth = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    //calculate the vetors across the horizontal and vertical viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    //calculate uv deltas from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    //calcultate the location of the 00pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_legnth) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut img_buffer = ImageBuffer::new(image_width, image_height);

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        eprint!("\rScanlines remaining: {} ", image_height - y);
        io::stderr().flush()?;
        let pixel_center = pixel00_loc + (y as f64 * pixel_delta_u) + (x as f64 * pixel_delta_v);
        let ray_direction = pixel_center - camera_center;
        let ray = Ray::new(camera_center, ray_direction);
        let color = ray_color(&ray);
        *pixel = Rgb([color.x() as u8, color.y() as u8, color.z() as u8]);
    }

    // Save the image as "output.png", the format is deduced from the path
    img_buffer.save("output.png")?;

    eprintln!("\nDone.");
    Ok(())
}
