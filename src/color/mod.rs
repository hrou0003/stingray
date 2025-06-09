use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

//
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::new(0., 0., 0.)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            r: (self.r as f64 * other),
            g: (self.g as f64 * other),
            b: (self.b as f64 * other),
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx_eq, traits::ApproxEq};

    use super::*;

    #[test]
    fn test_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 + c2;
        assert_eq!(c3.r, 1.6);
        assert_eq!(c3.g, 0.7);
        assert_eq!(c3.b, 1.);
    }

    #[test]
    fn test_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 - c2;
        assert!(c3.r.approx_cmp(&0.2));
        assert_eq!(c3.g, 0.5);
        assert_eq!(c3.b, 0.5);
    }

    #[test]
    fn test_mul() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);
        let c3 = c1 * c2;
        dbg!(&c3);
        assert_approx_eq!(c3.r, 0.9);
        assert_approx_eq!(c3.g, 0.2);
        assert_approx_eq!(c3.b, 0.04);
    }

    #[test]
    fn test_mul_f64() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = c1 * 0.5;
        assert_eq!(c2.r, 0.45);
    }
}
