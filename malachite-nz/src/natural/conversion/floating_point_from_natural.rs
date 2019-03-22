use malachite_base::misc::{CheckedFrom, Named, RoundingFrom, WrappingFrom};
use malachite_base::num::{BitAccess, FloorLogTwo, PrimitiveFloat, ShrRoundAssign};
use malachite_base::round::RoundingMode;
use natural::Natural;

macro_rules! float_impls {
    ($f: ident, $larger_than_max_finite_float: ident) => {
        //TODO make more efficient!
        fn $larger_than_max_finite_float(n: &Natural) -> bool {
            *n > Natural::from($f::MAX_FINITE)
        }

        /// # Example
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::misc::RoundingFrom;
        /// use malachite_base::round::RoundingMode;
        /// use malachite_nz::natural::Natural;
        /// use std::str::FromStr;
        ///
        /// fn main() {
        ///     assert_eq!(f32::rounding_from(Natural::from_str("3").unwrap(), RoundingMode::Floor),
        ///         3.0);
        ///     assert_eq!(f32::rounding_from(Natural::from_str("123").unwrap(),
        ///         RoundingMode::Floor), 123.0);
        ///     assert_eq!(f32::rounding_from(Natural::from_str("0").unwrap(), RoundingMode::Floor),
        ///         0.0);
        ///     assert_eq!(f32::rounding_from(Natural::from_str("1000000000").unwrap(),
        ///         RoundingMode::Floor), 1.0e9);
        ///     assert_eq!(f32::rounding_from(Natural::from_str(
        ///         "340282346638528859811704183484516925440").unwrap(), RoundingMode::Floor),
        ///         3.4028235e38);
        ///     assert_eq!(f32::rounding_from(
        ///         Natural::from_str("10000000000000000000000000000000000000000000000000000")
        ///         .unwrap(), RoundingMode::Floor), 3.4028235e38);
        /// }
        /// ```
        impl RoundingFrom<Natural> for $f {
            fn rounding_from(mut value: Natural, rm: RoundingMode) -> $f {
                if value == 0 {
                    return 0.0;
                }
                if $larger_than_max_finite_float(&value) {
                    return match rm {
                        RoundingMode::Exact => {
                            panic!("Value cannot be represented exactly as an {}", $f::NAME)
                        }
                        RoundingMode::Floor | RoundingMode::Down => $f::MAX_FINITE,
                        _ => $f::POSITIVE_INFINITY,
                    };
                }
                let exponent = value.floor_log_two();
                let shift = i32::checked_from(exponent).unwrap()
                    - i32::checked_from($f::MANTISSA_WIDTH).unwrap();
                value.shr_round_assign(shift, rm);
                let mut mantissa =
                    <$f as PrimitiveFloat>::UnsignedOfEqualWidth::wrapping_from(value);
                mantissa.clear_bit(u64::from($f::MANTISSA_WIDTH));
                let exponent = u32::wrapping_from(exponent) + $f::MAX_EXPONENT;
                $f::from_adjusted_mantissa_and_exponent(mantissa, exponent)
            }
        }
    };
}

float_impls!(f32, larger_than_max_finite_f32);
float_impls!(f64, larger_than_max_finite_f64);
