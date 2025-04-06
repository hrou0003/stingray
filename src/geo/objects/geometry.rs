use crate::geo::{matrix::Matrix, point::Point, ray::Ray, vector::Vector};

pub trait Geometry {
    fn set_transform(&mut self, transform: Matrix);
    fn intersect(&self, ray: &Ray) -> Result<Vec<f64>, String>;
    fn normal(&self, point: Point) -> Result<Vector, String>;
}
