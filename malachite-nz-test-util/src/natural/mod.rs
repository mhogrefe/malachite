use malachite_base::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use malachite_base_test_util::stats::moments::CheckedToF64;
use malachite_nz::natural::Natural;

pub struct NaturalCheckedToF64Wrapper(pub Natural);

impl CheckedToF64 for NaturalCheckedToF64Wrapper {
    #[inline]
    fn checked_to_f64(&self) -> f64 {
        if self.0 == 0 {
            0.0
        } else {
            let (mantissa, exponent) = self.0.sci_mantissa_and_exponent();
            f64::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent)).unwrap()
        }
    }
}

pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod logic;
