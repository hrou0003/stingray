pub trait Hit {
    fn hit(&self) -> Option<f64>;
}

impl Hit for Vec<f64> {
    fn hit(&self) -> Option<f64> {
        self.iter()
            .filter(|&&t| t >= 0.0) // Filter out negative intersections
            .min_by(|a, b| a.partial_cmp(b).unwrap()) // Get the smallest positive t
            .copied() // Convert &f64 to f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit() {
        let hit = vec![0.5, 1.];

        assert_eq!(hit.hit().unwrap(), 0.5);
    }

    #[test]
    fn test_hit_multiple() {
        let hit = vec![5., 7., -3., 2.];

        assert_eq!(hit.hit().unwrap(), 2.);
    }

    #[test]
    fn test_hit_behind() {
        let hit = vec![-6., -4.];

        assert!(dbg!(hit.hit()).is_none());
    }
}
