use std::fs::File;
use std::io::{BufWriter, Error};

mod pnm;
mod vec;

use crate::pnm::Ppm;
use crate::vec::Vec3;

struct Ray {
    origin: Vec3,
    normal: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.normal
    }
}

#[derive(Clone, Copy)]
struct Interval {
    min: f64,
    max: f64,
}

#[allow(dead_code)]
impl Interval {
    fn from(min: f64) -> Interval {
        Interval {
            min,
            max: f64::INFINITY,
        }
    }

    fn to(max: f64) -> Interval {
        Interval {
            min: f64::NEG_INFINITY,
            max,
        }
    }

    fn all() -> Interval {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    fn pos() -> Interval {
        Interval {
            min: 0.0,
            max: f64::INFINITY,
        }
    }

    fn neg() -> Interval {
        Interval {
            min: f64::NEG_INFINITY,
            max: 0.0,
        }
    }

    fn contains(&self, value: &f64) -> bool {
        self.min <= *value && *value <= self.max
    }
}

trait Visible {
    fn intersect(&self, ray: &Ray, within: Interval) -> Option<(f64, Ray)>;
}

impl<T> Visible for Vec<T>
where
    T: Visible,
{
    fn intersect(&self, ray: &Ray, within: Interval) -> Option<(f64, Ray)> {
        self.iter()
            .filter_map(|obj| obj.intersect(ray, within))
            .filter(|x| within.contains(&x.0))
            .min_by(|x, y| x.0.total_cmp(&y.0))
    }
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Visible for Sphere {
    fn intersect(&self, ray: &Ray, within: Interval) -> Option<(f64, Ray)> {
        let co = self.center - ray.origin;

        let a = ray.normal.dot(&ray.normal);
        let h = co.dot(&ray.normal); // = -b/2
        let c = co.dot(&co) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Choose the closer solution
        let sqrtd = discriminant.sqrt();
        let t = [(h - sqrtd) / a, (h + sqrtd) / a]
            .into_iter()
            .find(|t| within.contains(t))?;

        let point = ray.at(t);
        let normal = (point - co) / self.radius;
        Some((
            t,
            Ray {
                origin: point,
                normal,
            },
        ))
    }
}

fn rgb_normal(ray: Vec3) -> (u8, u8, u8) {
    let ray = ray.norm();
    let scaled = 0.5 * (ray + 1.0);
    (
        (scaled.x * u8::MAX as f64) as u8,
        (scaled.y * u8::MAX as f64) as u8,
        (scaled.z * u8::MAX as f64) as u8,
    )
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

    let mut image = Ppm::new(image_width, image_height, max_value);

    // --- Camera Setup ---
    let camera_width = unit_width as f64;
    let camera_height = unit_height as f64;
    let lower_left = Vec3::new(-camera_width / 2.0, -camera_height / 2.0, -1.0);
    let camera_horizontal = Vec3::x_vec(camera_width);
    let camera_vertical = Vec3::y_vec(camera_height);
    let origin = Vec3::z_vec(0.0);

    // --- Sphere Setup ---
    let front_sphere = Sphere {
        center: Vec3::z_vec(-1.0),
        radius: 0.5,
    };
    let back_sphere = Sphere {
        center: Vec3 {
            x: -1.0,
            y: 1.0,
            z: -2.0,
        },
        radius: 0.5,
    };

    let world = vec![front_sphere, back_sphere];

    // Some fake data to help me get started
    for row in 0..image_height {
        for col in 0..image_width {
            let idx = row * image_width + col;
            let u = row as f64 / image_height as f64;
            let v = col as f64 / image_width as f64;
            let direction = lower_left + camera_vertical * u + camera_horizontal * v - origin;
            let ray = Ray {
                origin,
                normal: direction,
            };
            let pixel_value = match world.intersect(&ray, Interval::all()) {
                None => (125, 125, 125),
                Some((_, reflection)) => rgb_normal(reflection.normal),
            };
            image.data[idx] = pixel_value;
        }
    }

    // Contents
    image.write(&mut output_buffer)?;

    Ok(())
}
