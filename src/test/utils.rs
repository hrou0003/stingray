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

#[cfg(test)]
mod tests {
    use crate::geo::objects::geometry::Geometry;

    use super::*;
    use crate::canvas::Canvas;
    use crate::color::Color;
    use crate::geo::objects::sphere::Sphere;
    use crate::geo::ray::Ray;

    #[test]
    fn test_sphere_shadow() {
        // Create a canvas with 300x300 pixels
        let mut canvas = Canvas::new(300, 300);

        // Create a sphere at the center of the image
        let mut sphere = Sphere::new(Point::new(150., 150., 50.0), 50.0);

        // Create a light position (above and to the side)
        let light_position = Point::new(200.0, 225.0, -200.0);

        // Define colors
        let background_color = Color::new(50.0, 50.0, 50.0); // Dark gray
        let shadow_color = Color::new(0.0, 0.0, 0.0); // Black

        // Draw the sphere shadow
        for i in 0..canvas.height {
            for j in 0..canvas.width {
                // Create a point on the canvas plane (z=0)
                let canvas_point = Point::new(j as f64, i as f64, 0.0);

                // Calculate ray from light to canvas point
                let direction = (canvas_point - light_position).norm();
                let ray = Ray::new(light_position, direction);

                // Check for intersection
                let intersections = sphere.intersect(&ray).unwrap();

                // A valid intersection occurs when we have intersection points
                // and at least one of them is positive (in front of the ray origin)
                let has_valid_intersection =
                    !intersections.is_empty() && intersections.iter().any(|&t| t > 0.0);

                let color = if has_valid_intersection {
                    shadow_color.clone()
                } else {
                    background_color.clone()
                };
                canvas.set_pixel(j, i, color);
            }
        }

        // Save the result
        canvas.save_ppm("src/test/output/sphere_shadow.ppm");
    }
}
