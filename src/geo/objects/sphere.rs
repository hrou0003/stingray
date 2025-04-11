use std::ops::Mul;

use crate::geo::{matrix::Matrix, point::Point, ray::Ray, vector::Vector};

use super::geometry::Geometry;

pub struct Sphere {
    center: Point,
    radius: f64,
    transformation: Matrix,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere {
            center,
            radius,
            transformation: Matrix::identity(),
        }
    }

    pub fn unit_sphere() -> Self {
        Sphere::new(Point::new(0., 0., 0.), 1.)
    }
}

impl Geometry for Sphere {
    fn set_transform(&mut self, transformation: Matrix) {
        self.transformation = transformation * self.transformation.clone();
    }

    fn intersect(&self, ray: &Ray) -> Result<Vec<f64>, String> {
        // Transform the ray by the inverse of the sphere's transformation
        let inverse = self.transformation.inverse()?;
        let ray = inverse * ray;

        let oc = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let b = (oc * 2.) * ray.direction;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return Ok(vec![]);
        }

        let t = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        Ok(vec![t, t2])
    }

    fn normal(&self, point: Point) -> Result<Vector, String> {
        let transform_inverse = dbg!(self.transformation.inverse()?);
        let object_point = transform_inverse.clone() * point;
        let object_normal = (object_point - self.center).norm();
        let world_normal = transform_inverse.transpose()? * object_normal;
        Ok(world_normal.norm())
    }
}

impl Mul<&Matrix> for Sphere {
    type Output = Sphere;

    fn mul(self, rhs: &Matrix) -> Sphere {
        Sphere {
            center: self.center,
            radius: self.radius,
            transformation: self.transformation * rhs.clone(),
        }
    }
}

impl Mul<Sphere> for Matrix {
    type Output = Sphere;

    fn mul(self, rhs: Sphere) -> Sphere {
        Sphere {
            center: rhs.center,
            radius: rhs.radius,
            transformation: self * rhs.transformation,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{
        geo::{matrix::Rotation, objects::hit::Hit, ray::Ray, vector::Vector},
        traits::SpecificRound,
    };

    #[test]
    fn test_sphere_intersection() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));

        assert_eq!(sphere.intersect(&ray).unwrap(), vec![5., 5.]);
    }

    #[test]
    fn test_sphere_intersection_ray_inside() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));

        assert_eq!(sphere.intersect(&ray).unwrap(), vec![-1., 1.]);
    }

    #[test]
    fn test_sphere_intersection_behind_ray() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));

        assert_eq!(sphere.intersect(&ray).unwrap(), vec![-6., -4.]);
    }

    #[test]
    fn test_hit_behind() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));

        let intersect = sphere.intersect(&ray).unwrap();

        assert_eq!(intersect.hit(), None);
    }

    #[test]
    fn test_transformation() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let scaling = Matrix::scaling(2., 2., 2.);
        let sphere = scaling.clone() * sphere;

        assert_eq!(sphere.radius, 1.);
        assert_eq!(sphere.transformation, scaling);
    }

    #[test]
    fn scale_ray() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let scaling = Matrix::scaling(2., 2., 2.);
        let sphere = scaling * sphere;
        let intersect = sphere.intersect(&ray).unwrap();

        assert_eq!(intersect.len(), 2);
        assert_eq!(intersect[0], 3.);
        assert_eq!(intersect[1], 7.);
    }

    #[test]
    fn translate_ray() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let translation = Matrix::translation(5., 0., 0.);
        let sphere = translation * sphere;
        let intersect = sphere.intersect(&ray).unwrap();

        assert_eq!(intersect.len(), 0);
    }

    #[test]
    fn test_normal() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);

        assert_eq!(
            sphere.normal(Point::new(1., 0., 0.)).unwrap(),
            Vector::new(1., 0., 0.)
        );
    }

    #[test]
    fn test_normal_nonaxial() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let sqrt_3 = f64::sqrt(3.);

        assert_eq!(
            sphere
                .normal(Point::new(sqrt_3 / 3., sqrt_3 / 3., sqrt_3 / 3.))
                .unwrap(),
            Vector::new(sqrt_3 / 3., sqrt_3 / 3., sqrt_3 / 3.)
        );

        assert_eq!(
            sphere
                .normal(Point::new(sqrt_3 / 3., -sqrt_3 / 3., sqrt_3 / 3.))
                .unwrap(),
            Vector::new(sqrt_3 / 3., -sqrt_3 / 3., sqrt_3 / 3.).norm()
        );
    }

    #[test]
    fn test_normal_transformed() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let translation = Matrix::translation(0., 1., 0.);

        let sphere = translation * sphere;
        let normal = sphere.normal(Point::new(0., 1.70711, -0.70711)).unwrap();

        assert_eq!(normal.x.specific_round(3), 0.);
        assert_eq!(normal.y.specific_round(5), 0.70711);
        assert_eq!(normal.z.specific_round(5), -0.70711);
    }

    #[test]
    fn test_normal_transformed_inverse() {
        let mut sphere = Sphere::unit_sphere();
        let m = Matrix::scaling(1., 0.5, 1.) * Matrix::rotation(PI / 5., Rotation::Z);
        sphere.set_transform(m);
        let normal = sphere
            .normal(Point::new(0., f64::sqrt(2.) / 2., -f64::sqrt(2.) / 2.))
            .unwrap();
        assert_eq!(normal.x.specific_round(3), 0.);
        assert_eq!(normal.y.specific_round(5), 0.97014);
        assert_eq!(normal.z.specific_round(5), -0.24254);
    }
}
