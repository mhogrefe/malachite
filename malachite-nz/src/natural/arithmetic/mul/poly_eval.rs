// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_toom_eval_dgr3_pm2`, `mpn_toom_eval_dgr3_pm1`, `mpn_toom_eval_pm1`, and
//      `mpn_toom_eval_pm2exp` contributed to the GNU project by Niels Möller.
//
//      `DO_addlsh2` and `mpn_toom_eval_pm2` contributed to the GNU project by Niels Möller and
//      Marco Bodrato.
//
//      `DO_mpn_addlsh_n` and `mpn_toom_eval_pm2rexp` contributed to the GNU project by Marco
//      Bodrato.
//
//      Copyright © 2009 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_add_to_out,
    limbs_add_to_out_aliased, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::sub::limbs_sub_same_length_to_out;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::platform::Limb;
use core::cmp::Ordering::*;
use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{Parity, WrappingAddAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::NotAssign;

// Evaluate a degree-3 polynomial in +1 and -1, where each coefficient has width `n` limbs, except
// the last, which has width `n_high` limbs.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_toom_eval_dgr3_pm1` in `mpn/generic/toom_eval_dgr3_pm1.c`, GMP 6.2.1,
// where `s` is omitted from the inputs because it can be determined from `ap` and `n`.
pub(crate) fn limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
    v_1: &mut [Limb],
    v_neg_1: &mut [Limb],
    poly: &[Limb],
    n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_eq!(v_1.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);
    split_into_chunks!(poly, n, [poly_0, poly_1, poly_2], poly_3);
    assert!(poly_3.len() <= n);
    v_1[n] = Limb::from(limbs_add_same_length_to_out(v_1, poly_0, poly_2));
    scratch[n] = Limb::from(limbs_add_to_out(scratch, poly_1, poly_3));
    let v_neg_1_neg = limbs_cmp_same_length(v_1, scratch) == Less;
    if v_neg_1_neg {
        limbs_sub_same_length_to_out(v_neg_1, scratch, v_1);
    } else {
        limbs_sub_same_length_to_out(v_neg_1, v_1, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_1, scratch);
    assert!(v_1[n] <= 3);
    assert!(v_neg_1[n] <= 1);
    v_neg_1_neg
}

// Evaluate a degree-3 polynomial in +2 and -2, where each coefficient has width `n` limbs, except
// the last, which has width `n_high` limbs.
//
// Needs n + 1 limbs of temporary storage.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_toom_eval_dgr3_pm2` from `mpn/generic/toom_eval_dg3_pm2.c`, GMP 6.2.1,
// where `s` is omitted from the inputs because it can be determined from `ap` and `n`.
pub(crate) fn limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    poly: &[Limb],
    n: usize,
    scratch: &mut [Limb],
) -> bool {
    split_into_chunks!(poly, n, [poly_0, poly_1, poly_2], poly_3);
    let n_high = poly_3.len();
    assert!(n_high <= n);
    assert_eq!(v_2.len(), n + 1);
    let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
    assert_eq!(scratch_init.len(), n);
    // scratch <- (poly_0 + 4 * poly_2) +/- (2 * poly_1 + 8 * poly_3)
    v_2[n] = limbs_shl_to_out(scratch_init, poly_2, 2);
    if limbs_add_same_length_to_out(v_2, scratch_init, poly_0) {
        v_2[n] += 1;
    }
    if n_high < n {
        scratch_init[n_high] = limbs_shl_to_out(scratch_init, poly_3, 2);
        *scratch_last = Limb::from(limbs_add_to_out_aliased(scratch_init, n_high + 1, poly_1));
    } else {
        *scratch_last = limbs_shl_to_out(scratch_init, poly_3, 2);
        if limbs_slice_add_same_length_in_place_left(scratch_init, poly_1) {
            *scratch_last += 1;
        }
    }
    limbs_slice_shl_in_place(scratch, 1);
    let v_neg_2_neg = limbs_cmp_same_length(v_2, scratch) == Less;
    if v_neg_2_neg {
        limbs_sub_same_length_to_out(v_neg_2, scratch, v_2);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, v_2, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_2, scratch);
    assert!(v_2[n] < 15);
    assert!(v_neg_2[n] < 10);
    v_neg_2_neg
}

// Evaluates a polynomial of degree 3 < `degree` < `Limb::WIDTH`, in the points +1 and -1, where
// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// # Worst-case complexity
// $T(m) = O(m)$
//
// $M(m) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $m$ is `n * degree`.
//
// This is equivalent to `mpn_toom_eval_pm1` from `mpn/generic/toom_eval_pm1.c`, GMP 6.2.1, where
// `hn` is omitted from the inputs because it can be determined from `xp` and `n`.
pub(crate) fn limbs_mul_toom_evaluate_poly_in_1_and_neg_1(
    v_1: &mut [Limb],
    v_neg_1: &mut [Limb],
    degree: usize,
    poly: &[Limb],
    n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert!(degree > 3);
    assert_eq!(v_1.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);

    // The degree `degree` is also the number of full-size coefficients, so that the last
    // coefficient, of size `n_high`, starts at `poly[degree * n..]`.
    let coefficients = poly.chunks(n).collect_vec();
    assert_eq!(coefficients.len(), degree + 1);

    // The degree `degree` is also the number of full-size coefficients, so that the last
    // coefficient, of size `n_high`, starts at poly + degree * n.
    v_1[n] = Limb::from(limbs_add_same_length_to_out(
        v_1,
        coefficients[0],
        coefficients[2],
    ));
    let mut i = 4;
    while i < degree {
        assert!(!limbs_slice_add_greater_in_place_left(v_1, coefficients[i]));
        i += 2;
    }
    scratch[n] = Limb::from(limbs_add_same_length_to_out(
        scratch,
        coefficients[1],
        coefficients[3],
    ));
    let mut i = 5;
    while i < degree {
        assert!(!limbs_slice_add_greater_in_place_left(
            scratch,
            coefficients[i],
        ));
        i += 2;
    }
    assert!(!limbs_slice_add_greater_in_place_left(
        if degree.even() { v_1 } else { scratch },
        coefficients[degree],
    ));
    let v_neg_1_neg = limbs_cmp_same_length(v_1, scratch) == Less;
    if v_neg_1_neg {
        limbs_sub_same_length_to_out(v_neg_1, scratch, v_1);
    } else {
        limbs_sub_same_length_to_out(v_neg_1, v_1, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_1, scratch);
    let degree = Limb::exact_from(degree);
    assert!(v_1[n] <= degree);
    assert!(v_neg_1[n] <= (degree >> 1) + 1);
    v_neg_1_neg
}

// Given a `Natural` whose highest limb is `carry` and remaining limbs are `xs`, multiplies the
// `Natural` by 4 and adds the `Natural` whose limbs are `ys`. The highest limb of the result is
// written back to `carry` and the remaining limbs are written to `out`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `DO_addlsh2` from `mpn/generic/toom_eval_pm2.c`, GMP 6.2.1, with `d ==
// out`, `a == xs`, and `b == ys`.
fn shl_2_and_add_with_carry_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb], carry: &mut Limb) {
    *carry <<= 2;
    *carry += limbs_shl_to_out(out, xs, 2);
    if limbs_slice_add_same_length_in_place_left(&mut out[..ys.len()], ys) {
        *carry += 1;
    }
}

// Given a `Natural` whose highest limb is `carry` and remaining limbs are `xs`, multiplies the
// `Natural` by 4 and adds the `Natural` whose limbs are `ys`. The highest limb of the result is
// written back to `carry` and the remaining limbs are written to `xs`. `xs` and `ys` must have the
// same length.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ys.len()`.
//
// This is equivalent to `DO_addlsh2` from `mpn/generic/toom_eval_pm2.c`, GMP 6.2.1, with `d == b ==
// ys` and `a == xs`.
fn shl_2_and_add_with_carry_in_place_left(xs: &mut [Limb], ys: &[Limb], carry: &mut Limb) {
    *carry <<= 2;
    *carry += limbs_slice_shl_in_place(xs, 2);
    if limbs_slice_add_same_length_in_place_left(xs, ys) {
        *carry += 1;
    }
}

// Evaluates a polynomial of degree 3 < `degree` < `Limb::WIDTH`, in the points +2 and -2, where
// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// # Worst-case complexity
// $T(m) = O(m)$
//
// $M(m) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $m$ is `n * degree`.
//
// This is equivalent to `mpn_toom_eval_pm2` from `mpn/generic/toom_eval_pm2.c`, GMP 6.2.1, where
// `hn` is omitted from the inputs because it can be determined from `xp` and `n`.
pub(crate) fn limbs_mul_toom_evaluate_poly_in_2_and_neg_2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    degree: usize,
    poly: &[Limb],
    n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert!(degree >= 3);
    assert!(degree < usize::wrapping_from(Limb::WIDTH));
    assert_eq!(v_2.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);
    // The degree `degree` is also the number of full-size coefficients, so that the last
    // coefficient, of size `n_high`, starts at `poly[degree * n..]`.
    let coefficients = poly.chunks(n).collect_vec();
    assert_eq!(coefficients.len(), degree + 1);
    let n_high = coefficients[degree].len();
    let (v_2_last, v_2_init) = v_2.split_last_mut().unwrap();
    let mut carry = 0;
    shl_2_and_add_with_carry_to_out(
        v_2_init,
        coefficients[degree],
        &coefficients[degree - 2][..n_high],
        &mut carry,
    );
    if n_high != n {
        carry = Limb::from(limbs_add_limb_to_out(
            &mut v_2_init[n_high..],
            &coefficients[degree - 2][n_high..],
            carry,
        ));
    }
    if degree >= 4 {
        let mut i = degree - 4;
        loop {
            shl_2_and_add_with_carry_in_place_left(v_2_init, coefficients[i], &mut carry);
            if i < 2 {
                break;
            }
            i -= 2;
        }
    }
    *v_2_last = carry;
    let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
    let mut carry = 0;
    shl_2_and_add_with_carry_to_out(
        scratch_init,
        coefficients[degree - 1],
        coefficients[degree - 3],
        &mut carry,
    );
    if degree >= 5 {
        let mut i = degree - 5;
        loop {
            shl_2_and_add_with_carry_in_place_left(scratch_init, coefficients[i], &mut carry);
            if i < 2 {
                break;
            }
            i -= 2;
        }
    }
    *scratch_last = carry;
    assert_eq!(
        limbs_slice_shl_in_place(if degree.even() { scratch } else { v_2 }, 1),
        0
    );
    let mut v_neg_2_neg = limbs_cmp_same_length(v_2, scratch) == Less;
    if v_neg_2_neg {
        limbs_sub_same_length_to_out(v_neg_2, scratch, v_2);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, v_2, scratch);
    }
    if degree.odd() {
        v_neg_2_neg.not_assign();
    }
    limbs_slice_add_same_length_in_place_left(v_2, scratch);
    let mut shift = 1 << (degree + 1);
    if shift != 0 {
        assert!(v_2[n] < shift - 1);
    }
    shift <<= 1;
    if shift != 0 {
        assert!(v_neg_2[n] < shift / 3);
    }
    v_neg_2_neg
}

// Evaluates a polynomial of degree `degree` > 2, in the points 2 ^ `shift` and -2 ^ `shift`, where
// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// # Worst-case complexity
// $T(m) = O(m)$
//
// $M(m) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $m$ is `n * degree`.
//
// This is equivalent to `mpn_toom_eval_pm2exp` from `mpn/generic/toom_eval_pm2exp.c`, GMP 6.2.1,
// where `hn` is omitted from the inputs because it can be determined from `xp` and `n`.
pub(crate) fn limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow(
    v_2_pow: &mut [Limb],
    v_neg_2_pow: &mut [Limb],
    degree: usize,
    poly: &[Limb],
    n: usize,
    shift: u64,
    scratch: &mut [Limb],
) -> bool {
    assert!(degree >= 3);
    let degree_u64 = u64::exact_from(degree);
    assert!(shift * degree_u64 < Limb::WIDTH);
    assert_eq!(v_2_pow.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);
    let coefficients = poly.chunks(n).collect_vec();
    assert_eq!(coefficients.len(), degree + 1);
    let n_high = coefficients[degree].len();
    let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
    let (v_2_pow_last, v_2_pow_init) = v_2_pow.split_last_mut().unwrap();
    // The degree `degree` is also the number of full-size coefficients, so that the last
    // coefficient, of size `n_high`, starts at `poly + degree * n`.
    *v_2_pow_last = limbs_shl_to_out(scratch_init, coefficients[2], shift << 1);
    if limbs_add_same_length_to_out(v_2_pow_init, coefficients[0], scratch_init) {
        v_2_pow_last.wrapping_add_assign(1);
    }
    let mut i = 4;
    let mut local_shift = shift << 2;
    while i < degree {
        v_2_pow_last.wrapping_add_assign(limbs_shl_to_out(
            scratch_init,
            coefficients[i],
            local_shift,
        ));
        if limbs_slice_add_same_length_in_place_left(v_2_pow_init, scratch_init) {
            v_2_pow_last.wrapping_add_assign(1);
        }
        i += 2;
        local_shift += shift << 1;
    }

    *scratch_last = limbs_shl_to_out(scratch_init, coefficients[1], shift);
    let mut i = 3;
    let mut local_shift = shift * 3;
    while i < degree {
        *scratch_last += limbs_shl_to_out(v_neg_2_pow, coefficients[i], local_shift);
        if limbs_slice_add_same_length_in_place_left(scratch_init, &v_neg_2_pow[..n]) {
            scratch_last.wrapping_add_assign(1);
        }
        i += 2;
        local_shift += shift << 1;
    }

    v_neg_2_pow[n_high] = limbs_shl_to_out(v_neg_2_pow, coefficients[degree], degree_u64 * shift);
    limbs_slice_add_greater_in_place_left(
        if degree.even() { v_2_pow } else { scratch },
        &v_neg_2_pow[..=n_high],
    );
    let v_neg_2_pow_neg = limbs_cmp_same_length(v_2_pow, scratch) == Less;
    if v_neg_2_pow_neg {
        limbs_sub_same_length_to_out(v_neg_2_pow, scratch, v_2_pow);
    } else {
        limbs_sub_same_length_to_out(v_neg_2_pow, v_2_pow, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_2_pow, scratch);
    v_neg_2_pow_neg
}

// Given a `Natural` whose limbs are `ys`, multiplies the `Natural` by `2 ^ shift` and adds the
// `Natural` whose limbs are the lowest `ys.len()` limbs of `xs`, writing the lowest `ys.len()`
// limbs of the result to those limbs, and returning the highest limb as a carry.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ys.len()`.
//
// This is equivalent to `DO_mpn_addlsh_n` from `mpn/generic/toom_eval_pm2rexp.c`, GMP 6.2.1.
pub(crate) fn limbs_shl_and_add_same_length_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    shift: u64,
    scratch: &mut [Limb],
) -> Limb {
    let n = ys.len();
    let scratch = &mut scratch[..n];
    let mut carry = limbs_shl_to_out(scratch, ys, shift);
    if limbs_slice_add_same_length_in_place_left(&mut xs[..n], scratch) {
        carry.wrapping_add_assign(1);
    }
    carry
}

// Evaluates a polynomial of degree `degree` > 2, in the points 2 ^ -`shift` and -2 ^ -`shift`,
// where each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// # Worst-case complexity
// $T(m) = O(m)$
//
// $M(m) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $m$ is `n * degree`.
//
// This is equivalent to `mpn_toom_eval_pm2rexp` from `mpn/generic/toom_eval_pm2rexp.c`, GMP 6.2.1,
// where `t` is omitted from the inputs because it can be determined from `ap` and `n`.
pub(crate) fn limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg(
    v_2_pow_neg: &mut [Limb],
    v_neg_2_pow_neg: &mut [Limb],
    degree: usize,
    poly: &[Limb],
    n: usize,
    shift: u64,
    scratch: &mut [Limb],
) -> bool {
    assert_ne!(shift, 0); // or `limbs_mul_toom_evaluate_poly_in_1_and_neg_1` should be used
    assert!(degree > 1);
    let degree_u64 = u64::exact_from(degree);
    assert_eq!(v_2_pow_neg.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);
    let coefficients = poly.chunks(n).collect_vec();
    assert_eq!(coefficients.len(), degree + 1);
    v_2_pow_neg[n] = limbs_shl_to_out(v_2_pow_neg, coefficients[0], shift * degree_u64);
    scratch[n] = limbs_shl_to_out(scratch, coefficients[1], shift * (degree_u64 - 1));
    if degree.even() {
        assert!(!limbs_slice_add_greater_in_place_left(
            v_2_pow_neg,
            coefficients[degree],
        ));
    } else {
        assert!(!limbs_slice_add_greater_in_place_left(
            scratch,
            coefficients[degree],
        ));
        let carry = limbs_shl_and_add_same_length_in_place_left(
            v_2_pow_neg,
            coefficients[degree - 1],
            shift,
            v_neg_2_pow_neg,
        );
        v_2_pow_neg[n].wrapping_add_assign(carry);
    }
    let mut i = 2;
    let mut local_shift = shift * (degree_u64 - 2);
    while i < degree - 1 {
        let carry = limbs_shl_and_add_same_length_in_place_left(
            v_2_pow_neg,
            coefficients[i],
            local_shift,
            v_neg_2_pow_neg,
        );
        v_2_pow_neg[n].wrapping_add_assign(carry);
        i += 1;
        local_shift -= shift;
        let carry = limbs_shl_and_add_same_length_in_place_left(
            scratch,
            coefficients[i],
            local_shift,
            v_neg_2_pow_neg,
        );
        scratch[n].wrapping_add_assign(carry);
        i += 1;
        local_shift -= shift;
    }
    let v_2_pow_neg_neg = limbs_cmp_same_length(v_2_pow_neg, scratch) == Less;
    if v_2_pow_neg_neg {
        limbs_sub_same_length_to_out(v_neg_2_pow_neg, scratch, v_2_pow_neg);
    } else {
        limbs_sub_same_length_to_out(v_neg_2_pow_neg, v_2_pow_neg, scratch);
    }
    assert!(!limbs_slice_add_same_length_in_place_left(
        v_2_pow_neg,
        scratch,
    ));
    v_2_pow_neg_neg
}
