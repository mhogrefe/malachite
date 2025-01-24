// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Contributed by Niels Möller and Marco Bodrato.
//
//      Copyright © 2003-2005, 2008, 2009 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out,
};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::platform::{Limb, MATRIX22_STRASSEN_THRESHOLD};
use core::cmp::Ordering::*;

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `abs_sub_n` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1, where `rp != ap`.
fn limbs_sub_abs_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if limbs_cmp_same_length(xs, ys) == Less {
        limbs_sub_same_length_to_out(out, ys, xs);
        true
    } else {
        limbs_sub_same_length_to_out(out, xs, ys);
        false
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`. This is equivalent to
// `abs_sub_n` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1, where `rp == ap`.
fn limbs_sub_abs_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if limbs_cmp_same_length(xs, ys) == Less {
        limbs_sub_same_length_in_place_right(ys, xs);
        true
    } else {
        limbs_sub_same_length_in_place_left(xs, ys);
        false
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `abs_sub_n` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1, where `rp == bp`.
fn limbs_sub_abs_same_length_in_place_right(xs: &[Limb], ys: &mut [Limb]) -> bool {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if limbs_cmp_same_length(xs, ys) == Less {
        limbs_sub_same_length_in_place_left(ys, xs);
        true
    } else {
        limbs_sub_same_length_in_place_right(xs, ys);
        false
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `add_signed_n` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1, where `rp !=
// ap`.
fn limbs_add_signed_same_length_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    x_sign: bool,
    ys: &[Limb],
    y_sign: bool,
) -> bool {
    if x_sign == y_sign {
        assert!(!limbs_add_same_length_to_out(out, xs, ys));
        x_sign
    } else {
        x_sign != limbs_sub_abs_same_length_to_out(out, xs, ys)
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `add_signed_n` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1, where `rp ==
// ap`.
fn limbs_add_signed_same_length_in_place_left(
    xs: &mut [Limb],
    x_sign: bool,
    ys: &[Limb],
    y_sign: bool,
) -> bool {
    if x_sign == y_sign {
        assert!(!limbs_slice_add_same_length_in_place_left(xs, ys));
        x_sign
    } else {
        x_sign != limbs_sub_abs_same_length_in_place_left(xs, ys)
    }
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_matrix22_mul_itch` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1.
pub_const_test! {limbs_matrix_mul_2_2_scratch_len(xs_len: usize, ys_len: usize) -> usize {
    if xs_len < MATRIX22_STRASSEN_THRESHOLD || ys_len < MATRIX22_STRASSEN_THRESHOLD {
        3 * xs_len + 2 * ys_len
    } else {
        3 * (xs_len + ys_len) + 5
    }
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs_len, ys00.len())`.
pub_test! {limbs_matrix_2_2_mul_small(
    xs00: &mut [Limb],
    xs01: &mut [Limb],
    xs10: &mut [Limb],
    xs11: &mut [Limb],
    xs_len: usize,
    ys00: &[Limb],
    ys01: &[Limb],
    ys10: &[Limb],
    ys11: &[Limb],
    scratch: &mut [Limb],
) {
    let ys_len = ys00.len();
    // The actual output length is one limb larger than this
    let out_len = xs_len + ys_len;
    let (scratch, remainder) = scratch.split_at_mut(xs_len);
    split_into_chunks_mut!(remainder, out_len, [p0, p1], _unused);
    let mut t0 = &mut *xs00;
    let mut t1 = &mut *xs01;
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(xs_len, ys_len)];
    for _ in 0..2 {
        let t0_0 = &t0[..xs_len];
        scratch.copy_from_slice(t0_0);
        if xs_len >= ys_len {
            limbs_mul_greater_to_out(p0, t0_0, ys00, &mut mul_scratch);
            let t1_0 = &t1[..xs_len];
            limbs_mul_greater_to_out(p1, t1_0, ys11, &mut mul_scratch);
            limbs_mul_greater_to_out(t0, t1_0, ys10, &mut mul_scratch);
            limbs_mul_greater_to_out(t1, scratch, ys01, &mut mul_scratch);
        } else {
            limbs_mul_greater_to_out(p0, ys00, t0_0, &mut mul_scratch);
            let t1_0 = &t1[..xs_len];
            limbs_mul_greater_to_out(p1, ys11, t1_0, &mut mul_scratch);
            limbs_mul_greater_to_out(t0, ys10, t1_0, &mut mul_scratch);
            limbs_mul_greater_to_out(t1, ys01, scratch, &mut mul_scratch);
        }
        let (t0_last, t0_init) = t0[..=out_len].split_last_mut().unwrap();
        *t0_last = Limb::from(limbs_slice_add_same_length_in_place_left(t0_init, p0));
        let (t1_last, t1_init) = t1[..=out_len].split_last_mut().unwrap();
        *t1_last = Limb::from(limbs_slice_add_same_length_in_place_left(t1_init, p1));
        t0 = &mut *xs10;
        t1 = &mut *xs11;
    }
}}

// Algorithm:
//
// ```
// / s0 \   /  1  0   0  0 \ / xs00 \
// | s1 |   |  0  1   0  1 | | xs01 |
// | s2 |   |  0  0  -1  1 | | xs10 |
// | s3 | = |  0  1  -1  1 | \ xs11 /
// | s4 |   | -1  1  -1  1 |
// | s5 |   |  0  1   0  0 |
// \ s6 /   \  0  0   1  0 /
//
// / t0 \   /  1  0   0  0 \ / ys00 \
// | t1 |   |  0  1   0  1 | | ys01 |
// | t2 |   |  0  0  -1  1 | | ys10 |
// | t3 | = |  0  1  -1  1 | \ ys11 /
// | t4 |   | -1  1  -1  1 |
// | t5 |   |  0  1   0  0 |
// \ t6 /   \  0  0   1  0 /
// ```
//
// Note: the two matrices above are the same, but s_i and t_i are used in the same product, only for
// i < 4, see "A Strassen-like Matrix Multiplication suited for squaring and higher power
// computation" by M. Bodrato, in Proceedings of ISSAC 2010.
//
// ```
// / xs00 \   / 1 0   0   0   0   1   0 \ / s0 * t0 \
// | xs01 | = | 0 0  -1   1  -1   1   0 | | s1 * t1 |
// | xs10 |   | 0 1   0  -1   0  -1  -1 | | s2 * t2 |
// \ xs11 /   \ 0 1   1  -1   0  -1   0 / | s3 * t3 |
//		                                  | s4 * t5 |
//		                                  | s5 * t6 |
//		                                  \ s6 * t4 /
// ```
//
// The scheduling uses two temporaries U0 and U1 to store products, and two, S0 and T0, to store
// combinations of entries of the two operands.
//
// Computes R = R * M. Elements are numbers R = (xs00, xs01; xs10, xs11).
//
// Resulting elements are of size up to xs_len + ys_len + 1.
//
// Temporary storage: 3 * xs_len + 3 * ys_len + 5.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs_len, ys00.len())`.
//
// This is equivalent to `mpn_matrix22_mul_strassen` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1.
pub_test! {limbs_matrix_2_2_mul_strassen(
    xs00: &mut [Limb],
    xs01: &mut [Limb],
    xs10: &mut [Limb],
    xs11: &mut [Limb],
    xs_len: usize,
    ys00: &[Limb],
    ys01: &[Limb],
    ys10: &[Limb],
    ys11: &[Limb],
    scratch: &mut [Limb],
) {
    let ys_len = ys00.len();
    let sum_len = xs_len + ys_len;
    let (s0, remainder) = scratch.split_at_mut(xs_len + 1);
    let (s0_last, s0_init) = s0.split_last_mut().unwrap();
    let (t0, remainder) = remainder.split_at_mut(ys_len + 1);
    let (t0_last, t0_init) = t0.split_last_mut().unwrap();
    let (u0, u1) = remainder.split_at_mut(sum_len + 1);
    let u1 = &mut u1[..sum_len + 2];
    let xs00_lo = &xs00[..xs_len];
    let xs01_lo = &mut xs01[..=xs_len];
    let (xs01_lo_last, xs01_lo_init) = xs01_lo.split_last_mut().unwrap();
    let xs10 = &mut xs10[..=sum_len];
    let xs10_lo = &xs10[..xs_len];
    let xs11 = &mut xs11[..=sum_len];
    let xs11_lo = &mut xs11[..xs_len];
    // u5 = s5 * t6
    let mut mul_scratch = vec![
        0;
        max!(
            limbs_mul_to_out_scratch_len(sum_len + 1, ys_len + 1),
            limbs_mul_to_out_scratch_len(xs_len + 1, ys_len),
            limbs_mul_to_out_scratch_len(xs_len, ys_len + 1),
            limbs_mul_to_out_scratch_len(xs_len + 1, ys_len + 1)
        )
    ];
    assert!(xs01_lo_init.len() <= sum_len + 1);
    assert!(ys10.len() <= ys_len + 1);
    limbs_mul_to_out(u0, xs01_lo_init, ys10, &mut mul_scratch);
    // xs11 - xs10
    let mut x11_sign = limbs_sub_abs_same_length_in_place_left(xs11_lo, xs10_lo);
    let x01_sign = if x11_sign {
        *xs01_lo_last = 0;
        limbs_sub_abs_same_length_in_place_left(xs01_lo_init, xs11_lo)
    } else {
        // xs01 - xs10 + xs11
        *xs01_lo_last = Limb::from(limbs_slice_add_same_length_in_place_left(
            xs01_lo_init,
            xs11_lo,
        ));
        false
    };
    let s0_sign = if x01_sign {
        *s0_last = Limb::from(limbs_add_same_length_to_out(s0_init, xs01_lo_init, xs00_lo));
        false
    } else if *xs01_lo_last != 0 {
        *s0_last = *xs01_lo_last;
        if limbs_sub_same_length_to_out(s0_init, xs01_lo_init, xs00_lo) {
            s0[xs_len] -= 1;
        }
        // Reverse sign! s4 = -xs00 + xs01 - xs10 + xs11
        true
    } else {
        *s0_last = 0;
        limbs_sub_abs_same_length_to_out(s0_init, xs00_lo, xs01_lo_init)
    };
    // u0 = s0 * t0
    limbs_mul_to_out(u1, xs00_lo, ys00, &mut mul_scratch);
    let (u0_last, u0_init) = u0.split_last_mut().unwrap();
    xs00[sum_len] = Limb::from(limbs_add_same_length_to_out(xs00, u0_init, &u1[..sum_len]));
    // u0 + u5
    assert!(xs00[sum_len] < 2);
    let mut t0_sign = limbs_sub_abs_same_length_to_out(t0_init, ys11, ys10);
    // Reverse sign!
    let u1_sign = x11_sign == t0_sign;
    // u2 = s2 * t2
    limbs_mul_to_out(u1, xs11_lo, t0_init, &mut mul_scratch);
    u1[sum_len] = 0;
    *t0_last = if t0_sign {
        t0_sign = limbs_sub_abs_same_length_in_place_right(ys01, t0_init);
        0
    } else {
        Limb::from(limbs_slice_add_same_length_in_place_left(t0_init, ys01))
    };
    if *t0_last != 0 {
        // u3 = s3 * t3
        limbs_mul_to_out(xs11, xs01_lo_init, t0, &mut mul_scratch);
        assert!(*xs01_lo_last < 2);
        if *xs01_lo_last != 0 {
            limbs_slice_add_same_length_in_place_left(&mut xs11[xs_len..], t0);
        }
    } else {
        limbs_mul_to_out(xs11, xs01_lo, t0_init, &mut mul_scratch);
    }
    assert!(xs11[sum_len] < 4);
    *u0_last = 0;
    x11_sign = if x01_sign == t0_sign {
        // u3 + u5
        assert!(!limbs_slice_add_same_length_in_place_left(xs11, u0));
        false
    } else {
        limbs_sub_abs_same_length_in_place_right(u0, xs11)
    };
    let (t0_last, t0_init) = t0.split_last_mut().unwrap();
    if t0_sign {
        *t0_last = Limb::from(limbs_slice_add_same_length_in_place_left(t0_init, ys00));
    } else if *t0_last != 0 {
        if limbs_sub_same_length_in_place_left(t0_init, ys00) {
            *t0_last -= 1;
        }
    } else {
        t0_sign = limbs_sub_abs_same_length_in_place_left(t0_init, ys00);
    }
    // u6 = s6 * t4
    limbs_mul_to_out(u0, xs10_lo, t0, &mut mul_scratch);
    assert!(u0[sum_len] < 2);
    let (xs01_lo_last, xs01_lo_init) = xs01_lo.split_last_mut().unwrap();
    if x01_sign {
        assert!(!limbs_sub_same_length_in_place_right(xs10_lo, xs01_lo_init));
    } else if limbs_slice_add_same_length_in_place_left(xs01_lo_init, xs10_lo) {
        *xs01_lo_last += 1;
    }
    t0_sign = limbs_add_signed_same_length_to_out(xs10, xs11, x11_sign, u0, t0_sign);
    // u3 + u5 + u6
    assert!(xs10[sum_len] < 4);
    x11_sign =
        limbs_add_signed_same_length_in_place_left(xs11, x11_sign, &u1[..=sum_len], u1_sign);
    // -u2 + u3 + u5
    assert!(xs11[sum_len] < 3);
    // u4 = s4 * t5
    limbs_mul_to_out(u0, s0, ys01, &mut mul_scratch);
    assert!(u0[sum_len] < 2);
    t0[ys_len] = Limb::from(limbs_add_same_length_to_out(t0, ys11, ys01));
    // u1 = s1 * t1
    limbs_mul_to_out(u1, xs01_lo, t0, &mut mul_scratch);
    assert!(u1[sum_len] < 4);
    let (u1_last, u1_init) = u1.split_last_mut().unwrap();
    assert_eq!(*u1_last, 0);
    limbs_add_signed_same_length_to_out(xs01, xs11, x11_sign, u0, s0_sign);
    // -u2 + u3 - u4 + u5
    assert!(xs01[sum_len] < 2);
    if x11_sign {
        assert!(!limbs_slice_add_same_length_in_place_left(xs11, u1_init));
    } else {
        // u1 + u2 - u3 - u5
        assert!(!limbs_sub_same_length_in_place_right(u1_init, xs11));
    }
    assert!(xs11[sum_len] < 2);
    if t0_sign {
        assert!(!limbs_slice_add_same_length_in_place_left(xs10, u1_init));
    } else {
        // u1 - u3 - u5 - u6
        assert!(!limbs_sub_same_length_in_place_right(u1_init, xs10));
    }
    assert!(xs10[sum_len] < 2);
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs_len, ys00.len())`.
//
// This is equivalent to `mpn_matrix22_mul` from `mpn/generic/matrix22_mul.c`, GMP 6.2.1.
pub_crate_test! {limbs_matrix_2_2_mul(
    xs00: &mut [Limb],
    xs01: &mut [Limb],
    xs10: &mut [Limb],
    xs11: &mut [Limb],
    xs_len: usize,
    ys00: &[Limb],
    ys01: &[Limb],
    ys10: &[Limb],
    ys11: &[Limb],
    scratch: &mut [Limb],
) {
    let ys_len = ys00.len();
    assert_eq!(ys01.len(), ys_len);
    assert_eq!(ys10.len(), ys_len);
    assert_eq!(ys11.len(), ys_len);
    if xs_len < MATRIX22_STRASSEN_THRESHOLD || ys_len < MATRIX22_STRASSEN_THRESHOLD {
        limbs_matrix_2_2_mul_small(
            xs00, xs01, xs10, xs11, xs_len, ys00, ys01, ys10, ys11, scratch,
        );
    } else {
        limbs_matrix_2_2_mul_strassen(
            xs00, xs01, xs10, xs11, xs_len, ys00, ys01, ys10, ys11, scratch,
        );
    }
}}
