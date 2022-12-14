use std::f64::consts::PI;
use std::fmt;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Sub, SubAssign,
};

use rand::{random, Rng};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }
    pub fn dot(&self, v: Vec3) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * v[2] - self[2] * v[1],
                self[2] * v[0] - self[0] * v[2],
                self[0] * v[1] - self[1] * v[0],
            ],
        }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    pub fn print(&self) -> String {
        format!("{} {} {}", self.x(), self.y(), self.z(),)
    }

    pub fn print_png(&self) -> Vec<u8> {
        let x = (self.x() * 255.999).floor() as u8;
        let y = (self.y() * 255.999).floor() as u8;
        let z = (self.z() * 255.999).floor() as u8;
        assert!(0.0 <= self.x() && self.x() <= 1.0);
        assert!(0.0 <= self.y() && self.y() <= 1.0);
        assert!(0.0 <= self.z() && self.z() <= 1.0);
        vec![x, y, z, 255]
    }

    pub fn print_sample_png(&self, samples_per_pixel: u64) -> Vec<u8> {
        // let x = (256. * (self[0] / (samples_per_pixel as f64)).clamp(0., 0.999)) as u8;
        // let y = (256. * (self[1] / (samples_per_pixel as f64)).clamp(0., 0.999)) as u8;
        // let z = (256. * (self[2] / (samples_per_pixel as f64)).clamp(0., 0.999)) as u8;
        // vec![x, y, z, 255]

        let x = (256.
            * (self[0] / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0., 0.999)) as u8;
        let y = (256.
            * (self[1] / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0., 0.999)) as u8;
        let z = (256.
            * (self[2] / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0., 0.999)) as u8;
        vec![x, y, z, 255]
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            e: [
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.);
            if v.length() < 1. {
                return v;
            }
        }
        // let a = Vec3::random(0.0..2. * PI);
        // let z = Vec3::random(-1.0..1.0);
    }

    pub fn random_in_heimisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {
            in_unit_sphere
        } else {
            (-1.) * in_unit_sphere
        }
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn mul(self, n: Vec3) -> Vec3 {
        Color::new(self[0] * n[0], self[1] * n[1], self[2] * n[2])
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(n);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let mut rng = rand::thread_rng();
            let p = Vec3::new(rng.gen_range(-1.0..1.), rng.gen_range(-1.0..1.), 0.);

            if p.length() < 1. {
                return p;
            }
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.e.len(), "out of length: {}", index);
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        assert!(index < self.e.len(), "out of length: {}", index);
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] + v[0], self[1] + v[1], self[2] + v[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = Vec3 {
            e: [self[0] + v[0], self[1] + v[1], self[2] + v[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - v[0], self[1] - v[1], self[2] - v[2]],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        *self = Vec3 {
            e: [self[0] - v[0], self[1] - v[1], self[2] - v[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self[0] * t, self[1] * t, self[2] * t],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self * v[0], self * v[1], self * v[2]],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        *self = Vec3 {
            e: [self[0] * t, self[1] * t, self[2] * t],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self[0] / t, self[1] / t, self[2] / t],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self = Vec3 {
            e: [self[0] / t, self[1] / t, self[2] / t],
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}
