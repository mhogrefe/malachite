use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::Float;
use std::cmp::Ordering;

#[allow(clippy::type_complexity)]
fn f(
    prec: u64,
) -> (
    Float,
    Float,
    Float,
    Float,
    Float,
    Float,
    Float,
    Float,
    Float,
    Ordering,
) {
    // Sources at a different precision, so that binding them is not itself redundant.
    let big = prec + 2;
    // The receiver was bound at `prec` and is then re-rounded to `prec`: flagged.
    let arg = Float::pi_prec(big)
        .0
        .mul_prec_round(Float::one_prec(big), prec, Floor)
        .0;
    let a = arg.exp_prec_round(prec, Floor).0;
    // The house tuple shape pins the precision too: flagged.
    let (arg, o) = Float::pi_prec(big)
        .0
        .add_prec_round(Float::one_prec(big), prec, Ceiling);
    let b = arg.exp_prec_round(prec, Ceiling).0;
    // The `Nearest` shorthand with the `Ordering` discarded: flagged, suggesting plain `exp()`.
    let arg = Float::pi_prec(big)
        .0
        .mul_prec(Float::one_prec(big), prec)
        .0;
    let c = arg.exp_prec(prec).0;
    // Constants and constructors pin the precision as well: flagged.
    let pi = Float::pi_prec(prec).0;
    let d = pi.exp_prec_round(prec, Floor).0;
    // A different precision: fine.
    let arg = Float::pi_prec(big)
        .0
        .mul_prec_round(Float::one_prec(big), prec, Floor)
        .0;
    let e = arg.exp_prec_round(big, Floor).0;
    // The precision variable changes between the binding and the use: fine.
    let mut prec_2 = prec;
    let arg = Float::pi_prec(big)
        .0
        .mul_prec_round(Float::one_prec(big), prec_2, Floor)
        .0;
    prec_2 += 1;
    let g = arg.exp_prec_round(prec_2, Floor).0;
    // A mutable receiver may have been modified since it was bound: fine. (Negated twice rather
    // than once only so that `assign_then_consumed_once` does not fire here too.)
    let mut arg = Float::pi_prec(big)
        .0
        .mul_prec_round(Float::one_prec(big), prec, Floor)
        .0;
    arg.neg_assign();
    arg.neg_assign();
    let h = arg.exp_prec_round(prec, Floor).0;
    // Binary operations qualify when every `Float` operand is pinned at the same precision:
    // flagged.
    let u = Float::pi_prec(prec).0;
    let v = Float::one_prec(prec);
    let i = u.mul_prec_round(v, prec, Floor).0;
    // But not when another `Float` operand's precision is unknown: fine.
    let u = Float::pi_prec(prec).0;
    let w = Float::one_prec(big);
    let j = u.mul_prec_round(w, prec, Floor).0;
    (a, b, c, d, e, g, h, i, j, o)
}

fn main() {
    let _ = f(std::hint::black_box(20));
}
