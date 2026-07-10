use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::Floor;
use std::cmp::max;
use malachite_float::Float;

fn f(x: u64, y: i64) -> (Float, Float, Float, Float, Float, Float, Float) {
    // Exact conversion at the argument's significant-bit precision, ordering discarded: flagged.
    let a = Float::from_unsigned_prec(x, x.significant_bits()).0;
    // The `.max(1)` guard against a zero precision is also just `Float::from`: flagged.
    let b = Float::from_unsigned_prec(x, x.significant_bits().max(1)).0;
    // The signed constructor is the same idiom: flagged.
    let c = Float::from_signed_prec(y, y.significant_bits()).0;
    // The `_round` variants are the same exact conversion with a dead rounding mode: flagged.
    let d = Float::from_unsigned_prec_round(x, x.significant_bits(), Floor).0;
    // The free-function `max` spelling: flagged.
    let e = Float::from_unsigned_prec(x, max(x.significant_bits(), 1)).0;
    // A precision other than the significant bits may round: fine.
    let f = Float::from_unsigned_prec(x, x.significant_bits() + 1).0;
    // A fixed, unrelated precision: fine.
    let g = Float::from_unsigned_prec(x, 100).0;
    (a, b, c, d, e, f, g)
}

fn main() {
    let _ = f(std::hint::black_box(3u64), std::hint::black_box(-5i64));
}
