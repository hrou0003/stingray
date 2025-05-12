use super::Material;

pub trait Texture {
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> Material;
}

impl Texture for Material {
    fn set_material(&mut self, material: Material) {
        *self = material;
    }

    fn get_material(&self) -> Material {
        *self
    }
}
