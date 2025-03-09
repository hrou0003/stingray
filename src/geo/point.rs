use std::ops::{Add, Mul, Neg, Sub};

use super::{matrix::Matrix, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul for Point {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<Matrix> for Point {
    type Output = Point;

    fn mul(self, other: Matrix) -> Point {
        let x = self.x * other[0][0] + self.y * other[1][0] + self.z * other[2][0];
        let y = self.x * other[0][1] + self.y * other[1][1] + self.z * other[2][1];
        let z = self.x * other[0][2] + self.y * other[1][2] + self.z * other[2][2];
        Point { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_points() {
        let p1 = Point::new(3., -2., 5.);
        let p2 = Point::new(2., -1., 4.);
        let p3 = p1 + p2;
        assert_eq!(p3.x, 5.);
        assert_eq!(p3.y, -3.);
        assert_eq!(p3.z, 9.);
    }

    #[test]
    fn add_vector() {
        let p1 = Point::new(3., -2., 5.);
        let v1 = Vector::new(2., -1., 4.);
        let p2 = p1 + v1;
        assert_eq!(p2.x, 5.);
        assert_eq!(p2.y, -3.);
        assert_eq!(p2.z, 9.);
    }
}
