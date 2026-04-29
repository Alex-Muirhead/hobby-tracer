use std::fmt::Display;
use std::fs::File;
use std::io::{Error, Write};
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
struct PGM {
    width: usize,
    height: usize,
    max_value: u8,
    data: Vec<u8>,
}

impl PGM {
    fn new(width: usize, height: usize, max_value: u8) -> Self {
        let data = vec![0; width * height];
        PGM {
            width,
            height,
            max_value,
            data,
        }
    }

    fn write(&self, output: &mut File) -> Result<(), Error> {
        // Header
        write!(
            output,
            "P2\n{} {}\n{}\n",
            self.width, self.height, self.max_value
        )?;
        // Data
        for idx in 0..(self.width * self.height) {
            write!(output, "{} ", self.data[idx])?;
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }
    fn norm(&self) -> Vec3 {
        let l = self.length();
        Vec3::new(self.x / l, self.y / l, self.z / l)
    }
    fn x_vec(x: f64) -> Self {
        Vec3 { x, y: 0.0, z: 0.0 }
    }
    fn y_vec(y: f64) -> Self {
        Vec3 { x: 0.0, y, z: 0.0 }
    }
    fn z_vec(z: f64) -> Self {
        Vec3 { x: 0.0, y: 0.0, z }
    }
}

fn grayscale(ray: Vec3) -> u8 {
    let ray = ray.norm();
    // normed y-component will be in [-1.0, +1.0]
    let scaled = 0.5 * (1.0 - ray.z);
    // Scaled is now in [0.0, 1.0]
    (scaled * u8::MAX as f64) as u8
}

fn hit_sphere(center: Vec3, radius: f64, ray_origin: Vec3, ray_dir: Vec3) -> Option<Vec3> {
    // (center - (ray_origin + t*ray_dir))^2 == radius^2
    // ((center - ray_origin) - t*ray_dir)^2 == radius^2
    // (co).dot(co) - 2.0 * t*co.dot(ray_dir) + t^2*ray_dir.dot(ray_dir)
    let co = center - ray_origin;
    let a = ray_dir.dot(&ray_dir);
    let b = -2.0 * co.dot(&ray_dir);
    let c = co.dot(&co) - radius * radius;
    let determinant = b * b - 4.0 * a * c;
    if determinant < 0.0 {
        return None;
    }
    // Always use a positive solution
    let t = (-b + determinant.sqrt()) / (2.0 * a);
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

    let filename = "render.pgm";
    let mut output = File::create(filename)?;

    let mut image = PGM::new(image_width, image_height, max_value);

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
                None => 125,
                Some(point) => grayscale(point - sphere_center),
            };
            image.data[idx] = pixel_value;
        }
    }

    // Contents
    image.write(&mut output)?;

    Ok(())
}
