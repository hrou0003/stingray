use crate::geo::{point::Point, ray::Ray};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }

    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let b = (oc * 2.) * ray.direction;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return vec![];
        }

        let t = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        vec![t, t2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::{objects::hit::Hit, ray::Ray, vector::Vector};

    #[test]
    fn test_sphere_intersection() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));

        assert_eq!(sphere.intersect(&ray), vec![5., 5.]);
    }

    #[test]
    fn test_sphere_intersection_ray_inside() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));

        assert_eq!(sphere.intersect(&ray), vec![-1., 1.]);
    }

    #[test]
    fn test_sphere_intersection_behind_ray() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));

        assert_eq!(sphere.intersect(&ray), vec![-6., -4.]);
    }

    #[test]
    fn test_hit_behind() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.0);
        let ray = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));

        let intersect = sphere.intersect(&ray);

        assert_eq!(intersect.hit(), None);
    }
}
