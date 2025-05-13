pub mod utils;
use crate::traits::ApproxEq;
#[macro_export]
macro_rules! assert_approx_eq {
    // Case with custom epsilon
    ($left:expr, $right:expr) => {{
        let left_val = $left;
        let right_val = $right;

        assert!(
            (left_val).approx_cmp(&right_val),
            "assertion failed: `(left ≈ right)`\n  left: `{:?}`\n right: `{:?}`\n",
            left_val,
            right_val
        );
    }};
}

#[macro_export]
macro_rules! assert_fractional {
    ($val:expr) => {{
        let val = $val;

        assert!(
            val > 0.0 && val < 1.0,
            "assertion failed: value not fractional\n  value: `{:?}`\n ",
            val,
        );
    }};
}
