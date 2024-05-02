// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_mullo_n`, `mpn_mullo_n_itch`, and `mpn_dc_mullo_n` contributed to the GNU project by
//      Torbjörn Granlund and Marco Bodrato.
//
//      Copyright © 2000, 2002, 2004, 2005, 2009, 2010, 2012, 2015 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::toom::{TUNE_PROGRAM_BUILD, WANT_FAT_BINARY};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out_basecase, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len,
};
use crate::platform::Limb;
use crate::platform::{
    MULLO_BASECASE_THRESHOLD, MULLO_DC_THRESHOLD, MULLO_MUL_N_THRESHOLD, MUL_TOOM22_THRESHOLD,
    MUL_TOOM33_THRESHOLD, MUL_TOOM44_THRESHOLD, MUL_TOOM8H_THRESHOLD,
};
use malachite_base::num::arithmetic::traits::WrappingAddAssign;

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_mullo_basecase` from `mpn/generic/mullo_basecase.c`, GMP 6.2.1,
// `MULLO_VARIANT == 2`.
pub_crate_test! {limbs_mul_low_same_length_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_eq!(ys.len(), n);
    let (ys_last, ys_init) = ys.split_last().unwrap();
    let (out_last, out_init) = out[..n].split_last_mut().unwrap();
    let mut p = xs[0].wrapping_mul(*ys_last);
    if n != 1 {
        let y = ys_init[0];
        let (xs_last, xs_init) = xs.split_last().unwrap();
        let product = xs_last
            .wrapping_mul(y)
            .wrapping_add(limbs_mul_limb_to_out(out_init, xs_init, y));
        p.wrapping_add_assign(product);
        let m = n - 1;
        for i in 1..m {
            let y = ys_init[i];
            let (xs_lo, xs_hi) = xs_init.split_at(m - i);
            let limb_p = xs_hi[0].wrapping_mul(y).wrapping_add(
                limbs_slice_add_mul_limb_same_length_in_place_left(&mut out_init[i..], xs_lo, y),
            );
            p.wrapping_add_assign(limb_p);
        }
    }
    *out_last = p;
}}

// TODO tune
const SCALED_MUL_TOOM22_THRESHOLD: usize = MUL_TOOM22_THRESHOLD * 36 / (36 - 11);
// TODO tune
const SCALED_MUL_TOOM33_THRESHOLD: usize = MUL_TOOM33_THRESHOLD * 36 / (36 - 11);
// TODO tune
const SCALED_MUL_TOOM44_THRESHOLD: usize = MUL_TOOM44_THRESHOLD.saturating_mul(40) / (40 - 9);
// TODO tune
const SCALED_MUL_TOOM8H_THRESHOLD: usize = MUL_TOOM8H_THRESHOLD * 10 / 9;

// TODO tune
const MAYBE_RANGE_BASECASE_MUL_LOW: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (MULLO_DC_THRESHOLD == 0 && MULLO_BASECASE_THRESHOLD < SCALED_MUL_TOOM22_THRESHOLD
        || MULLO_DC_THRESHOLD != 0 && MULLO_DC_THRESHOLD < SCALED_MUL_TOOM22_THRESHOLD);

// TODO tune
const MAYBE_RANGE_TOOM22_MUL_LOW: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (MULLO_DC_THRESHOLD == 0 && MULLO_BASECASE_THRESHOLD < SCALED_MUL_TOOM33_THRESHOLD
        || MULLO_DC_THRESHOLD != 0 && MULLO_DC_THRESHOLD < SCALED_MUL_TOOM33_THRESHOLD);

// We need fractional approximation of the value 0 < a <= 1/2 giving the minimum in the function k =
// (1 - a) ^ e / (1 - 2 * a ^ e).
const fn get_n_lo(n: usize) -> usize {
    if MAYBE_RANGE_BASECASE_MUL_LOW && n < SCALED_MUL_TOOM22_THRESHOLD {
        n >> 1
    } else if MAYBE_RANGE_TOOM22_MUL_LOW && n < SCALED_MUL_TOOM33_THRESHOLD {
        n * 11 / 36 // n_lo ~= n * (1 - .694...)
    } else if n < SCALED_MUL_TOOM44_THRESHOLD {
        n * 9 / 40 // n_lo ~= n * (1 - .775...)
    } else if n < SCALED_MUL_TOOM8H_THRESHOLD {
        n * 7 / 39 // n_lo ~= n * (1 - .821...)
    } else {
        n / 10 // n_lo ~= n * (1 - .899...) [TOOM88]
    }
}

// See `limbs_mul_low_same_length_divide_and_conquer` documentation for more details.
//
// # Worst-case complexity
// $T(n) = O(n^{\log_8 15}) \approx O(n^{1.302})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_dc_mullo_n` from `mpn/generic/mullo_n.c`, GMP 6.2.1, where `rp == tp`.
pub_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_mul_low_same_length_divide_and_conquer_shared_scratch(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert!(n >= 2);
    let n_lo = get_n_lo(n);
    let n_hi = n - n_lo;
    // Split as x = x_1 *  2 ^ (n_hi * Limb::WIDTH) + x_0, y = y_1 * 2 ^ (n_hi * Limb::WIDTH) + y_0
    let (xs_lo, xs_hi) = xs.split_at(n_hi);
    // x_0 * y_0
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(n_hi)];
    limbs_mul_same_length_to_out(out, xs_lo, &ys[..n_hi], &mut mul_scratch);
    let ys_lo = &ys[..n_lo];
    let (out_lo, out_hi) = out.split_at_mut(n);
    // x_1 * y_0 * 2 ^ (n_hi * Limb::WIDTH)
    if n_lo < MULLO_BASECASE_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out_hi, xs_hi, ys_lo);
    } else if n_lo < MULLO_DC_THRESHOLD {
        limbs_mul_low_same_length_basecase(out_hi, xs_hi, ys_lo);
    } else {
        limbs_mul_low_same_length_divide_and_conquer_shared_scratch(out_hi, xs_hi, ys_lo);
    }
    limbs_slice_add_same_length_in_place_left(&mut out_lo[n_hi..], &out_hi[..n_lo]);
    let xs_lo = &xs[..n_lo];
    let ys_hi = &ys[n_hi..];
    // x_0 * y_1 * 2 ^ (n_hi * Limb::WIDTH)
    if n_lo < MULLO_BASECASE_THRESHOLD {
        limbs_mul_greater_to_out_basecase(out_hi, xs_lo, ys_hi);
    } else if n_lo < MULLO_DC_THRESHOLD {
        limbs_mul_low_same_length_basecase(out_hi, xs_lo, ys_hi);
    } else {
        limbs_mul_low_same_length_divide_and_conquer_shared_scratch(out_hi, xs_lo, ys_hi);
    }
    limbs_slice_add_same_length_in_place_left(&mut out_lo[n_hi..], &out_hi[..n_lo]);
}}

// Compute the least significant half of the product {xs, n} * {ys, n}, or formally {rp, n} = {xs,
// n} * {ys, n} mod (2 ^ `Limb::WIDTH * n`).
//
// Above the given threshold, the Divide and Conquer strategy is used. The operands are split in
// two, and a full product plus two mul_low are used to obtain the final result. The more natural
// strategy is to split in two halves, but this is far from optimal when a sub-quadratic
// multiplication is used.
//
// Mulders suggests an unbalanced split in favour of the full product, split n = n_lo + n_hi, where
// a * n = n_lo <= n_hi = (1 - a) * n; i.e. 0 < a <= 1/2.
//
// To compute the value of a, we assume that the cost of mul_lo for a given size ML(n) is a fraction
// of the cost of a full product with same size M(n), and the cost M(n) = n ^ e for some exponent 1
// < e <= 2. Then we can write:
//
// ML(n) = 2 * ML(a * n) + M((1 - a) * n) => k * M(n) = 2 * k * M(n) * a ^ e + M(n) * (1 - a) ^ e
//
// Given a value for e, want to minimise the value of k, i.e. the function k = (1 - a) ^ e / (1 - 2
// * a ^ e).
//
// With e = 2, the exponent for schoolbook multiplication, the minimum is given by the values a = 1
// - a = 1/2.
//
// With e = log(3) / log(2), the exponent for Karatsuba (aka toom22), Mulders computes (1 - a) =
// 0.694... and we approximate a with 11 / 36.
//
// Other possible approximations follow:
// - e = log(5) / log(3) [Toom-3] -> a ~= 9/40
// - e = log(7) / log(4) [Toom-4] -> a ~= 7/39
// - e = log(11) / log(6) [Toom-6] -> a ~= 1/8
// - e = log(15) / log(8) [Toom-8] -> a ~= 1/10
//
// The values above where obtained with the following trivial commands in the gp-pari shell:
//
// - fun(e,a)=(1-a)^e/(1-2*a^e)
// - mul(a,b,c)={local(m,x,p);if(b-c<1/10000,(b+c)/2,m=1;x=b;
// - forstep(p=c,b,(b-c)/8,if(fun(a,p)<m,m=fun(a,p);x=p));mul(a,(b+x)/2,(c+x)/2))}
//
// - contfracpnqn(contfrac(mul(log(2*2-1)/log(2),1/2,0),5))
// - contfracpnqn(contfrac(mul(log(3*2-1)/log(3),1/2,0),5))
// - contfracpnqn(contfrac(mul(log(4*2-1)/log(4),1/2,0),5))
// - contfracpnqn(contfrac(mul(log(6*2-1)/log(6),1/2,0),3))
// - contfracpnqn(contfrac(mul(log(8*2-1)/log(8),1/2,0),3))
//
// ```
// ,
// |\
// | \
// +----,
// |    |
// |    |
// |    |\
// |    | \
// +----+--`
// ^n_hi^__^ <- n_low
// ```
//
// For an actual implementation, the assumption that M(n) = n ^ e is incorrect, and, as a
// consequence, the assumption that ML(n) = k * M(n) with a constant k is wrong.
//
// But theory suggests us two things:
// - the faster multiplication is (the lower e is), the more k approaches 1 and a approaches 0.
//
// - A smaller-than-optimal value for a is probably less bad than a bigger one: e.g. let e = log(3)
//   / log(2), a = 0.3058... (the optimal value), and k(a) = 0.808...,  the mul / mul_low speed
//   ratio. We get k * (a + 1 / 6) = 0.929..., but k(a - 1/6) = 0.865....
//
// # Worst-case complexity
// $T(n) = O(n^{\log_8 15}) \approx O(n^{1.302})$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_dc_mullo_n` from `mpn/generic/mullo_n.c`, GMP 6.2.1, where `rp != tp`.
pub_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_mul_low_same_length_divide_and_conquer(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert!(n >= 2);
    let n_lo = get_n_lo(n);
    let n_hi = n - n_lo;
    let (out_lo, out_hi) = out[..n].split_at_mut(n_hi);
    // Split as x = x_1 *  2 ^ (n_hi * Limb::WIDTH) + x_0, y = y_1 * 2 ^ (n_hi * Limb::WIDTH) + y_0
    let (xs_lo, xs_hi) = xs.split_at(n_hi);
    // x_0 * y_0
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(n_hi)];
    limbs_mul_same_length_to_out(scratch, xs_lo, &ys[..n_hi], &mut mul_scratch);
    out_lo.copy_from_slice(&scratch[..n_hi]);
    let ys_lo = &ys[..n_lo];
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(n);
    // x_1 * y_0 * 2 ^ (n_hi * Limb::WIDTH)
    if n_lo < MULLO_BASECASE_THRESHOLD {
        limbs_mul_greater_to_out_basecase(scratch_hi, xs_hi, ys_lo);
    } else if n_lo < MULLO_DC_THRESHOLD {
        limbs_mul_low_same_length_basecase(scratch_hi, xs_hi, ys_lo);
    } else {
        limbs_mul_low_same_length_divide_and_conquer_shared_scratch(scratch_hi, xs_hi, ys_lo);
    }
    limbs_add_same_length_to_out(out_hi, &scratch_lo[n_hi..], &scratch_hi[..n_lo]);
    let xs_lo = &xs[..n_lo];
    let ys_hi = &ys[n_hi..];
    // x_0 * y_1 * 2 ^ (n_hi * Limb::WIDTH)
    if n_lo < MULLO_BASECASE_THRESHOLD {
        limbs_mul_greater_to_out_basecase(scratch_hi, xs_lo, ys_hi);
    } else if n_lo < MULLO_DC_THRESHOLD {
        limbs_mul_low_same_length_basecase(scratch_hi, xs_lo, ys_hi);
    } else {
        limbs_mul_low_same_length_divide_and_conquer_shared_scratch(scratch_hi, xs_lo, ys_hi);
    }
    limbs_slice_add_same_length_in_place_left(out_hi, &scratch_hi[..n_lo]);
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_mullo_n_itch` from `mpn/generic/mullo_n.c`, GMP 6.2.1.
pub_const_test! {limbs_mul_low_same_length_divide_and_conquer_scratch_len(n: usize) -> usize {
    n << 1
}}

// TODO tune
const MULLO_BASECASE_THRESHOLD_LIMIT: usize = MULLO_BASECASE_THRESHOLD;

pub_test! {limbs_mul_low_same_length_large(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    // For really large operands, use plain limbs_mul_same_length_to_out but throw away the upper n
    // limbs of the result.
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(n)];
    limbs_mul_same_length_to_out(scratch, xs, ys, &mut mul_scratch);
    out.copy_from_slice(&scratch[..n]);
}}

// Multiply two n-limb numbers and return the lowest n limbs of their products.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$, assuming $k = O(\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_mullo_n` from `mpn/generic/mullo_n.c`, GMP 6.2.1.
pub_crate_test! {
#[allow(clippy::absurd_extreme_comparisons)]
limbs_mul_low_same_length(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    assert!(n >= 1);
    let out = &mut out[..n];
    if n < MULLO_BASECASE_THRESHOLD {
        // Allocate workspace of fixed size on stack: fast!
        let scratch = &mut [0; MULLO_BASECASE_THRESHOLD_LIMIT];
        limbs_mul_greater_to_out_basecase(scratch, xs, ys);
        out.copy_from_slice(&scratch[..n]);
    } else if n < MULLO_DC_THRESHOLD {
        limbs_mul_low_same_length_basecase(out, xs, ys);
    } else {
        let mut scratch = vec![0; limbs_mul_low_same_length_divide_and_conquer_scratch_len(n)];
        if n < MULLO_MUL_N_THRESHOLD {
            limbs_mul_low_same_length_divide_and_conquer(out, xs, ys, &mut scratch);
        } else {
            limbs_mul_low_same_length_large(out, xs, ys, &mut scratch);
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_mullo_basecase` from `mpn/generic/mullo_basecase.c`, GMP 6.2.1,
// `MULLO_VARIANT == 1`.
pub_crate_test! {limbs_mul_low_same_length_basecase_alt(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb]
) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert_eq!(ys.len(), n);
    let out = &mut out[..n];
    limbs_mul_limb_to_out(out, xs, ys[0]);
    for i in 1..n {
        limbs_slice_add_mul_limb_same_length_in_place_left(&mut out[i..], &xs[..n - i], ys[i]);
    }
}}
