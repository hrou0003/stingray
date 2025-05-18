use crate::geo::vector::Vector;

pub trait ApproxEq {
    fn approx_cmp(&self, other: &Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_cmp(&self, other: &Self) -> bool {
        // TODO: Use a better epsilon
        (self - other).abs() < f64::EPSILON
    }
}

impl ApproxEq for Vector {
    fn approx_cmp(&self, other: &Self) -> bool {
        self.x.approx_cmp(&other.x) && self.y.approx_cmp(&other.y) && self.z.approx_cmp(&other.z)
    }
}

pub trait SpecificRound {
    fn specific_round(&self, places: u32) -> Self;
}

impl SpecificRound for f64 {
    fn specific_round(&self, places: u32) -> Self {
        let factor = 10.0_f64.powi(places as i32);
        (self * factor).round() / factor
    }
}
