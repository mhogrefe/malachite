use malachite_base::num::arithmetic::traits::{Pow, PowAssign};
use malachite_nz::natural::Natural;
use malachite_q::Rational;

fn main() {
    let x = const { Natural::const_from(100) };
    // Multiplying a value by itself: flagged.
    let _ = &x * &x;
    // Raising to the power of 2: flagged.
    let _ = (&x).pow(2);
    let mut q = const { Rational::const_from_unsigneds(22, 7) };
    q.pow_assign(2u64);
    let _ = q;
    // Different operands: fine.
    let y = const { Natural::const_from(7) };
    let _ = &x * &y;
    // Other exponents: fine.
    let _ = (&x).pow(3);
}
