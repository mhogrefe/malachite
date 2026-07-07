use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;

fn f(x: &Rational, y: &Rational) -> (u64, u64, u64, u64, u64) {
    // Summing the numerator and denominator significant bits: flagged.
    let a = x.numerator_ref().significant_bits() + x.denominator_ref().significant_bits();
    // The reverse order is the same reimplementation: flagged.
    let b = x.denominator_ref().significant_bits() + x.numerator_ref().significant_bits();
    // The `to_*` accessors are also flagged.
    let c = x.to_numerator().significant_bits() + x.to_denominator().significant_bits();
    // Two different Rationals: fine.
    let d = x.numerator_ref().significant_bits() + y.denominator_ref().significant_bits();
    // Both numerators: fine (not the numerator + denominator pattern).
    let e = x.numerator_ref().significant_bits() + x.numerator_ref().significant_bits();
    (a, b, c, d, e)
}

fn main() {
    let x = Rational::from(std::hint::black_box(3u32));
    let y = Rational::from(std::hint::black_box(7u32));
    let _ = f(&x, &y);
}
