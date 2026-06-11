use std::fs::File;
use std::io::{BufWriter, Error};

mod pnm;
mod vec;

use crate::pnm::PPM;
use crate::vec::Vec3;

fn coloured(ray: Vec3) -> (u8, u8, u8) {
    let scaled = 0.5 * (1.0 - ray.norm());
    (
        (scaled.x * u8::MAX as f64) as u8,
        (scaled.y * u8::MAX as f64) as u8,
        (scaled.z * u8::MAX as f64) as u8,
    )
}

fn hit_sphere(center: Vec3, radius: f64, ray_origin: Vec3, ray_dir: Vec3) -> Option<Vec3> {
    // (center - (ray_origin + t*ray_dir))^2 == radius^2
    // ((center - ray_origin) - t*ray_dir)^2 == radius^2
    // (co).dot(co) - 2.0 * t*co.dot(ray_dir) + t^2*ray_dir.dot(ray_dir)
    let co = center - ray_origin;
    let a = ray_dir.dot(&ray_dir);
    let h = co.dot(&ray_dir); // = -b/2
    let c = co.dot(&co) - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return None;
    }
    // Always use a positive solution
    let t = (h + discriminant.sqrt()) / a;
    Some(ray_origin + ray_dir * t)
}

fn main() -> Result<(), Error> {
    // --- No libraries at first! --

    let pixel_per_unit = 500;

    let unit_width = 4;
    let unit_height = 2;

    // Let's write a simple PBM file
    let image_width = pixel_per_unit * unit_width;
    let image_height = pixel_per_unit * unit_height;
    let max_value = 255;

    let filename = "render.ppm";
    let output = File::create(filename)?;
    let mut output_buffer = BufWriter::new(output);

    let mut image = PPM::new(image_width, image_height, max_value);

    // --- Camera Setup ---
    let camera_width = unit_width as f64;
    let camera_height = unit_height as f64;
    let lower_left = Vec3::new(-camera_width / 2.0, -camera_height / 2.0, -1.0);
    let camera_horizontal = Vec3::x_vec(camera_width);
    let camera_vertical = Vec3::y_vec(camera_height);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // --- Sphere Setup ---
    let sphere_center = Vec3::new(0.0, 0.0, 5.0);
    let sphere_radius = 2.4;

    // Some fake data to help me get started
    for row in 0..image_height {
        for col in 0..image_width {
            let idx = row * image_width + col;
            let u = row as f64 / image_height as f64;
            let v = col as f64 / image_width as f64;
            let direction = lower_left + camera_vertical * u + camera_horizontal * v - origin;
            let hit_point = hit_sphere(sphere_center, sphere_radius, origin, direction);

            let pixel_value = match hit_point {
                None => (125, 125, 125),
                Some(point) => coloured(point - sphere_center),
            };
            image.data[idx] = pixel_value;
        }
    }

    // Contents
    image.write(&mut output_buffer)?;

    Ok(())
}
