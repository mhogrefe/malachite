use malachite_base::num::arithmetic::traits::{
    PowerOfTwo, XMulYIsZZ, XXSubYYIsZZ, XXXAddYYYIsZZZ, XXXSubYYYIsZZZ, XXXXAddYYYYIsZZZZ,
};
use malachite_base::num::basic::integers::PrimitiveInteger;

use natural::Natural;
use platform::Limb;

/// This is part of fmpz_mod_ctx_init from fmpz_mod/ctx_init.c, FLINT Dev 1.
pub fn precompute_mod_mul_two_limbs(ns: &[Limb]) -> (Limb, Limb, Limb) {
    let out_limbs =
        (Natural::power_of_two(Limb::WIDTH << 2) / Natural::from_limbs_asc(ns)).into_limbs_asc();
    assert_eq!(out_limbs.len(), 3);
    (out_limbs[2], out_limbs[1], out_limbs[0])
}

/// Standard Barrett reduction: (set r = FLINT_BITS)
///
/// We have n fits into 2 words and 2^r < n < 2^(2r). Therefore
/// 2^(3r) > 2^(4r) / n > 2^(2r) and the precomputed number
/// ninv = floor(2^(4r) / n) fits into 3 words.
/// The inputs b and c are < n and therefore fit into 2 words.
///
/// The computation of a = b*c mod n is:
/// x = b*c             x < n^2 and therefore fits into 4 words
/// z = (x >> r)*ninv   z <= n*2^(3*r) and therefore fits into 5 words
/// q = (z >> (3r))*n   q fits into 4 words
/// x = x - q           x fits into 3 words after the subtraction
/// at this point the canonical reduction in the range [0, n) is one of
///     a = x, a = x - n, or a = x - 2n
///
/// This is _fmpz_mod_mul2 from fmpz_mod/mul.c, FLINT Dev 1.
pub fn _limbs_mod_mul_two_limbs(
    b1: Limb,
    b0: Limb,
    c1: Limb,
    c0: Limb,
    m1: Limb,
    m0: Limb,
    inv2: Limb,
    inv1: Limb,
    inv0: Limb,
) -> (Limb, Limb) {
    // x[3:0] = b[1:0]*c[1:0]
    let (t2, t1) = Limb::x_mul_y_is_zz(b0, c1);
    let (s2, s1) = Limb::x_mul_y_is_zz(b1, c0);
    let (x3, x2) = Limb::x_mul_y_is_zz(b1, c1);
    let (x1, x0) = Limb::x_mul_y_is_zz(b0, c0);
    let t3 = 0;
    let (t3, t2, t1) = Limb::xxx_add_yyy_is_zzz(t3, t2, t1, 0, s2, s1);
    let (x3, x2, x1) = Limb::xxx_add_yyy_is_zzz(x3, x2, x1, t3, t2, t1);

    // z[5:0] = x[3:1] * ninv[2:0], z[5] should end up zero
    let (z1, _) = Limb::x_mul_y_is_zz(x1, inv0);
    let (z3, z2) = Limb::x_mul_y_is_zz(x2, inv1);
    let z4 = x3.wrapping_mul(inv2);
    let (t3, t2) = Limb::x_mul_y_is_zz(x3, inv0);
    let (s3, s2) = Limb::x_mul_y_is_zz(x1, inv2);
    let t4 = 0;
    let (t4, t3, t2) = Limb::xxx_add_yyy_is_zzz(t4, t3, t2, 0, s3, s2);
    let (u2, u1) = Limb::x_mul_y_is_zz(x2, inv0);
    let (u4, u3) = Limb::x_mul_y_is_zz(x3, inv1);
    let (z4, z3, z2) = Limb::xxx_add_yyy_is_zzz(z4, z3, z2, t4, t3, t2);
    let (v2, v1) = Limb::x_mul_y_is_zz(x1, inv1);
    let (v4, v3) = Limb::x_mul_y_is_zz(x2, inv2);
    let (z4, z3, z2, z1) = Limb::xxxx_add_yyyy_is_zzzz(z4, z3, z2, z1, u4, u3, u2, u1);
    let (z4, z3, _, _) = Limb::xxxx_add_yyyy_is_zzzz(z4, z3, z2, z1, v4, v3, v2, v1);

    // q[3:0] = z[4:3] * n[1:0], q[3] is not needed
    // x[3:0] -= q[3:0], x[3] should end up zero
    let (t2, t1) = Limb::x_mul_y_is_zz(z3, m1);
    let (s2, s1) = Limb::x_mul_y_is_zz(z4, m0);
    let (q1, q0) = Limb::x_mul_y_is_zz(z3, m0);
    let (x2, x1) = Limb::xx_sub_yy_is_zz(x2, x1, t2, t1);
    let q2 = z4.wrapping_mul(m1);
    let (x2, x1) = Limb::xx_sub_yy_is_zz(x2, x1, s2, s1);
    let (x2, x1, x0) = Limb::xxx_sub_yyy_is_zzz(x2, x1, x0, q2, q1, q0);

    // at most two subtractions of n, use q as temp space
    let (q2, q1, q0) = Limb::xxx_sub_yyy_is_zzz(x2, x1, x0, 0, m1, m0);
    let a1;
    let a0;
    if !q2.get_highest_bit() {
        let (x2, x1, x0) = Limb::xxx_sub_yyy_is_zzz(q2, q1, q0, 0, m1, m0);
        if !x2.get_highest_bit() {
            a1 = x1;
            a0 = x0;
        } else {
            a1 = q1;
            a0 = q0;
        }
    } else {
        a1 = x1;
        a0 = x0;
    }
    (a1, a0)
}
