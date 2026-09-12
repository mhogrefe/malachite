use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_q::Rational;

fn f(q: Rational, r: Rational) -> (Rational, Rational, Rational, Rational, Rational) {
    // the integer part truncated toward zero, subtracted in a second statement: flagged, since
    // `q % Rational::ONE` is the same
    let whole = Rational::from(Integer::rounding_from(&q, Down).0);
    let a = &q - whole;
    // the same, inline: flagged
    let b = &q - Rational::from(Integer::rounding_from(&q, Down).0);
    // the integer part toward negative infinity: flagged, with `mod_op` rather than `%`
    let c = &q - Rational::from(Integer::rounding_from(&q, Floor).0);
    // the nearest integer, giving a symmetric remainder, which has no such shorthand: fine
    let d = &q - Rational::from(Integer::rounding_from(&q, Nearest).0);
    // some other value's integer part: fine
    let e = &q - Rational::from(Integer::rounding_from(&r, Down).0);
    (a, b, c, d, e)
}

fn main() {
    let _ = f(
        Rational::from(std::hint::black_box(3u32)),
        Rational::from(std::hint::black_box(5u32)),
    );
}
