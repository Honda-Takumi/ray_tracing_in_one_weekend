use super::sphere::Sphere;

use super::ray::Ray;
use super::vec::{Point3, Vec3};

// pub type World = Vec<Box<dyn Hit>>;

#[derive(Clone)]
pub enum Type {
    World(World),
    Sphere(Sphere),
}

impl Type {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::World(world) => world.hit(r, t_min, t_max),
            Self::Sphere(sphere) => sphere.hit(r, t_min, t_max),
        }
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1. * outward_normal
        };
    }
}

#[derive(Clone)]
pub struct World {
    pub center: Point3,
    pub radius: f64,
}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        // for object in self {
        //     if let Some(rec) = object.hit(r, t_min, closest_so_far) {
        if let Some(rec) = Sphere::hit(
            &Sphere {
                center: self.center,
                radius: self.radius,
            },
            r,
            t_min,
            closest_so_far,
        ) {
            closest_so_far = rec.t;
            tmp_rec = Some(rec);
        }
        // }
        // }

        tmp_rec
    }
}
