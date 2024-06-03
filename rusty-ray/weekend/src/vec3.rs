use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut}; // Allows us to define operations on vectors
use std::fmt::{Display, Formatter, Result}; // Allows us to define how to format the output of a vector

// Uses Vec3 instead of vec3 which the linter likes better qwq

#[derive(Clone, Copy, Debug, PartialEq)] // Gets these impls out of the way
pub struct Vec3 { // A struct to represent a 3D vector, done this way instead of list of floats for clarity
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { // Constructor
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// https://doc.rust-lang.org/book/operators-and-overloading.html

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 } 
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 { // returns reference to the value at the index which is x/y/z
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 { // returns Mutable reference to the value at the index
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        self * (1.0 / scalar)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    // different method unit_vector(vec) -> vec
    // instead of vec.unit_vector() -> vec
    *v / v.length()
}


pub type Point3 = Vec3; // alias for Vec3, used to represent a point in 3D space

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length(), (14.0 as f64).sqrt());
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.length_squared(), 14.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let unit_v = v.unit_vector();
        assert_eq!(unit_v.length(), 1.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let cross_v = v1.cross(&v2);
        assert_eq!(cross_v.x, -3.0);
        assert_eq!(cross_v.y, 6.0);
        assert_eq!(cross_v.z, -3.0);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let neg_v = -v;
        assert_eq!(neg_v.x, -1.0);
        assert_eq!(neg_v.y, -2.0);
        assert_eq!(neg_v.z, -3.0);
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[0] = 4.0;
        v[1] = 5.0;
        v[2] = 6.0;
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, 5.0);
        assert_eq!(v.z, 6.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let sum_v = v1 + v2;
        assert_eq!(sum_v.x, 5.0);
        assert_eq!(sum_v.y, 7.0);
        assert_eq!(sum_v.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let diff_v = v1 - v2;
        assert_eq!(diff_v.x, 3.0);
        assert_eq!(diff_v.y, 3.0);
        assert_eq!(diff_v.z, 3.0);
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let scaled_v = v * 2.0;
        assert_eq!(scaled_v.x, 2.0);
        assert_eq!(scaled_v.y, 4.0);
        assert_eq!(scaled_v.z, 6.0);
    }

    #[test]
    fn test_mul_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let scaled_v = v2 * v1;
        assert_eq!(scaled_v.x, 4.0);
        assert_eq!(scaled_v.y, 10.0);
        assert_eq!(scaled_v.z, 18.0);
    }

    #[test]
    fn test_div() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let div_v = v / 2.0;
        assert_eq!(div_v.x, 1.0);
        assert_eq!(div_v.y, 2.0);
        assert_eq!(div_v.z, 3.0);
    }

    #[test]
    fn test_display() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", v), "1 2 3");
    }

    #[test]
    fn test_dot_function() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(dot(&v1, &v2), 32.0);
    }

    #[test]
    fn test_cross_function() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let cross_v = cross(&v1, &v2);
        assert_eq!(cross_v.x, -3.0);
        assert_eq!(cross_v.y, 6.0);
        assert_eq!(cross_v.z, -3.0);
    }

    #[test]
    fn test_unit_vector_function() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let unit_v = unit_vector(&v);
        assert_eq!(unit_v.length(), 1.0);
    }
}