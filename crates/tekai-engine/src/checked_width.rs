//! Exact transient width arithmetic with the existing signed 32-bit storage.
//!
//! Callers evaluate each complete expression in i64 before converting here.
//! This preserves cancellation even when an intermediate partial sum would
//! not fit in i32. Unrepresentable stored widths are capacity failures, never
//! wrapping or saturated layout values. MemoryWord and format layouts stay
//! unchanged.

#![warn(clippy::all)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WidthOverflow;

#[inline]
pub(crate) fn checked(value: i64) -> Result<i32, WidthOverflow> {
    i32::try_from(value).map_err(|_| WidthOverflow)
}

/// Convert a complete transient expression or stop through TeX's diagnostic.
///
/// # Safety
/// TeX string storage and the ordinary fatal-error machinery must be live.
#[inline]
pub(crate) unsafe fn tex(value: i64) -> i32 {
    match checked(value) {
        Ok(value) => value,
        Err(_) => capacity_exceeded(),
    }
}

#[cold]
#[inline(never)]
unsafe fn capacity_exceeded() -> ! {
    crate::generated::pdftex0::zoverflow(
        crate::utils::maketexstring(c"paragraph or box width in scaled points".as_ptr()),
        i32::MAX,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn exact_storage_limits_accept_and_neighbors_reject() {
        assert_eq!(checked(i64::from(i32::MIN)), Ok(i32::MIN));
        assert_eq!(checked(i64::from(i32::MAX)), Ok(i32::MAX));
        for value in [
            i64::from(i32::MIN) - 1,
            i64::from(i32::MAX) + 1,
            i64::MIN,
            i64::MAX,
        ] {
            assert_eq!(checked(value), Err(WidthOverflow));
        }
    }

    #[test]
    fn full_expression_cancellation_does_not_reject_partial_overflow() {
        let max = i64::from(i32::MAX);
        let min = i64::from(i32::MIN);
        assert_eq!(checked(max + max - max), Ok(i32::MAX));
        assert_eq!(checked(min - max + max), Ok(i32::MIN));
        assert_eq!(checked(max - min + min), Ok(i32::MAX));
        assert_eq!(checked(min + min - min), Ok(i32::MIN));
        assert_eq!(checked(-min + min), Ok(0));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn bounded_width_expressions_match_independent_i128_reference(
            a in any::<i32>(), b in any::<i32>(), c in any::<i32>(),
            d in any::<i32>(), e in any::<i32>(),
        ) {
            for (actual, reference) in [
                (i64::from(a) + i64::from(b), i128::from(a) + i128::from(b)),
                (i64::from(a) - i64::from(b), i128::from(a) - i128::from(b)),
                (
                    i64::from(a) - i64::from(b) + i64::from(c),
                    i128::from(a) - i128::from(b) + i128::from(c),
                ),
                (
                    i64::from(a) + i64::from(b) - i64::from(c) + i64::from(d) - i64::from(e),
                    i128::from(a) + i128::from(b) - i128::from(c) + i128::from(d) - i128::from(e),
                ),
                (-i64::from(a), -i128::from(a)),
            ] {
                prop_assert_eq!(checked(actual), i32::try_from(reference).map_err(|_| WidthOverflow));
            }
        }
    }
}
