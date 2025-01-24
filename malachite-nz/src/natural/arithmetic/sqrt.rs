// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Contributed to the GNU project by Paul Zimmermann (most code), Torbjörn Granlund
//      (`mpn_sqrtrem1`) and Marco Bodrato (`mpn_dc_sqrt`).
//
//      Copyright © 1999-2002, 2004, 2005, 2008, 2010, 2012, 2015, 2017 Free Software Foundation,
//      Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
    limbs_vec_add_limb_in_place,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::div::{
    limbs_div_barrett_approx, limbs_div_barrett_approx_scratch_len,
    limbs_div_divide_and_conquer_approx, limbs_div_schoolbook_approx,
};
use crate::natural::arithmetic::div_mod::{
    limbs_div_limb_to_out_mod, limbs_div_mod_qs_to_out_rs_to_ns, limbs_two_limb_inverse_helper,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::limbs_shl_to_out;
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
};
use crate::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{Limb, SignedLimb, DC_DIVAPPR_Q_THRESHOLD, MU_DIVAPPR_Q_THRESHOLD};
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::sqrt::sqrt_rem_newton;
use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, ModPowerOf2, Parity,
    ShrRound, SqrtAssignRem, SqrtRem, Square, WrappingAddAssign, WrappingSquare, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LeadingZeros, LowMask};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_test_zero;

// Returns (sqrt, r_hi, r_lo) such that [n_lo, n_hi] = sqrt ^ 2 + [r_lo, r_hi].
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_sqrtrem2` from `mpn/generic/sqrtrem.c`, GMP 6.2.1.
pub_test! {sqrt_rem_2_newton(n_hi: Limb, n_lo: Limb) -> (Limb, bool, Limb) {
    assert!(n_hi.leading_zeros() < 2);
    let (mut sqrt, mut r_lo) = sqrt_rem_newton::<Limb, SignedLimb>(n_hi);
    const PREC: u64 = Limb::WIDTH >> 1;
    const PREC_P_1: u64 = PREC + 1;
    const PREC_M_1: u64 = PREC - 1;
    // r_lo <= 2 * sqrt < 2 ^ (prec + 1)
    r_lo = (r_lo << PREC_M_1) | (n_lo >> PREC_P_1);
    let mut q = r_lo / sqrt;
    // q <= 2 ^ prec, if q = 2 ^ prec, reduce the overestimate.
    q -= q >> PREC;
    // now we have q < 2 ^ prec
    let u = r_lo - q * sqrt;
    // now we have (rp_lo << prec + n_lo >> prec) / 2 = q * sqrt + u
    sqrt = (sqrt << PREC) | q;
    let mut r_hi = (u >> PREC_M_1) + 1;
    r_lo = (u << PREC_P_1) | (n_lo.mod_power_of_2(PREC_P_1));
    let q_squared = q.square();
    if r_lo < q_squared {
        assert_ne!(r_hi, 0);
        r_hi -= 1;
    }
    r_lo.wrapping_sub_assign(q_squared);
    if r_hi == 0 {
        r_lo.wrapping_add_assign(sqrt);
        if r_lo < sqrt {
            r_hi += 1;
        }
        sqrt -= 1;
        r_lo.wrapping_add_assign(sqrt);
        if r_lo < sqrt {
            r_hi += 1;
        }
    }
    r_hi -= 1;
    assert!(r_hi < 2);
    (sqrt, r_hi == 1, r_lo)
}}

pub_const_test! {limbs_sqrt_rem_helper_scratch_len(n: usize) -> usize {
    (n >> 1) + 1
}}

// - Let n be out.len().
// - Let x be xs[..2 * n] before execution.
// - Let s be out after execution.
// - Let r be xs[..n] after execution.
//
// xs[2 * n - 1].leading_zeros() must be less than 2.
//
// If approx = 0, then s = floor(sqrt(x)) and r = x - s ^ 2.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_dc_sqrtrem` from `mpn/generic/sqrtrem.c`, GMP 6.2.1.
pub_test! {limbs_sqrt_rem_helper(
    out: &mut [Limb],
    xs: &mut [Limb],
    approx: Limb,
    scratch: &mut [Limb],
) -> bool {
    let n = out.len();
    assert!(n > 1);
    let xs = &mut xs[..n << 1];
    assert!(xs.last().unwrap().leading_zeros() < 2);
    let h1 = n >> 1;
    let h2 = n - h1;
    let two_h1 = h1 << 1;
    let xs_hi = &mut xs[two_h1..];
    let out_hi = &mut out[h1..];
    let q = if h2 == 1 {
        let r_hi;
        (out_hi[0], r_hi, xs_hi[0]) = sqrt_rem_2_newton(xs_hi[1], xs_hi[0]);
        r_hi
    } else {
        limbs_sqrt_rem_helper(out_hi, xs_hi, 0, scratch)
    };
    if q {
        assert!(limbs_sub_same_length_in_place_left(
            &mut xs_hi[..h2],
            out_hi
        ));
    }
    let xs_hi = &mut xs[h1..];
    if h2 == 1 {
        xs_hi[0] = limbs_div_limb_to_out_mod(scratch, &xs_hi[..n], out_hi[0]);
    } else {
        limbs_div_mod_qs_to_out_rs_to_ns(scratch, &mut xs_hi[..n], out_hi);
    }
    let mut q = Limb::from(q);
    q += scratch[h1];
    let mut r_hi = scratch[0].odd();
    limbs_shr_to_out(out, &scratch[..h1], 1);
    out[h1 - 1] |= q << (Limb::WIDTH - 1);
    if (out[0] & approx) != 0 {
        return true;
    }
    q >>= 1;
    let (out_lo, out_hi) = out.split_at_mut(h1);
    if r_hi {
        r_hi = limbs_slice_add_same_length_in_place_left(&mut xs_hi[..h2], out_hi);
    }
    let (xs, xs_hi_hi) = xs.split_at_mut(n);
    let (xs_lo, xs_hi) = xs.split_at_mut(two_h1);
    let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(h1)];
    limbs_square_to_out(xs_hi_hi, out_lo, &mut square_scratch);
    let mut b = q;
    if limbs_sub_same_length_in_place_left(xs_lo, &xs_hi_hi[..two_h1]) {
        b += 1;
    }
    let mut r_hi = SignedLimb::from(r_hi);
    r_hi -= if h1 == h2 {
        SignedLimb::exact_from(b)
    } else {
        SignedLimb::from(limbs_sub_limb_in_place(xs_hi, b))
    };
    if r_hi < 0 {
        q = Limb::from(limbs_slice_add_limb_in_place(out_hi, q));
        r_hi += SignedLimb::exact_from(
            limbs_slice_add_mul_limb_same_length_in_place_left(xs, out, 2) + (q << 1),
        );
        if limbs_sub_limb_in_place(xs, 1) {
            r_hi -= 1;
        }
        limbs_sub_limb_in_place(out, 1);
    }
    assert!(r_hi >= 0);
    assert!(r_hi < 2);
    r_hi == 1
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_divappr_q` from `mpn/generic/sqrtrem.c`, GMP 6.2.1.
fn limbs_sqrt_div_approx_helper(qs: &mut [Limb], ns: &[Limb], ds: &[Limb], scratch: &mut [Limb]) {
    let n_len = ns.len();
    let d_len = ds.len();
    assert!(d_len > 2);
    assert!(n_len >= d_len);
    assert!(ds.last().unwrap().get_highest_bit());
    let scratch = &mut scratch[..n_len];
    scratch.copy_from_slice(ns);
    let inv = limbs_two_limb_inverse_helper(ds[d_len - 1], ds[d_len - 2]);
    qs[n_len - d_len] = Limb::from(if d_len < DC_DIVAPPR_Q_THRESHOLD {
        limbs_div_schoolbook_approx(qs, scratch, ds, inv)
    } else if d_len < MU_DIVAPPR_Q_THRESHOLD {
        limbs_div_divide_and_conquer_approx(qs, scratch, ds, inv)
    } else {
        let mut new_scratch = vec![0; limbs_div_barrett_approx_scratch_len(n_len, d_len)];
        limbs_div_barrett_approx(qs, ns, ds, &mut new_scratch)
    });
}

// - Let n be out.len().
// - Let m be xs.len().
// - n must be ceiling(m / 2).
// - odd must be m.odd().
// - shift must be floor(xs[m - 1].leading_zeros() / 2).
// - Let x be xs before execution.
// - Let s be out after execution.
// - Then s = floor(sqrt(x)).
// - The return value is true iff there is a remainder (that is, x is not a perfect square).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_dc_sqrt` from `mpn/generic/sqrtrem.c`, GMP 6.2.1.
pub_test! { limbs_sqrt_helper(out: &mut [Limb], xs: &[Limb], shift: u64, odd: bool) -> bool {
    let n = out.len();
    let odd = usize::from(odd);
    assert_eq!(xs.len(), (n << 1) - odd);
    assert_ne!(*xs.last().unwrap(), 0);
    assert!(n > 4);
    assert!(shift < Limb::WIDTH >> 1);
    let h1 = (n - 1) >> 1;
    let h2 = n - h1;
    let (out_lo, out_hi) = out.split_at_mut(h1);
    let mut scratch = vec![0; (n << 1) + h1 + 4];
    let scratch_hi = &mut scratch[n..]; // length is n + h1 + 4
    if shift != 0 {
        // o is used to exactly set the lowest bits of the dividend.
        let o = usize::from(h1 > (1 + odd));
        assert_eq!(
            limbs_shl_to_out(
                &mut scratch_hi[1 - o..],
                &xs[h1 - 1 - o - odd..(n << 1) - odd],
                shift << 1
            ),
            0
        );
    } else {
        scratch_hi[1..n + h2 + 2].copy_from_slice(&xs[h1 - 1 - odd..(n << 1) - odd]);
    }
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(n + 1); // scratch_hi len is n + h1 + 3
    let r_hi = limbs_sqrt_rem_helper(out_hi, &mut scratch_hi[h1 + 1..=n + h2], 0, scratch_lo);
    if r_hi {
        assert!(limbs_sub_same_length_in_place_left(
            &mut scratch_hi[h1 + 1..=n],
            out_hi
        ));
    }
    // qs len is h1 + 2
    let (scratch_hi_lo, qs) = scratch_hi.split_at_mut(n + 1);
    limbs_sqrt_div_approx_helper(qs, scratch_hi_lo, out_hi, scratch_lo);
    // qs_tail len is h1 + 1
    let (qs_head, qs_tail) = qs.split_first_mut().unwrap();
    let mut qs_last = Limb::from(r_hi);
    qs_last += qs_tail[h1];
    let mut nonzero_remainder = true;
    if qs_last > 1 {
        for x in out_lo {
            *x = Limb::MAX;
        }
    } else {
        limbs_shr_to_out(out_lo, &qs_tail[..h1], 1);
        if qs_last != 0 {
            out_lo.last_mut().unwrap().set_bit(Limb::WIDTH - 1);
        }
        let s = (Limb::WIDTH >> odd) - shift - 1;
        if (*qs_head >> 3) | qs_tail[0].mod_power_of_2(Limb::WIDTH - s) == 0 {
            // Approximation is not good enough, the extra limb(+ shift bits) is smaller than needed
            // to absorb the possible error. {qs + 1, h1 + 1} equals 2*{out, h1}
            let mut mul_scratch =
                vec![0; limbs_mul_greater_to_out_scratch_len(out_hi.len(), qs_tail.len())];
            assert_eq!(
                limbs_mul_greater_to_out(scratch_lo, out_hi, qs_tail, &mut mul_scratch),
                0
            );
            // scratch_hi_1 len is n + h1 + 2
            let scratch_hi_1 = &mut scratch_hi[1..];
            // scratch_lo_hi len is h1 + 1
            let (scratch_lo_lo, scratch_lo_hi) = scratch_lo.split_at_mut(h2);
            // scratch_hi_1_hi len is 2 * h1 + 2
            let (scratch_hi_1_lo, scratch_hi_1_hi) = scratch_hi_1.split_at_mut(h2);
            // Compute the remainder of the previous mpn_div(appr)_q.
            if limbs_sub_same_length_in_place_left(scratch_hi_1_lo, scratch_lo_lo) {
                assert!(!limbs_sub_limb_in_place(&mut scratch_hi_1_hi[..h1], 1));
            }
            let cmp = limbs_cmp_same_length(&scratch_hi_1_hi[..h1], &scratch_lo_hi[..h1]);
            assert_ne!(cmp, Greater);
            if cmp == Less {
                // May happen only if div result was not exact.
                let carry =
                    limbs_slice_add_mul_limb_same_length_in_place_left(scratch_hi_1_lo, out_hi, 2);
                assert!(!limbs_slice_add_limb_in_place(
                    &mut scratch_hi_1_hi[..h1],
                    carry
                ));
                assert!(!limbs_sub_limb_in_place(out_lo, 1));
            }
            // scratch_hi_1_hi len is 2 * h1 + 2
            let (scratch_hi_1_lo, scratch_hi_1_hi) = scratch_hi_1.split_at_mut(h1);
            if slice_test_zero(&scratch_hi_1_hi[..h2 - h1]) {
                let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(out_lo.len())];
                limbs_square_to_out(scratch_lo, out_lo, &mut square_scratch);
                // scratch_lo_hi len is h2 + 1
                let (scratch_lo_lo, scratch_lo_hi) = scratch_lo.split_at(h1);
                let mut cmp = limbs_cmp_same_length(scratch_hi_1_lo, &scratch_lo_hi[..h1]);
                if cmp == Equal {
                    let scratch = &scratch_lo_lo[odd..];
                    cmp = if shift != 0 {
                        limbs_shl_to_out(scratch_hi, &xs[..h1], shift << 1);
                        limbs_cmp_same_length(&scratch_hi[..h1 - odd], scratch)
                    } else {
                        limbs_cmp_same_length(&xs[..h1 - odd], scratch)
                    };
                }
                if cmp == Less {
                    assert!(!limbs_sub_limb_in_place(out_lo, 1));
                }
                nonzero_remainder = cmp != Equal;
            }
        }
    }
    if odd == 1 || shift != 0 {
        let mut shift = shift;
        if odd == 1 {
            shift.set_bit(Limb::LOG_WIDTH - 1);
        }
        limbs_slice_shr_in_place(out, shift);
    }
    nonzero_remainder
}}

// Computes the floor of the square root of a `Natural`.
//
// - Let $n$ be `xs.len()` and $x$ be the `Natural` whose limbs are `xs`.
// - Let $s$ be the `Natural` whose limbs are the first $\lceil n/2 \rceil$ limbs of `out`.
// - Then $s = \lfloor \sqrt x \rfloor$.
//
// All limbs are in ascending order (least-significant first).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_sqrtrem` from `mpn/generic/sqrtrem.c`, GMP 6.2.1, where `rp` is
// `NULL`.
pub_test! {limbs_sqrt_to_out(out: &mut [Limb], xs: &[Limb]) {
    let xs_len = xs.len();
    let high = xs[xs_len - 1];
    assert_ne!(high, 0);
    let mut shift = LeadingZeros::leading_zeros(high) >> 1;
    let two_shift = shift << 1;
    match xs_len {
        1 => {
            out[0] = sqrt_rem_newton::<Limb, SignedLimb>(high << two_shift).0 >> shift;
        }
        2 => {
            out[0] = if shift == 0 {
                sqrt_rem_2_newton(xs[1], xs[0]).0
            } else {
                let lo = xs[0];
                sqrt_rem_2_newton(
                    (high << two_shift) | (lo >> (Limb::WIDTH - two_shift)),
                    lo << two_shift,
                )
                .0 >> shift
            };
        }
        _ if xs_len > 8 => {
            let out_len = xs_len.shr_round(1, Ceiling).0;
            limbs_sqrt_helper(&mut out[..out_len], xs, shift, xs_len.odd());
        }
        _ => {
            let out_len = xs_len.shr_round(1, Ceiling).0;
            let out = &mut out[..out_len];
            if xs_len.odd() || shift != 0 {
                let scratch_1_len = out_len << 1;
                let mut scratch = vec![0; scratch_1_len + (out_len >> 1) + 1];
                let (scratch_1, scratch_2) = scratch.split_at_mut(scratch_1_len);
                // needed only when 2 * out_len > xs_len, but saves a test
                let shifted_scratch_1 = if xs_len.odd() {
                    &mut scratch_1[1..]
                } else {
                    scratch_1[0] = 0;
                    &mut *scratch_1
                };
                if shift == 0 {
                    shifted_scratch_1.copy_from_slice(xs);
                } else {
                    limbs_shl_to_out(shifted_scratch_1, xs, two_shift);
                }
                if xs_len.odd() {
                    shift += Limb::WIDTH >> 1;
                }
                limbs_sqrt_rem_helper(out, scratch_1, Limb::low_mask(shift) - 1, scratch_2);
                limbs_slice_shr_in_place(out, shift);
            } else {
                let mut rem = xs.to_vec();
                let mut scratch = vec![0; (out_len >> 1) + 1];
                limbs_sqrt_rem_helper(out, &mut rem, 0, &mut scratch);
            }
        }
    }
}}

// Computes the square root and remainder of a `Natural`.
//
// Let $n$ be `xs.len()` and $x$ be the `Natural` whose limbs are `xs`. Let $s$ be the `Natural`
// whose limbs are the first $\lceil n/2 \rceil$ limbs of `out_sqrt`, $m$ the return value, and $r$
// be the `Natural` whose limbs are the first $m$ limbs of `out_rem`. Then $s = \lfloor \sqrt x
// \rfloor$ and $s^2 + r = x$. This implies that $r \leq 2x$.
//
// All limbs are in ascending order (least-significant first).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_sqrtrem` from `mpn/generic/sqrtrem.c`, GMP 6.2.1, where `rp` is not
// `NULL`.
pub_test! {limbs_sqrt_rem_to_out(
    out_sqrt: &mut [Limb],
    out_rem: &mut [Limb],
    xs: &[Limb]
) -> usize {
    let xs_len = xs.len();
    let high = xs[xs_len - 1];
    assert_ne!(high, 0);
    let mut shift = LeadingZeros::leading_zeros(high) >> 1;
    let two_shift = shift << 1;
    match xs_len {
        1 => {
            let r_lo = if shift == 0 {
                let r;
                (out_sqrt[0], r) = sqrt_rem_newton::<Limb, SignedLimb>(high);
                r
            } else {
                let sqrt = sqrt_rem_newton::<Limb, SignedLimb>(high << two_shift).0 >> shift;
                out_sqrt[0] = sqrt;
                high - sqrt.square()
            };
            out_rem[0] = r_lo;
            usize::from(r_lo != 0)
        }
        2 => {
            if shift == 0 {
                let r_hi;
                (out_sqrt[0], r_hi, out_rem[0]) = sqrt_rem_2_newton(xs[1], xs[0]);
                if r_hi {
                    out_rem[1] = 1;
                    2
                } else {
                    usize::from(out_rem[0] != 0)
                }
            } else {
                let mut lo = xs[0];
                let hi = (high << two_shift) | (lo >> (Limb::WIDTH - two_shift));
                out_sqrt[0] = sqrt_rem_2_newton(hi, lo << two_shift).0 >> shift;
                lo.wrapping_sub_assign(out_sqrt[0].wrapping_square());
                out_rem[0] = lo;
                usize::from(lo != 0)
            }
        }
        _ => {
            let mut out_len = xs_len.shr_round(1, Ceiling).0;
            let out_sqrt = &mut out_sqrt[..out_len];
            if xs_len.odd() || shift != 0 {
                let scratch_1_len = out_len << 1;
                let mut scratch = vec![0; scratch_1_len + (out_len >> 1) + 1];
                let (mut scratch_1, scratch_2) = scratch.split_at_mut(scratch_1_len);
                // needed only when 2 * out_len > xs_len, but saves a test
                let shifted_scratch_1 = if xs_len.odd() {
                    &mut scratch_1[1..]
                } else {
                    scratch_1[0] = 0;
                    &mut *scratch_1
                };
                if shift == 0 {
                    shifted_scratch_1.copy_from_slice(xs);
                } else {
                    limbs_shl_to_out(shifted_scratch_1, xs, two_shift);
                }
                if xs_len.odd() {
                    shift += Limb::WIDTH >> 1;
                }
                let r_hi = limbs_sqrt_rem_helper(out_sqrt, scratch_1, 0, scratch_2);
                let s = out_sqrt[0] & Limb::low_mask(shift);
                let scratch_1_lo = &mut scratch_1[..out_len];
                let mut r_lo = limbs_slice_add_mul_limb_same_length_in_place_left(
                    scratch_1_lo,
                    out_sqrt,
                    s << 1,
                );
                if r_hi {
                    r_lo += 1;
                }
                let (scratch_1_lo_lo, scratch_1_lo_hi) = scratch_1_lo.split_at_mut(1);
                let carry = limbs_sub_mul_limb_same_length_in_place_left(scratch_1_lo_lo, &[s], s);
                if limbs_sub_limb_in_place(scratch_1_lo_hi, carry) {
                    r_lo -= 1;
                }
                limbs_slice_shr_in_place(out_sqrt, shift);
                scratch_1[out_len] = r_lo;
                shift <<= 1;
                if shift < Limb::WIDTH {
                    out_len += 1;
                } else {
                    scratch_1 = &mut scratch_1[1..];
                    shift -= Limb::WIDTH;
                }
                let scratch_1 = &mut scratch_1[..out_len];
                if shift == 0 {
                    out_rem[..out_len].copy_from_slice(scratch_1);
                } else {
                    limbs_shr_to_out(out_rem, scratch_1, shift);
                }
            } else {
                out_rem[..xs_len].copy_from_slice(xs);
                let mut scratch = vec![0; (out_len >> 1) + 1];
                if limbs_sqrt_rem_helper(out_sqrt, out_rem, 0, &mut scratch) {
                    out_rem[out_len] = 1;
                    out_len += 1;
                }
            }
            out_len
        }
    }
}}

// Computes the floor of the square root of a `Natural`.
//
// Let $x$ be the `Natural` whose limbs are `xs` and $s$ be the `Natural` whose limbs are returned.
// Then $s = \lfloor \sqrt x \rfloor$.
//
// All limbs are in ascending order (least-significant first).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_floor_sqrt(xs: &[Limb]) -> Vec<Limb> {
    let mut out = vec![0; xs.len().shr_round(1, Ceiling).0];
    limbs_sqrt_to_out(&mut out, xs);
    out
}}

// Computes the ceiling of the square root of a `Natural`.
//
// Let $x$ be the `Natural` whose limbs are `xs` and $s$ be the `Natural` whose limbs are returned.
// Then $s = \lceil \sqrt x \rceil$.
//
// All limbs are in ascending order (least-significant first).
//
// # Worst-case complexity
// TODO
pub_test! {limbs_ceiling_sqrt(xs: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let mut out_sqrt = vec![0; xs_len.shr_round(1, Ceiling).0];
    let mut out_rem = vec![0; xs_len];
    let rem_len = limbs_sqrt_rem_to_out(&mut out_sqrt, &mut out_rem, xs);
    if !slice_test_zero(&out_rem[..rem_len]) {
        limbs_vec_add_limb_in_place(&mut out_sqrt, 1);
    }
    out_sqrt
}}

// Computes the square root of a `Natural`, returning `None` if the `Natural` is not a perfect
// square.
//
// Let $x$ be the `Natural` whose limbs are `xs` and $s$ be the `Natural` whose limbs are returned.
//
// $$
// s = \\begin{cases}
//     \operatorname{Some}(\sqrt{x}) & \sqrt{x} \in \Z \\\\
//     \operatorname{None} & \textrm{otherwise}.
// \\end{cases}
// $$
//
// All limbs are in ascending order (least-significant first).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_checked_sqrt(xs: &[Limb]) -> Option<Vec<Limb>> {
    let xs_len = xs.len();
    let mut out_sqrt = vec![0; xs_len.shr_round(1, Ceiling).0];
    let mut out_rem = vec![0; xs_len];
    let rem_len = limbs_sqrt_rem_to_out(&mut out_sqrt, &mut out_rem, xs);
    if slice_test_zero(&out_rem[..rem_len]) {
        Some(out_sqrt)
    } else {
        None
    }
}}

// Computes the square root and remainder of a `Natural`.
//
// Let `out_sqrt` and `out_rem` be the two returned `Limb` `Vec`s. Let $n$ be `xs.len()` and $x$ be
// the `Natural` whose limbs are `xs`. Let $s$ be the `Natural` whose limbs are the first $\lceil
// n/2 \rceil$ limbs of `out_sqrt` and $r$ be the `Natural` whose limbs are the first $n$ limbs of
// `out_rem`.  Then $s = \lfloor \sqrt x \rfloor$ and $s^2 + r = x$. This implies that $r \leq 2x$.
//
// All limbs are in ascending order (least-significant first).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_sqrt_rem(xs: &[Limb]) -> (Vec<Limb>, Vec<Limb>) {
    let xs_len = xs.len();
    let mut out_sqrt = vec![0; xs_len.shr_round(1, Ceiling).0];
    let mut out_rem = vec![0; xs_len];
    let rem_len = limbs_sqrt_rem_to_out(&mut out_sqrt, &mut out_rem, xs);
    out_rem.truncate(rem_len);
    (out_sqrt, out_rem)
}}

impl FloorSqrt for Natural {
    type Output = Natural;

    /// Returns the floor of the square root of a [`Natural`], taking it by value.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).floor_sqrt(), 9);
    /// assert_eq!(Natural::from(100u8).floor_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).floor_sqrt(), 10);
    /// assert_eq!(Natural::from(1000000000u32).floor_sqrt(), 31622);
    /// assert_eq!(Natural::from(10000000000u64).floor_sqrt(), 100000);
    /// ```
    #[inline]
    fn floor_sqrt(self) -> Natural {
        (&self).floor_sqrt()
    }
}

impl FloorSqrt for &Natural {
    type Output = Natural;

    /// Returns the floor of the square root of a [`Natural`], taking it by value.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(99u8)).floor_sqrt(), 9);
    /// assert_eq!((&Natural::from(100u8)).floor_sqrt(), 10);
    /// assert_eq!((&Natural::from(101u8)).floor_sqrt(), 10);
    /// assert_eq!((&Natural::from(1000000000u32)).floor_sqrt(), 31622);
    /// assert_eq!((&Natural::from(10000000000u64)).floor_sqrt(), 100000);
    /// ```
    fn floor_sqrt(self) -> Natural {
        match self {
            Natural(Small(small)) => Natural::from(small.floor_sqrt()),
            Natural(Large(ref limbs)) => Natural::from_owned_limbs_asc(limbs_floor_sqrt(limbs)),
        }
    }
}

impl FloorSqrtAssign for Natural {
    /// Replaces a [`Natural`] with the floor of its square root.
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorSqrtAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(100u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(101u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1000000000u32);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 31622);
    ///
    /// let mut x = Natural::from(10000000000u64);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn floor_sqrt_assign(&mut self) {
        *self = (&*self).floor_sqrt();
    }
}

impl CeilingSqrt for Natural {
    type Output = Natural;

    /// Returns the ceiling of the square root of a [`Natural`], taking it by value.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(100u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).ceiling_sqrt(), 11);
    /// assert_eq!(Natural::from(1000000000u32).ceiling_sqrt(), 31623);
    /// assert_eq!(Natural::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt(self) -> Natural {
        (&self).ceiling_sqrt()
    }
}

impl CeilingSqrt for &Natural {
    type Output = Natural;

    /// Returns the ceiling of the square root of a [`Natural`], taking it by value.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(100u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).ceiling_sqrt(), 11);
    /// assert_eq!(Natural::from(1000000000u32).ceiling_sqrt(), 31623);
    /// assert_eq!(Natural::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    fn ceiling_sqrt(self) -> Natural {
        match self {
            Natural(Small(small)) => Natural::from(small.ceiling_sqrt()),
            Natural(Large(ref limbs)) => Natural::from_owned_limbs_asc(limbs_ceiling_sqrt(limbs)),
        }
    }
}

impl CeilingSqrtAssign for Natural {
    /// Replaces a [`Natural`] with the ceiling of its square root.
    ///
    /// $x \gets \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingSqrtAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(100u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(101u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 11);
    ///
    /// let mut x = Natural::from(1000000000u32);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 31623);
    ///
    /// let mut x = Natural::from(10000000000u64);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt_assign(&mut self) {
        *self = (&*self).ceiling_sqrt();
    }
}

impl CheckedSqrt for Natural {
    type Output = Natural;

    /// Returns the the square root of a [`Natural`], or `None` if it is not a perfect square. The
    /// [`Natural`] is taken by value.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(\sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(
    ///     Natural::from(100u8).checked_sqrt().to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     Natural::from(101u8).checked_sqrt().to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(1000000000u32)
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(10000000000u64)
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "Some(100000)"
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Natural> {
        (&self).checked_sqrt()
    }
}

impl CheckedSqrt for &Natural {
    type Output = Natural;

    /// Returns the the square root of a [`Natural`], or `None` if it is not a perfect square. The
    /// [`Natural`] is taken by value.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(\sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(99u8)).checked_sqrt().to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(100u8)).checked_sqrt().to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(101u8)).checked_sqrt().to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1000000000u32))
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10000000000u64))
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "Some(100000)"
    /// );
    /// ```
    fn checked_sqrt(self) -> Option<Natural> {
        match self {
            Natural(Small(small)) => small.checked_sqrt().map(Natural::from),
            Natural(Large(ref limbs)) => {
                limbs_checked_sqrt(limbs).map(Natural::from_owned_limbs_asc)
            }
        }
    }
}

impl SqrtRem for Natural {
    type SqrtOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a [`Natural`] and the remainder (the difference
    /// between the [`Natural`] and the square of the floor). The [`Natural`] is taken by value.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).sqrt_rem().to_debug_string(), "(9, 18)");
    /// assert_eq!(Natural::from(100u8).sqrt_rem().to_debug_string(), "(10, 0)");
    /// assert_eq!(Natural::from(101u8).sqrt_rem().to_debug_string(), "(10, 1)");
    /// assert_eq!(
    ///     Natural::from(1000000000u32).sqrt_rem().to_debug_string(),
    ///     "(31622, 49116)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10000000000u64).sqrt_rem().to_debug_string(),
    ///     "(100000, 0)"
    /// );
    /// ```
    #[inline]
    fn sqrt_rem(self) -> (Natural, Natural) {
        (&self).sqrt_rem()
    }
}

impl SqrtRem for &Natural {
    type SqrtOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a [`Natural`] and the remainder (the difference
    /// between the [`Natural`] and the square of the floor). The [`Natural`] is taken by reference.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(99u8)).sqrt_rem().to_debug_string(),
    ///     "(9, 18)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(100u8)).sqrt_rem().to_debug_string(),
    ///     "(10, 0)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(101u8)).sqrt_rem().to_debug_string(),
    ///     "(10, 1)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1000000000u32)).sqrt_rem().to_debug_string(),
    ///     "(31622, 49116)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10000000000u64))
    ///         .sqrt_rem()
    ///         .to_debug_string(),
    ///     "(100000, 0)"
    /// );
    /// ```
    fn sqrt_rem(self) -> (Natural, Natural) {
        match self {
            Natural(Small(small)) => {
                let (sqrt, rem) = small.sqrt_rem();
                (Natural::from(sqrt), Natural::from(rem))
            }
            Natural(Large(ref limbs)) => {
                let (sqrt_limbs, rem_limbs) = limbs_sqrt_rem(limbs);
                (
                    Natural::from_owned_limbs_asc(sqrt_limbs),
                    Natural::from_owned_limbs_asc(rem_limbs),
                )
            }
        }
    }
}

impl SqrtAssignRem for Natural {
    type RemOutput = Natural;

    /// Replaces a [`Natural`] with the floor of its square root and returns the remainder (the
    /// difference between the original [`Natural`] and the square of the floor).
    ///
    /// $f(x) = x - \lfloor\sqrt{x}\rfloor^2$,
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SqrtAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// assert_eq!(x.sqrt_assign_rem(), 18);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(100u8);
    /// assert_eq!(x.sqrt_assign_rem(), 0);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(101u8);
    /// assert_eq!(x.sqrt_assign_rem(), 1);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1000000000u32);
    /// assert_eq!(x.sqrt_assign_rem(), 49116);
    /// assert_eq!(x, 31622);
    ///
    /// let mut x = Natural::from(10000000000u64);
    /// assert_eq!(x.sqrt_assign_rem(), 0);
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn sqrt_assign_rem(&mut self) -> Natural {
        let rem;
        (*self, rem) = (&*self).sqrt_rem();
        rem
    }
}
