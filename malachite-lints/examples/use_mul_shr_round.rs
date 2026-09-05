use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::gaussian_rational::GaussianRational;

const X: Natural = Natural::const_from(5);
const Y: Natural = Natural::const_from(3);
const A: Integer = Integer::const_from_signed(-5);
const B: Integer = Integer::const_from_unsigned(3);

fn main() {
    // A bignum product immediately shift-rounded: flagged.
    let _ = (&X * &Y).shr_round(100u32, Floor);
    let _ = (X.clone() * Y.clone()).shr_round(3u32, Nearest);
    let _ = (&A * &B).shr_round(10u32, Ceiling);
    // A bignum product immediately shifted: flagged, since `>>` is a Floor shift.
    let _ = (&X * &Y) >> 50u32;
    let _ = (&A * &B) >> 5u32;
    // The widening idiom on primitives, by `From` or by `as`: flagged.
    let a = 123_456_789_u64;
    let b = 987_654_321_u64;
    let _ = (u128::from(a) * u128::from(b)) >> 64;
    let _ = ((a as u128) * (b as u128)) >> 64;
    let _ = (u128::from(a) * u128::from(b)).shr_round(64u32, Nearest);

    // A plain primitive product shifted: NOT flagged. The in-type product has already discarded
    // any overflow, so the fused operation, which computes the exact double-width product, is
    // not an equivalent rewrite.
    let _ = (a * b) >> 20;
    // A named product shifted later: not flagged; the lint only sees inline compositions.
    let p = &X * &Y;
    let _ = p >> 10u32;
    // Widening by more than a factor of two: not the fused operation's shape.
    let c = 123u16;
    let d = 45u16;
    let _ = (u128::from(c) * u128::from(d)) >> 64;
    // Widened operands of different original types: not flagged.
    let e = 7u32;
    let _ = (u64::from(e) * u64::from(d)) >> 32;
    // A shift of something that is not a product: not flagged.
    let _ = (&X + &Y) >> 5u32;
    // `GaussianRational` has no `mul_shr_round`: not flagged.
    let h = GaussianRational::from(3u32);
    let h2 = GaussianRational::from(4u32);
    let _ = (&h * &h2) >> 3u32;
}
