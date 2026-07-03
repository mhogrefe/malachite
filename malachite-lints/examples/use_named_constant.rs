use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

fn main() {
    // Constructing named constants the long way: flagged.
    let _ = Natural::from(0u32);
    let _ = Natural::from(1u32);
    let _ = Integer::from(2);
    let _ = Integer::from(-1);
    let _ = Integer::const_from_signed(1);
    let _ = Rational::from(1u32);
    let _ = Rational::from_unsigneds(1u32, 2u32);
    let _ = Rational::from_signeds(1, 2);
    let _ = Float::one_prec(1);
    let _ = Float::two_prec(1);
    let _ = Float::negative_one_prec(1);
    let _ = Float::one_half_prec(1);
    // Other values don't get the named-constant advice (`runtime_literal_conversion` fires
    // instead).
    let _ = Natural::from(3u32);
    let _ = Integer::from(-2);
    let _ = Rational::from_unsigneds(1u32, 3u32);
    // A `Float` constant at a precision other than 1 is not the named constant: fine.
    let _ = Float::one_prec(100);
    // A non-literal argument: fine.
    let k = 1u32;
    let _ = Natural::from(k);
}
