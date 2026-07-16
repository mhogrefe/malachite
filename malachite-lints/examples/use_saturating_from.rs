use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom};

fn f(err: i64, big: i128) -> (u64, u64, u64, i32, u64, u64) {
    // `exact_from` of a `max(0)` clamp into an unsigned type: flagged.
    let a = u64::exact_from(err.max(0));
    // The zero may be on either side of `max`: flagged.
    let b = u64::exact_from(0.max(err));
    // A wider source is still the pattern (the human decides whether the value can overflow high):
    // flagged.
    let c = u64::exact_from(big.max(0));
    // A signed target clamps low to its minimum, not 0, so it is not equivalent: fine.
    let d = i32::exact_from(err.max(0));
    // `exact_from` without a `max(0)` clamp: fine.
    let e = u64::exact_from(err.unsigned_abs());
    // Already using `saturating_from`: fine.
    let g = u64::saturating_from(err);
    (a, b, c, d, e, g)
}

fn main() {
    let _ = f(
        std::hint::black_box(-5),
        std::hint::black_box(1 << 70),
    );
}
