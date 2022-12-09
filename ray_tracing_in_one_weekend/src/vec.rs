use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

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
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn dot(&self, v: &Vec3) -> f64 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] + v.e[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * v.e[2] - self.e[2] * v.e[1],
                self.e[2] * v.e[0] - self.e[0] * v.e[2],
                self.e[0] * v.e[1] - self.e[1] * v.e[0],
            ],
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn print(&self) -> String {
        format!("{} {} {}", self.x(), self.y(), self.z(),)
    }

    pub fn print_png(&self) -> Vec<u8> {
        let x = (self.x() * 255.999).floor() as u8;
        let y = (self.y() * 255.999).floor() as u8;
        let z = (self.z() * 255.999).floor() as u8;
        vec![x, y, z, 255]
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
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
    fn add(self, v: Vec3) -> Self::Output {
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
    fn sub(self, v: Vec3) -> Self::Output {
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
    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            e: [self[0] * t, self[1] * t, self[2] * t],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
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
    fn div(self, t: f64) -> Self::Output {
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
