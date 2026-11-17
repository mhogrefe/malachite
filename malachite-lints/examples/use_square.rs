use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_base::num::basic::traits::Two;
use malachite_base::rounding_modes::RoundingMode::Floor;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

// A function named for squaring may define it by multiplying: fine.
fn square_prec_naive(x: &Float, prec: u64) -> Float {
    x.mul_prec_ref_ref(x, prec).0
}

fn main() {
    let _ = square_prec_naive(&Float::TWO, 10);
    let x = const { Natural::const_from(100) };
    // Multiplying a value by itself: flagged.
    let _ = &x * &x;
    // Raising to the power of 2: flagged.
    let _ = (&x).pow(2);
    let mut q = const { Rational::const_from_unsigneds(22, 7) };
    q.pow_assign(2u64);
    let _ = q;
    // Different operands: fine.
    let y = const { Natural::const_from(7) };
    let _ = &x * &y;
    // Other exponents: fine.
    let _ = (&x).pow(3);
    let g = GaussianInteger::from(3u32);
    let h = GaussianRational::from(3u32);
    let _ = &g * &g;
    let _ = &h * &h;
    // The rounding multiplication functions of `Float` and `BigExponentFloat`, with a value
    // multiplied by itself: flagged.
    let f = Float::TWO;
    let _ = f.mul_prec_ref_ref(&f, 10);
    let _ = f.mul_round_ref_ref(&f, Floor);
    let _ = f.clone().mul_prec_round_val_ref(&f, 10, Floor);
    let b = BigExponentFloat::TWO;
    let _ = &b * &b;
    let _ = b.mul_prec_ref_ref(&b, 10).0;
    let _ = b.mul_prec_round_ref_val(b.clone(), 10, Floor);
    let _ = b.clone().mul_prec(b.clone(), 10);
    let mut c = b.clone();
    let _ = c.mul_prec_assign(c.clone(), 10);
    // Different operands: fine.
    let _ = b.mul_prec_ref_ref(&c, 10);
    let _ = f.mul_prec_ref_val(Float::TWO, 10);
}
