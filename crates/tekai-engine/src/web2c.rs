//! Small Rust-owned web2c support shims.

use crate::stdio::Stream as FILE;
use libc::{c_double, c_int};

#[inline]
unsafe fn fast_fgetc(stream: *mut FILE) -> c_int {
    unsafe { crate::stdio::fgetc(stream) }
}

pub fn zround(value: c_double) -> c_int {
    if value > 2_147_483_647.0 {
        2_147_483_647
    } else if value < -2_147_483_647.0 {
        -2_147_483_647
    } else if value >= 0.0 {
        (value + 0.5) as c_int
    } else {
        (value - 0.5) as c_int
    }
}

pub fn uexit(unix_code: c_int) -> ! {
    let final_code = match unix_code {
        0 => libc::EXIT_SUCCESS,
        1 => libc::EXIT_FAILURE,
        other => other,
    };
    unsafe { crate::stdio::exit(final_code) }
}

pub(crate) unsafe fn eof(file: *mut FILE) -> c_int {
    if file.is_null() || unsafe { crate::stdio::feof(file) } != 0 {
        return 1;
    }

    let c = unsafe { fast_fgetc(file) };
    if c == libc::EOF {
        return 1;
    }
    unsafe {
        crate::stdio::ungetc(c, file);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::zround;
    use proptest::prelude::*;

    #[test]
    fn zround_matches_web2c_boundaries() {
        assert_eq!(zround(1.49), 1);
        assert_eq!(zround(1.5), 2);
        assert_eq!(zround(-1.49), -1);
        assert_eq!(zround(-1.5), -2);
        assert_eq!(zround(3_000_000_000.0), 2_147_483_647);
        assert_eq!(zround(-3_000_000_000.0), -2_147_483_647);
        assert_eq!(zround(2_147_483_647.0), 2_147_483_647);
        assert_eq!(zround(-2_147_483_647.0), -2_147_483_647);
        assert_eq!(zround(f64::INFINITY), 2_147_483_647);
        assert_eq!(zround(f64::NEG_INFINITY), -2_147_483_647);
        assert_eq!(zround(f64::NAN), 0);
        assert_eq!(zround(0.0), 0);
        assert_eq!(zround(-0.0), 0);

        // Preserve the legacy floating addition at the neighbor below 0.5.
        // Replacing the algorithm with f64::round changes this result.
        let below_half = f64::from_bits(0.5_f64.to_bits() - 1);
        assert_eq!(zround(below_half), 1);
        assert_eq!(zround(-below_half), -1);
        assert_eq!(zround(f64::from_bits(1.5_f64.to_bits() - 1)), 1);
        assert_eq!(zround(-f64::from_bits(1.5_f64.to_bits() - 1)), -1);
    }

    proptest! {
        #[test]
        fn zround_matches_exact_quarter_steps(integer in any::<i32>(), quarters in -3_i8..=3) {
            let value = f64::from(integer) + f64::from(quarters) / 4.0;
            let expected = value.round().clamp(-2_147_483_647.0, 2_147_483_647.0) as i32;
            prop_assert_eq!(zround(value), expected);
        }

        #[test]
        fn zround_is_monotone_and_bounded(left_bits in any::<u64>(), right_bits in any::<u64>()) {
            let left = f64::from_bits(left_bits);
            let right = f64::from_bits(right_bits);
            prop_assume!(!left.is_nan() && !right.is_nan());
            let low = zround(left.min(right));
            let high = zround(left.max(right));
            prop_assert_ne!(low, i32::MIN);
            prop_assert_ne!(high, i32::MIN);
            prop_assert!(low <= high);
        }
    }
}
