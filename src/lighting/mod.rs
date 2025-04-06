use crate::{
    color::Color,
    geo::{objects::sphere::Sphere, point::Point},
};

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

pub trait Texture {
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> Material;
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Texture for Material {
    fn set_material(&mut self, material: Material) {
        *self = material;
    }

    fn get_material(&self) -> Material {
        *self
    }
}
