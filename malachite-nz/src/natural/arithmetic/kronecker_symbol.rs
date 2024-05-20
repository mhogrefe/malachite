// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::arithmetic::kronecker_symbol::{
    limbs_kronecker_symbol, limbs_kronecker_symbol_single,
};
use crate::natural::arithmetic::gcd::half_gcd::{
    extract_number, limbs_gcd_div, limbs_gcd_subdivide_step, limbs_gcd_subdivide_step_scratch_len,
    limbs_half_gcd_matrix_1_mul_inverse_vector, limbs_half_gcd_matrix_adjust,
    limbs_half_gcd_matrix_init_scratch_len, limbs_half_gcd_matrix_mul_matrix,
    limbs_half_gcd_matrix_mul_matrix_1, limbs_half_gcd_matrix_update_q, limbs_half_gcd_scratch_len,
    GcdSubdivideStepContext, HalfGcdMatrix, HalfGcdMatrix1, GCD_DC_THRESHOLD, HGCD_THRESHOLD,
};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use core::cmp::max;
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DivMod, JacobiSymbol, KroneckerSymbol, LegendreSymbol, ModPowerOf2, Parity, XXSubYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{JoinHalves, WrappingFrom};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::slices::slice_trailing_zeros;

// This is equivalent to `jacobi_table` from `mpn/jacobi.c`, GMP 6.2.1.
const JACOBI_TABLE: [u8; 208] = [
    0, 0, 0, 0, 0, 12, 8, 4, 1, 1, 1, 1, 1, 13, 9, 5, 2, 2, 2, 2, 2, 6, 10, 14, 3, 3, 3, 3, 3, 7,
    11, 15, 4, 16, 6, 18, 4, 0, 12, 8, 5, 17, 7, 19, 5, 1, 13, 9, 6, 18, 4, 16, 6, 10, 14, 2, 7,
    19, 5, 17, 7, 11, 15, 3, 8, 10, 9, 11, 8, 4, 0, 12, 9, 11, 8, 10, 9, 5, 1, 13, 10, 9, 11, 8,
    10, 14, 2, 6, 11, 8, 10, 9, 11, 15, 3, 7, 12, 22, 24, 20, 12, 8, 4, 0, 13, 23, 25, 21, 13, 9,
    5, 1, 25, 21, 13, 23, 14, 2, 6, 10, 24, 20, 12, 22, 15, 3, 7, 11, 16, 6, 18, 4, 16, 16, 16, 16,
    17, 7, 19, 5, 17, 17, 17, 17, 18, 4, 16, 6, 18, 22, 19, 23, 19, 5, 17, 7, 19, 23, 18, 22, 20,
    12, 22, 24, 20, 20, 20, 20, 21, 13, 23, 25, 21, 21, 21, 21, 22, 24, 20, 12, 22, 19, 23, 18, 23,
    25, 21, 13, 23, 18, 22, 19, 24, 20, 12, 22, 15, 3, 7, 11, 25, 21, 13, 23, 14, 2, 6, 10,
];

// This is equivalent to `mpn_jacobi_update` from `gmp-impl.h`, GMP 6.2.1.
fn limbs_jacobi_update(bits: u8, denominator: u8, q: Limb) -> u8 {
    assert!(bits < 26);
    assert!(denominator < 2);
    assert!(q < 4);
    JACOBI_TABLE[usize::wrapping_from(((bits << 3) + (denominator << 2)) | u8::wrapping_from(q))]
}

// This is equivalent to `hgcd_jacobi_context` from `mpn/hgcd_jacobi.c`, GMP 6.2.1.
struct HalfGcdJacobiContext<'a, 'b, 'c> {
    m: &'a mut HalfGcdMatrix<'b>,
    bits_mut: &'c mut u8,
}

impl<'a, 'b, 'c> GcdSubdivideStepContext for HalfGcdJacobiContext<'a, 'b, 'c> {
    // This is equivalent to `hgcd_jacobi_hook` from `mpn/hgcd_jacobi.c`, GMP 6.2.1.
    fn gcd_subdiv_step_hook(
        &mut self,
        gs: Option<&[Limb]>,
        qs: Option<&mut [Limb]>,
        mut qs_len: usize,
        d: i8,
    ) {
        assert!(gs.is_none());
        assert!(d >= 0);
        let qs = qs.unwrap();
        qs_len -= slice_trailing_zeros(&qs[..qs_len]);
        if qs_len != 0 {
            let (qs, scratch) = qs.split_at_mut(qs_len);
            let d = u8::wrapping_from(d);
            limbs_half_gcd_matrix_update_q(self.m, qs, d, scratch);
            *self.bits_mut = limbs_jacobi_update(*self.bits_mut, d, qs[0].mod_power_of_2(2));
        }
    }
}

const HALF_WIDTH: u64 = Limb::WIDTH >> 1;

const TWO_POW_HALF_WIDTH: Limb = 1 << HALF_WIDTH;

const TWICE_TWO_POW_HALF_WIDTH: Limb = TWO_POW_HALF_WIDTH << 1;

// This is equivalent to `mpn_hgcd2_jacobi` from `mpn/hgcd2_jacobi.c`, GMP 6.2.1, returning `bitsp`
// along with a bool.
fn limbs_half_gcd_2_jacobi(
    mut x_1: Limb,
    mut x_0: Limb,
    mut y_1: Limb,
    mut y_0: Limb,
    m: &mut HalfGcdMatrix1,
    mut bits: u8,
) -> (u8, bool) {
    if x_1 < 2 || y_1 < 2 {
        return (bits, false);
    }
    let mut u00 = 1;
    let mut u01;
    let mut u10;
    let mut u11 = 1;
    (u01, u10, bits) = if x_1 > y_1 || x_1 == y_1 && x_0 > y_0 {
        (x_1, x_0) = Limb::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0);
        if x_1 < 2 {
            return (bits, false);
        }
        (1, 0, limbs_jacobi_update(bits, 1, 1))
    } else {
        (y_1, y_0) = Limb::xx_sub_yy_to_zz(y_1, y_0, x_1, x_0);
        if y_1 < 2 {
            return (bits, false);
        }
        (0, 1, limbs_jacobi_update(bits, 0, 1))
    };
    let mut subtract_a = x_1 < y_1;
    let mut subtract_a_1 = false;
    loop {
        if !subtract_a {
            assert!(x_1 >= y_1);
            if x_1 == y_1 {
                m.data[0][0] = u00;
                m.data[0][1] = u01;
                m.data[1][0] = u10;
                m.data[1][1] = u11;
                return (bits, true);
            }
            if x_1 < TWO_POW_HALF_WIDTH {
                x_1 = (x_1 << HALF_WIDTH) + (x_0 >> HALF_WIDTH);
                y_1 = (y_1 << HALF_WIDTH) + (y_0 >> HALF_WIDTH);
                break;
            }
            // Subtract x -= q * y, and multiply m from the right by (1 q ; 0 1), affecting the
            // second column of m.
            assert!(x_1 > y_1);
            (x_1, x_0) = Limb::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0);
            if x_1 < 2 {
                m.data[0][0] = u00;
                m.data[0][1] = u01;
                m.data[1][0] = u10;
                m.data[1][1] = u11;
                return (bits, true);
            }
            let bits_copy = bits;
            bits = limbs_jacobi_update(
                bits_copy,
                1,
                if x_1 <= y_1 {
                    // Use q = 1
                    u01 += u00;
                    u11 += u10;
                    1
                } else {
                    let mut q;
                    (q, x_1, x_0) = limbs_gcd_div(x_1, x_0, y_1, y_0);
                    if x_1 < 2 {
                        // X is too small, but q is correct.
                        u01 += q * u00;
                        u11 += q * u10;
                        bits = limbs_jacobi_update(bits, 1, q.mod_power_of_2(2));
                        m.data[0][0] = u00;
                        m.data[0][1] = u01;
                        m.data[1][0] = u10;
                        m.data[1][1] = u11;
                        return (bits, true);
                    }
                    q += 1;
                    u01 += q * u00;
                    u11 += q * u10;
                    q.mod_power_of_2(2)
                },
            );
        }
        subtract_a = false;
        assert!(y_1 >= x_1);
        if x_1 == y_1 {
            m.data[0][0] = u00;
            m.data[0][1] = u01;
            m.data[1][0] = u10;
            m.data[1][1] = u11;
            return (bits, true);
        }
        if y_1 < TWO_POW_HALF_WIDTH {
            x_1 = (x_1 << HALF_WIDTH) + (x_0 >> HALF_WIDTH);
            y_1 = (y_1 << HALF_WIDTH) + (y_0 >> HALF_WIDTH);
            subtract_a_1 = true;
            break;
        }
        // Subtract b -= q a, and multiply M from the right by (1 0 ; q 1), affecting the first
        // column of M.
        (y_1, y_0) = Limb::xx_sub_yy_to_zz(y_1, y_0, x_1, x_0);
        if y_1 < 2 {
            m.data[0][0] = u00;
            m.data[0][1] = u01;
            m.data[1][0] = u10;
            m.data[1][1] = u11;
            return (bits, true);
        }
        let bits_copy = bits;
        bits = limbs_jacobi_update(
            bits_copy,
            0,
            if y_1 <= x_1 {
                // Use q = 1
                u00 += u01;
                u10 += u11;
                1
            } else {
                let mut q;
                (q, y_1, y_0) = limbs_gcd_div(y_1, y_0, x_1, x_0);
                if y_1 < 2 {
                    // Y is too small, but q is correct.
                    u00 += q * u01;
                    u10 += q * u11;
                    bits = limbs_jacobi_update(bits, 0, q.mod_power_of_2(2));
                    m.data[0][0] = u00;
                    m.data[0][1] = u01;
                    m.data[1][0] = u10;
                    m.data[1][1] = u11;
                    return (bits, true);
                }
                q += 1;
                u00 += q * u01;
                u10 += q * u11;
                q.mod_power_of_2(2)
            },
        );
    }
    // Since we discard the least significant half limb, we don't　get a truly maximal m
    // (corresponding to |x - y| < 2^(W+1)).
    //
    // Single precision loop
    loop {
        if !subtract_a_1 {
            assert!(x_1 >= y_1);
            if x_1 == y_1 {
                break;
            }
            x_1 -= y_1;
            if x_1 < TWICE_TWO_POW_HALF_WIDTH {
                break;
            }
            let bits_copy = bits;
            bits = limbs_jacobi_update(
                bits_copy,
                1,
                if x_1 <= y_1 {
                    // Use q = 1
                    u01 += u00;
                    u11 += u10;
                    1
                } else {
                    let (mut q, r) = x_1.div_mod(y_1);
                    x_1 = r;
                    if x_1 < TWICE_TWO_POW_HALF_WIDTH {
                        // X is too small, but q is correct.
                        u01 += q * u00;
                        u11 += q * u10;
                        bits = limbs_jacobi_update(bits, 1, q.mod_power_of_2(2));
                        break;
                    }
                    q += 1;
                    u01 += q * u00;
                    u11 += q * u10;
                    q.mod_power_of_2(2)
                },
            );
        }
        subtract_a_1 = false;
        assert!(y_1 >= x_1);
        if x_1 == y_1 {
            break;
        }
        y_1 -= x_1;
        if y_1 < TWICE_TWO_POW_HALF_WIDTH {
            break;
        }
        let bits_copy = bits;
        bits = limbs_jacobi_update(
            bits_copy,
            0,
            if y_1 <= x_1 {
                // Use q = 1
                u00 += u01;
                u10 += u11;
                1
            } else {
                let mut q;
                (q, y_1) = y_1.div_mod(x_1);
                if y_1 < TWICE_TWO_POW_HALF_WIDTH {
                    // Y is too small, but q is correct.
                    u00 += q * u01;
                    u10 += q * u11;
                    bits = limbs_jacobi_update(bits, 0, q.mod_power_of_2(2));
                    break;
                }
                q += 1;
                u00 += q * u01;
                u10 += q * u11;
                q.mod_power_of_2(2)
            },
        );
    }
    m.data[0][0] = u00;
    m.data[0][1] = u01;
    m.data[1][0] = u10;
    m.data[1][1] = u11;
    (bits, true)
}

// Perform a few steps, using some of `limbs_half_gcd_2_jacobi`, subtraction and division. Reduces
// the size by almost one limb or more, but never below the given size s. Return new size for x and
// y, or 0 if no more steps are possible.
//
// If `limbs_half_gcd_2_jacobi` succeeds, needs temporary space for
// `limbs_half_gcd_matrix_mul_matrix_1`, m.n limbs, and
// `limbs_half_gcd_matrix_1_mul_inverse_vector`, n limbs. If `limbs_half_gcd_2_jacobi` fails, needs
// space for the quotient, qs_len <= n - s + 1 limbs, for and `update_q`, qs_len + (size of the
// appropriate column of M) <= resulting size of m.
//
// If n is the input size to the calling hgcd, then s = floor(N / 2) + 1, m.n < N, qs_len + matrix
// size <= n - s + 1 + n - s = 2 (n - s) + 1 < N, so N is sufficient.
//
// This is equivalent to `hgcd_jacobi_step` from `mpn/hgcd_jacobi.c`, GMP 6.2.1, where `bitsp` is
// returned along with the `usize`.
fn limbs_half_gcd_jacobi_step(
    xs: &mut [Limb],
    ys: &mut [Limb],
    s: usize,
    m: &mut HalfGcdMatrix,
    mut bits: u8,
    scratch: &mut [Limb],
) -> (u8, usize) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    let scratch = &mut scratch[..n];
    assert!(n > s);
    let mask = xs[n - 1] | ys[n - 1];
    assert_ne!(mask, 0);
    let (x_hi, x_lo, y_hi, y_lo) = if n == s + 1 {
        if mask < 4 {
            let u = limbs_gcd_subdivide_step(
                xs,
                ys,
                s,
                &mut HalfGcdJacobiContext {
                    m,
                    bits_mut: &mut bits,
                },
                scratch,
            );
            return (bits, u);
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
    // Try a `limbs_half_gcd_2_jacobi` step
    let mut m1 = HalfGcdMatrix1::default();
    let b;
    (bits, b) = limbs_half_gcd_2_jacobi(x_hi, x_lo, y_hi, y_lo, &mut m1, bits);
    if b {
        // Multiply m <- m * m1
        limbs_half_gcd_matrix_mul_matrix_1(m, &m1, scratch);
        // Can't swap inputs, so we need to copy
        scratch.copy_from_slice(xs);
        // Multiply m1^(-1) (x;y)
        (
            bits,
            limbs_half_gcd_matrix_1_mul_inverse_vector(&m1, xs, scratch, ys),
        )
    } else {
        let u = limbs_gcd_subdivide_step(
            xs,
            ys,
            s,
            &mut HalfGcdJacobiContext {
                m,
                bits_mut: &mut bits,
            },
            scratch,
        );
        (bits, u)
    }
}

// Reduces x, y until |x - y| fits in n / 2 + 1 limbs. Constructs matrix m with elements of size at
// most (n + 1) / 2 - 1. Returns new size of x, y, or zero if no reduction is possible.
//
// Same scratch requirements as for `limbs_half_gcd`.
//
// This is equivalent to `mpn_hgcd_jacobi` from `mpn/hgcd_jacobi.c`, GMP 6.2.1, where `bitsp` is
// also returned.
fn limbs_half_gcd_jacobi(
    xs: &mut [Limb],
    ys: &mut [Limb],
    m: &mut HalfGcdMatrix<'_>,
    mut bits: u8,
    scratch: &mut [Limb],
) -> (u8, usize) {
    let mut n = xs.len();
    assert_eq!(ys.len(), n);
    let s = (n >> 1) + 1;
    let mut success = false;
    assert!(s < n);
    assert!(xs[n - 1] != 0 || ys[n - 1] != 0);
    assert!(((n + 1) >> 1) - 1 < s);
    if n >= HGCD_THRESHOLD {
        let n2 = ((3 * n) >> 2) + 1;
        let p = n >> 1;
        let mut nn;
        (bits, nn) = limbs_half_gcd_jacobi(&mut xs[p..n], &mut ys[p..n], m, bits, scratch);
        if nn != 0 {
            // Needs 2 * (p + m.n) <= 2 * (floor(n / 2) + ceiling(n / 2) - 1) = 2 * (n - 1)
            n = limbs_half_gcd_matrix_adjust(m, p + nn, xs, ys, p, scratch);
            success = true;
        }
        while n > n2 {
            // Needs n + 1 storage
            (bits, nn) =
                limbs_half_gcd_jacobi_step(&mut xs[..n], &mut ys[..n], s, m, bits, scratch);
            if nn == 0 {
                return (bits, if success { n } else { 0 });
            }
            n = nn;
            success = true;
        }
        if n > s + 2 {
            let p = 2 * s - n + 1;
            let scratch_len = limbs_half_gcd_matrix_init_scratch_len(n - p);
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(scratch_len);
            let mut m1 = HalfGcdMatrix::init(n - p, scratch_lo);
            (bits, nn) =
                limbs_half_gcd_jacobi(&mut xs[p..n], &mut ys[p..n], &mut m1, bits, scratch_hi);
            if nn != 0 {
                // We always have max(m) > 2^(-(W + 1)) * max(m1)
                assert!(m.n + 2 >= m1.n);
                // Furthermore, assume m ends with a quotient (1, q; 0, 1); then either q or q + 1
                // is a correct quotient, and m1 will start with either (1, 0; 1, 1) or (2, 1; 1,
                // 1). This rules out the case that the size of m * m1 is much smaller than the
                // expected m.n + m1.n.
                assert!(m.n + m1.n < m.s);
                // Needs 2 * (p + m.n) <= 2 * (2 * s - nn + 1 + nn - s - 1) = 2 * s <= 2 * (floor(n
                // / 2) + 1) <= n + 2.
                n = limbs_half_gcd_matrix_adjust(&m1, p + nn, xs, ys, p, scratch_hi);
                // We need a bound for of m.n + m1.n. Let n be the original input size. Then
                //
                // ceiling(n / 2) - 1 >= size of product >= m.n + m1.n - 2
                //
                // and it follows that
                //
                // m.n + m1.n <= ceiling(n / 2) + 1
                //
                // Then 3 * (m.n + m1.n) + 5 <= 3 * ceiling(n / 2) + 8 is the amount of needed
                // scratch space.
                limbs_half_gcd_matrix_mul_matrix(m, &m1, scratch_hi);
                success = true;
            }
        }
    }
    loop {
        // Needs s + 3 < n
        let nn;
        (bits, nn) = limbs_half_gcd_jacobi_step(&mut xs[..n], &mut ys[..n], s, m, bits, scratch);
        if nn == 0 {
            return (bits, if success { n } else { 0 });
        }
        n = nn;
        success = true;
    }
}

// This is equivalent to `CHOOSE_P` from `mpn/jacobi.c`, GMP 6.2.1.
const fn choose_p(n: usize) -> usize {
    (n << 1) / 3
}

const BITS_FAIL: u8 = 31;

// This is equivalent to `mpn_jacobi_finish` from `gmp-impl.h`, GMP 6.2.1, which also handles the
// `bits == BITS_FAIL` case.
fn limbs_jacobi_finish(bits: u8) -> i8 {
    // (a, b) = (1,0) or (0,1)
    if bits == BITS_FAIL {
        0
    } else if bits.even() {
        1
    } else {
        -1
    }
}

// This is equivalent to `mpn_jacobi_init` from `gmp-impl.h`, GMP 6.2.1.
pub_crate_test! {limbs_jacobi_symbol_init(a: Limb, b: Limb, s: u8) -> u8 {
    assert!(b.odd());
    assert!(s <= 1);
    u8::wrapping_from((a.mod_power_of_2(2) << 2) + (b & 2)) + s
}}

struct JacobiContext<'a> {
    bits_mut: &'a mut u8,
}

impl<'a> GcdSubdivideStepContext for JacobiContext<'a> {
    // This is equivalent to `jacobi_hook` from `mpn/jacobi.c`, GMP 6.2.1.
    fn gcd_subdiv_step_hook(
        &mut self,
        gs: Option<&[Limb]>,
        qs: Option<&mut [Limb]>,
        qs_len: usize,
        d: i8,
    ) {
        if let Some(gs) = gs {
            let gs_len = gs.len();
            assert_ne!(gs_len, 0);
            if gs_len != 1 || gs[0] != 1 {
                *self.bits_mut = BITS_FAIL;
                return;
            }
        }
        if let Some(qs) = qs {
            assert_ne!(qs_len, 0);
            assert!(d >= 0);
            *self.bits_mut = limbs_jacobi_update(
                *self.bits_mut,
                u8::wrapping_from(d),
                qs[0].mod_power_of_2(2),
            );
        } else {
            fail_on_untested_path("JacobiContext::gcd_subdiv_step_hook, qs == None");
        }
    }
}

const JACOBI_DC_THRESHOLD: usize = GCD_DC_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_jacobi_n` from `mpn/jacobi.c`, GMP 6.2.1.
pub_crate_test! {
    limbs_jacobi_symbol_same_length(xs: &mut [Limb], ys: &mut [Limb], mut bits: u8) -> i8 {
    let mut n = xs.len();
    assert_eq!(ys.len(), n);
    assert_ne!(n, 0);
    assert!(xs[n - 1] != 0 || ys[n - 1] != 0);
    assert!((ys[0] | xs[0]).odd());
    let mut scratch_len = limbs_gcd_subdivide_step_scratch_len(n);
    if n >= JACOBI_DC_THRESHOLD {
        let p = choose_p(n);
        let matrix_scratch_len = limbs_half_gcd_matrix_init_scratch_len(n - p);
        let hgcd_scratch_len = limbs_half_gcd_scratch_len(n - p);
        let update_scratch_len = p + n - 1;
        let dc_scratch_len = matrix_scratch_len + max(hgcd_scratch_len, update_scratch_len);
        assert!(dc_scratch_len > scratch_len);
        scratch_len = dc_scratch_len;
    }
    let mut scratch = vec![0; scratch_len];
    let mut xs: &mut [Limb] = &mut xs[..];
    let mut ys: &mut [Limb] = &mut ys[..];
    let mut scratch: &mut [Limb] = &mut scratch;
    while n >= JACOBI_DC_THRESHOLD {
        let p = (n << 1) / 3;
        let matrix_scratch_len = limbs_half_gcd_matrix_init_scratch_len(n - p);
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(matrix_scratch_len);
        let mut m = HalfGcdMatrix::init(n - p, scratch_lo);
        let nn;
        (bits, nn) = limbs_half_gcd_jacobi(&mut xs[p..n], &mut ys[p..n], &mut m, bits, scratch_hi);
        if nn != 0 {
            assert!(m.n <= (n - p - 1) >> 1);
            assert!(m.n + p <= (p + n - 1) >> 1);
            // Temporary storage 2 (p + M->n) <= p + n - 1.
            n = limbs_half_gcd_matrix_adjust(&m, p + nn, xs, ys, p, scratch_hi);
        } else {
            // Temporary storage n
            n = limbs_gcd_subdivide_step(
                &mut xs[..n],
                &mut ys[..n],
                0,
                &mut JacobiContext {
                    bits_mut: &mut bits,
                },
                scratch,
            );
            if n == 0 {
                return limbs_jacobi_finish(bits);
            }
        }
    }
    while n > 2 {
        let mask = xs[n - 1] | ys[n - 1];
        assert_ne!(mask, 0);
        let (xs_hi, xs_lo, ys_hi, ys_lo) = if mask.get_highest_bit() {
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
        // Try a `limbs_half_gcd_2_jacobi` step
        let b;
        (bits, b) = limbs_half_gcd_2_jacobi(xs_hi, xs_lo, ys_hi, ys_lo, &mut m, bits);
        if b {
            n = limbs_half_gcd_matrix_1_mul_inverse_vector(
                &m,
                &mut scratch[..n],
                &xs[..n],
                &mut ys[..n],
            );
            swap(&mut xs, &mut scratch);
        } else {
            // `limbs_half_gcd_2_jacobi` has failed. Then either one of x or y is very small, or the
            // difference is very small. Perform one subtraction followed by one division.
            n = limbs_gcd_subdivide_step(
                &mut xs[..n],
                &mut ys[..n],
                0,
                &mut JacobiContext {
                    bits_mut: &mut bits,
                },
                scratch,
            );
            if n == 0 {
                return limbs_jacobi_finish(bits);
            }
        }
    }
    if bits >= 16 {
        swap(&mut xs, &mut ys);
    }
    assert!(ys[0].odd());
    let j = if n == 1 {
        let x_lo = xs[0];
        let y_lo = ys[0];
        if y_lo == 1 {
            1
        } else {
            x_lo.jacobi_symbol(y_lo)
        }
    } else {
        DoubleLimb::join_halves(xs[1], xs[0]).jacobi_symbol(DoubleLimb::join_halves(ys[1], ys[0]))
    };
    if bits.even() {
        j
    } else {
        -j
    }
}}

impl LegendreSymbol<Natural> for Natural {
    /// Computes the Legendre symbol of two [`Natural`]s, taking both by value.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).legendre_symbol(Natural::from(5u32)), 0);
    /// assert_eq!(Natural::from(7u32).legendre_symbol(Natural::from(5u32)), -1);
    /// assert_eq!(Natural::from(11u32).legendre_symbol(Natural::from(5u32)), 1);
    /// ```
    #[inline]
    fn legendre_symbol(self, other: Natural) -> i8 {
        assert_ne!(other, 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(&other)
    }
}

impl<'a> LegendreSymbol<&'a Natural> for Natural {
    /// Computes the Legendre symbol of two [`Natural`]s, taking the first by value and the second
    /// by reference.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).legendre_symbol(&Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).legendre_symbol(&Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     Natural::from(11u32).legendre_symbol(&Natural::from(5u32)),
    ///     1
    /// );
    /// ```
    #[inline]
    fn legendre_symbol(self, other: &'a Natural) -> i8 {
        assert_ne!(*other, 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(other)
    }
}

impl<'a> LegendreSymbol<Natural> for &'a Natural {
    /// Computes the Legendre symbol of two [`Natural`]s, taking both the first by reference and the
    /// second by value.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).legendre_symbol(Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).legendre_symbol(Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).legendre_symbol(Natural::from(5u32)),
    ///     1
    /// );
    /// ```
    #[inline]
    fn legendre_symbol(self, other: Natural) -> i8 {
        assert_ne!(other, 0u32);
        assert!(other.odd());
        self.kronecker_symbol(&other)
    }
}

impl<'a, 'b> LegendreSymbol<&'a Natural> for &'b Natural {
    /// Computes the Legendre symbol of two [`Natural`]s, taking both by reference.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).legendre_symbol(&Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).legendre_symbol(&Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).legendre_symbol(&Natural::from(5u32)),
    ///     1
    /// );
    /// ```
    #[inline]
    fn legendre_symbol(self, other: &'a Natural) -> i8 {
        assert_ne!(*other, 0u32);
        assert!(other.odd());
        self.kronecker_symbol(other)
    }
}

impl JacobiSymbol<Natural> for Natural {
    /// Computes the Jacobi symbol of two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).jacobi_symbol(Natural::from(5u32)), 0);
    /// assert_eq!(Natural::from(7u32).jacobi_symbol(Natural::from(5u32)), -1);
    /// assert_eq!(Natural::from(11u32).jacobi_symbol(Natural::from(5u32)), 1);
    /// assert_eq!(Natural::from(11u32).jacobi_symbol(Natural::from(9u32)), 1);
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: Natural) -> i8 {
        assert_ne!(other, 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(&other)
    }
}

impl<'a> JacobiSymbol<&'a Natural> for Natural {
    /// Computes the Jacobi symbol of two [`Natural`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).jacobi_symbol(&Natural::from(5u32)), 0);
    /// assert_eq!(Natural::from(7u32).jacobi_symbol(&Natural::from(5u32)), -1);
    /// assert_eq!(Natural::from(11u32).jacobi_symbol(&Natural::from(5u32)), 1);
    /// assert_eq!(Natural::from(11u32).jacobi_symbol(&Natural::from(9u32)), 1);
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: &'a Natural) -> i8 {
        assert_ne!(*other, 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(other)
    }
}

impl<'a> JacobiSymbol<Natural> for &'a Natural {
    /// Computes the Jacobi symbol of two [`Natural`]s, taking the first by reference and the second
    /// by value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).jacobi_symbol(Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).jacobi_symbol(Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).jacobi_symbol(Natural::from(5u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).jacobi_symbol(Natural::from(9u32)),
    ///     1
    /// );
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: Natural) -> i8 {
        assert_ne!(other, 0u32);
        assert!(other.odd());
        self.kronecker_symbol(&other)
    }
}

impl<'a, 'b> JacobiSymbol<&'a Natural> for &'b Natural {
    /// Computes the Jacobi symbol of two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).jacobi_symbol(&Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).jacobi_symbol(&Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).jacobi_symbol(&Natural::from(5u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).jacobi_symbol(&Natural::from(9u32)),
    ///     1
    /// );
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: &'a Natural) -> i8 {
        assert_ne!(*other, 0u32);
        assert!(other.odd());
        self.kronecker_symbol(other)
    }
}

impl KroneckerSymbol<Natural> for Natural {
    /// Computes the Kronecker symbol of two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).kronecker_symbol(Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).kronecker_symbol(Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     Natural::from(11u32).kronecker_symbol(Natural::from(5u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(11u32).kronecker_symbol(Natural::from(9u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(11u32).kronecker_symbol(Natural::from(8u32)),
    ///     -1
    /// );
    /// ```
    #[inline]
    fn kronecker_symbol(self, other: Natural) -> i8 {
        (&self).kronecker_symbol(&other)
    }
}

impl<'a> KroneckerSymbol<&'a Natural> for Natural {
    /// Computes the Kronecker symbol of two [`Natural`]s, taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).kronecker_symbol(&Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).kronecker_symbol(&Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     Natural::from(11u32).kronecker_symbol(&Natural::from(5u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(11u32).kronecker_symbol(&Natural::from(9u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     Natural::from(11u32).kronecker_symbol(&Natural::from(8u32)),
    ///     -1
    /// );
    /// ```
    #[inline]
    fn kronecker_symbol(self, other: &'a Natural) -> i8 {
        (&self).kronecker_symbol(other)
    }
}

impl<'a> KroneckerSymbol<Natural> for &'a Natural {
    /// Computes the Kronecker symbol of two [`Natural`]s, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).kronecker_symbol(Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).kronecker_symbol(Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).kronecker_symbol(Natural::from(5u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).kronecker_symbol(Natural::from(9u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).kronecker_symbol(Natural::from(8u32)),
    ///     -1
    /// );
    /// ```
    #[inline]
    fn kronecker_symbol(self, other: Natural) -> i8 {
        self.kronecker_symbol(&other)
    }
}

impl<'a, 'b> KroneckerSymbol<&'a Natural> for &'b Natural {
    /// Computes the Kronecker symbol of two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).kronecker_symbol(Natural::from(5u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).kronecker_symbol(Natural::from(5u32)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).kronecker_symbol(Natural::from(5u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).kronecker_symbol(Natural::from(9u32)),
    ///     1
    /// );
    /// assert_eq!(
    ///     (&Natural::from(11u32)).kronecker_symbol(Natural::from(8u32)),
    ///     -1
    /// );
    /// ```
    fn kronecker_symbol(self, other: &'a Natural) -> i8 {
        match (self, other) {
            (x, &Natural::ZERO) => i8::from(*x == 1u32),
            (&Natural::ZERO, y) => i8::from(*y == 1u32),
            (Natural(Small(x)), Natural(Small(y))) => {
                limbs_kronecker_symbol_single(true, *x, true, *y)
            }
            (Natural(Small(x)), Natural(Large(ys))) => {
                limbs_kronecker_symbol(true, &[*x], true, ys)
            }
            (Natural(Large(xs)), Natural(Small(y))) => {
                limbs_kronecker_symbol(true, xs, true, &[*y])
            }
            (Natural(Large(xs)), Natural(Large(ys))) => limbs_kronecker_symbol(true, xs, true, ys),
        }
    }
}
