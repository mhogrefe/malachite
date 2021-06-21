use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::stats::moments::CheckedToF64;
use malachite_nz::integer::Integer;

pub struct IntegerCheckedToF64Wrapper(pub Integer);

impl CheckedToF64 for IntegerCheckedToF64Wrapper {
    #[inline]
    fn checked_to_f64(&self) -> f64 {
        if self.0 == 0 {
            0.0
        } else {
            let (mantissa, exponent) = self.0.unsigned_abs_ref().sci_mantissa_and_exponent();
            let f =
                f64::from_sci_mantissa_and_exponent(mantissa, i64::exact_from(exponent)).unwrap();
            if self.0 > 0 {
                f
            } else {
                -f
            }
        }
    }
}

pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod logic;
