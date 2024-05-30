// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2019 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_to_out_aliased, limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::div_mod::{
    limbs_div_limb_to_out_mod, limbs_div_mod_qs_to_out_rs_to_ns,
};
use crate::natural::arithmetic::gcd::matrix_2_2::limbs_matrix_2_2_mul;
use crate::natural::arithmetic::mul::limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use crate::natural::arithmetic::mul::mul_mod::{
    limbs_mul_mod_base_pow_n_minus_1, limbs_mul_mod_base_pow_n_minus_1_next_size,
    limbs_mul_mod_base_pow_n_minus_1_scratch_len,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len, limbs_mul_to_out,
    limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
};
use crate::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::platform::{DoubleLimb, Limb};
use core::cmp::{max, min, Ordering::*};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DivMod, Gcd, Parity, WrappingAddAssign, XMulYToZZ, XXDivModYToQR, XXSubYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves, SplitInHalf, WrappingFrom};
use malachite_base::num::logic::traits::{LeadingZeros, NotAssign, TrailingZeros};
use malachite_base::slices::{slice_set_zero, slice_test_zero, slice_trailing_zeros};

pub(crate) trait GcdSubdivideStepContext {
    fn gcd_subdiv_step_hook(
        &mut self,
        g: Option<&[Limb]>,
        q: Option<&mut [Limb]>,
        q_len: usize,
        d: i8,
    );

    fn gcd_subdiv_step_hook_with_1(&mut self, d: i8) {
        self.gcd_subdiv_step_hook(None, Some(&mut [1]), 1, d);
    }
}

/// This is equivalent to `gcd_ctx` from `mpn/gcd.c`, GMP 6.2.1.
struct GcdContext<'a>(&'a mut [Limb]);

impl<'a> GcdSubdivideStepContext for GcdContext<'a> {
    /// This is equivalent to `gcd_hook` from `mpn/gcd.c`, GMP 6.2.1.
    fn gcd_subdiv_step_hook(
        &mut self,
        g: Option<&[Limb]>,
        _q: Option<&mut [Limb]>,
        _q_len: usize,
        _d: i8,
    ) {
        if let Some(g) = g {
            self.0[..g.len()].copy_from_slice(g);
        }
    }
}

#[cfg(feature = "test_build")]
pub struct HalfGcdMatrix<'a> {
    pub(crate) data: &'a mut [Limb],
    pub(crate) s: usize,
    pub(crate) two_s: usize,
    pub(crate) three_s: usize,
    pub(crate) n: usize,
}

#[cfg(not(feature = "test_build"))]
pub(crate) struct HalfGcdMatrix<'a> {
    data: &'a mut [Limb],
    pub(crate) s: usize,
    two_s: usize,
    three_s: usize,
    pub(crate) n: usize,
}

impl<'a> HalfGcdMatrix<'a> {
    // # Worst-case complexity
    // Constant time and additional memory.
    pub_crate_test! {get(&self, row: u8, column: u8) -> &[Limb] {
        match (row, column) {
            (0, 0) => &self.data[..self.s],
            (0, 1) => &self.data[self.s..self.two_s],
            (1, 0) => &self.data[self.two_s..self.three_s],
            (1, 1) => &self.data[self.three_s..],
            _ => panic!(),
        }
    }}

    // # Worst-case complexity
    // Constant time and additional memory.
    pub_test! {get_mut(&mut self, row: u8, column: u8) -> &mut [Limb] {
        match (row, column) {
            (0, 0) => &mut self.data[..self.s],
            (0, 1) => &mut self.data[self.s..self.two_s],
            (1, 0) => &mut self.data[self.two_s..self.three_s],
            (1, 1) => &mut self.data[self.three_s..],
            _ => panic!(),
        }
    }}

    // # Worst-case complexity
    // Constant time and additional memory.
    #[inline]
    fn get_two_mut(
        &mut self,
        row_1: u8,
        column_1: u8,
        row_2: u8,
        column_2: u8,
    ) -> (&mut [Limb], &mut [Limb]) {
        match (row_1, column_1, row_2, column_2) {
            (0, 0, 0, 1) => self.data[..self.two_s].split_at_mut(self.s),
            (0, 1, 0, 0) => {
                let (xs, ys) = self.data[..self.two_s].split_at_mut(self.s);
                (ys, xs)
            }
            (1, 0, 1, 1) => self.data[self.two_s..].split_at_mut(self.s),
            (1, 1, 1, 0) => {
                let (xs, ys) = self.data[self.two_s..].split_at_mut(self.s);
                (ys, xs)
            }
            _ => panic!(),
        }
    }

    // # Worst-case complexity
    // Constant time and additional memory.
    #[inline]
    pub(crate) fn get_four(&mut self) -> (&[Limb], &[Limb], &[Limb], &[Limb]) {
        split_into_chunks!(self.data, self.s, [x00, x01, x10], x11);
        (x00, x01, x10, x11)
    }

    // # Worst-case complexity
    // Constant time and additional memory.
    #[inline]
    fn get_four_mut(&mut self) -> (&mut [Limb], &mut [Limb], &mut [Limb], &mut [Limb]) {
        split_into_chunks_mut!(self.data, self.s, [x00, x01, x10], x11);
        (x00, x01, x10, x11)
    }

    // # Worst-case complexity
    // Constant time and additional memory.
    pub_const_test! {min_init_scratch(n: usize) -> usize {
        (((n + 1) >> 1) + 1) << 2
    }}

    // For input of size n, matrix elements are of size at most ceil(n / 2) - 1, but we need two
    // limbs extra.
    //
    // # Worst-case complexity
    // $T(n) = O(n)$
    //
    // $M(n) = O(1)$
    //
    // where $T$ is time, $M$ is additional memory, and $n$ is `p.len()`.
    //
    // This is equivalent to `mpn_hgcd_matrix_init` from `mpn/generic/hgcd_matrix.c`, GMP 6.2.1,
    // where the matrix is returned.
    pub_crate_test! {init(n: usize, p: &mut [Limb]) -> HalfGcdMatrix {
        let s = (n + 1) / 2 + 1;
        let two_s = s << 1;
        let three_s = two_s + s;
        slice_set_zero(&mut p[..s << 2]);
        let mut m = HalfGcdMatrix {
            data: p,
            s,
            two_s,
            three_s,
            n: 1,
        };
        m.get_mut(0, 0)[0] = 1;
        m.get_mut(1, 1)[0] = 1;
        m
    }}

    // # Worst-case complexity
    // Constant time and additional memory.
    pub_const_test! {update_q_scratch_len(&self, qs_len: usize) -> usize {
        self.n + qs_len
    }}

    // # Worst-case complexity
    // Constant time and additional memory.
    fn all_elements_zero_at_index(&self, i: usize) -> bool {
        self.get(0, 0)[i] == 0
            && self.get(0, 1)[i] == 0
            && self.get(1, 0)[i] == 0
            && self.get(1, 1)[i] == 0
    }
}

// Multiply M by M1 from the right. Needs 3*(M->n + M1->n) + 5 limbs of temporary storage (see
// mpn_matrix22_mul_itch).
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(a.n, b.n)`.
//
// This is equivalent to `mpn_hgcd_matrix_mul` from `mpn/generic/hgcd_matrix.c`, GMP 6.2.1.
pub_crate_test! {limbs_half_gcd_matrix_mul_matrix(
    a: &mut HalfGcdMatrix,
    b: &HalfGcdMatrix,
    scratch: &mut [Limb]
) {
    // About the new size of M:s elements. Since M1's diagonal elements are > 0, no element can
    // decrease. The new elements are of size M->n + M1->n, one limb more or less. The computation
    // of the matrix product produces elements of size M->n + M1->n + 1. But the true size, after
    // normalization, may be three limbs smaller.
    //
    // The reason that the product has normalized size >= M->n + M1->n - 2 is subtle. It depends on
    // the fact that M and M1 can be factored as products of (1,1; 0,1) and (1,0; 1,1), and that we
    // can't have M ending with a large power and M1 starting with a large power of the same matrix.
    assert!(a.n + b.n < a.s);
    assert!(!a.all_elements_zero_at_index(a.n - 1));
    let b_n = b.n;
    assert!(!b.all_elements_zero_at_index(b_n - 1));
    let n = a.n;
    let (x00, x01, x10, x11) = a.get_four_mut();
    limbs_matrix_2_2_mul(
        x00,
        x01,
        x10,
        x11,
        n,
        &b.get(0, 0)[..b_n],
        &b.get(0, 1)[..b_n],
        &b.get(1, 0)[..b_n],
        &b.get(1, 1)[..b_n],
        scratch,
    );
    // Index of last potentially non-zero limb, size is one greater.
    let mut n = a.n + b_n;
    for _ in 0..3 {
        if a.all_elements_zero_at_index(n) {
            n -= 1;
        }
    }
    assert!(!a.all_elements_zero_at_index(n));
    a.n = n + 1;
}}

// Multiply M by M1 from the right. Since the M1 elements fit in Limb::WIDTH - 1 bits, M grows by at
// most one limb. Needs temporary space M->n
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `a.n`.
//
// This is equivalent to `mpn_hgcd_matrix_mul_1` from `mpn/generic/hgcd_matrix.c`, GMP 6.2.1.
pub_crate_test! {limbs_half_gcd_matrix_mul_matrix_1(
    a: &mut HalfGcdMatrix,
    b: &HalfGcdMatrix1,
    scratch: &mut [Limb],
) {
    let n = a.n;
    let scratch = &mut scratch[..n];
    scratch.copy_from_slice(&a.get(0, 0)[..n]);
    let (a_0_0, a_0_1) = a.get_two_mut(0, 0, 0, 1);
    let n0 = limbs_half_gcd_matrix_1_mul_vector(b, a_0_0, scratch, a_0_1);
    scratch.copy_from_slice(&a.get(1, 0)[..n]);
    let (a_1_0, a_1_1) = a.get_two_mut(1, 0, 1, 1);
    let n1 = limbs_half_gcd_matrix_1_mul_vector(b, a_1_0, scratch, a_1_1);
    a.n = max(n0, n1);
    assert!(a.n <= a.s);
}}

// Update column `column`, adding in Q * column (1-`col`). Temporary storage: qn + n <= `self.s`,
// where n is the size of the largest element in column 1 - `column`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `m.n`.
//
// This is equivalent to `mpn_hgcd_matrix_update_q` from `mpn/generic/hgcd_matrix.c`, GMP 6.2.1.
pub_crate_test! {limbs_half_gcd_matrix_update_q(
    m: &mut HalfGcdMatrix,
    qs: &[Limb],
    column: u8,
    scratch: &mut [Limb],
) {
    let qs_len = qs.len();
    assert!(qs_len + m.n <= m.s);
    assert!(column < 2);
    if qs_len == 1 {
        let q = qs[0];
        let n = m.n;
        let (m_0_a, m_0_b) = m.get_two_mut(0, column, 0, 1 - column);
        let carry_0 =
            limbs_slice_add_mul_limb_same_length_in_place_left(&mut m_0_a[..n], &m_0_b[..n], q);
        let (m_1_a, m_1_b) = m.get_two_mut(1, column, 1, 1 - column);
        let carry_1 =
            limbs_slice_add_mul_limb_same_length_in_place_left(&mut m_1_a[..n], &m_1_b[..n], q);
        m.get_mut(0, column)[n] = carry_0;
        m.get_mut(1, column)[n] = carry_1;
        if carry_0 != 0 || carry_1 != 0 {
            m.n += 1;
        }
    } else {
        // Carries for the unlikely case that we get both high words from the multiplication and
        // carries from the addition.
        let mut carries = [0; 2];
        // The matrix will not necessarily grow in size by qn, so we need normalization in order not
        // to overflow m.
        let mut n = m.n;
        while n + qs_len > m.n {
            assert_ne!(n, 0);
            if m.get(0, 1 - column)[n - 1] > 0 || m.get(1, 1 - column)[n - 1] > 0 {
                break;
            }
            n -= 1;
        }
        assert!(qs_len + n <= m.s);
        if n != 0 {
            let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(n, qs_len)];
            for row in 0..2 {
                limbs_mul_to_out(
                    scratch,
                    &m.get(row, 1 - column)[..n],
                    &qs[..qs_len],
                    &mut mul_scratch,
                );
                assert!(n + qs_len >= m.n);
                let m_n = m.n;
                if limbs_add_to_out_aliased(m.get_mut(row, column), m_n, &scratch[..n + qs_len]) {
                    carries[usize::wrapping_from(row)] = 1;
                }
            }
        }
        n += qs_len;
        if carries[0] != 0 || carries[1] != 0 {
            m.get_mut(0, column)[n] = carries[0];
            m.get_mut(1, column)[n] = carries[1];
            n += 1;
        } else if m.get(0, column)[n - 1] == 0 && m.get(1, column)[n - 1] == 0 {
            n -= 1;
        }
        m.n = n;
    }
    assert!(m.n <= m.s);
}}

// - Multiplies the least significant p limbs of (X;Y) by M^-1.
// - Temporary space needed: 2 * (p + m.n)
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpn_hgcd_matrix_adjust` from `mpn/generic/hgcd_matrix.c`, GMP 6.2.1.
pub(crate) fn limbs_half_gcd_matrix_adjust(
    m: &HalfGcdMatrix,
    mut n: usize,
    xs: &mut [Limb],
    ys: &mut [Limb],
    p: usize,
    scratch: &mut [Limb],
) -> usize {
    // M^-1 (X; Y) = (r11, -r01; -r10, r00) (a ; b) = (r11 x - r01 y; - r10 x + r00 y)
    let xs_init = &mut xs[..n];
    let ys_init = &mut ys[..n];
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(p + m.n);
    assert!(p + m.n < n);
    let (xs_lo, xs_hi) = xs_init.split_at_mut(p);
    // First compute the two values depending on X, before overwriting X
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(m.n, p)];
    limbs_mul_to_out(scratch_lo, &m.get(1, 1)[..m.n], xs_lo, &mut mul_scratch);
    limbs_mul_to_out(scratch_hi, &m.get(1, 0)[..m.n], xs_lo, &mut mul_scratch);
    // Update X
    let (scratch_lo_lo, scratch_lo_hi) = scratch_lo.split_at(p);
    xs_lo.copy_from_slice(scratch_lo_lo);
    let mut x_high = limbs_slice_add_greater_in_place_left(xs_hi, scratch_lo_hi);
    let (ys_lo, ys_hi) = ys_init.split_at_mut(p);
    limbs_mul_to_out(scratch_lo, &m.get(0, 1)[..m.n], ys_lo, &mut mul_scratch);
    if limbs_sub_greater_in_place_left(xs_init, scratch_lo) {
        assert!(x_high);
        x_high = false;
    }
    // Update Y
    limbs_mul_to_out(scratch_lo, &m.get(0, 0)[..m.n], ys_lo, &mut mul_scratch);
    let (scratch_lo_lo, scratch_lo_hi) = scratch_lo.split_at(p);
    ys_lo.copy_from_slice(scratch_lo_lo);
    let mut y_high = limbs_slice_add_greater_in_place_left(ys_hi, scratch_lo_hi);
    if limbs_sub_greater_in_place_left(ys_init, &scratch_hi[..p + m.n]) {
        assert!(y_high);
        y_high = false;
    }
    if x_high || y_high {
        xs[n] = Limb::from(x_high);
        ys[n] = Limb::from(y_high);
        n += 1;
    } else {
        // The subtraction can reduce the size by at most one limb.
        if xs[n - 1] == 0 && ys[n - 1] == 0 {
            n -= 1;
        }
    }
    assert!(xs[n - 1] != 0 || ys[n - 1] != 0);
    n
}

// Computes (x, y) <- M^(-1) (x; y)
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `m.len()`.
//
// This is equivalent to `mpn_hgcd_matrix_apply` from `mpn/generic/hgcd_reduce.c`, GMP 6.2.1.
fn limbs_half_gcd_matrix_apply(m: &HalfGcdMatrix, xs: &mut [Limb], ys: &mut [Limb]) -> usize {
    let mut n = xs.len();
    assert_eq!(ys.len(), n);
    assert!(xs[n - 1] != 0 || ys[n - 1] != 0);
    let xs_len = n - slice_trailing_zeros(&xs[..n]);
    let ys_len = n - slice_trailing_zeros(&ys[..n]);
    let xs_lo = &mut xs[..xs_len];
    let ys_lo = &mut ys[..ys_len];
    let mut m_lens = [[0usize; 2]; 2];
    for (i, row) in m_lens.iter_mut().enumerate() {
        for (j, len) in row.iter_mut().enumerate() {
            *len = m.n - slice_trailing_zeros(&m.get(i as u8, j as u8)[..m.n]);
        }
    }
    assert_ne!(m_lens[0][0], 0);
    assert_ne!(m_lens[1][1], 0);
    assert!(m_lens[0][1] != 0 || m_lens[1][0] != 0);
    if m_lens[0][1] == 0 {
        // X unchanged, M = (1, 0; q, 1)
        assert_eq!(m_lens[0][0], 1);
        assert_eq!(m.get(0, 0)[0], 1);
        assert_eq!(m_lens[1][1], 1);
        assert_eq!(m.get(1, 1)[0], 1);
        // Put Y <- Y - q X
        limbs_gcd_sub_mul(ys_lo, xs_lo, &m.get(1, 0)[..m_lens[1][0]])
    } else if m_lens[1][0] == 0 {
        fail_on_untested_path("limbs_half_gcd_matrix_apply, m_lens[1][0] == 0");
        // Y unchanged, M = (1, q; 0, 1)
        assert_eq!(m_lens[0][0], 1);
        assert_eq!(m.get(0, 0)[0], 1);
        assert_eq!(m_lens[1][1], 1);
        assert_eq!(m.get(1, 1)[0], 1);
        // Put X <- X - q * Y
        limbs_gcd_sub_mul(xs_lo, ys_lo, &m.get(0, 1)[..m_lens[0][1]])
    } else {
        // - X = m00 x + m01 y => x <= X / m00, y <= X / m01.
        // - Y = m10 x + m11 y => x <= Y / m10, y <= Y / m11.
        let mut new_n = max(
            min(xs_len - m_lens[0][0], ys_len - m_lens[1][0]),
            min(xs_len - m_lens[0][1], ys_len - m_lens[1][1]),
        ) + 1;
        // In the range of interest, mulmod_bnm1 should always beat mullo.
        let mod_n = limbs_mul_mod_base_pow_n_minus_1_next_size(new_n + 1);
        let mut big_scratch =
            vec![0; (mod_n << 1) + limbs_mul_mod_base_pow_n_minus_1_scratch_len(mod_n, mod_n, m.n)];
        split_into_chunks_mut!(big_scratch, mod_n, [scratch, scratch_lo], scratch_hi);
        assert!(n <= mod_n << 1);
        if n > mod_n {
            let (xs_lo, xs_hi) = xs.split_at_mut(mod_n);
            if limbs_slice_add_greater_in_place_left(xs_lo, xs_hi) {
                assert!(!limbs_slice_add_limb_in_place(xs, 1));
            }
            let (ys_lo, ys_hi) = ys.split_at_mut(mod_n);
            if limbs_slice_add_greater_in_place_left(ys_lo, ys_hi) {
                assert!(!limbs_slice_add_limb_in_place(ys, 1));
            }
            n = mod_n;
        }
        let xs = &mut xs[..n];
        let ys = &mut ys[..n];
        limbs_mul_mod_base_pow_n_minus_1(
            scratch,
            mod_n,
            xs,
            &m.get(1, 1)[..m_lens[1][1]],
            scratch_hi,
        );
        limbs_mul_mod_base_pow_n_minus_1(
            scratch_lo,
            mod_n,
            ys,
            &m.get(0, 1)[..m_lens[0][1]],
            scratch_hi,
        );
        if n + m_lens[1][1] < mod_n {
            slice_set_zero(&mut scratch[n + m_lens[1][1]..]);
        }
        if n + m_lens[0][1] < mod_n {
            slice_set_zero(&mut scratch_lo[n + m_lens[0][1]..]);
        }
        if limbs_sub_same_length_in_place_left(scratch, scratch_lo) {
            assert!(!limbs_sub_limb_in_place(scratch, 1));
        }
        let (scratch_0, scratch_1) = scratch.split_at(new_n);
        assert!(slice_test_zero(scratch_1));
        limbs_mul_mod_base_pow_n_minus_1(
            scratch_lo,
            mod_n,
            xs,
            &m.get(1, 0)[..m_lens[1][0]],
            scratch_hi,
        );
        xs[..new_n].copy_from_slice(scratch_0);
        limbs_mul_mod_base_pow_n_minus_1(
            scratch,
            mod_n,
            ys,
            &m.get(0, 0)[..m_lens[0][0]],
            scratch_hi,
        );
        if n + m_lens[1][0] < mod_n {
            slice_set_zero(&mut scratch_lo[n + m_lens[1][0]..]);
        }
        if n + m_lens[0][0] < mod_n {
            slice_set_zero(&mut scratch[n + m_lens[0][0]..]);
        }
        if limbs_sub_same_length_in_place_left(scratch, scratch_lo) {
            assert!(!limbs_sub_limb_in_place(scratch, 1));
        }
        let (scratch_0, scratch_1) = scratch.split_at(new_n);
        assert!(slice_test_zero(scratch_1));
        ys[..new_n].copy_from_slice(scratch_0);
        while xs[new_n - 1] | ys[new_n - 1] == 0 {
            new_n -= 1;
            assert_ne!(new_n, 0);
        }
        new_n
    }
}

/// This is equivalent to `mpn_hgcd_reduce` from `mpn/generic/hgcd_reduce.c`, GMP 6.2.1.
fn limbs_half_gcd_matrix_reduce(
    m: &mut HalfGcdMatrix,
    xs: &mut [Limb],
    ys: &mut [Limb],
    p: usize,
    scratch: &mut [Limb],
) -> usize {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    let com_p = n - p;
    if n < HGCD_REDUCE_THRESHOLD {
        let new_n = limbs_half_gcd(&mut xs[p..], &mut ys[p..], m, scratch);
        if new_n == 0 {
            0
        } else {
            // Needs 2 * (p + m.n) <= 2 * (floor(n / 2) + ceil(n / 2) - 1) = 2 (n - 1)
            limbs_half_gcd_matrix_adjust(m, p + new_n, xs, ys, p, scratch)
        }
    } else {
        split_into_chunks_mut!(scratch, com_p, [scratch_0, scratch_1], scratch_2);
        scratch_0.copy_from_slice(&xs[p..]);
        scratch_1.copy_from_slice(&ys[p..]);
        if limbs_half_gcd_approx(scratch_0, scratch_1, m, scratch_2) {
            limbs_half_gcd_matrix_apply(m, xs, ys)
        } else {
            0
        }
    }
}

// Computes R -= X * Y. Result must be non-negative. Normalized down to size xs_len, and resulting
// size is returned.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `submul` from `mpn/generic/hgcd_reduce.c`, GMP 6.2.1.
fn limbs_gcd_sub_mul(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> usize {
    let mut out_len = out.len();
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(out_len >= xs_len);
    let sum_len = xs_len + ys_len;
    assert!(sum_len <= out_len + 1);
    let mut scratch = vec![0; sum_len];
    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs_len, ys_len)];
    limbs_mul_greater_to_out(&mut scratch, xs, ys, &mut mul_scratch);
    assert!(sum_len <= out_len || scratch[out_len] == 0);
    let mut scratch_len = sum_len;
    if scratch_len > out_len {
        scratch_len -= 1;
    }
    assert!(!limbs_sub_greater_in_place_left(
        out,
        &scratch[..scratch_len]
    ));
    while out_len > xs_len && out[out_len - 1] == 0 {
        out_len -= 1;
    }
    out_len
}

#[cfg(feature = "test_build")]
#[derive(Clone, Debug, Default)]
pub struct HalfGcdMatrix1 {
    pub data: [[Limb; 2]; 2],
}

#[cfg(not(feature = "test_build"))]
#[derive(Default)]
pub(crate) struct HalfGcdMatrix1 {
    pub(crate) data: [[Limb; 2]; 2],
}

// Sets (r;b) = (a;b) M, with M = (u00, u01; u10, u11). Vector must have space for n + 1 limbs. Uses
// three buffers to avoid a copy
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `m.n`.
//
// This is equivalent to `mpn_hgcd_mul_matrix1_vector` from `mpn/generic/hgcd2.c`, GMP 6.2.1.
pub_crate_test! {limbs_half_gcd_matrix_1_mul_vector(
    m: &HalfGcdMatrix1,
    out: &mut [Limb],
    xs: &[Limb],
    ys: &mut [Limb],
) -> usize {
    let n = xs.len();
    assert!(ys.len() > n);
    assert!(out.len() > n);
    let (out_lo, out_hi) = out.split_at_mut(n);
    let (ys_lo, ys_hi) = ys.split_at_mut(n);
    let mut x_high = limbs_mul_limb_to_out(out_lo, xs, m.data[0][0]);
    x_high.wrapping_add_assign(
        limbs_slice_add_mul_limb_same_length_in_place_left(out_lo, ys_lo, m.data[1][0])
    );
    let mut y_high = limbs_slice_mul_limb_in_place(ys_lo, m.data[1][1]);
    y_high.wrapping_add_assign(
        limbs_slice_add_mul_limb_same_length_in_place_left(ys_lo, xs, m.data[0][1])
    );
    out_hi[0] = x_high;
    ys_hi[0] = y_high;
    if x_high == 0 && y_high == 0 {
        n
    } else {
        n + 1
    }
}}

// Compute (r;y) <- (u11 x - u01 y; -u10 x + u00 y) xs
// ```
// r  = u11 * x
// r -= u01 * y
// y *= u00
// y -= u10 * x
// ```
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `m.len()`.
//
// This is equivalent to `mpn_matrix22_mul1_inverse_vector` from
// `mpn/generic/matrix22_mul1_inverse_vector.c`, GMP 6.2.1.
pub(crate) fn limbs_half_gcd_matrix_1_mul_inverse_vector(
    m: &HalfGcdMatrix1,
    out: &mut [Limb],
    xs: &[Limb],
    ys: &mut [Limb],
) -> usize {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert_eq!(out.len(), n);
    let h0 = limbs_mul_limb_to_out(out, xs, m.data[1][1]);
    let h1 = limbs_sub_mul_limb_same_length_in_place_left(out, ys, m.data[0][1]);
    assert_eq!(h0, h1);
    let h0 = limbs_slice_mul_limb_in_place(ys, m.data[0][0]);
    let h1 = limbs_sub_mul_limb_same_length_in_place_left(ys, xs, m.data[1][0]);
    assert_eq!(h0, h1);
    if out[n - 1] == 0 && ys[n - 1] == 0 {
        n - 1
    } else {
        n
    }
}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_gcd_subdiv_step` from `mpn/generic/gcd_subdiv_step.c`, GMP 6.2.1.
pub(crate) fn limbs_gcd_subdivide_step<'a, CTX: GcdSubdivideStepContext>(
    mut xs: &'a mut [Limb],
    mut ys: &'a mut [Limb],
    s: usize,
    context: &mut CTX,
    scratch: &mut [Limb],
) -> usize {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert_ne!(n, 0);
    assert!(xs[n - 1] > 0 || ys[n - 1] > 0);
    let mut xs_len = n - slice_trailing_zeros(xs);
    let mut ys_len = n - slice_trailing_zeros(ys);
    let mut xs_init = &mut xs[..xs_len];
    let mut ys_init = &mut ys[..ys_len];
    let mut swapped = false;
    // Arrange so that x < y, subtract y -= x, and maintain normalization.
    match xs_len.cmp(&ys_len) {
        Equal => {
            match limbs_cmp_same_length(xs_init, ys_init) {
                Equal => {
                    // For gcdext, return the smallest of the two cofactors.
                    if s == 0 {
                        context.gcd_subdiv_step_hook(Some(xs_init), None, 0, -1);
                    }
                    return 0;
                }
                Greater => {
                    swap(&mut xs, &mut ys);
                    xs_init = &mut xs[..xs_len];
                    ys_init = &mut ys[..ys_len];
                    swapped.not_assign();
                }
                _ => {}
            }
        }
        Greater => {
            swap(&mut xs, &mut ys);
            swap(&mut xs_len, &mut ys_len);
            xs_init = &mut xs[..xs_len];
            ys_init = &mut ys[..ys_len];
            swapped.not_assign();
        }
        Less => {}
    }
    if xs_len <= s {
        if s == 0 {
            context.gcd_subdiv_step_hook(Some(ys_init), None, 0, i8::from(!swapped));
        }
        return 0;
    }
    assert!(!limbs_sub_greater_in_place_left(ys_init, xs_init));
    ys_len -= slice_trailing_zeros(ys_init);
    ys_init = &mut ys_init[..ys_len];
    assert_ne!(ys_len, 0);
    if ys_len <= s {
        // Undo subtraction
        if limbs_add_to_out_aliased(ys, ys_len, xs_init) {
            ys[xs_len] = 1;
        }
        return 0;
    }
    // Arrange so that x < y
    match xs_len.cmp(&ys_len) {
        Equal => {
            match limbs_cmp_same_length(xs_init, ys_init) {
                Equal => {
                    fail_on_untested_path("limbs_gcd_subdivide_step, c == Equal");
                    if s != 0 {
                        // Just record subtraction and return
                        context.gcd_subdiv_step_hook_with_1(i8::from(swapped));
                        context.gcd_subdiv_step_hook_with_1(i8::from(swapped));
                    } else {
                        // Found gcd.
                        context.gcd_subdiv_step_hook(Some(ys_init), None, 0, i8::from(swapped));
                        return 0;
                    }
                }
                Greater => {
                    context.gcd_subdiv_step_hook_with_1(i8::from(swapped));
                    swap(&mut xs, &mut ys);
                    xs_init = &mut xs[..xs_len];
                    ys_init = &mut ys[..ys_len];
                    swapped.not_assign();
                }
                Less => {
                    context.gcd_subdiv_step_hook_with_1(i8::from(swapped));
                }
            }
        }
        Greater => {
            context.gcd_subdiv_step_hook_with_1(i8::from(swapped));
            swap(&mut xs, &mut ys);
            swap(&mut xs_len, &mut ys_len);
            xs_init = &mut xs[..xs_len];
            ys_init = &mut ys[..ys_len];
            swapped.not_assign();
        }
        Less => {
            context.gcd_subdiv_step_hook_with_1(i8::from(swapped));
        }
    }
    if xs_len == 1 {
        if ys_init.len() == 1 {
            (scratch[0], ys_init[0]) = ys_init[0].div_mod(xs_init[0]);
        } else {
            ys_init[0] = limbs_div_limb_to_out_mod(scratch, ys_init, xs_init[0]);
        }
    } else {
        limbs_div_mod_qs_to_out_rs_to_ns(scratch, ys_init, xs_init);
    }
    let qn = ys_len - xs_len + 1;
    let ys_len = xs_len - slice_trailing_zeros(&ys_init[..xs_len]);
    if ys_len <= s {
        if s == 0 {
            context.gcd_subdiv_step_hook(Some(xs_init), Some(scratch), qn, i8::from(swapped));
            return 0;
        }
        // Quotient is one too large, so decrement it and add back X.
        if ys_len != 0 {
            if limbs_add_to_out_aliased(ys, ys_len, xs_init) {
                ys[xs_len] = 1;
                xs_len += 1;
            }
        } else {
            ys[..xs_len].copy_from_slice(xs_init);
        }
        assert!(!limbs_sub_limb_in_place(&mut scratch[..qn], 1));
    }
    context.gcd_subdiv_step_hook(None, Some(scratch), qn, i8::from(swapped));
    xs_len
}

impl<'a> GcdSubdivideStepContext for HalfGcdMatrix<'a> {
    // # Worst-case complexity
    // $T(n) = O(n)$
    //
    // $M(n) = O(1)$
    //
    // where $T$ is time, $M$ is additional memory, and $n$ is `q_len`.
    //
    // This is equivalent to `hgcd_hook` from `mpn/generic/hgcd_step.c`, GMP 6.2.1.
    fn gcd_subdiv_step_hook(
        &mut self,
        g: Option<&[Limb]>,
        q: Option<&mut [Limb]>,
        mut q_len: usize,
        d: i8,
    ) {
        assert!(g.is_none());
        let q = q.unwrap();
        q_len -= slice_trailing_zeros(&q[..q_len]);
        if q_len != 0 {
            let (q, scratch) = q.split_at_mut(q_len);
            limbs_half_gcd_matrix_update_q(self, q, u8::exact_from(d), scratch);
        }
    }
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `MPN_EXTRACT_NUMB` from `gmp-impl.h`, GMP 6.2.1.
pub(crate) const fn extract_number(count: u64, x1: Limb, x0: Limb) -> Limb {
    (x1 << count) | (x0 >> (Limb::WIDTH - count))
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `div2` from `mpn/generic/hgcd2.c`, GMP 6.2.1, where `HGCD2_DIV2_METHOD ==
// 1`.
pub_crate_test! {limbs_gcd_div(
    mut n1: Limb,
    mut n0: Limb,
    mut d1: Limb,
    mut d0: Limb
) -> (Limb, Limb, Limb) {
    let (mut q, r) = n1.div_mod(d1);
    if q > d1 {
        // Normalize
        let c = LeadingZeros::leading_zeros(d1);
        let width_comp = Limb::WIDTH - c;
        assert_ne!(c, 0);
        let n2 = n1 >> width_comp;
        n1 = (n1 << c) | (n0 >> width_comp);
        n0 <<= c;
        d1 = (d1 << c) | (d0 >> width_comp);
        d0 <<= c;
        (q, n1) = Limb::xx_div_mod_y_to_qr(n2, n1, d1);
        let (mut t1, mut t0) = Limb::x_mul_y_to_zz(q, d0);
        if t1 > n1 || t1 == n1 && t0 > n0 {
            assert_ne!(q, 0);
            q -= 1;
            (t1, t0) = Limb::xx_sub_yy_to_zz(t1, t0, d1, d0);
        }
        (n1, n0) = Limb::xx_sub_yy_to_zz(n1, n0, t1, t0);
        // Undo normalization
        (q, n1 >> c, (n0 >> c) | (n1 << width_comp))
    } else {
        n1 = r;
        let (mut t1, mut t0) = Limb::x_mul_y_to_zz(q, d0);
        if t1 >= n1 && (t1 > n1 || t0 > n0) {
            assert_ne!(q, 0);
            q -= 1;
            (t1, t0) = Limb::xx_sub_yy_to_zz(t1, t0, d1, d0);
        }
        let (r1, r0) = Limb::xx_sub_yy_to_zz(n1, n0, t1, t0);
        (q, r1, r0)
    }
}}

// Reduces a, b until |a - b| (almost) fits in one limb + 1 bit. Constructs matrix M. Returns 1 if
// we make progress, i.e. can perform at least one subtraction. Otherwise returns zero.
//
// This is equivalent to `mpn_hgcd2` from `mpn/generic/hgcd2.c`, GMP 6.2.1.
pub(crate) fn limbs_half_gcd_2(
    mut x_high: Limb,
    mut a_low: Limb,
    mut y_high: Limb,
    mut b_low: Limb,
    m: &mut HalfGcdMatrix1,
) -> bool {
    if x_high < 2 || y_high < 2 {
        return false;
    }
    let mut m01;
    let mut m10;
    if x_high > y_high || x_high == y_high && a_low > b_low {
        (x_high, a_low) = Limb::xx_sub_yy_to_zz(x_high, a_low, y_high, b_low);
        if x_high < 2 {
            return false;
        }
        m01 = 1;
        m10 = 0;
    } else {
        (y_high, b_low) = Limb::xx_sub_yy_to_zz(y_high, b_low, x_high, a_low);
        if y_high < 2 {
            return false;
        }
        m01 = 0;
        m10 = 1;
    }
    let mut m00 = 1;
    let mut m11 = 1;
    const HALF_WIDTH: u64 = Limb::WIDTH >> 1;
    const HALF_LIMIT_1: Limb = 1 << HALF_WIDTH;
    let mut subtract_a = x_high < y_high;
    let mut subtract_a1 = false;
    let mut done = false;
    loop {
        if subtract_a {
            subtract_a = false;
        } else {
            assert!(x_high >= y_high);
            if x_high == y_high {
                done = true;
                break;
            }
            if x_high < HALF_LIMIT_1 {
                x_high = (x_high << HALF_WIDTH) + (a_low >> HALF_WIDTH);
                y_high = (y_high << HALF_WIDTH) + (b_low >> HALF_WIDTH);
                break;
            }
            // Subtract a -= q * b, and multiply M from the right by (1 q ; 0 1), affecting the
            // second column of M.
            assert!(x_high > y_high);
            (x_high, a_low) = Limb::xx_sub_yy_to_zz(x_high, a_low, y_high, b_low);
            if x_high < 2 {
                done = true;
                break;
            }
            if x_high <= y_high {
                // Use q = 1.
                m01 += m00;
                m11 += m10;
            } else {
                let mut q;
                (q, x_high, a_low) = limbs_gcd_div(x_high, a_low, y_high, b_low);
                if x_high < 2 {
                    // A is too small, but q is correct.
                    m01 += q * m00;
                    m11 += q * m10;
                    done = true;
                    break;
                }
                q += 1;
                m01 += q * m00;
                m11 += q * m10;
            }
        }
        assert!(y_high >= x_high);
        if x_high == y_high {
            done = true;
            break;
        }
        if y_high < HALF_LIMIT_1 {
            x_high = (x_high << HALF_WIDTH) + (a_low >> HALF_WIDTH);
            y_high = (y_high << HALF_WIDTH) + (b_low >> HALF_WIDTH);
            subtract_a1 = true;
            break;
        }
        // Subtract b -= q * a, and multiply M from the right by (1 0 ; q 1), affecting the first
        // column of M.
        (y_high, b_low) = Limb::xx_sub_yy_to_zz(y_high, b_low, x_high, a_low);
        if y_high < 2 {
            done = true;
            break;
        }
        if x_high >= y_high {
            // Use q = 1.
            m00 += m01;
            m10 += m11;
        } else {
            let mut q;
            (q, y_high, b_low) = limbs_gcd_div(y_high, b_low, x_high, a_low);
            if y_high < 2 {
                // B is too small, but q is correct.
                m00 += q * m01;
                m10 += q * m11;
                done = true;
                break;
            }
            q += 1;
            m00 += q * m01;
            m10 += q * m11;
        }
    }
    // Since we discard the least significant half limb, we don't get a truly maximal M
    // corresponding to |a - b| < 2 ^ (W + 1)).
    if !done {
        const HALF_LIMIT_2: Limb = 1 << (HALF_WIDTH + 1);
        loop {
            if subtract_a1 {
                subtract_a1 = false;
            } else {
                assert!(x_high >= y_high);
                x_high -= y_high;
                if x_high < HALF_LIMIT_2 {
                    break;
                }
                if x_high <= y_high {
                    // Use q = 1.
                    m01 += m00;
                    m11 += m10;
                } else {
                    let mut q;
                    (q, x_high) = x_high.div_mod(y_high);
                    if x_high < HALF_LIMIT_2 {
                        // A is too small, but q is correct.
                        m01 += q * m00;
                        m11 += q * m10;
                        break;
                    }
                    q += 1;
                    m01 += q * m00;
                    m11 += q * m10;
                }
            }
            assert!(y_high >= x_high);
            y_high -= x_high;
            if y_high < HALF_LIMIT_2 {
                break;
            }
            if x_high >= y_high {
                // Use q = 1.
                m00 += m01;
                m10 += m11;
            } else {
                let mut q;
                (q, y_high) = y_high.div_mod(x_high);
                if y_high < HALF_LIMIT_2 {
                    // B is too small, but q is correct.
                    m00 += q * m01;
                    m10 += q * m11;
                    break;
                }
                q += 1;
                m00 += q * m01;
                m10 += q * m11;
            }
        }
    }
    m.data[0][0] = m00;
    m.data[0][1] = m01;
    m.data[1][0] = m10;
    m.data[1][1] = m11;
    true
}

/// This is equivalent to `mpn_hgcd_step` from `mpn/generic/hgcd_step.c`, GMP 6.2.1.
fn limbs_half_gcd_step(
    xs: &mut [Limb],
    ys: &mut [Limb],
    s: usize,
    a: &mut HalfGcdMatrix,
    scratch: &mut [Limb],
) -> usize {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert!(n > s);
    let mask = xs[n - 1] | ys[n - 1];
    assert_ne!(mask, 0);
    let (x_high, a_low, y_high, b_low) = if n == s + 1 {
        if mask < 4 {
            return limbs_gcd_subdivide_step(xs, ys, s, a, scratch);
        }
        (xs[n - 1], xs[n - 2], ys[n - 1], ys[n - 2])
    } else if mask.get_highest_bit() {
        (xs[n - 1], xs[n - 2], ys[n - 1], ys[n - 2])
    } else {
        let shift = LeadingZeros::leading_zeros(mask);
        (
            extract_number(shift, xs[n - 1], xs[n - 2]),
            extract_number(shift, xs[n - 2], xs[n - 3]),
            extract_number(shift, ys[n - 1], ys[n - 2]),
            extract_number(shift, ys[n - 2], ys[n - 3]),
        )
    };
    // Try a limbs_half_gcd_2 step
    let mut b = HalfGcdMatrix1::default();
    if limbs_half_gcd_2(x_high, a_low, y_high, b_low, &mut b) {
        // Multiply A <- A * B.
        limbs_half_gcd_matrix_mul_matrix_1(a, &b, scratch);
        let scratch = &mut scratch[..n];
        // Can't swap inputs, so we need to copy.
        scratch.copy_from_slice(xs);
        // Multiply B^(-1) (x; y).
        limbs_half_gcd_matrix_1_mul_inverse_vector(&b, xs, scratch, ys)
    } else {
        limbs_gcd_subdivide_step(xs, ys, s, a, scratch)
    }
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `MPN_GCD_SUBDIV_STEP_ITCH` from `gmp-impl.h`, GMP 6.2.1.
pub(crate) const fn limbs_gcd_subdivide_step_scratch_len(n: usize) -> usize {
    n
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `CHOOSE_P` from `mpn/generic/gcd.c`, GMP 6.2.1.
const fn limbs_gcd_choose_p(n: usize) -> usize {
    (n << 1) / 3
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `MPN_HGCD_MATRIX_INIT_ITCH` from `gmp-impl.h`, GMP 6.2.1.
pub(crate) const fn limbs_half_gcd_matrix_init_scratch_len(n: usize) -> usize {
    (((n + 1) >> 1) + 1) << 2
}

// TODO tune
pub(crate) const HGCD_THRESHOLD: usize = 101;

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_hgcd_itch` from `mpn/generic/hgcd.c`, GMP 6.2.1.
pub(crate) fn limbs_half_gcd_scratch_len(n: usize) -> usize {
    if n < HGCD_THRESHOLD {
        n
    } else {
        // Get the recursion depth.
        let count = LeadingZeros::leading_zeros((n - 1) / (HGCD_THRESHOLD - 1));
        20 * ((n + 3) >> 2) + 22 * usize::exact_from(usize::WIDTH - count) + HGCD_THRESHOLD
    }
}

// TODO tune
const HGCD_REDUCE_THRESHOLD: usize = 1679;

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_hgcd_reduce_itch` from `mpn/generic/hgcd_reduce.c`, GMP 6.2.1.
pub_test! {limbs_half_gcd_reduce_scratch_len(n: usize, p: usize) -> usize {
    assert!(n >= p);
    let diff = n - p;
    if n < HGCD_REDUCE_THRESHOLD {
        let scratch_len = limbs_half_gcd_scratch_len(diff);
        // - For arbitrary p, the storage for adjust is
        // - 2 * (p + M.n) = 2 * (p + ceil((n - p) / 2) - 1 <= n + p - 1
        let sum = n + p - 1;
        if scratch_len < sum {
            sum
        } else {
            scratch_len
        }
    } else {
        (diff << 1) + limbs_half_gcd_scratch_len(diff)
    }
}}

// TODO tune
const HGCD_APPR_THRESHOLD: usize = 104;

/// Destroys inputs.
///
/// This is equivalent to `mpn_hgcd_appr` from `mpn/generic/hgcd_appr.c`, GMP 6.2.1.
fn limbs_half_gcd_approx(
    mut xs: &mut [Limb],
    mut ys: &mut [Limb],
    a: &mut HalfGcdMatrix,
    scratch: &mut [Limb],
) -> bool {
    let mut n = xs.len();
    assert_eq!(ys.len(), n);
    assert_ne!(n, 0);
    assert!(xs[n - 1] != 0 || ys[n - 1] != 0);
    if n <= 2 {
        fail_on_untested_path("limbs_half_gcd_approx, n <= 2");
        // Implies s = n
        return false;
    }
    assert!(((n + 1) >> 1) - 1 < a.s);
    // We aim for reduction of to W * s bits. But each time we discard some of the least significant
    // limbs, we must keep one additional bit to account for the truncation error. We maintain the W
    // * s - extra_bits as the current target size.
    let mut s = (n >> 1) + 1;
    let mut offset = 0;
    let mut success = false;
    if n < HGCD_APPR_THRESHOLD {
        let mut extra_bits = 0u64;
        let mut xs_chunk = &mut *xs;
        let mut ys_chunk = &mut *ys;
        while n > 2 {
            assert!(n > s);
            assert!(n <= s << 1);
            let new_n = limbs_half_gcd_step(xs_chunk, ys_chunk, s, a, scratch);
            if new_n == 0 {
                break;
            }
            n = new_n;
            xs_chunk = &mut xs_chunk[..n];
            ys_chunk = &mut ys_chunk[..n];
            success = true;
            // We can truncate and discard the lower p bits whenever n <= 2 * s - p. To account for
            // the truncation error, we must adjust s <- s + 1 - p, rather than just sbits <- sbits
            // - p. This adjustment makes the produced matrix slightly smaller than it could be.
            let lhs = ((n + 1) << Limb::LOG_WIDTH) + (usize::exact_from(extra_bits) << 1);
            let rhs = s << (Limb::LOG_WIDTH + 1);
            if lhs <= rhs {
                let p = ((((s << 1) - n) << Limb::LOG_WIDTH)
                    - (usize::exact_from(extra_bits) << 1))
                    >> Limb::LOG_WIDTH;
                if extra_bits == 0 {
                    // We cross a limb boundary and bump s. We can't do that if the result is that
                    // it makes makes min(X, Y) smaller than 2^W * s.
                    if s + 1 == n
                        || slice_test_zero(&xs_chunk[s + 1..])
                        || slice_test_zero(&ys_chunk[s + 1..])
                    {
                        continue;
                    }
                    extra_bits = Limb::WIDTH - 1;
                    s += 1;
                } else {
                    extra_bits -= 1;
                }
                // Drop the p least-significant limbs.
                offset += p;
                xs_chunk = &mut xs_chunk[p..];
                ys_chunk = &mut ys_chunk[p..];
                n -= p;
                s -= p;
            }
        }
        assert_ne!(s, 0);
        if extra_bits != 0 {
            // We can get here only of we have dropped at least one of the least-significant bits,
            // so we can decrement xs and ys. We can then shift left extra bits using
            // limbs_slice_shr_in_place.
            assert_ne!(offset, 0);
            let xs = &mut xs[offset - 1..];
            let ys = &mut ys[offset - 1..];
            let (xs_head, xs_tail) = xs[..=n].split_first_mut().unwrap();
            let (ys_head, ys_tail) = ys[..=n].split_first_mut().unwrap();
            let comp_bits = Limb::WIDTH - extra_bits;
            *xs_head = limbs_slice_shr_in_place(xs_tail, comp_bits);
            *ys_head = limbs_slice_shr_in_place(ys_tail, comp_bits);
            if xs[n] != 0 || ys[n] != 0 {
                n += 1;
            }
            assert!(success);
            while n > 2 {
                assert!(n > s);
                assert!(n <= s << 1);
                n = limbs_half_gcd_step(&mut xs[..n], &mut ys[..n], s, a, scratch);
                if n == 0 {
                    return true;
                }
            }
        }
        if n == 2 {
            fail_on_untested_path("limbs_half_gcd_approx, n == 2");
            assert_eq!(s, 1);
            let mut b = HalfGcdMatrix1::default();
            if limbs_half_gcd_2(xs[1], xs[0], ys[1], ys[0], &mut b) {
                // Multiply A <- A * B.
                limbs_half_gcd_matrix_mul_matrix_1(a, &b, scratch);
                success = true;
            }
        }
        success
    } else {
        let limit = ((3 * n) >> 2) + 1;
        let mut p = n >> 1;
        let new_n = limbs_half_gcd_matrix_reduce(a, xs, ys, p, scratch);
        if new_n != 0 {
            n = new_n;
            xs = &mut xs[..n];
            ys = &mut ys[..n];
            success = true;
        }
        while n > limit {
            // Needs n + 1 storage
            n = limbs_half_gcd_step(xs, ys, s, a, scratch);
            if n == 0 {
                return success;
            }
            xs = &mut xs[..n];
            ys = &mut ys[..n];
            success = true;
        }
        if n > s + 2 {
            p = (s << 1) - n + 1;
            let scratch_len = limbs_half_gcd_matrix_init_scratch_len(n - p);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(scratch_len);
            let mut b = HalfGcdMatrix::init(n - p, scratch_lo);
            if limbs_half_gcd_approx(&mut xs[p..], &mut ys[p..], &mut b, scratch_hi) {
                // We always have max(A) > 2 ^ (-(W + 1)) * max(B).
                assert!(a.n + 2 >= b.n);
                // Furthermore, assume A ends with a quotient (1, q; 0, 1); then either q or q + 1
                // is a correct quotient, and B will start with either (1, 0; 1, 1) or (2, 1; 1, 1).
                // This rules out the case that the size of A * B is much smaller than the expected
                // A.n + B.n.
                assert!(a.n + b.n < a.s);
                // We need a bound for of A.n + B.n. Let n be the original input size. Then ceil(n /
                // 2) - 1 >= size of product >= A.n + B.n - 2, and it follows that A.n + B.n <=
                // ceil(n / 2) + 1. Then 3 * (A.n + B.n) + 5 <= 3 * ceil(n / 2) + 8 is the amount of
                // needed scratch space.
                limbs_half_gcd_matrix_mul_matrix(a, &b, scratch_hi);
                return true;
            }
        }
        loop {
            assert!(n > s);
            assert!(n <= s << 1);
            let new_n = limbs_half_gcd_step(xs, ys, s, a, scratch);
            if new_n == 0 {
                return success;
            }
            n = new_n;
            xs = &mut xs[..n];
            ys = &mut ys[..n];
            success = true;
        }
    }
}

// Reduces x, y until |x - y| fits in n / 2 + 1 limbs. Constructs matrix A with elements of size at
// most (n + 1) / 2 - 1. Returns new size of a, b, or zero if no reduction is possible.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_hgcd` from `mpn/generic/hgcd.c`, GMP 6.2.1.
pub(crate) fn limbs_half_gcd(
    xs: &mut [Limb],
    ys: &mut [Limb],
    a: &mut HalfGcdMatrix,
    scratch: &mut [Limb],
) -> usize {
    let mut n = xs.len();
    assert_eq!(ys.len(), n);
    let s = (n >> 1) + 1;
    let mut success = false;
    if n <= s {
        fail_on_untested_path("limbs_half_gcd, n <= s");
        return 0;
    }
    assert!(xs[n - 1] != 0 || ys[n - 1] != 0);
    assert!(((n + 1) >> 1) - 1 < a.s);
    if n >= HGCD_THRESHOLD {
        let limit = ((3 * n) >> 2) + 1;
        let p = n >> 1;
        let mut new_n = limbs_half_gcd_matrix_reduce(a, xs, ys, p, scratch);
        if new_n != 0 {
            n = new_n;
            success = true;
        }
        while n > limit {
            // Needs n + 1 storage
            let new_n = limbs_half_gcd_step(&mut xs[..n], &mut ys[..n], s, a, scratch);
            if new_n == 0 {
                return if success { n } else { 0 };
            }
            n = new_n;
            success = true;
        }
        if n > s + 2 {
            let p = (s << 1) - n + 1;
            let scratch_len = limbs_half_gcd_matrix_init_scratch_len(n - p);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(scratch_len);
            let mut b = HalfGcdMatrix::init(n - p, scratch_lo);
            new_n = limbs_half_gcd(&mut xs[p..n], &mut ys[p..n], &mut b, scratch_hi);
            if new_n != 0 {
                // We always have max(A) > 2 ^ (-(W + 1)) * max(B).
                assert!(a.n + 2 >= b.n);
                // Furthermore, assume A ends with a quotient (1, q; 0, 1); then either q or q + 1
                // is a correct quotient, and B will start with either (1, 0; 1, 1) or (2, 1; 1,
                // 1).This rules out the case that the size of A * B is much smaller than the
                // expected A.n + B.n.
                assert!(a.n + b.n < a.s);
                // Needs 2 * (p + A.n) <= 2 * (2 * s - limit + 1 + limit - s - 1) = 2 * s <= 2 *
                // (floor(n / 2) + 1) <= n + 2.
                n = limbs_half_gcd_matrix_adjust(&b, p + new_n, xs, ys, p, scratch_hi);
                // We need a bound for of A.n + B.n. Let n be the original input size. Then ceil(n /
                // 2) - 1 >= size of product >= A.n + B.n - 2 and it follows that A.n + B.n <=
                // ceil(n / 2) + 1. Then 3 * (A.n + B.n) + 5 <= 3 * ceil(n / 2) + 8 is the amount of
                // needed scratch space.
                limbs_half_gcd_matrix_mul_matrix(a, &b, scratch_hi);
                success = true;
            }
        }
    }
    loop {
        // Needs s + 3 < n
        let new_n = limbs_half_gcd_step(&mut xs[..n], &mut ys[..n], s, a, scratch);
        if new_n == 0 {
            return if success { n } else { 0 };
        }
        n = new_n;
        success = true;
    }
}

// TODO tune
pub(crate) const GCD_DC_THRESHOLD: usize = 330;

// X >= Y, X and Y not both even.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_gcd` from `mpn/generic/gcd.c`, GMP 6.2.1.
pub_crate_test! {limbs_gcd_reduced(out: &mut [Limb], xs: &mut [Limb], ys: &mut [Limb]) -> usize {
    let mut xs = &mut *xs;
    let mut ys = &mut *ys;
    let xs_len = xs.len();
    let mut n = ys.len();
    assert!(xs_len >= n);
    assert_ne!(n, 0);
    assert_ne!(ys[n - 1], 0);
    let mut scratch_len = max(xs_len - n + 1, limbs_gcd_subdivide_step_scratch_len(n));
    if n >= GCD_DC_THRESHOLD {
        let p = limbs_gcd_choose_p(n);
        let matrix_scratch_len = limbs_half_gcd_matrix_init_scratch_len(n - p);
        let half_gcd_scratch_len = limbs_half_gcd_scratch_len(n - p);
        let update_scratch_len = p + n - 1;
        scratch_len = max(
            scratch_len,
            matrix_scratch_len + max(half_gcd_scratch_len, update_scratch_len),
        );
    }
    let mut scratch = vec![0; scratch_len];
    let mut scratch = &mut scratch[..];
    if xs_len > n {
        limbs_div_mod_qs_to_out_rs_to_ns(scratch, xs, ys);
        if slice_test_zero(&xs[..n]) {
            out[..n].copy_from_slice(ys);
            return n;
        }
    }
    while n >= GCD_DC_THRESHOLD {
        let xs = &mut xs[..n];
        let ys = &mut ys[..n];
        let p = limbs_gcd_choose_p(n);
        let comp_p = n - p;
        let matrix_scratch_len = limbs_half_gcd_matrix_init_scratch_len(comp_p);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(matrix_scratch_len);
        let mut m = HalfGcdMatrix::init(comp_p, scratch_lo);
        let new_n = limbs_half_gcd(&mut xs[p..], &mut ys[p..], &mut m, scratch_hi);
        if new_n != 0 {
            assert!(m.n <= (comp_p - 1) >> 1);
            assert!(m.n + p <= (p + n - 1) >> 1);
            // Temporary storage 2 * (p + M.n) <= p + n - 1.
            n = limbs_half_gcd_matrix_adjust(&m, p + new_n, xs, ys, p, scratch_hi);
        } else {
            // Temporary storage n.
            let out_len = n;
            n = limbs_gcd_subdivide_step(xs, ys, 0, &mut GcdContext(out), scratch);
            if n == 0 {
                return out_len;
            }
        }
    }
    while n > 2 {
        let mask = xs[n - 1] | ys[n - 1];
        assert_ne!(mask, 0);
        let (x_hi, x_lo, y_hi, y_lo) = if mask.get_highest_bit() {
            (xs[n - 1], xs[n - 2], ys[n - 1], ys[n - 2])
        } else {
            let shift = LeadingZeros::leading_zeros(mask);
            (
                extract_number(shift, xs[n - 1], xs[n - 2]),
                extract_number(shift, xs[n - 2], xs[n - 3]),
                extract_number(shift, ys[n - 1], ys[n - 2]),
                extract_number(shift, ys[n - 2], ys[n - 3]),
            )
        };
        let mut m = HalfGcdMatrix1::default();
        // Try a limbs_half_gcd_2 step.
        if limbs_half_gcd_2(x_hi, x_lo, y_hi, y_lo, &mut m) {
            n = limbs_half_gcd_matrix_1_mul_inverse_vector(
                &m,
                &mut scratch[..n],
                &xs[..n],
                &mut ys[..n],
            );
            swap(&mut xs, &mut scratch);
        } else {
            // limbs_half_gcd_2 has failed. Then either one of x or y is very small, or the
            // difference is very small. Perform one subtraction followed by one division.
            let out_len = n;
            n = limbs_gcd_subdivide_step(
                &mut xs[..n],
                &mut ys[..n],
                0,
                &mut GcdContext(out),
                scratch,
            );
            if n == 0 {
                return out_len;
            }
        }
    }
    assert!(xs[n - 1] != 0 || ys[n - 1] != 0);
    // Due to the calling convention for limbs_gcd_reduced, at most one can be even.
    if xs[0].even() {
        swap(&mut xs, &mut ys);
    }
    assert!(xs[0].odd());
    let x_0 = xs[0];
    let mut y_0 = ys[0];
    if n == 1 {
        out[0] = x_0.gcd(y_0 >> y_0.trailing_zeros());
        return 1;
    }
    let mut y_1 = ys[1];
    if y_0 == 0 {
        y_0 = y_1;
        y_1 = 0;
    }
    if y_0.even() {
        let zeros = TrailingZeros::trailing_zeros(y_0);
        y_0 = (y_1 << (Limb::WIDTH - zeros)) | (y_0 >> zeros);
        y_1 >>= zeros;
    }
    let x_1 = xs[1];
    // TODO try mpn_gcd_22
    (out[1], out[0]) = DoubleLimb::join_halves(x_1, x_0)
        .gcd(DoubleLimb::join_halves(y_1, y_0))
        .split_in_half();
    if out[1] == 0 {
        1
    } else {
        2
    }
}}
