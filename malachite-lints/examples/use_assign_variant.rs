use core::ops::Neg;
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::{One, Two};
use malachite_float::Float;

fn main() {
    let mut x = Float::ONE;
    let y = Float::TWO;
    let p = 10u64;
    // Reassigning the result of a method on the receiver: flagged (inherent assign family).
    x = x.add_prec_val_ref(&y, p).0;
    // Trait-based assign companion: flagged.
    x = x.abs();
    // Behind `&` or `.clone()`, the receiver is still the assigned place: flagged.
    x = x.clone().neg();
    // A different receiver: fine.
    let mut z = Float::ONE;
    z = (&x).add_prec_ref_ref(&y, p).0;
    let _ = z;
}
