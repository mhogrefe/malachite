use crate::Rational;
use malachite_base::num::basic::traits::Zero;

pub fn mul_naive(x: Rational, y: Rational) -> Rational {
    if x == 0u32 || y == 0u32 {
        Rational::ZERO
    } else {
        let sign = (x >= 0) == (y >= 0);
        let (xn, xd) = x.into_numerator_and_denominator();
        let (yn, yd) = y.into_numerator_and_denominator();
        Rational::from_sign_and_naturals(sign, xn * yn, xd * yd)
    }
}
