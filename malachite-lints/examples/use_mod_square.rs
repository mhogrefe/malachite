use malachite_base::num::arithmetic::traits::{
    ModMul, ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_nz::natural::Natural;

const M: Natural = Natural::const_from(497);
const X: Natural = Natural::const_from(100);
const Y: Natural = Natural::const_from(101);

fn main() {
    let m = M;
    let x = X;
    let y = Y;

    // Equal multiplicand operands: flagged.
    let _ = (&x).mod_mul(&x, &m);
    let _ = x.clone().mod_mul(x.clone(), m.clone());
    let mut w = x.clone();
    w.mod_mul_assign(w.clone(), &m);
    let _ = 100u32.mod_mul(100u32, 497u32);

    // Equal operands with precomputed data on a Natural: flagged.
    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);
    let _ = (&x).mod_mul_precomputed(&x, &m, &data);
    let mut w = x.clone();
    w.mod_mul_precomputed_assign(w.clone(), &m, &data);

    // Different operands: not flagged.
    let _ = (&x).mod_mul(&y, &m);
    let _ = (&x).mod_mul_precomputed(&y, &m, &data);
    let mut w = x.clone();
    w.mod_mul_precomputed_assign(&y, &m, &data);

    // Equal operands with precomputed data on a primitive: not flagged, since the square form
    // takes the modular-exponentiation data rather than the bare multiplication inverse.
    let inverse = u32::precompute_mod_mul_data(&497);
    let _ = 100u32.mod_mul_precomputed(100u32, 497, &inverse);
}
