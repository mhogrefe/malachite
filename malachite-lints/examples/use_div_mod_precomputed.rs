use malachite_base::num::arithmetic::traits::{DivAssignMod, DivMod, DivRem};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

const D: Natural = Natural::const_from(123);
const D_7: Natural = Natural::const_from(7);
const E: Integer = Integer::const_from_signed(-123);
const X_0: Natural = Natural::const_from(5);
const X_1: Natural = Natural::const_from(1000);
const Y_0: Integer = Integer::const_from_signed(-5);

fn main() {
    let d = D;
    let e = E;
    let xs = [X_0, X_1];
    let ys = [Y_0];

    // A loop-invariant bignum divisor: flagged.
    for x in &xs {
        let _ = x.div_mod(&d);
    }
    let mut i = 0;
    while i < 2 {
        let _ = &xs[i] % &d;
        i += 1;
    }
    let mut i = 0;
    while i < 2 {
        let _ = &xs[i] / &d;
        i += 1;
    }
    for y in &ys {
        let _ = y.div_rem(&e);
    }
    loop {
        let mut x = xs[0].clone();
        let _ = x.div_assign_mod(&d);
        break;
    }

    // The divisor is defined inside the loop: not flagged.
    for x in &xs {
        let d_inner = D_7;
        let _ = x.div_mod(&d_inner);
    }
    // The divisor is mutated inside the loop: not flagged.
    let mut dm = D;
    for x in &xs {
        let _ = x.div_mod(&dm);
        dm += Natural::ONE;
    }
    // A primitive divisor: deliberately not flagged, since on processors with fast dividers the
    // preinverted form is not a clear win.
    let dp = 123u64;
    for x in [5u64, 1000] {
        let _ = x.div_mod(dp);
    }
    // Not in a loop: not flagged.
    let _ = (&xs[0]).div_mod(&d);
    // The division is inside a closure: not seen.
    let _: Vec<_> = xs.iter().map(|x| x.div_mod(&d)).collect();
}
