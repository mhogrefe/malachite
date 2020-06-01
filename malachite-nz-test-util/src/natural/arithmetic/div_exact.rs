use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_nz::natural::arithmetic::div_exact::MAX_OVER_3;
use malachite_nz::platform::Limb;

// This is MODLIMB_INVERSE_3 from gmp-impl.h, GMP 6.1.2.
const MODLIMB_INVERSE_3: Limb = (MAX_OVER_3 << 1) | 1;
const CEIL_MAX_OVER_3: Limb = MAX_OVER_3 + 1;
const CEIL_2_MAX_OVER_3: Limb = ((Limb::MAX >> 1) / 3 + 1) | (1 << (Limb::WIDTH - 1));

/// Benchmarks show that this algorithm is always worse than the default.
///
/// This is mpn_divexact_by3c from mpn/generic diveby3.c, GMP 6.1.2, with DIVEXACT_BY3_METHOD == 1,
/// no carry-in, and no return value.
pub fn limbs_div_exact_3_to_out_alt(out: &mut [Limb], xs: &[Limb]) {
    let len = xs.len();
    assert_ne!(len, 0);
    assert!(out.len() >= len);
    let last_index = len - 1;
    let mut big_carry = 0;
    for i in 0..last_index {
        let (diff, carry) = xs[i].overflowing_sub(big_carry);
        big_carry = if carry { 1 } else { 0 };
        let out_limb = diff.wrapping_mul(MODLIMB_INVERSE_3);
        out[i] = out_limb;
        if out_limb >= CEIL_MAX_OVER_3 {
            big_carry += 1;
            if out_limb >= CEIL_2_MAX_OVER_3 {
                big_carry += 1;
            }
        }
    }
    out[last_index] = xs[last_index]
        .wrapping_sub(big_carry)
        .wrapping_mul(MODLIMB_INVERSE_3);
}

/// Benchmarks show that this algorithm is always worse than the default.
///
/// This is mpn_divexact_by3c from mpn/generic diveby3.c, GMP 6.1.2, with DIVEXACT_BY3_METHOD == 1,
/// no carry-in, and no return value, where rp == up.
pub fn limbs_div_exact_3_in_place_alt(xs: &mut [Limb]) {
    let len = xs.len();
    assert_ne!(len, 0);
    let last_index = len - 1;
    let mut big_carry = 0;
    for limb in xs[..last_index].iter_mut() {
        let (diff, carry) = limb.overflowing_sub(big_carry);
        big_carry = if carry { 1 } else { 0 };
        let out_limb = diff.wrapping_mul(MODLIMB_INVERSE_3);
        *limb = out_limb;
        if out_limb >= CEIL_MAX_OVER_3 {
            big_carry += 1;
            if out_limb >= CEIL_2_MAX_OVER_3 {
                big_carry += 1;
            }
        }
    }
    xs[last_index] = xs[last_index]
        .wrapping_sub(big_carry)
        .wrapping_mul(MODLIMB_INVERSE_3);
}
