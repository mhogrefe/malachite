// `z`'s initial value is never read; it only gives the assignment below a different receiver.
#![allow(unused_assignments)]

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

    // Primitive receivers: the `*_assign` companion comes from a `malachite_base` trait.
    let mut n = 10u64;
    let k = 3u64;
    // Saturating, wrapping, and checked-style families: flagged.
    n = n.saturating_mul(k);
    n = n.wrapping_add(k);
    // An `overflowing_*` method's `(value, bool)` has the same shape as `(value, Ordering)`:
    // flagged, and the discarded flag becomes the assign variant's return value.
    n = n.overflowing_sub(k).0;
    // Shadowing `let` rebinds work the same way: flagged.
    let m = 7u64;
    let m = m.wrapping_mul(k);
    let (m, overflow) = m.overflowing_add(k);
    let _ = overflow;
    // A different receiver: fine.
    n = k.saturating_mul(m);
    // No `*Assign` trait for this family, so there is nothing to suggest: fine.
    n = n.count_ones().into();
    let _ = n;
}
