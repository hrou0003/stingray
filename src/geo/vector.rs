use std::ops::{Add, Div, Mul, Neg, Sub};

use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

//
impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm(&self) -> Self {
        self.clone() / self.mag()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(&self, n: Vector) -> Self {
        let dot = self.dot(n);
        self.clone() - n.clone() * (2. * dot)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Point> for Vector {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::SpecificRound;

    use super::*;

    #[test]
    fn test_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn add_vector() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(4., 5., 6.);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 5.);
        assert_eq!(v3.y, 7.);
        assert_eq!(v3.z, 9.);
    }

    #[test]
    fn add_point() {
        let v1 = Vector::new(1., 2., 3.);
        let p1 = Point::new(4., 5., 6.);
        let p2 = v1 + p1;
        assert_eq!(p2.x, 5.);
        assert_eq!(p2.y, 7.);
        assert_eq!(p2.z, 9.);
    }

    #[test]
    fn sub_vector_from_point() {
        let v1 = Vector::new(1., 2., 3.);
        let p1 = Point::new(4., 5., 6.);
        let p2 = p1 - v1;
        assert_eq!(p2.x, 3.);
        assert_eq!(p2.y, 3.);
        assert_eq!(p2.z, 3.);
    }

    #[test]
    fn negation() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = -v1;
        assert_eq!(v2.x, -1.);
        assert_eq!(v2.y, -2.);
        assert_eq!(v2.z, -3.);
    }

    #[test]
    fn mag() {
        let v = Vector::new(0., 1., 0.);
        assert_eq!(v.mag(), 1.);

        let v = Vector::new(0., 0., 1.);
        assert_eq!(v.mag(), 1.);

        let v = Vector::new(1., 2., 3.);
        assert_eq!(v.mag(), (14.0_f64).sqrt());

        let v = -Vector::new(1., 2., 3.);
        assert_eq!(v.mag(), (14.0_f64).sqrt());
    }

    #[test]
    fn norm() {
        let v = Vector::new(0., 1., 0.);
        assert_eq!(v.norm(), Vector::new(0., 1., 0.));

        let v = Vector::new(0., 0., 4.);
        assert_eq!(v.norm(), Vector::new(0., 0., 1.));

        let v = Vector::new(1., 2., 3.);
        assert_eq!(
            v.norm(),
            Vector::new(
                1. / 14.0_f64.sqrt(),
                2. / 14.0_f64.sqrt(),
                3. / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn dot() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(4., 5., 6.);
        assert_eq!(v1 * v2, 32.);
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(2., 3., 4.);
        let v3 = v1.cross(v2);
        assert_eq!(v3.x, -1.);
        assert_eq!(v3.y, 2.);
        assert_eq!(v3.z, -1.);

        let v4 = v2.cross(v1);
        assert_eq!(v4.x, 1.);
        assert_eq!(v4.y, -2.);
        assert_eq!(v4.z, 1.);
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Projectile {
        pub position: Point,
        pub velocity: Vector,
    }

    struct Environment {
        pub gravity: Vector,
        pub wind: Vector,
        pub projectile: Projectile,
    }

    impl Environment {
        pub fn new(gravity: Vector, wind: Vector, position: Point, velocity: Vector) -> Self {
            let projectile = Projectile { position, velocity };
            Environment {
                gravity,
                wind,
                projectile,
            }
        }

        pub fn tick(&mut self) {
            let position = self.projectile.position + self.projectile.velocity;
            let velocity = self.projectile.velocity + self.gravity + self.wind;
            self.projectile = Projectile { position, velocity };
        }
    }

    #[test]
    fn test_environment() {
        let gravity = Vector::new(0., -0.1, 0.);
        let wind = Vector::new(-0.01, 0., 0.);
        let position = Point::new(0., 1., 0.);
        let velocity = Vector::new(1., 1., 0.).norm();
        let mut env = Environment::new(gravity, wind, position, velocity);
        while env.projectile.position.y > 0. {
            env.tick();
            dbg!(&env.projectile.position);
        }
        assert!(env.projectile.position.y < 0.);
    }

    #[test]
    fn test_reflect_vector() {
        let v = Vector::new(0., -1., 0.);
        let n = Vector::new(f64::sqrt(2.) / 2., f64::sqrt(2.) / 2., 0.);
        let r = v.reflect(n);
        assert_eq!(r.x.specific_round(3), 1.);
        assert_eq!(r.y.specific_round(3), 0.);
        assert_eq!(r.z.specific_round(3), 0.);
    }
}
