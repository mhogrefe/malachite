use malachite_base::num::arithmetic::traits::ModMulAssign;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitIterable;
use malachite_nz::natural::Natural;

pub fn simple_binary_mod_pow(x: &Natural, exp: &Natural, m: &Natural) -> Natural {
    if *m == 1 {
        return Natural::ZERO;
    }
    let mut out = Natural::ONE;
    for bit in exp.bits().rev() {
        out.mod_mul_assign(out.clone(), m);
        if bit {
            out.mod_mul_assign(x, m);
        }
    }
    out
}
