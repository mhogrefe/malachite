use malachite_base::num::conversion::traits::ExactFrom;
use std::hint::black_box;

fn main() {
    let x = black_box(1u64);
    let n = black_box(3i128);
    let m = black_box(3u32);
    // A signed amount converted to a narrower signed type: the shift already accepts the wider
    // one, so the conversion only obscures it. Flagged.
    let a = x << i64::exact_from(n);
    // The same for a compound assignment.
    let mut b = black_box(1u64);
    b <<= i64::exact_from(n);
    // Widening an unsigned amount is redundant too.
    let c = x >> u64::from(m);
    // Converting a signed amount to an unsigned type asserts that it is non-negative, and dropping
    // it could reverse the shift's direction, so it is left alone.
    let d = x << u64::exact_from(n);
    // No conversion at all: fine.
    let e = x << n;
    black_box((a, b, c, d, e));
}
