use std::ops::{Add, Mul};

use super::{matrix::Matrix, point::Point, vector::Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }

    pub fn evaluate(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

impl Add<Matrix> for Ray {
    type Output = Ray;

    fn add(self, rhs: Matrix) -> Ray {
        Ray {
            origin: rhs + self.origin,
            direction: self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray() {
        let ray = Ray::new(Point::new(2., 3., 4.), Vector::new(1., 0., 0.));
        let point = ray.point_at(0.);
        assert_eq!(point, Point::new(2., 3., 4.));
        let point = ray.point_at(1.);
        assert_eq!(point, Point::new(3., 3., 4.));
        let point = ray.point_at(-1.);
        assert_eq!(point, Point::new(1., 3., 4.));
        let point = ray.point_at(2.5);
        assert_eq!(point, Point::new(4.5, 3., 4.));
    }

    #[test]
    fn translate_ray() {
        let ray = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let translation = Matrix::translation(3., 4., 5.);
        let result = translation * ray;
        assert_eq!(result.origin, Point::new(4., 6., 8.));
        assert_eq!(result.direction, Vector::new(0., 1., 0.));
    }

    #[test]
    fn scale_ray() {
        let ray = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let scale = Matrix::scaling(2., 3., 4.);
        let result = scale * ray;
        assert_eq!(result.origin, Point::new(2., 6., 12.));
        assert_eq!(result.direction, Vector::new(0., 3., 0.));
    }
}
