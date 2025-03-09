use crate::geo::{point::Point, vector::Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

pub struct Environment {
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
