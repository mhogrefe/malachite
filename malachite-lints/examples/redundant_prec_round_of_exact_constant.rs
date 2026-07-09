use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two};
use malachite_base::rounding_modes::RoundingMode;
use malachite_float::Float;

fn f(x: Float, prec: u64, rm: RoundingMode) -> (Float, Float, Float, Float, Float) {
    // Rounding a single-significant-bit constant is a no-op: flagged.
    let a = Float::from_float_prec_round(Float::ONE, prec, rm).0;
    let b = Float::from_float_prec_round(Float::TWO, prec, rm).0;
    // The `Nearest` shorthand `from_float_prec` too: flagged.
    let c = Float::from_float_prec(Float::NEGATIVE_ONE, prec).0;
    // `ONE_HALF` is 2^-1, also exact everywhere: flagged.
    let d = Float::from_float_prec_round(Float::ONE_HALF, prec, rm).0;
    // An arbitrary Float may round: fine.
    let e = Float::from_float_prec_round(x, prec, rm).0;
    (a, b, c, d, e)
}

fn main() {
    let x = Float::from(std::hint::black_box(3u32));
    let _ = f(x, std::hint::black_box(10), RoundingMode::Nearest);
}
