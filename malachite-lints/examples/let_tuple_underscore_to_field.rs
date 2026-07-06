use malachite_base::num::basic::traits::{One, Two};
use malachite_float::Float;

fn main() {
    let p = 10u64;
    // Keeping the value, discarding the ordering: flagged.
    let (x, _) = Float::ONE.add_prec(Float::TWO, p);
    // Keeping the ordering, discarding the value: flagged (`.1`).
    let (_, o) = Float::ONE.add_prec(Float::TWO, p);
    // `mut` binding is preserved in the suggestion: flagged.
    let (mut y, _) = Float::ONE.add_prec(Float::TWO, p);
    y.add_prec_assign_ref(&Float::ONE, p);
    // Both fields used: fine.
    let (z, oz) = Float::ONE.add_prec(Float::TWO, p);
    let _ = (x, o, y, z, oz);
}
