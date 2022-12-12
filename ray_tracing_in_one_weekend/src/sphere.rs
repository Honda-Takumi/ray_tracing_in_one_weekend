use super::hit::Hit;
use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::{Point3, Vec3};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        // if discriminant > 0. {
        //     let root = discriminant.sqrt();
        //     let tmp = (-half_b - root) / a;

        //     if tmp < t_max && tmp > t_min {
        //         rec.t = tmp;
        //         rec.p = r.at(rec.t);
        //         // rec.normal = (rec.p - self.center) / self.radius;
        //         let outward_normal = (rec.p - self.center) / self.radius;
        //         rec.set_face_normal(r, outward_normal);
        //         return true;
        //     }
        //     let tmp = (-half_b + root) / a;
        //     if tmp < t_max && tmp > t_min {
        //         rec.t = tmp;
        //         rec.p = r.at(rec.t);
        //         // rec.normal = (rec.p - self.center) / self.radius;
        //         let outward_normal = (rec.p - self.center) / self.radius;
        //         rec.set_face_normal(r, outward_normal);
        //         return true;
        //     }
        // }
        // false
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }
        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            normal: Vec3::new(0., 0., 0.),
            front_face: false,
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
