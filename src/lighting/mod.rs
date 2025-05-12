use crate::{
    assert_fractional,
    color::Color,
    geo::{objects::sphere::Sphere, point::Point},
};

pub mod texture;

pub struct PointLight {
    pub position: Point,
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        assert_fractional!(ambient);
        assert_fractional!(diffuse);
        assert_fractional!(specular);
        assert_fractional!(shininess);
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}
