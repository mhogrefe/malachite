use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_nz::integer::Integer;
use malachite_q::Rational;

pub fn sub_naive(x: Rational, y: Rational) -> Rational {
    let x_sign = x >= 0u32;
    let y_sign = y >= 0u32;
    let (xn, xd) = x.into_numerator_and_denominator();
    let (yn, yd) = y.into_numerator_and_denominator();
    let n =
        Integer::from_sign_and_abs(x_sign, xn * &yd) - Integer::from_sign_and_abs(y_sign, yn * &xd);
    Rational::from_sign_and_naturals(n >= 0u32, n.unsigned_abs(), xd * yd)
}
