use std::f64::consts::PI;

use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        // const ASPECT_RATIO: f64 = 16.0 / 9.0;
        // const VIEWPORT_HEIGHT: f64 = 2.0;
        let theta = PI / 180. * vfov;
        let viewport_height = 2. * (theta / 2.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(cw).normalized();
        let cv = cw.cross(cu);

        let orig = lookfrom;
        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;
        let llc = orig - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera {
            origin: orig,
            lower_left_corner: llc,
            horizontal: h,
            vertical: v,
            cu: cu,
            cv: cv,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.cu * rd.x() + self.cv * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
