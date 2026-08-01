use core::cmp::Ordering::*;
use malachite_base::num::comparison::traits::{OrdAbs, OrdAbsDouble, OrdDouble};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

const X: Natural = Natural::const_from(5);
const Y: Natural = Natural::const_from(3);
const A: Integer = Integer::const_from_signed(-5);
const B: Integer = Integer::const_from_unsigned(3);

fn main() {
    // Reversing a symmetric comparison: flagged.
    let _ = X.cmp(&Y).reverse();
    let _ = A.cmp_abs(&B).reverse();
    let _ = 1.0f64.total_cmp(&2.0).reverse();
    // Still flagged when the result feeds a `match`, which is where the flip is easiest to lose.
    match X.cmp(&Y).reverse() {
        Less => println!("less"),
        Equal => println!("equal"),
        Greater => println!("greater"),
    }

    // `cmp_double` is not symmetric: `a` against twice `b` reversed is not `b` against twice `a`,
    // so `.reverse()` is the only way to flip the operands here: fine.
    let _ = X.cmp_double(&Y).reverse();
    let _ = A.cmp_abs_double(&B).reverse();
    // A comparison that is not reversed: fine.
    let _ = X.cmp(&Y);
    // `reverse` on something that is not an `Ordering`: fine.
    let mut v = vec![1, 2, 3];
    v.reverse();
}
