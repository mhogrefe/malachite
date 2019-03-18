use malachite_base::misc::{CheckedFrom, RoundingFrom};
use malachite_base::num::{BitAccess, PrimitiveFloat, ShlRound, Zero};
use malachite_base::round::RoundingMode;
use natural::Natural;

/// Converts an `f32` or an `f64` to a `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `value` is NaN or infinite, if it would round to a negative integer, or if the
/// rounding mode is exact and `value` is not an integer.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::misc::RoundingFrom;
/// use malachite_base::round::RoundingMode;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::rounding_from(0.0, RoundingMode::Exact).to_string(), "0");
///     assert_eq!(Natural::rounding_from(-0.0, RoundingMode::Exact).to_string(), "0");
///     assert_eq!(Natural::rounding_from(123.0, RoundingMode::Exact).to_string(), "123");
///     assert_eq!(Natural::rounding_from(1.0e9, RoundingMode::Exact).to_string(), "1000000000");
///     assert_eq!(Natural::rounding_from(1.0e9, RoundingMode::Exact).to_string(), "1000000000");
///     assert_eq!(Natural::rounding_from(4294967295.0, RoundingMode::Exact).to_string(),
///         "4294967295");
///     assert_eq!(Natural::rounding_from(4294967296.0, RoundingMode::Exact).to_string(),
///         "4294967296");
///     assert_eq!(Natural::rounding_from(1.0e100, RoundingMode::Exact).to_string(),
///         "10000000000000000159028911097599180468360808563945281389781327557747838772170381060813\
///         469985856815104");
///     assert_eq!(Natural::rounding_from(123.1, RoundingMode::Floor).to_string(), "123");
///     assert_eq!(Natural::rounding_from(123.1, RoundingMode::Ceiling).to_string(), "124");
///     assert_eq!(Natural::rounding_from(123.1, RoundingMode::Nearest).to_string(), "123");
///     assert_eq!(Natural::rounding_from(123.9, RoundingMode::Floor).to_string(), "123");
///     assert_eq!(Natural::rounding_from(123.9, RoundingMode::Ceiling).to_string(), "124");
///     assert_eq!(Natural::rounding_from(123.9, RoundingMode::Nearest).to_string(), "124");
///     assert_eq!(Natural::rounding_from(123.5, RoundingMode::Nearest).to_string(), "124");
///     assert_eq!(Natural::rounding_from(124.5, RoundingMode::Nearest).to_string(), "124");
///     assert_eq!(Natural::rounding_from(-0.99, RoundingMode::Ceiling).to_string(), "0");
///     assert_eq!(Natural::rounding_from(-0.499, RoundingMode::Nearest).to_string(), "0");
///     assert_eq!(Natural::rounding_from(-0.5, RoundingMode::Nearest).to_string(), "0");
/// }
/// ```
impl<T: PrimitiveFloat> RoundingFrom<T> for Natural
where
    Natural: From<T::UnsignedOfEqualWidth>,
{
    fn rounding_from(value: T, rm: RoundingMode) -> Self {
        if value.is_nan() || value.is_infinite() {
            panic!("Cannot convert {} to Natural", value);
        } else if value == T::ZERO {
            Natural::ZERO
        } else {
            let (mut mantissa, exponent) = value.to_adjusted_mantissa_and_exponent();
            let value_negative = value < T::ZERO;
            mantissa.set_bit(u64::from(T::MANTISSA_WIDTH));
            let n = Natural::from(mantissa).shl_round(
                i32::checked_from(exponent).unwrap() + T::MIN_EXPONENT - 1,
                if value_negative { -rm } else { rm },
            );
            if value_negative && n != 0 {
                panic!("Result is negative and cannot be converted to a Natural");
            }
            n
        }
    }
}

impl From<f32> for Natural {
    fn from(value: f32) -> Self {
        Natural::rounding_from(value, RoundingMode::Nearest)
    }
}

impl From<f64> for Natural {
    fn from(value: f64) -> Self {
        Natural::rounding_from(value, RoundingMode::Nearest)
    }
}
