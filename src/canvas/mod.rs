pub mod scene_object;
use crate::{color::Color, geo::point::Point, lighting::PointLight};

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
    pub lights: Vec<PointLight>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::black(); width as usize * height as usize],
            lights: vec![],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::from("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");
        for pixel in self.pixels.iter() {
            ppm.push_str(&format!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
        }
        ppm
    }

    pub fn save_ppm(&self, path: &str) {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_ppm().as_bytes()).unwrap();
    }

    pub fn draw_point(&mut self, point: Point, color: Color) {
        let (x, y) = (point.x as u32, self.height - point.y as u32);
        self.set_pixel(x, y, color);
    }

    pub fn add_light(&mut self, light: PointLight) {}
}

#[cfg(test)]
mod tests {
    use crate::{
        geo::{point::Point, vector::Vector},
        test::utils::Environment,
    };

    use super::*;

    #[test]
    fn test_canvas() {
        let mut canvas = Canvas::new(900, 550);
        let gravity = Vector::new(0., -0.1, 0.);
        let wind = Vector::new(-0.01, 0., 0.);
        let position = Point::new(0., 1., 0.);
        let velocity = Vector::new(1., 1.8, 0.).norm() * 11.25;
        let mut env = Environment::new(gravity, wind, position, velocity);
        while env.projectile.position.y > 0. {
            env.tick();
            dbg!(&env.projectile.position);
            let (x, y) = (
                env.projectile.position.x as u32,
                env.projectile.position.y as u32,
            );
            let y = canvas.height - y;
            let color = Color::new(255., 0., 0.);
            dbg!(x, y);
            // circle around x,y
            let circle_points = vec![
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x + 1, y + 1),
                (x - 1, y + 1),
                (x, y),
            ];
            for point in circle_points {
                let (x, y) = (point.0 as u32, point.1 as u32);
                canvas.set_pixel(x, y, color.clone());
            }
        }
        assert!(env.projectile.position.y < 0.);
        let path = "test.ppm";
        canvas.save_ppm(path);
    }
}
