pub mod utils;
use crate::float::ApproxEq;
#[macro_export]
macro_rules! assert_approx_eq {
    // Case with custom epsilon
    ($left:expr, $right:expr) => {{
        let left_val = $left;
        let right_val = $right;

        assert!(
            (left_val).approx_cmp(&right_val),
            "assertion failed: `(left ≈ right)`\n  left: `{:?}`\n right: `{:?}`\n diff: `{:?}`",
            left_val,
            right_val,
            (left_val - right_val).abs()
        );
    }};
}
