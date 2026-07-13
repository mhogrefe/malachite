use core::cmp::Ordering::*;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_nz::natural::Natural;
use malachite_q::Rational;

const X: Rational = Rational::const_from_signeds(22, 7);
const N: Natural = Natural::const_from(10);

fn main() {
    let x = X;
    let n = N;
    let k = 10u32;
    // Operator comparison with a named constant: flagged.
    if x == Rational::ONE {
        return;
    }
    if x > Rational::TWO {
        return;
    }
    if x < Rational::NEGATIVE_ONE {
        return;
    }
    if n == Natural::ZERO {
        return;
    }
    // Comparison methods with a named constant: flagged.
    match x.cmp(&Rational::ONE) {
        Equal => (),
        Greater => (),
        Less => (),
    }
    let _ = x.partial_cmp(&Rational::TWO);
    // Comparison methods with `from(primitive)`: flagged.
    let _ = n.cmp(&Natural::from(k));
    // Comparing with the primitive directly: fine.
    if x == 1u32 {
        return;
    }
    let _ = x.partial_cmp(&1u32);
    // Comparing two general bignums: fine.
    let y = X;
    let _ = x.cmp(&y);
    if x == y {}
}
