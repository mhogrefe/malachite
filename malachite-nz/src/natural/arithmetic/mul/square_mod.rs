use malachite_base::num::arithmetic::traits::{RoundToMultipleOfPowerOfTwo, WrappingAddAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_test_zero;

use fail_on_untested_path;
use natural::arithmetic::add::{
    limbs_add_greater_to_out, limbs_add_same_length_to_out, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::mul::fft::{
    _limbs_mul_fft, _limbs_mul_fft_best_k, SQR_FFT_MODF_THRESHOLD,
};
use natural::arithmetic::mul::mul_mod::{FFT_FIRST_K, MUL_FFT_MODF_THRESHOLD};
use natural::arithmetic::shr::limbs_slice_shr_in_place;
use natural::arithmetic::square::limbs_square_to_out;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_right, limbs_sub_in_place_left,
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
    limbs_sub_to_out,
};
use platform::Limb;

//TODO tune
const SQRMOD_BNM1_THRESHOLD: usize = 16;

/// This is mpn_sqrmod_bnm1_next_size from mpn/generic/sqrmod_bnm1.c, GMP 6.1.2.
pub(crate) fn _limbs_square_mod_base_pow_n_minus_1_next_size(n: usize) -> usize {
    if n < SQRMOD_BNM1_THRESHOLD {
        n
    } else if n < 4 * (SQRMOD_BNM1_THRESHOLD - 1) + 1 {
        (n + (2 - 1)) & 2usize.wrapping_neg()
    } else if n < 8 * (SQRMOD_BNM1_THRESHOLD - 1) + 1 {
        (n + (4 - 1)) & 4usize.wrapping_neg()
    } else {
        let nh = (n + 1) >> 1;
        if nh < SQR_FFT_MODF_THRESHOLD {
            (n + (8 - 1)) & 8usize.wrapping_neg()
        } else {
            nh.round_to_multiple_of_power_of_two(
                u64::exact_from(_limbs_mul_fft_best_k(nh, true)),
                RoundingMode::Ceiling,
            ) << 1
        }
    }
}

/// This is mpn_sqrmod_bnm1_itch from gmp-impl.h, GMP 6.1.2.
pub(crate) fn _limbs_square_mod_base_pow_n_minus_1_scratch_len(rn: usize, an: usize) -> usize {
    rn + 3 + (if an > rn >> 1 { an } else { 0 })
}

/// Input is {ap,rn}; output is {rp,rn}, computation is
/// mod B^rn - 1, and values are semi-normalised; zero is represented
/// as either 0 or B^n - 1.  Needs a scratch of 2rn limbs at tp.
///
/// This is mpn_bc_sqrmod_bnm1 from mpn/generic/sqrmod_bnm1.c, GMP 6.1.2, where rp != tp.
fn _limbs_square_mod_base_pow_n_minus_1_basecase(
    out: &mut [Limb],
    xs: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert!(scratch.len() >= xs.len() << 1);
    limbs_square_to_out(scratch, xs);
    split_into_chunks_mut!(scratch, n, [scratch_lo, scratch_hi], _unused);
    // If carry == 1, then the value of out is at most B ^ n - 2, so there can be no overflow when
    // adding in the carry.
    if limbs_add_same_length_to_out(out, scratch_lo, scratch_hi) {
        assert!(!limbs_slice_add_limb_in_place(&mut out[..n], 1));
    }
}

/// Input is {ap,rn+1}; output is {rp,rn+1}, in
/// semi-normalised representation, computation is mod B^rn + 1. Needs
/// a scratch area of 2rn + 2 limbs at tp; tp == rp is allowed.
/// Output is normalised.
///
/// This is mpn_bc_sqrmod_bnp1 from mpn/generic/sqrmod_bnm1.c, GMP 6.1.2, where rp == tp.
fn _limbs_square_mod_base_pow_n_plus_1_basecase(out: &mut [Limb], xs: &[Limb], rn: usize) {
    assert_ne!(rn, 0);
    limbs_square_to_out(out, &xs[..rn + 1]);
    split_into_chunks_mut!(out, rn, [out_0, out_1], out_2);
    assert_eq!(out_2[1], 0);
    assert!(out_2[0] < Limb::MAX);
    let cy = out_2[0]
        + if limbs_sub_same_length_in_place_left(out_0, out_1) {
            1
        } else {
            0
        };
    out_1[0] = 0;
    assert!(!limbs_slice_add_limb_in_place(&mut out[..rn + 1], cy));
}

/// Computes {rp,MIN(rn,2an)} <- {ap,an}^2 Mod(B^rn-1)
///
/// The result is expected to be ZERO if and only if the operand
/// already is. Otherwise the class [0] Mod(B^rn-1) is represented by
/// B^rn-1.
/// It should not be a problem if sqrmod_bnm1 is used to
/// compute the full square with an <= 2*rn, because this condition
/// implies (B^an-1)^2 < (B^rn-1) .
///
/// Requires rn/4 < an <= rn
/// Scratch need: rn/2 + (need for recursive call OR rn + 3). This gives
///
/// S(n) <= rn/2 + MAX (rn + 4, S(n/2)) <= 3/2 rn + 4
///
/// This is mpn_sqrmod_bnm1 from mpn/generic/sqrmod_bnm1.c, GMP 6.1.2.  
pub fn _limbs_square_mod_base_pow_n_minus_1(
    out: &mut [Limb],
    rn: usize,
    xs: &[Limb],
    scratch: &mut [Limb],
) {
    let an = xs.len();
    assert_ne!(an, 0);
    assert!(an <= rn);
    if (rn & 1) != 0 || rn < SQRMOD_BNM1_THRESHOLD {
        if an < rn {
            if 2 * an <= rn {
                limbs_square_to_out(out, xs);
            } else {
                fail_on_untested_path(
                    "((rn & 1) != 0 || rn < SQRMOD_BNM1_THRESHOLD) && an < rn < 2 * an",
                );
                limbs_square_to_out(scratch, xs);
                let cy = if limbs_add_greater_to_out(out, &scratch[..rn], &scratch[rn..2 * an]) {
                    1
                } else {
                    0
                };
                assert!(!limbs_slice_add_limb_in_place(&mut out[..rn], cy));
            }
        } else {
            _limbs_square_mod_base_pow_n_minus_1_basecase(out, &xs[..rn], scratch);
        }
    } else {
        let n = rn >> 1;
        assert!(2 * an > n);
        // Compute xm = a^2 mod (B^n - 1), xp = a^2 mod (B^n + 1)
        // and crt together as
        // x = -xp * B^n + (B^n + 1) * [ (xp + xm)/2 mod (B^n-1)]
        let a0 = &xs[..];
        if an > n {
            let a1 = &xs[n..];
            let (xp_lo, xp_hi) = scratch.split_at_mut(n);
            let cy = if limbs_add_greater_to_out(xp_lo, &a0[..n], &a1[..an - n]) {
                1
            } else {
                0
            };
            limbs_slice_add_limb_in_place(xp_lo, cy);
            _limbs_square_mod_base_pow_n_minus_1(out, n, xp_lo, xp_hi);
        } else {
            _limbs_square_mod_base_pow_n_minus_1(out, n, &a0[..an], scratch);
        }
        let (xp, sp1) = scratch.split_at_mut(2 * n + 2);
        if an > n {
            let a1 = &xs[n..];
            let cy = if limbs_sub_to_out(sp1, &a0[..n], &a1[..an - n]) {
                1
            } else {
                0
            };
            sp1[n] = 0;
            assert!(!limbs_slice_add_limb_in_place(&mut sp1[..n + 1], cy));
            let anp = n + usize::exact_from(sp1[n]);
            let k = if n < MUL_FFT_MODF_THRESHOLD {
                0
            } else {
                let mut k = _limbs_mul_fft_best_k(n, true);
                let mut mask = (1 << k) - 1;
                while n & mask != 0 {
                    k -= 1;
                    mask >>= 1;
                }
                k
            };
            if k >= FFT_FIRST_K {
                let xs = &sp1[..anp];
                xp[n] = if _limbs_mul_fft(xp, n, xs, xs, k) {
                    1
                } else {
                    0
                };
            } else {
                _limbs_square_mod_base_pow_n_plus_1_basecase(xp, sp1, n);
            }
        } else {
            let ap1 = a0;
            let anp = an;
            let k = if n < MUL_FFT_MODF_THRESHOLD {
                0
            } else {
                let mut k = _limbs_mul_fft_best_k(n, true);
                let mut mask = (1 << k) - 1;
                while n & mask != 0 {
                    k -= 1;
                    mask >>= 1;
                }
                k
            };
            if k >= FFT_FIRST_K {
                let xs = &ap1[..anp];
                xp[n] = if _limbs_mul_fft(xp, n, xs, xs, k) {
                    1
                } else {
                    0
                };
            } else {
                assert!(anp <= n);
                assert!(anp << 1 > n);
                limbs_square_to_out(xp, &a0[..an]);
                let anp = 2 * an - n;
                let (xp_lo, xp_hi) = xp.split_at_mut(n);
                let cy = if limbs_sub_in_place_left(xp_lo, &xp_hi[..anp]) {
                    1
                } else {
                    0
                };
                xp_hi[0] = 0;
                assert!(!limbs_slice_add_limb_in_place(&mut xp[..n + 1], cy));
            }
        }
        // Here the CRT recomposition begins.
        //
        // xm <- (xp + xm)/2 = (xp + xm)B^n/2 mod (B^n-1)
        // Division by 2 is a bitwise rotation.
        //
        // Assumes xp normalised mod (B^n+1).
        //
        // The residue class [0] is represented by [B^n-1]; except when
        // both input are ZERO.
        // xp[n] == 1 implies {xp,n} == ZERO
        let mut cy = xp[n].wrapping_add(
            if limbs_slice_add_same_length_in_place_left(&mut out[..n], &xp[..n]) {
                1
            } else {
                0
            },
        );
        cy.wrapping_add_assign(out[0] & 1);
        limbs_slice_shr_in_place(&mut out[..n], 1);
        assert!(cy <= 2);
        let hi = cy << (Limb::WIDTH - 1); // (cy&1) << ...
        cy >>= 1;
        // We can have cy != 0 only if hi = 0...
        assert!(!out[n - 1].get_highest_bit());
        out[n - 1] |= hi;
        // ... rp[n-1] + cy can not overflow, the following INCR is correct.
        assert!(cy <= 1);
        // Next increment can not overflow, read the previous comments about cy.
        assert!(cy == 0 || !out[n - 1].get_highest_bit());
        assert!(!limbs_slice_add_limb_in_place(&mut out[..n], cy));
        //  Compute the highest half:
        // ([(xp + xm)/2 mod (B^n-1)] - xp ) * B^n
        if 2 * an < rn {
            // Note that in this case, the only way the result can equal
            // zero mod B^{rn} - 1 is if the input is zero, and
            // then the output of both the recursive calls and this CRT
            // reconstruction is zero, not B^{rn} - 1.
            let (rp_lo, rp_hi) = out.split_at_mut(n);
            cy = if limbs_sub_same_length_to_out(rp_hi, &rp_lo[..2 * an - n], &xp[..2 * an - n]) {
                1
            } else {
                0
            };
            cy = xp[n].wrapping_add(
                if _limbs_sub_same_length_with_borrow_in_in_place_right(
                    &out[2 * an - n..rn - n],
                    &mut xp[2 * an - n..rn - n],
                    cy != 0,
                ) {
                    1
                } else {
                    0
                },
            );
            assert!(slice_test_zero(&xp[2 * an - n + 1..rn - n]));
            cy = if limbs_sub_limb_in_place(&mut out[..2 * an], cy) {
                1
            } else {
                0
            };
            assert_eq!(cy, xp[2 * an - n]);
        } else {
            let (rp_lo, rp_hi) = out.split_at_mut(n);
            cy = xp[n].wrapping_add(if limbs_sub_same_length_to_out(rp_hi, rp_lo, &xp[..n]) {
                1
            } else {
                0
            });
            // cy = 1 only if {xp,n+1} is not ZERO, i.e. {rp,n} is not ZERO.
            // DECR will affect _at most_ the lowest n limbs.
            assert!(!limbs_sub_limb_in_place(&mut out[..2 * n], cy));
        }
    }
}
