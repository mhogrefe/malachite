use malachite_base::num::basic::traits::{One, OneHalf, Two};
use malachite_base::rounding_modes::RoundingMode::Floor;
use malachite_float::{BigExponentFloat, Float};

fn f(prec: u64, k: u32, j: u64) -> Vec<Float> {
    vec![
        // A conversion of 1: flagged, `Float::one_prec(prec)`.
        Float::from_unsigned_prec(1u32, prec).0,
        // A conversion of another power of 2: flagged, `Float::power_of_2_prec(3, prec).0`.
        Float::from_signed_prec(8i32, prec).0,
        // The `_round` variants: flagged.
        Float::from_unsigned_prec_round(1u32, prec, Floor).0,
        // A shift of a conversion of 1: flagged once, `Float::one_half_prec(100)`.
        Float::from_unsigned_prec(1u32, 100).0 >> 1u32,
        // A shift of `one_prec`: flagged, `Float::power_of_2_prec(2, prec).0`.
        Float::one_prec(prec) << 2u32,
        // A shift that brings `two_prec` back to 1: flagged, `Float::one_prec(prec)`.
        Float::two_prec(prec) >> 1u32,
        // A shift by a variable: flagged, `Float::power_of_2_prec(-i64::from(k), prec).0`.
        Float::one_prec(prec) >> k,
        // A shift by a `u64` variable: flagged, with `i64::exact_from`.
        Float::one_half_prec(prec) << j,
        // A shift of a constant, at precision 1: flagged, `Float::power_of_2(-3i64)`.
        Float::ONE >> 3u32,
        // A parenthesized shift amount: flagged, `Float::power_of_2(-i64::from(k + 1))`,
        // without the doubled parentheses.
        Float::ONE >> (k + 1),
        // Precision 1, given explicitly: flagged, `Float::ONE_HALF`.
        Float::one_prec(1) >> 1u32,
        // A conversion of a number that is not a power of 2: fine.
        Float::from_unsigned_prec(3u32, prec).0,
        // A shift of a value that is not a power of 2: fine.
        Float::from_unsigned_prec(3u32, prec).0 >> 1u32,
        // A shift to 2: flagged, `Float::two_prec(prec)`.
        Float::one_prec(prec) << 1u32,
        // The dedicated constructors alone: fine.
        Float::one_prec(prec),
        Float::power_of_2_prec(-1, prec).0,
    ]
}

fn g(k: u32) -> Vec<BigExponentFloat> {
    vec![
        // A `BigExponentFloat` has no precision: flagged, `BigExponentFloat::ONE`.
        BigExponentFloat::from_unsigned_prec(1u32, 100).0,
        // Flagged, `BigExponentFloat::ONE_HALF`.
        BigExponentFloat::from_unsigned_prec(1u32, 100).0 >> 1u32,
        // Flagged, `BigExponentFloat::power_of_2(-2i64)`.
        BigExponentFloat::ONE >> 2u32,
        // Flagged, `BigExponentFloat::power_of_2(1 + i64::from(k))`.
        BigExponentFloat::TWO << k,
        // Fine.
        BigExponentFloat::ONE_HALF,
        BigExponentFloat::from_unsigned_prec(5u32, 100).0,
    ]
}

fn main() {
    let _ = f(
        std::hint::black_box(100),
        std::hint::black_box(3),
        std::hint::black_box(4),
    );
    let _ = g(std::hint::black_box(3));
}
