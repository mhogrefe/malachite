use malachite_base::num::basic::traits::{One, Two};
use malachite_base::rounding_modes::RoundingMode::Floor;
use malachite_float::{BigExponentFloat, Float};
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

// A function named for the reciprocal may define it by dividing: fine.
fn reciprocal_prec_naive(x: Float, prec: u64) -> Float {
    Float::ONE.div_prec(x, prec).0
}

fn main() {
    let _ = reciprocal_prec_naive(Float::TWO, 10);
    let f = Float::TWO;
    let q = const { Rational::const_from_unsigneds(22, 7) };
    // Dividing `ONE` by a value: flagged.
    let _ = Float::ONE / &f;
    let _ = Rational::ONE / &q;
    // Other numerators: fine.
    let _ = Float::TWO / &f;
    // `ONE` as the denominator is not a reciprocal: fine.
    let _ = &q / Rational::ONE;
    let h = GaussianRational::from(3u32);
    let _ = GaussianRational::ONE / &h;
    // Dividing `ONE` by a value with a rounding division function: flagged.
    let _ = Float::ONE.div_prec_val_ref(&f, 10);
    let _ = Float::ONE.div_round_ref_ref(&f, Floor);
    let b = BigExponentFloat::TWO;
    let _ = BigExponentFloat::ONE.div_prec_val_ref(&b, 10);
    let _ = BigExponentFloat::ONE.div_prec_round(b.clone(), 10, Floor);
    // Other numerators: fine.
    let _ = BigExponentFloat::TWO.div_prec_val_ref(&b, 10);
    // `ONE` as the denominator is not a reciprocal: fine.
    let _ = b.div_prec(BigExponentFloat::ONE, 10);
}
