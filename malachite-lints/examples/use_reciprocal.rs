use malachite_base::num::basic::traits::{One, Two};
use malachite_float::Float;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

fn main() {
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
}
