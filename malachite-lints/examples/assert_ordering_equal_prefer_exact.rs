use malachite_base::num::basic::traits::{One, Two};
use malachite_float::Float;
use malachite_nz::natural::Natural;
use std::cmp::Ordering::*;

fn main() {
    let p = 10u64;
    // Method-call form: flagged, since `add_prec_round` exists.
    let (x, o) = Float::ONE.add_prec(Float::TWO, p);
    assert_eq!(o, Equal);
    // Free associated-function form, argument order swapped: still flagged.
    let (y, oy) = Float::from_natural_prec(Natural::from(p), p);
    assert_eq!(Equal, oy);
    // Ordering compared against something other than Equal: fine.
    let (z, oz) = Float::ONE.add_prec(Float::TWO, p);
    assert_eq!(oz, Less);
    let _ = (x, y, z);
}
