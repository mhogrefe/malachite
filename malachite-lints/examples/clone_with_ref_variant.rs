use malachite_base::num::basic::traits::{One, Two};
use malachite_float::Float;
use malachite_nz::natural::Natural;

fn main() {
    let x = const { Natural::const_from(100) };
    let y = const { Natural::const_from(7) };
    // Cloned operands where the operator is implemented for references: flagged.
    let _ = x.clone() + &y;
    let _ = &x + y.clone();
    // A plain clone into a binding: fine.
    let mut m = x.clone();
    // A cloned right side of a compound assignment: flagged.
    m += y.clone();
    let _ = m;
    // A cloned receiver with a by-reference sibling: flagged.
    let f = Float::TWO;
    let g = Float::ONE;
    let p = 10u64;
    let _ = f.clone().exp_prec(p);
    // A cloned argument with a by-reference sibling: flagged.
    let _ = (&f).add_prec_ref_val(g.clone(), p);
}
