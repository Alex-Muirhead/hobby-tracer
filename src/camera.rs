#![allow(dead_code)]
use crate::{Ray, vec::Vec3};

#[derive(Copy, Clone)]
enum Length {
    M(f64),
    MM(f64),
    CM(f64),
    INCH(f64),
}

impl Length {
    fn to_m(self) -> f64 {
        match self {
            Length::MM(mm) => mm / 1000.0,
            Length::CM(cm) => cm / 100.0,
            Length::INCH(inch) => inch * 25.4e-3,
            Length::M(metre) => metre,
        }
    }
}

// Measured in pitch maybe?
#[derive(Copy, Clone)]
struct PixelDensity(f64);

impl PixelDensity {
    fn from_dpi(dpi: f64) -> Self {
        let dots_per_mm = dpi * 25.4;
        let pitch = 1000.0 / dots_per_mm;
        Self(pitch)
    }

    #[allow(dead_code)]
    fn from_dpcm(dpcm: f64) -> Self {
        let dots_per_mm = dpcm * 10.0;
        let pitch = 1000.0 / dots_per_mm;
        Self(pitch)
    }

    fn to_dpmm(self) -> f64 {
        1e-3 / self.0
    }
}

#[derive(Copy, Clone)]
struct AspectRatio {
    width: u32,
    height: u32,
}

impl AspectRatio {
    fn scale_to(&self, size: Size) -> (f64, f64) {
        let (width, height) = (self.width as f64, self.height as f64);
        let scaling = match size {
            Size::Diameter(length) => length / f64::sqrt(width.powi(2) + height.powi(2)),
            Size::Width(length) => length / width,
            Size::Height(length) => length / height,
        };
        (width * scaling, height * scaling)
    }
}

#[derive(Clone, Copy)]
pub enum Size {
    Diameter(f64),
    Width(f64),
    Height(f64),
}

pub struct Camera {
    pub origin: Vec3,
    // direction: Vec3,
    // aspect_ratio: AspectRatio,
    // pixel_density: PixelDensity,
    pub resolution: (u32, u32),
    pub size: Size,
    pub focal_length: f64,
}

impl Camera {
    pub fn create_ray(&self, u: f64, v: f64) -> Ray {
        // Future variables
        let direction = Vec3::z_vec(1.0);
        // ----------------

        let (width, height) = AspectRatio {
            width: self.resolution.0,
            height: self.resolution.1,
        }
        .scale_to(self.size);

        let ray_offset =
            width * (u - 0.5) * Vec3::x_vec(1.0) + height * (v - 0.5) * Vec3::y_vec(1.0);

        // Infinite focal length would give NaN values under normalisation
        let ray_direction = if self.focal_length.is_finite() {
            (ray_offset + self.focal_length * direction).unit()
        } else {
            direction
        };
        Ray {
            origin: self.origin + ray_offset,
            direction: ray_direction,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Vec3::zero(),
            // direction: Vec3::z_vec(1.0),
            // aspect_ratio: AspectRatio {
            //     width: 16,
            //     height: 9,
            // },
            // pixel_density: PixelDensity::from_dpi(900.0),
            size: Size::Diameter(1.0),
            resolution: (1600, 900),
            focal_length: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64;

    use super::*;

    #[test]
    fn middle_ray() {
        let camera = Camera::default();
        let ray = camera.create_ray(0.5, 0.5);

        assert_eq!(ray.origin, camera.origin);
        assert_eq!(ray.direction, Vec3::z_vec(1.0));
    }

    #[test]
    fn corner_rays() {
        let camera = Camera {
            resolution: (1, 1),
            size: Size::Height(2.0),
            ..Camera::default()
        };
        let bl_ray = camera.create_ray(0.0, 0.0);
        let tr_ray = camera.create_ray(1.0, 1.0);

        assert_eq!(
            bl_ray.origin,
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0
            }
        );
        assert_eq!(
            bl_ray.direction,
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 1.0
            }
            .unit()
        );
        assert_eq!(
            tr_ray.origin,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0
            }
        );
        assert_eq!(
            tr_ray.direction,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
            .unit()
        );
    }

    #[test]
    fn aspect_ratio() {
        let camera = Camera {
            resolution: (2, 1),
            size: Size::Height(2.0),
            ..Camera::default()
        };
        let bl_ray = camera.create_ray(0.0, 0.0);
        let tr_ray = camera.create_ray(1.0, 1.0);

        assert_eq!(
            bl_ray.origin,
            Vec3 {
                x: -2.0,
                y: -1.0,
                z: 0.0
            }
        );
        assert_eq!(
            bl_ray.direction,
            Vec3 {
                x: -2.0,
                y: -1.0,
                z: 1.0
            }
            .unit()
        );
        assert_eq!(
            tr_ray.origin,
            Vec3 {
                x: 2.0,
                y: 1.0,
                z: 0.0
            }
        );
        assert_eq!(
            tr_ray.direction,
            Vec3 {
                x: 2.0,
                y: 1.0,
                z: 1.0
            }
            .unit()
        );
    }

    #[test]
    fn orthographic_corners() {
        let camera = Camera {
            focal_length: f64::INFINITY,
            ..Camera::default()
        };
        let bl_ray = camera.create_ray(0.0, 0.0);
        let tr_ray = camera.create_ray(1.0, 1.0);

        assert_eq!(bl_ray.direction, Vec3::z_vec(1.0));
        assert_eq!(tr_ray.direction, Vec3::z_vec(1.0));
    }

    #[test]
    fn scaling() {
        let aspect = AspectRatio {
            width: 2000,
            height: 1000,
        };
        let (width, height) = aspect.scale_to(Size::Height(1.0));

        assert_eq!(width, 2.0);
        assert_eq!(height, 1.0);
    }
}
