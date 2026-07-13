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
    // Shadowing `let` rebinds of the same name: flagged.
    let t = Float::TWO;
    let t = t.exp_prec(p).0;
    let t = t.div_prec_val_ref(&y, p).0;
    // The `(value, Ordering)` tuple form, first element shadowing the receiver: flagged.
    let (t, o) = t.add_prec_val_ref(&y, p);
    let _ = o;
    // A shadowing `let` from a different receiver: fine.
    let u = (&t).add_prec_ref_ref(&y, p).0;
    let _ = u;
    // A reference-typed receiver shadowed by an owned result is a conversion, not an in-place
    // opportunity: fine.
    let v = &t;
    let v = v.abs();
    let _ = v;
    let _ = t;
}
