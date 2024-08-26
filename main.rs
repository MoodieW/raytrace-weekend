mod ray;
mod vector;

use ray::Ray;
use std::fs::File;
use std::io::{self, Write};
use vector::Point3;
use vector::{dot, Vec3};

//#[test]
fn ray_color(r: &Ray) -> Vec3 {
    if (hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r)) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
}

fn write_color(file: &mut File, pixel_color: Vec3) -> io::Result<()> {
    let ir = (255.999 * pixel_color.x()) as i32;
    let ig = (255.999 * pixel_color.y()) as i32;
    let ib = (255.999 * pixel_color.z()) as i32;
    writeln!(file, "{} {} {}", ir, ig, ib)
}

fn hit_sphere(center: &Vec3, raduis: f64, ray: &Ray) -> bool {
    let vek: Vec3 = *center - ray.origin();
    let a = dot(ray.direction(), ray.direction());
    let b = -2.0 * dot(ray.direction(), vek);
    let c = dot(vek, vek) - raduis * raduis;
    let disc = b * b - 4.0 * a * c;
    return disc >= 0.0;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the height and ensure it's at least 1
    let image_height = std::cmp::max((image_width as f64 / aspect_ratio) as u32, 1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and vertical viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate pixel delta vectors
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Create and write to the PPM file
    let mut file = File::create("output.ppm")?;

    // Write PPM header
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        io::stderr().flush()?;

        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut file, pixel_color)?;
        }
    }

    eprintln!("\nDone.");
    Ok(())
}
