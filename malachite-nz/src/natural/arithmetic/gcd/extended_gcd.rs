// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2016 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::add::{
    limbs_add_to_out_aliased, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::div_exact::limbs_div_exact_to_out;
use crate::natural::arithmetic::div_mod::{
    limbs_div_limb_to_out_mod, limbs_div_mod_qs_to_out_rs_to_ns,
};
use crate::natural::arithmetic::gcd::half_gcd::{
    extract_number, limbs_gcd_subdivide_step, limbs_half_gcd, limbs_half_gcd_2,
    limbs_half_gcd_matrix_1_mul_inverse_vector, limbs_half_gcd_matrix_1_mul_vector,
    limbs_half_gcd_matrix_adjust, limbs_half_gcd_matrix_init_scratch_len,
    limbs_half_gcd_scratch_len, GcdSubdivideStepContext, HalfGcdMatrix, HalfGcdMatrix1,
};
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::{limbs_mul_to_out, limbs_mul_to_out_scratch_len};
use crate::natural::arithmetic::sub::limbs_sub_greater_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::{max, Ordering::*};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DivExact, ExtendedGcd, NegAssign, OverflowingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::{slice_set_zero, slice_test_zero, slice_trailing_zeros};

// This is equivalent to `gcdext_ctx` from `gmp-impl.h`, GMP 6.2.1.
struct ExtendedGcdContext<'a> {
    // Result parameters.
    gs: &'a mut [Limb],
    gs_len: usize,
    ss: &'a mut [Limb],
    ss_len: usize,
    ss_sign: bool,

    // Cofactors updated in each step.
    us_len: usize,
    us0: &'a mut [Limb],
    us1: &'a mut [Limb],
    scratch: &'a mut [Limb],
}

impl<'a> ExtendedGcdContext<'a> {
    fn new(
        gs: &'a mut [Limb],
        ss: &'a mut [Limb],
        us_len: usize,
        us0: &'a mut [Limb],
        us1: &'a mut [Limb],
        scratch: &'a mut [Limb],
    ) -> ExtendedGcdContext<'a> {
        let gs_len = gs.len();
        ExtendedGcdContext {
            gs,
            gs_len,
            ss,
            ss_len: 0,
            ss_sign: false,
            us_len,
            us0,
            us1,
            scratch,
        }
    }
}

impl<'a> GcdSubdivideStepContext for ExtendedGcdContext<'a> {
    // This is equivalent to `mpn_gcdext_hook` from `mpn/gcdext_lehmer.c`, GMP 6.2.1.
    fn gcd_subdiv_step_hook(
        &mut self,
        gs: Option<&[Limb]>,
        qs: Option<&mut [Limb]>,
        mut qs_len: usize,
        mut d: i8,
    ) {
        let mut us_len = self.us_len;
        if let Some(gs) = gs {
            let gs_len = gs.len();
            assert_ne!(gs_len, 0);
            assert!(gs[gs_len - 1] > 0);
            self.gs_len = gs_len;
            self.gs[..gs_len].copy_from_slice(gs);
            if d == -1 {
                // Must return the smallest cofactor, +us1 or -us0
                let c = limbs_cmp_same_length(&self.us0[..us_len], &self.us1[..us_len]);
                assert!(c != Equal || us_len == 1 && self.us0[0] == 1 && self.us1[0] == 1);
                d = i8::from(c == Less);
            }
            let ss = if d == 0 {
                &mut *self.us1
            } else {
                &mut *self.us0
            };
            us_len -= slice_trailing_zeros(&ss[..us_len]);
            self.ss[..us_len].copy_from_slice(&ss[..us_len]);
            self.us_len = us_len;
            self.ss_len = us_len;
            self.ss_sign = d == 0;
        } else {
            let mut us0 = &mut *self.us0;
            let mut us1 = &mut *self.us1;
            if d != 0 {
                swap(&mut us0, &mut us1);
            }
            let qs = qs.as_ref().unwrap();
            if qs[qs_len - 1] == 0 {
                qs_len -= 1;
            }
            // Update us0 += q * us1
            let carry = if qs_len == 1 {
                let q = qs[0];
                let us0 = &mut us0[..us_len];
                let us1 = &us1[..us_len];
                if q == 1 {
                    Limb::from(limbs_slice_add_same_length_in_place_left(us0, us1))
                } else {
                    limbs_slice_add_mul_limb_same_length_in_place_left(us0, us1, q)
                }
            } else {
                let mut us1_len = us_len;
                us1_len -= slice_trailing_zeros(&us1[..us1_len]);
                if us1_len == 0 {
                    return;
                }
                // Should always have us1_len == us_len here, and us1 >= us0. The reason is that we
                // alternate adding us0 to us1 and us1 to us0 (corresponding to subtractions a - b
                // and b - a), and we can get a large quotient only just after a swscratch_len,
                // which means that we'll add (a multiple of) the larger u to the smaller.
                let scratch = &mut *self.scratch;
                let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(qs_len, us1_len)];
                limbs_mul_to_out(scratch, &qs[..qs_len], &us1[..us1_len], &mut mul_scratch);
                us1_len += qs_len;
                if scratch[us1_len - 1] == 0 {
                    us1_len -= 1;
                }
                let us0 = &mut us0[..us1_len];
                let scratch = &scratch[..us1_len];
                Limb::from(if us1_len >= us_len {
                    let us_len_old = us_len;
                    us_len = us1_len;
                    limbs_add_to_out_aliased(us0, us_len_old, scratch)
                } else {
                    fail_on_untested_path("gcd_subdiv_step_hook, us1_len < us_len");
                    // Note: Unlikely case, maybe never happens?
                    limbs_slice_add_greater_in_place_left(us0, scratch)
                })
            };
            us0[us_len] = carry;
            self.us_len = us_len;
            if carry > 0 {
                self.us_len += 1;
            }
        }
    }
}

// This is equivalent to `mpn_gcdext_lehmer_n` from `mpn/generic/gcdext_lehmer.c`, GMP 6.2.1.
fn limbs_extended_gcd_same_length_lehmer<'a>(
    gs: &mut [Limb],
    ss: &mut [Limb],
    mut xs: &'a mut [Limb],
    ys: &mut [Limb],
    scratch: &'a mut [Limb],
) -> (usize, usize, bool) {
    // Keeps track of the second row of the reduction matrix
    //
    // M = (v0, v1 ; us0, us1)
    //
    // which correspond to the first column of the inverse
    //
    // M^{-1} = (us1, -v1; -us0, v0)
    //
    // This implies that
    //
    // ```
    //   a =  us1 A (mod B)
    //   b = -us0 A (mod B)
    // ```
    //
    // where A, B denotes the input values.
    let mut n = xs.len();
    assert_eq!(ys.len(), n);
    let scratch_len = n + 1;
    let (us, mut scratch) = scratch.split_at_mut(3 * scratch_len);
    slice_set_zero(us);
    let (mut us0, remainder) = us.split_at_mut(scratch_len);
    let (us1, mut u2) = remainder.split_at_mut(scratch_len);
    us1[0] = 1;
    let mut us_len = 1;
    while n >= 2 {
        let mut m = HalfGcdMatrix1::default();
        let mask = xs[n - 1] | ys[n - 1];
        assert_ne!(mask, 0);
        let (ah, al, bh, bl) = if mask.get_highest_bit() {
            (xs[n - 1], xs[n - 2], ys[n - 1], ys[n - 2])
        } else if n == 2 {
            // We use the full inputs without truncation, so we can safely shift left.
            let shift = u64::from(mask.leading_zeros());
            (
                extract_number(shift, xs[1], xs[0]),
                xs[0] << shift,
                extract_number(shift, ys[1], ys[0]),
                ys[0] << shift,
            )
        } else {
            let shift = u64::from(mask.leading_zeros());
            (
                extract_number(shift, xs[n - 1], xs[n - 2]),
                extract_number(shift, xs[n - 2], xs[n - 3]),
                extract_number(shift, ys[n - 1], ys[n - 2]),
                extract_number(shift, ys[n - 2], ys[n - 3]),
            )
        };
        // Try a `limbs_half_gcd_2` step
        if limbs_half_gcd_2(ah, al, bh, bl, &mut m) {
            n = limbs_half_gcd_matrix_1_mul_inverse_vector(
                &m,
                &mut scratch[..n],
                &xs[..n],
                &mut ys[..n],
            );
            swap(&mut xs, &mut scratch);
            us_len = limbs_half_gcd_matrix_1_mul_vector(&m, u2, &us0[..us_len], us1);
            swap(&mut us0, &mut u2);
        } else {
            // `limbs_half_gcd_2` has failed. Then either one of a or b is very small, or the
            // difference is very small. Perform one subtraction followed by one division.
            let mut context = ExtendedGcdContext::new(gs, ss, us_len, us0, us1, u2);
            // Temporary storage `n` for the quotient and `scratch_len` for the new cofactor.
            n = limbs_gcd_subdivide_step(&mut xs[..n], &mut ys[..n], 0, &mut context, scratch);
            if n == 0 {
                return (context.gs_len, context.ss_len, context.ss_sign);
            }
            us_len = context.us_len;
        }
    }
    assert_ne!(xs[0], 0);
    assert_ne!(ys[0], 0);
    let negate;
    if xs[0] == ys[0] {
        // Which cofactor to return now? Candidates are +us1 and -us0, depending on which of a and b
        // was most recently reduced, which we don't keep track of. So compare and get the smallest
        // one.
        gs[0] = xs[0];
        let c = limbs_cmp_same_length(&us0[..us_len], &us1[..us_len]);
        assert!(c != Equal || us_len == 1 && us0[0] == 1 && us1[0] == 1);
        let ss_sign = c != Less;
        let u = if ss_sign { us1 } else { us0 };
        us_len -= slice_trailing_zeros(&u[..us_len]);
        ss[..us_len].copy_from_slice(&u[..us_len]);
        (1, us_len, ss_sign)
    } else {
        let (g, mut u, mut v) = xs[0].extended_gcd(ys[0]);
        gs[0] = g;
        // Set ss = u us1 - v us0. Keep track of size, us_len grows by one or two limbs.
        if u == 0 {
            assert_eq!(v, 1);
            us_len -= slice_trailing_zeros(&us0[..us_len]);
            ss[..us_len].copy_from_slice(&us0[..us_len]);
            return (1, us_len, false);
        } else if v == 0 {
            assert_eq!(u, 1);
            us_len -= slice_trailing_zeros(&us1[..us_len]);
            ss[..us_len].copy_from_slice(&us1[..us_len]);
            return (1, us_len, true);
        } else if u > 0 {
            negate = false;
            assert!(v < 0);
            v.neg_assign();
        } else {
            negate = true;
            assert!(v > 0);
            u.neg_assign();
        }
        let mut u_high = limbs_mul_limb_to_out(ss, &us1[..us_len], Limb::exact_from(u));
        let v_high = limbs_slice_add_mul_limb_same_length_in_place_left(
            &mut ss[..us_len],
            &us0[..us_len],
            Limb::exact_from(v),
        );
        if u_high != 0 || v_high != 0 {
            let overflow = u_high.overflowing_add_assign(v_high);
            ss[us_len] = u_high;
            us_len += 1;
            if overflow {
                fail_on_untested_path("limbs_extended_gcd_same_length_lehmer, overflow");
                ss[us_len] = 1;
                us_len += 1;
            }
        }
        us_len -= slice_trailing_zeros(&ss[..us_len]);
        assert_ne!(us_len, 0);
        (1, us_len, !negate)
    }
}

// Computes (r;b) = (a; b) M. Result is of size n + M->n +/- 1, and the size is returned (if inputs
// are non-normalized, result may be non-normalized too). Temporary space needed is M->n + n.
//
// This is equivalent to `hgcd_mul_matrix_vector` from `mpn/generic/gcdext.c`, GMP 6.2.1.
fn limbs_half_gcd_matrix_mul_vector(
    m: &mut HalfGcdMatrix<'_>,
    rp: &mut [Limb],
    xs: &[Limb],
    ys: &mut [Limb],
    scratch: &mut [Limb],
) -> usize {
    // Compute (r,b) <-- (u00 a + u10 b, u01 a + u11 b) as
    //
    // ```
    // t  = u00 * a
    // r  = u10 * b
    // r += t;
    //
    // t  = u11 * b
    // b  = u01 * a
    // b += t;
    // ```
    let n = xs.len();
    let ys_lo = &ys[..n];
    let m_n = m.n;
    let mut big_n = n + m_n;
    let scratch = &mut scratch[..big_n];
    let (m00, m01, m10, m11) = m.get_four();
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(m_n, n)];
    limbs_mul_to_out(scratch, &m00[..m_n], xs, &mut mul_scratch);
    limbs_mul_to_out(&mut rp[..big_n], &m10[..m_n], ys_lo, &mut mul_scratch);
    let a_high = limbs_slice_add_same_length_in_place_left(&mut rp[..big_n], scratch);
    limbs_mul_to_out(scratch, &m11[..m_n], ys_lo, &mut mul_scratch);
    limbs_mul_to_out(ys, &m01[..m_n], xs, &mut mul_scratch);
    let b_high = limbs_slice_add_same_length_in_place_left(&mut ys[..big_n], scratch);
    if a_high || b_high {
        rp[big_n] = Limb::from(a_high);
        ys[big_n] = Limb::from(b_high);
        big_n += 1;
    } else {
        // Normalize
        while rp[big_n - 1] == 0 && ys[big_n - 1] == 0 {
            big_n -= 1;
        }
    }
    big_n
}

// Computes |v| = |(g - u a)| / b, where u may be positive or negative, and v is of the opposite
// sign. max(a, b) is of size n, u and v at most size n, and v must have space for n+1 limbs.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `compute_v` from `mpn/generic/gcdext.c`, GMP 6.2.1, except that `ys` is
// consumed.
fn limbs_extended_gcd_cofactor(
    vs: &mut [Limb],
    xs: &[Limb],
    ys: &mut [Limb],
    gs: &[Limb],
    ss: &[Limb],
    ss_len: usize,
    ss_sign: bool,
    scratch: &mut [Limb],
) -> usize {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    let gs_len = gs.len();
    assert_ne!(n, 0);
    assert_ne!(gs_len, 0);
    assert_ne!(ss_len, 0);
    let mut size = ss_len;
    assert!(size <= n);
    assert_ne!(ss[size - 1], 0);
    let xs_len = n - slice_trailing_zeros(xs);
    assert!(gs_len <= xs_len);
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(xs_len, size)];
    limbs_mul_to_out(scratch, &xs[..xs_len], &ss[..size], &mut mul_scratch);
    size += xs_len;
    let scratch = &mut scratch[..size];
    if ss_len != 0 && ss_sign {
        // |v| = -v = (u a - g) / b
        assert!(!limbs_sub_greater_in_place_left(scratch, gs));
        size -= slice_trailing_zeros(scratch);
        if size == 0 {
            return 0;
        }
    } else {
        // |v| = v = (g - u a) / b = (g + |u| a) / b. Since g <= a, (g + |u| a) always fits in
        // (|usize| + xs_len) limbs.
        assert!(!limbs_slice_add_greater_in_place_left(scratch, gs));
        if scratch[size - 1] == 0 {
            size -= 1;
        }
    }
    // Now divide t / b. There must be no remainder
    let ys_len = n - slice_trailing_zeros(ys);
    assert!(size >= ys_len);
    let vs_len = size + 1 - ys_len;
    assert!(vs_len <= n + 1);
    limbs_div_exact_to_out(vs, &mut scratch[..size], &mut ys[..ys_len]);
    if vs[vs_len - 1] == 0 {
        vs_len - 1
    } else {
        vs_len
    }
}

// This is equivalent to `CHOOSE_P_1` from `mpn/generic/gcdext.c`, GMP 6.2.1.
const fn choose_p_1(n: usize) -> usize {
    n >> 1
}

// This is equivalent to `CHOOSE_P_2` from `mpn/generic/gcdext.c`, GMP 6.2.1.
const fn choose_p_2(n: usize) -> usize {
    n / 3
}

// This is equivalent to `MPN_GCDEXT_LEHMER_N_ITCH` from `gmp-impl.h`, GMP 6.2.1.
const fn limbs_extended_gcd_same_length_lehmer_scratch_len(n: usize) -> usize {
    (n << 2) + 3
}

// TODO tune
const GCDEXT_DC_THRESHOLD: usize = 242;

// Temporary storage:
//
// Initial division: Quotient of at most xs_len - n + 1 <= xs_len limbs.
//
// Storage for us0 and us1: 2(n+1).
//
// Storage for hgcd matrix M, with input ceil(n/2): 5 * ceil(n/4)
//
// Storage for hgcd, input (n + 1)/2: 9 n/4 plus some.
//
// When hgcd succeeds: 1 + floor(3n/2) for adjusting a and b, and 2(n+1) for the cofactors.
//
// When hgcd fails: 2n + 1 for mpn_gcdext_subdiv_step, which is less.
//
// For the lehmer call after the loop, Let T denote GCDEXT_DC_THRESHOLD. For the gcdext_lehmer call,
// we need T each for u, a and b, and 4T+3 scratch space. Next, for compute_v, we need T for u, T+1
// for v and 2T scratch space. In all, 7T + 3 is sufficient for both operations.
//
// Optimal choice of p seems difficult. In each iteration the division of work between hgcd and the
// updates of us0 and us1 depends on the current size of the u. It may be desirable to use a
// different choice of p in each iteration. Also the input size seems to matter; choosing p = n / 3
// in the first iteration seems to improve performance slightly for input size just above the
// threshold, but degrade performance for larger inputs.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpn_gcdext` from `mpn/generic/gcdext.c`, GMP 6.2.1, where the sign of
// `usizep` is returned, but its magnitude is not indicated anywhere; it can be inferred by taking
// the length of the nonzero portion of `ss`.
pub fn limbs_extended_gcd(
    gs: &mut [Limb],
    ss: &mut [Limb],
    xs: &mut [Limb],
    ys: &mut [Limb],
) -> (usize, bool) {
    let xs_len = xs.len();
    let mut n = ys.len();
    let scratch_len = n + 1;
    assert!(xs_len >= n);
    assert_ne!(n, 0);
    assert_ne!(ys[n - 1], 0);
    let scratch = xs_len - n + 1;
    let mut scratch_2 = max(
        scratch,
        limbs_extended_gcd_same_length_lehmer_scratch_len(n),
    );
    let mut matrix_scratch = 0;
    if n >= GCDEXT_DC_THRESHOLD {
        // For hgcd loop. If the definitions of choose_p_1 and choose_p_2 change, the min and max
        // might change too.
        let max_p = choose_p_1(n);
        let min_p = choose_p_2(n);
        matrix_scratch = limbs_half_gcd_matrix_init_scratch_len(n - min_p);
        let hgcd_scratch = limbs_half_gcd_scratch_len(n - min_p);
        let update_scratch = max_p + n - 1;
        let mut scratch = matrix_scratch + max(hgcd_scratch, update_scratch);
        scratch_2 = max(scratch_2, scratch);
        // Final `limbs_extended_gcd_same_length_lehmer` call. Need space for u and for copies of a
        // and b.
        scratch = limbs_extended_gcd_same_length_lehmer_scratch_len(GCDEXT_DC_THRESHOLD)
            + 3 * GCDEXT_DC_THRESHOLD;
        scratch_2 = max(scratch_2, scratch);
        // Cofactors us0 and us1
        scratch_2 += (n + 1) << 1;
    }
    let mut scratch = vec![0; scratch_2];
    if xs_len > n {
        if n == 1 {
            xs[0] = limbs_div_limb_to_out_mod(&mut scratch, xs, ys[0]);
        } else {
            limbs_div_mod_qs_to_out_rs_to_ns(&mut scratch, xs, ys);
        }
        if slice_test_zero(&xs[..n]) {
            gs[..n].copy_from_slice(ys);
            return (n, true);
        }
    }
    let xs_lo = &mut xs[..n];
    if n < GCDEXT_DC_THRESHOLD {
        let (gs_len, _, ss_sign) =
            limbs_extended_gcd_same_length_lehmer(gs, ss, xs_lo, ys, &mut scratch);
        return (gs_len, ss_sign);
    }
    slice_set_zero(&mut scratch[..scratch_len << 1]);
    split_into_chunks_mut!(scratch, scratch_len, [us0, us1], scratch);
    // For the first hgcd call, there are no u updates, and it makes some sense to use a different
    // choice for p.
    let p = choose_p_1(n);
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(matrix_scratch);
    let mut m = HalfGcdMatrix::init(n - p, scratch_lo);
    let nn = limbs_half_gcd(&mut xs_lo[p..], &mut ys[p..], &mut m, scratch_hi);
    let mut ss_sign;
    let mut us_len;
    if nn != 0 {
        assert!(m.n <= (n - p - 1) >> 1);
        assert!(m.n + p <= (p + n - 1) >> 1);
        // Temporary storage 2 (p + m.n) <= p + n - 1
        n = limbs_half_gcd_matrix_adjust(&m, p + nn, xs, ys, p, scratch_hi);
        us0[..m.n].copy_from_slice(&m.get(1, 0)[..m.n]);
        us1[..m.n].copy_from_slice(&m.get(1, 1)[..m.n]);
        us_len = m.n;
        while us0[us_len - 1] == 0 && us1[us_len - 1] == 0 {
            us_len -= 1;
        }
    } else {
        // mpn_hgcd has failed. Then either one of a or b is very small, or the difference is very
        // small. Perform one subtraction followed by one division.
        us1[0] = 1;
        let (scratch, scratch_hi) = scratch.split_at_mut(n);
        let mut context = ExtendedGcdContext::new(gs, ss, 1, us0, us1, &mut scratch_hi[n..]);
        // Temporary storage n
        n = limbs_gcd_subdivide_step(&mut xs[..n], ys, 0, &mut context, scratch);
        ss_sign = context.ss_sign;
        if n == 0 {
            return (context.gs_len, ss_sign);
        }
        us_len = context.us_len;
        assert!(us_len < scratch_len);
    }
    while n >= GCDEXT_DC_THRESHOLD {
        let p = choose_p_2(n);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(matrix_scratch);
        let mut m = HalfGcdMatrix::init(n - p, scratch_lo);
        let nn = limbs_half_gcd(&mut xs[p..n], &mut ys[p..n], &mut m, scratch_hi);
        if nn != 0 {
            let t0 = scratch_hi;
            assert!(m.n <= (n - p - 1) >> 1);
            assert!(m.n + p <= (p + n - 1) >> 1);
            // Temporary storage 2 (p + M->n) <= p + n - 1
            n = limbs_half_gcd_matrix_adjust(&m, p + nn, xs, ys, p, t0);
            // By the same analysis as for mpn_hgcd_matrix_mul
            assert!(m.n + us_len <= scratch_len);
            t0[..us_len].copy_from_slice(&us0[..us_len]);
            // Temporary storage scratch_len
            let (t0_lo, t0_hi) = t0.split_at_mut(us_len);
            us_len = limbs_half_gcd_matrix_mul_vector(&mut m, us0, t0_lo, us1, t0_hi);
            assert!(us_len < scratch_len);
            assert!(us0[us_len - 1] != 0 || us1[us_len - 1] != 0);
        } else {
            // mpn_hgcd has failed. Then either one of a or b is very small, or the difference is
            // very small. Perform one subtraction followed by one division.
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(n);
            let mut context = ExtendedGcdContext::new(gs, ss, us_len, us0, us1, scratch_hi);
            // Temporary storage n
            n = limbs_gcd_subdivide_step(&mut xs[..n], &mut ys[..n], 0, &mut context, scratch_lo);
            ss_sign = context.ss_sign;
            if n == 0 {
                return (context.gs_len, ss_sign);
            }
            us_len = context.us_len;
            assert!(us_len < scratch_len);
        }
    }
    let n = n;
    let xs = &mut xs[..n];
    let ys = &mut ys[..n];
    // We have
    // ```
    // A = ... a + ... b
    // B =  us0 a +  us1 b
    //
    // a = us1  A + ... B
    // b = -us0 A + ... B
    // ```
    //
    // with bounds |us0|, |us1| <= B / min(a, b)
    //
    // We always have us1 > 0, and us0 == 0 is possible only if us1 == 1, in which case the only
    // reduction done so far is a = A - k B for some k.
    //
    // Compute g = u a + v b = (u us1 - v us0) A + (...) B. Here, u, v are bounded by
    // ```
    // |u| <= b,
    // |v| <= a
    // ```
    assert!(*xs.last().unwrap() != 0 || *ys.last().unwrap() != 0);
    if limbs_cmp_same_length(xs, ys) == Equal {
        // Must return the smallest cofactor, +us1 or -us0
        gs[..n].copy_from_slice(xs);
        let c = limbs_cmp_same_length(&us0[..us_len], &us1[..us_len]);
        // c == 0 can happen only when A = (2k+1) G, B = 2 G. And in this case we choose the
        // cofactor + 1, corresponding to G = A - k B, rather than -1, corresponding to G = - A +
        // (k+1) B.
        assert!(c != Equal || us_len == 1 && us0[0] == 1 && us1[0] == 1);
        if c == Less {
            us_len -= slice_trailing_zeros(&us0[..us_len]);
            ss[..us_len].copy_from_slice(&us0[..us_len]);
            ss_sign = false;
        } else {
            us_len -= slice_trailing_zeros(&us1[..us_len]);
            assert_ne!(us_len, 0);
            ss[..us_len].copy_from_slice(&us1[..us_len]);
            ss_sign = true;
        }
        (n, ss_sign)
    } else if us0[0] == 0 && us_len == 1 {
        fail_on_untested_path(
            "limbs_extended_gcd, \
            limbs_cmp_same_length(..) != Equal && us0[0] == 0 && us_len == 1",
        );
        assert_eq!(us1[0], 1);
        // g = u a + v b = (u us1 - v us0) A + (...) B = u A + (...) B
        let (gs_len, _, ss_sign) = limbs_extended_gcd_same_length_lehmer(gs, ss, xs, ys, scratch);
        (gs_len, ss_sign)
    } else {
        let (lehmer_ss, scratch) = scratch.split_at_mut(n);
        // Call limbs_extended_gcd_same_length_lehmer with copies of a and b.
        split_into_chunks_mut!(scratch, n, [scratch_0, scratch_1], scratch_2);
        scratch_0.copy_from_slice(xs);
        scratch_1.copy_from_slice(ys);
        let (gs_len, lehmer_us_len, lehmer_us_sign) =
            limbs_extended_gcd_same_length_lehmer(gs, lehmer_ss, scratch_0, scratch_1, scratch_2);
        let us0_len = us_len - slice_trailing_zeros(&us0[..us_len]);
        let us0 = &us0[..us0_len];
        assert_ne!(us0_len, 0);
        if lehmer_us_len == 0 {
            // u == 0 ==> v = g / b == 1 ==> g = - us0 A + (...) B
            ss[..us0_len].copy_from_slice(us0);
            return (gs_len, false);
        }
        let lehmer_ss = &lehmer_ss[..lehmer_us_len];
        let (lehmer_vs, scratch_hi) = scratch.split_at_mut(n + 1);
        // Compute v = (g - u a) / b
        let lehmer_vs_len = limbs_extended_gcd_cofactor(
            lehmer_vs,
            xs,
            ys,
            &gs[..gs_len],
            lehmer_ss,
            lehmer_us_len,
            lehmer_us_sign,
            scratch_hi,
        );
        let mut us1_len = us_len - slice_trailing_zeros(&us1[..us_len]);
        assert_ne!(us1_len, 0);
        assert!(lehmer_vs_len + us0_len <= scratch_len);
        // - We may still have v == 0
        // - Compute u us0
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(us1_len, lehmer_ss.len())];
        limbs_mul_to_out(ss, &us1[..us1_len], lehmer_ss, &mut mul_scratch);
        us_len = us1_len + lehmer_us_len;
        assert!(us_len <= scratch_len);
        if ss[us_len - 1] == 0 {
            us_len -= 1;
        }
        if lehmer_vs_len != 0 {
            // Overwrites old us1 value
            let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(us0.len(), lehmer_vs_len)];
            limbs_mul_to_out(us1, us0, &lehmer_vs[..lehmer_vs_len], &mut mul_scratch);
            us1_len = us0_len + lehmer_vs_len;
            if us1[us1_len - 1] == 0 {
                us1_len -= 1;
            }
            let us1 = &us1[..us1_len];
            let carry = if us1_len <= us_len {
                limbs_slice_add_greater_in_place_left(&mut ss[..us_len], us1)
            } else {
                let old_us_len = us_len;
                us_len = us1_len;
                limbs_add_to_out_aliased(&mut ss[..us1_len], old_us_len, us1)
            };
            ss[us_len] = Limb::from(carry);
            if carry {
                us_len += 1;
            }
            assert!(us_len < scratch_len);
        }
        (gs_len, lehmer_us_sign)
    }
}

fn extended_gcd_helper(a: Natural, b: Natural) -> (Natural, Integer, Integer) {
    let mut xs = a.to_limbs_asc();
    let mut ys = b.to_limbs_asc();
    let mut a = Integer::from(a);
    let mut b = Integer::from(b);
    let mut swapped = false;
    if xs.len() < ys.len() {
        swap(&mut xs, &mut ys);
        swap(&mut a, &mut b);
        swapped = true;
    }
    let mut gs = vec![0; ys.len()];
    let mut ss = vec![0; ys.len() + 1];
    let (g_len, ss_sign) = limbs_extended_gcd(&mut gs, &mut ss, &mut xs, &mut ys);
    gs.truncate(g_len);
    let gcd = Natural::from_owned_limbs_asc(gs);
    let mut s = Integer::from_sign_and_abs(ss_sign, Natural::from_owned_limbs_asc(ss));
    let mut t = (Integer::from(&gcd) - a * &s).div_exact(b);
    if swapped {
        swap(&mut s, &mut t);
    }
    (gcd, s, t)
}

impl ExtendedGcd for Natural {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. Both [`Natural`]s are
    /// taken by value.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(a, b)$, where
    ///   $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq \lfloor
    ///   a/g \rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32)
    ///         .extended_gcd(Natural::from(5u32))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     Natural::from(240u32)
    ///         .extended_gcd(Natural::from(46u32))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// ```
    fn extended_gcd(self, other: Natural) -> (Natural, Integer, Integer) {
        match (self, other) {
            (Natural::ZERO, Natural::ZERO) => (Natural::ZERO, Integer::ZERO, Integer::ZERO),
            (a, b) if a == b => (b, Integer::ZERO, Integer::ONE),
            (Natural::ZERO, b) => (b, Integer::ZERO, Integer::ONE),
            (a, Natural::ZERO) => (a, Integer::ONE, Integer::ZERO),
            (Natural(Small(x)), Natural(Small(y))) => {
                let (gcd, s, t) = x.extended_gcd(y);
                (Natural::from(gcd), Integer::from(s), Integer::from(t))
            }
            (a, b) => extended_gcd_helper(a, b),
        }
    }
}

impl<'a> ExtendedGcd<&'a Natural> for Natural {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. The first [`Natural`] is
    /// taken by value and the second by reference.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(a, b)$, where
    ///   $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq \lfloor
    ///   a/g \rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32)
    ///         .extended_gcd(&Natural::from(5u32))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     Natural::from(240u32)
    ///         .extended_gcd(&Natural::from(46u32))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// ```
    fn extended_gcd(self, other: &'a Natural) -> (Natural, Integer, Integer) {
        match (self, other) {
            (Natural::ZERO, &Natural::ZERO) => (Natural::ZERO, Integer::ZERO, Integer::ZERO),
            (a, b) if a == *b => (b.clone(), Integer::ZERO, Integer::ONE),
            (Natural::ZERO, b) => (b.clone(), Integer::ZERO, Integer::ONE),
            (a, &Natural::ZERO) => (a, Integer::ONE, Integer::ZERO),
            (Natural(Small(x)), Natural(Small(y))) => {
                let (gcd, s, t) = x.extended_gcd(*y);
                (Natural::from(gcd), Integer::from(s), Integer::from(t))
            }
            (a, b) => extended_gcd_helper(a, b.clone()),
        }
    }
}

impl<'a> ExtendedGcd<Natural> for &'a Natural {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. The first [`Natural`] is
    /// taken by reference and the second by value.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(a, b)$, where
    ///   $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq \lfloor
    ///   a/g \rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32))
    ///         .extended_gcd(Natural::from(5u32))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(240u32))
    ///         .extended_gcd(Natural::from(46u32))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// ```
    fn extended_gcd(self, other: Natural) -> (Natural, Integer, Integer) {
        match (self, other) {
            (&Natural::ZERO, Natural::ZERO) => (Natural::ZERO, Integer::ZERO, Integer::ZERO),
            (a, b) if *a == b => (b, Integer::ZERO, Integer::ONE),
            (&Natural::ZERO, b) => (b, Integer::ZERO, Integer::ONE),
            (a, Natural::ZERO) => (a.clone(), Integer::ONE, Integer::ZERO),
            (Natural(Small(x)), Natural(Small(y))) => {
                let (gcd, s, t) = x.extended_gcd(y);
                (Natural::from(gcd), Integer::from(s), Integer::from(t))
            }
            (a, b) => extended_gcd_helper(a.clone(), b),
        }
    }
}

impl<'a, 'b> ExtendedGcd<&'a Natural> for &'b Natural {
    type Gcd = Natural;
    type Cofactor = Integer;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s $a$ and $b$, and also the
    /// coefficients $x$ and $y$ in Bézout's identity $ax+by=\gcd(a,b)$. Both [`Natural`]s are
    /// taken by reference.
    ///
    /// The are infinitely many $x$, $y$ that satisfy the identity for any $a$, $b$, so the full
    /// specification is more detailed:
    ///
    /// - $f(0, 0) = (0, 0, 0)$.
    /// - $f(a, ak) = (a, 1, 0)$ if $a > 0$ and $k \neq 1$.
    /// - $f(bk, b) = (b, 0, 1)$ if $b > 0$.
    /// - $f(a, b) = (g, x, y)$ if $a \neq 0$ and $b \neq 0$ and $\gcd(a, b) \neq \min(a, b)$, where
    ///   $g = \gcd(a, b) \geq 0$, $ax + by = g$, $x \leq \lfloor b/g \rfloor$, and $y \leq \lfloor
    ///   a/g \rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ExtendedGcd;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32))
    ///         .extended_gcd(&Natural::from(5u32))
    ///         .to_debug_string(),
    ///     "(1, 2, -1)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(240u32))
    ///         .extended_gcd(&Natural::from(46u32))
    ///         .to_debug_string(),
    ///     "(2, -9, 47)"
    /// );
    /// ```
    fn extended_gcd(self, other: &'a Natural) -> (Natural, Integer, Integer) {
        match (self, other) {
            (&Natural::ZERO, &Natural::ZERO) => (Natural::ZERO, Integer::ZERO, Integer::ZERO),
            (a, b) if a == b => (b.clone(), Integer::ZERO, Integer::ONE),
            (&Natural::ZERO, b) => (b.clone(), Integer::ZERO, Integer::ONE),
            (a, &Natural::ZERO) => (a.clone(), Integer::ONE, Integer::ZERO),
            (Natural(Small(x)), Natural(Small(y))) => {
                let (gcd, s, t) = x.extended_gcd(*y);
                (Natural::from(gcd), Integer::from(s), Integer::from(t))
            }
            (a, b) => extended_gcd_helper(a.clone(), b.clone()),
        }
    }
}
