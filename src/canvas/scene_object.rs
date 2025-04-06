use crate::{geo::objects::geometry::Geometry, lighting::Texture};

pub struct SceneObject<G: Geometry, T: Texture> {
    pub geometry: G,
    pub material: T,
}

impl<G: Geometry, T: Texture> SceneObject<G, T> {
    pub fn new(geometry: G, material: T) -> Self {
        Self { geometry, material }
    }
}

mod tests {
    use super::*;
    use crate::{
        color::Color,
        geo::{
            matrix::Matrix, objects::geometry::Geometry, objects::sphere::Sphere, point::Point,
            ray::Ray, vector::Vector,
        },
        lighting::{Material, PointLight, Texture},
    };

    #[test]
    fn test_scene_object() {
        let sphere = Sphere::new(Point::new(0., 0., 0.), 1.);
        let color = Color::new(0.5, 0.5, 0.5);
        let material = Material::new(color, 0.5, 0.5, 0.5, 10.);
        let scene_object = SceneObject::new(sphere, material);
    }
}
