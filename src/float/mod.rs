pub trait ApproxEq {
    fn approx_cmp(&self, other: &Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_cmp(&self, other: &Self) -> bool {
        (self - other).abs() < f64::EPSILON
    }
}
