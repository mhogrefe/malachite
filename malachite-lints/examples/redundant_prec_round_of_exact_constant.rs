use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two};
use malachite_base::rounding_modes::RoundingMode;
use malachite_float::{BigExponentFloat, Float};

fn f(x: Float, prec: u64, rm: RoundingMode) -> (Float, Float, Float, Float, Float, Float) {
    // Rounding a single-significant-bit constant is a no-op: flagged.
    let a = Float::from_float_prec_round(Float::ONE, prec, rm).0;
    let b = Float::from_float_prec_round(Float::TWO, prec, rm).0;
    // The `Nearest` shorthand `from_float_prec` too: flagged.
    let c = Float::from_float_prec(Float::NEGATIVE_ONE, prec).0;
    // `ONE_HALF` is 2^-1, also exact everywhere: flagged.
    let d = Float::from_float_prec_round(Float::ONE_HALF, prec, rm).0;
    // The by-reference variants are the same no-op rounding: flagged.
    let e = Float::from_float_prec_round_ref(&Float::ONE, prec, rm).0;
    // An arbitrary Float may round: fine.
    let f = Float::from_float_prec_round(x, prec, rm).0;
    (a, b, c, d, e, f)
}

fn g(
    x: BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (
    Float,
    Float,
    BigExponentFloat,
    BigExponentFloat,
    BigExponentFloat,
    Float,
) {
    // Converting a single-significant-bit `BigExponentFloat` constant to a `Float`: flagged.
    let a = Float::from_big_exponent_float_prec_round(BigExponentFloat::ONE, prec, rm).0;
    let b = Float::from_big_exponent_float_prec_ref(&BigExponentFloat::ONE_HALF, prec).0;
    // Rounding such a constant gives the constant back: flagged.
    let c = BigExponentFloat::TWO.round_to_prec_round(prec, rm).0;
    let d = BigExponentFloat::NEGATIVE_ONE.round_to_prec_ref(prec).0;
    // An arbitrary `BigExponentFloat` may round: fine.
    let e = x.round_to_prec_round_ref(prec, rm).0;
    let f = Float::from_big_exponent_float_prec_round(x, prec, rm).0;
    (a, b, c, d, e, f)
}

fn main() {
    let y = BigExponentFloat::from(std::hint::black_box(3u32));
    let _ = g(y, std::hint::black_box(10), RoundingMode::Nearest);
    let x = Float::from(std::hint::black_box(3u32));
    let _ = f(x, std::hint::black_box(10), RoundingMode::Nearest);
}
