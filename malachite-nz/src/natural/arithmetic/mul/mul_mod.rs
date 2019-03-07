use malachite_base::limbs::limbs_test_zero;
use malachite_base::misc::Max;
use malachite_base::num::PrimitiveInteger;
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul::fft::{mpn_fft_best_k, mpn_fft_next_size, mpn_mul_fft};
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_same_length_to_out};
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_place_right, limbs_sub_in_place_left,
    limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use platform::Limb;

//TODO tune
pub(crate) const MULMOD_BNM1_THRESHOLD: usize = 13;
pub(crate) const MUL_FFT_MODF_THRESHOLD: usize = 396;

//TODO test
// This is mpn_mulmod_bnm1_next_size from mpn/generic/mulmod_bnm1.c.
pub(crate) fn mpn_mulmod_bnm1_next_size(n: usize) -> usize {
    if n < MULMOD_BNM1_THRESHOLD {
        return n;
    } else if n < 4 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (2 - 1)) & 2_usize.wrapping_neg();
    } else if n < 8 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (4 - 1)) & 4_usize.wrapping_neg();
    }
    let nh = (n + 1) >> 1;
    if nh < MUL_FFT_MODF_THRESHOLD {
        (n + (8 - 1)) & 8_usize.wrapping_neg()
    } else {
        2 * mpn_fft_next_size(nh, mpn_fft_best_k(nh, false))
    }
}

//TODO test
// This is mpn_mulmod_bnm1_itch from gmp-impl.h.
pub(crate) fn mpn_mulmod_bnm1_itch(rn: usize, an: usize, bn: usize) -> usize {
    let n = rn >> 1;
    rn + 4
        + if an > n {
            if bn > n {
                rn
            } else {
                n
            }
        } else {
            0
        }
}

// First k to use for an FFT modF multiply.  A modF FFT is an order
// log(2^k)/log(2^(k-1)) algorithm, so k=3 is merely 1.5 like Karatsuba,
// whereas k=4 is 1.33 which is faster than toom3 at 1.485.
const FFT_FIRST_K: usize = 4;

// docs preserved
// Multiplication mod B ^ n - 1.
//
// Computes {rp, MIN(rn,an+bn)} <- {ap,an} * {bp,bn} Mod(B ^ rn-1)
//
// The result is expected to be 0 if and only if one of the operands already is. Otherwise the class
// [0] Mod(B ^ rn - 1) is represented by B ^ rn - 1. This should not be a problem if mulmod_bnm1 is
// used to combine results and obtain a natural number when one knows in advance that the final
// value is less than B ^ rn - 1. Moreover it should not be a problem if mulmod_bnm1 is used to
// compute the full product with an + bn <= rn, because this condition implies
// (B ^ an - 1)(B ^ bn - 1) < (B ^ rn - 1) .
//
// Requires 0 < bn <= an <= rn and an + bn > rn / 2
// Scratch need: rn + (need for recursive call OR rn + 4). This gives
// S(n) <= rn + MAX (rn + 4, S(n / 2)) <= 2 * rn + 4
//
// This is mpn_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c.
pub fn mpn_mulmod_bnm1(rp: &mut [Limb], rn: usize, ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let an = ap.len();
    let bn = bp.len();
    assert_ne!(0, bn);
    assert!(bn <= an);
    assert!(an <= rn);

    if (rn & 1) != 0 || rn < MULMOD_BNM1_THRESHOLD {
        if bn < rn {
            if an + bn <= rn {
                limbs_mul_greater_to_out(rp, ap, bp);
            } else {
                limbs_mul_greater_to_out(tp, ap, bp);
                let cy = if limbs_add_to_out(rp, &tp[..rn], &tp[rn..an + bn]) {
                    1
                } else {
                    0
                };
                assert!(!limbs_slice_add_limb_in_place(&mut rp[..rn], cy));
            }
        } else {
            mpn_bc_mulmod_bnm1(rp, &ap[..rn], bp, tp);
        }
    } else {
        let n = rn >> 1;

        // We need at least an + bn >= n, to be able to fit one of the
        // recursive products at rp. Requiring strict inequality makes
        // the code slightly simpler. If desired, we could avoid this
        // restriction by initially halving rn as long as rn is even and
        // an + bn <= rn/2.

        assert!(an + bn > n);

        // Compute xm = a*b mod (B^n - 1), xp = a*b mod (B^n + 1)
        // and crt together as
        //
        // x = -xp * B^n + (B^n + 1) * [ (xp + xm)/2 mod (B^n-1)]
        if an > n {
            let (a0, a1) = ap.split_at(n);
            let cy = if limbs_add_to_out(tp, &a0[..n], &a1[..an - n]) {
                1
            } else {
                0
            };
            assert!(!limbs_slice_add_limb_in_place(&mut tp[..n], cy));
            if bn > n {
                let (b0, b1) = bp.split_at(n);
                let cy = if limbs_add_to_out(&mut tp[n..], &b0[..n], &b1[..bn - n]) {
                    1
                } else {
                    0
                };
                assert!(!limbs_slice_add_limb_in_place(&mut tp[n..2 * n], cy));
                split_into_chunks_mut!(tp, n, [tp_0, tp_1], tp_2);
                mpn_mulmod_bnm1(rp, n, tp_0, tp_1, tp_2);
            } else {
                let (tp_lo, tp_hi) = tp.split_at_mut(n);
                mpn_mulmod_bnm1(rp, n, tp_lo, &bp[..bn], tp_hi);
            }
        } else {
            mpn_mulmod_bnm1(rp, n, &ap[..an], &bp[..bn], tp);
        }
        {
            let mut bp1_is_b0 = true;
            let mut bnp = bn;
            let mut ap1_is_a0 = true;
            let mut anp = an;
            if an > n {
                let (a0, a1) = ap.split_at(n);
                ap1_is_a0 = false;
                let cy = if limbs_sub_to_out(&mut tp[2 * n + 2..], &a0[..n], &a1[..an - n]) {
                    1
                } else {
                    0
                };
                tp[3 * n + 2] = 0;
                assert!(!limbs_slice_add_limb_in_place(
                    &mut tp[2 * n + 2..3 * n + 3],
                    cy
                ));
                anp = n + tp[3 * n + 2] as usize;

                if bn > n {
                    let (b0, b1) = bp.split_at(n);
                    bp1_is_b0 = false;
                    let cy = if limbs_sub_to_out(&mut tp[3 * n + 3..], &b0[..n], &b1[..bn - n]) {
                        1
                    } else {
                        0
                    };
                    tp[4 * n + 3] = 0;
                    assert!(!limbs_slice_add_limb_in_place(
                        &mut tp[3 * n + 3..4 * n + 4],
                        cy
                    ));
                    bnp = n + tp[4 * n + 3] as usize;
                }
            }

            let mut k;
            if n < MUL_FFT_MODF_THRESHOLD {
                k = 0;
            } else {
                k = mpn_fft_best_k(n, false);
                let mut mask = (1 << k) - 1;
                while (n & mask) != 0 {
                    k -= 1;
                    mask >>= 1;
                }
            }
            if k >= FFT_FIRST_K {
                if bp1_is_b0 {
                    if ap1_is_a0 {
                        tp[n] = mpn_mul_fft(tp, n, &ap[..anp], &bp[..bnp], k);
                    } else {
                        let (tp_lo, tp_hi) = tp.split_at_mut(2 * n + 2);
                        tp_lo[n] = mpn_mul_fft(tp_lo, n, &tp_hi[..anp], &bp[..bnp], k);
                    }
                } else {
                    if ap1_is_a0 {
                        let (tp_lo, tp_hi) = tp.split_at_mut(3 * n + 3);
                        tp_lo[n] = mpn_mul_fft(tp_lo, n, &ap[..anp], &tp_hi[..bnp], k);
                    } else {
                        let (tp_lo, tp_hi) = tp.split_at_mut(2 * n + 2);
                        tp_lo[n] =
                            mpn_mul_fft(tp_lo, n, &tp_hi[..anp], &tp_hi[n + 1..bnp + n + 1], k);
                    }
                }
            } else if bp1_is_b0 {
                assert!(anp + bnp <= 2 * n + 1);
                assert!(anp + bnp > n);
                assert!(anp >= bnp);
                if ap1_is_a0 {
                    limbs_mul_greater_to_out(tp, &ap[..anp], &bp[..bnp]);
                } else {
                    let (tp_lo, tp_hi) = tp.split_at_mut(2 * n + 2);
                    limbs_mul_greater_to_out(tp_lo, &tp_hi[..anp], &bp[..bnp]);
                }
                anp = anp + bnp - n;
                assert!(anp <= n || tp[2 * n] == 0);
                anp -= if anp > n { 1 } else { 0 };
                let cy;
                {
                    let (tp_lo, tp_hi) = tp.split_at_mut(n);
                    cy = if limbs_sub_in_place_left(tp_lo, &tp_hi[..anp]) {
                        1
                    } else {
                        0
                    };
                }
                tp[n] = 0;
                assert!(!limbs_slice_add_limb_in_place(&mut tp[..n + 1], cy));
            } else {
                assert!(!ap1_is_a0);
                let (tp_lo, tp_hi) = tp.split_at_mut(2 * n + 2);
                mpn_bc_mulmod_bnp1_tp_is_rp(tp_lo, tp_hi, &tp_hi[n + 1..], n);
            }
        }
        // Here the CRT recomposition begins.
        //
        // xm <- (tp + xm)/2 = (tp + xm)B^n/2 mod (B^n-1)
        // Division by 2 is a bitwise rotation.
        //
        // Assumes tp normalised mod (B^n+1).
        //
        // The residue class [0] is represented by [B^n-1]; except when
        // both input are ZERO.
        // tp[n] == 1 implies {tp,n} == ZERO
        let mut cy = tp[n]
            + if limbs_slice_add_same_length_in_place_left(&mut rp[..n], &tp[..n]) {
                1
            } else {
                0
            };
        cy += rp[0] & 1;
        limbs_slice_shr_in_place(&mut rp[..n], 1);
        assert!(cy <= 2);
        let hi = cy << (Limb::WIDTH - 1); // (cy&1) << ...
        cy >>= 1;
        // We can have cy != 0 only if hi = 0...
        assert!(!rp[n - 1].get_highest_bit());
        rp[n - 1] |= hi;
        // ... rp[n-1] + cy can not overflow, the following INCR is correct.
        assert!(cy <= 1);
        // Next increment can not overflow, read the previous comments about cy.
        assert!(cy == 0 || !rp[n - 1].get_highest_bit());
        assert!(!limbs_slice_add_limb_in_place(&mut rp[..n], cy));

        // Compute the highest half:
        // ([(tp + xm)/2 mod (B^n-1)] - tp ) * B^n
        if an + bn < rn {
            // Note that in this case, the only way the result can equal
            // zero mod B^{rn} - 1 is if one of the inputs is zero, and
            // then the output of both the recursive calls and this CRT
            // reconstruction is zero, not B^{rn} - 1. Which is good,
            // since the latter representation doesn't fit in the output
            // area.
            {
                let (rp_lo, rp_hi) = rp.split_at_mut(n);
                cy = if limbs_sub_same_length_to_out(
                    rp_hi,
                    &rp_lo[..an + bn - n],
                    &tp[..an + bn - n],
                ) {
                    1
                } else {
                    0
                };
            }
            cy = tp[n]
                + if _limbs_sub_same_length_with_borrow_in_place_right(
                    &rp[an + bn - n..rn - n],
                    &mut tp[an + bn - n..rn - n],
                    cy != 0,
                ) {
                    1
                } else {
                    0
                };
            assert!(an + bn == rn - 1 || limbs_test_zero(&tp[an + bn - n + 1..rn - n]));
            cy = if limbs_sub_limb_in_place(&mut rp[..an + bn], cy) {
                1
            } else {
                0
            };
            assert_eq!(cy, tp[an + bn - n]);
        } else {
            {
                let (rp_lo, rp_hi) = rp.split_at_mut(n);
                cy = tp[n]
                    + if limbs_sub_same_length_to_out(rp_hi, rp_lo, &tp[..n]) {
                        1
                    } else {
                        0
                    };
            }
            // cy = 1 only if {tp,n+1} is not ZERO, i.e. {rp,n} is not ZERO.
            // DECR will affect _at most_ the lowest n limbs.
            assert!(!limbs_sub_limb_in_place(&mut rp[..2 * n], cy));
        }
    }
}

// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, computation is mod
// B ^ rn - 1, and values are semi-normalised; zero is represented as either 0 or B ^ n - 1. Needs a
// scratch of 2rn limbs at tp.
// This is mpn_bc_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c.
fn mpn_bc_mulmod_bnm1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let rn = ap.len();
    assert_ne!(rn, 0);
    limbs_mul_same_length_to_out(tp, ap, bp);
    let cy = if limbs_add_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
        1
    } else {
        0
    };
    // If cy == 1, then the value of rp is at most B ^ rn - 2, so there can be no overflow when
    // adding in the carry.
    limbs_slice_add_limb_in_place(&mut rp[..rn], cy);
}

// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, in semi-normalised
// representation, computation is mod B ^ rn + 1. Needs a scratch area of 2rn + 2 limbs at tp.
// Output is normalised.
// This is mpn_bc_mulmod_bnp1 from mpn/generic/mulmod_bnm1.c, where rp != tp.
pub fn mpn_bc_mulmod_bnp1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], rn: usize, tp: &mut [Limb]) {
    assert_ne!(0, rn);
    limbs_mul_same_length_to_out(tp, &ap[..rn + 1], &bp[..rn + 1]);
    assert_eq!(tp[2 * rn + 1], 0);
    assert!(tp[2 * rn] < Limb::MAX);
    let cy = tp[2 * rn]
        + if limbs_sub_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
            1
        } else {
            0
        };
    rp[rn] = 0;
    assert!(!limbs_slice_add_limb_in_place(&mut rp[..rn + 1], cy));
}

// This is mpn_bc_mulmod_bnp1 from mpn/generic/mulmod_bnm1.c, where rp == tp.
fn mpn_bc_mulmod_bnp1_tp_is_rp(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], rn: usize) {
    assert_ne!(0, rn);
    limbs_mul_same_length_to_out(rp, &ap[..rn + 1], &bp[..rn + 1]);
    assert_eq!(rp[2 * rn + 1], 0);
    assert!(rp[2 * rn] < Limb::MAX);
    let cy;
    {
        let (rp_lo, rp_hi) = rp.split_at_mut(rn);
        cy = rp_hi[rn]
            + if limbs_sub_same_length_in_place_left(rp_lo, &rp_hi[..rn]) {
                1
            } else {
                0
            };
    }
    rp[rn] = 0;
    assert!(!limbs_slice_add_limb_in_place(&mut rp[..rn + 1], cy));
}
