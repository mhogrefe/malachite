// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul_middle::classical::mul_middle_to_out_classical;
use crate::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use crate::integer_polynomial::arithmetic::mul_middle::kronecker::mul_middle_to_out_kronecker;
use crate::integer_polynomial::arithmetic::mul_middle::tiny::{
    mul_middle_to_out_tiny_1, mul_middle_to_out_tiny_2,
};
use crate::integer_polynomial::arithmetic::mul_truncated::karatsuba::mul_truncated_to_out_karatsuba;
use crate::integer_polynomial::arithmetic::scalar_mul::integers_mul_scalar_to_out;
use crate::integer_polynomial::arithmetic::vec::max_bits::vec_max_bits;
use crate::integer_polynomial::arithmetic::vec::{
    TinyKernel, classical_preferred, fft_preferred, karatsuba_preferred, tiny_kernel,
};
use core::cmp::min;
use core::mem::swap;
use core::ptr;
use malachite_base::num::conversion::traits::ExactFrom;

pub mod classical;
pub mod fft;
pub mod kronecker;
pub mod tiny;

// Narrows the polynomials with coefficients `xs` and `ys` to the coefficients that can contribute
// to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of their product, returning the narrowed
// slices and the range shifted to match. A coefficient of $x^j$ with $j \geq$ `nhi` contributes
// only above the range, and dropping low coefficients of a factor shifts the whole product down.
//
// This is equivalent to the input truncation at the start of `_fmpz_poly_mulmid` from
// `fmpz_poly/mulmid.c`, FLINT 3.6.0.
pub(crate) fn truncate_mul_middle_inputs<'a>(
    xs: &'a [Integer],
    ys: &'a [Integer],
    mut nlo: usize,
    mut nhi: usize,
) -> (&'a [Integer], &'a [Integer], usize, usize) {
    // Low truncation of inputs
    let mut xs = &xs[..min(xs.len(), nhi)];
    let mut ys = &ys[..min(ys.len(), nhi)];
    // High truncation of inputs
    let nlo2 = xs.len() + ys.len() - 1 - nlo;
    if xs.len() > nlo2 {
        let trunc = xs.len() - nlo2;
        xs = &xs[trunc..];
        nlo -= trunc;
        nhi -= trunc;
    }
    if ys.len() > nlo2 {
        let trunc = ys.len() - nlo2;
        ys = &ys[trunc..];
        nlo -= trunc;
        nhi -= trunc;
    }
    (xs, ys, nlo, nhi)
}

// Sets `out` to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of the product of the
// polynomials with coefficients `xs` and `ys`, both nonempty. `nlo < nhi <= xs.len() + ys.len() -
// 1` must hold, and `out` must have length `nhi - nlo`.
//
// # Worst-case complexity
// $T(n, m) = O(n^{\log_2 3} m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n) \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is the
// largest number of significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mulmid` from `fmpz_poly/mulmid.c`, FLINT 3.6.0. FLINT's first
// choice for long inputs, which multiplies polynomials directly with its small-prime FFT
// (`_fmpz_poly_mul_mid_default_mpn_ctx` from `fft_small/fmpz_poly_mul.c`), and
// Schönhage–Strassen, which it chooses for some inputs of medium size, have not been ported yet;
// Kronecker substitution stands in for both. (Its single integer multiplication reaches the port of
// the small-prime FFT's integer multiplication in `natural/arithmetic/mul/fft.rs` when the operands
// are large.)
crate_test_fn! {mul_middle_to_out(
    out: &mut [Integer],
    xs: &[Integer],
    ys: &[Integer],
    nlo: usize,
    nhi: usize,
) {
    assert_ne!(xs.len(), 0);
    assert_ne!(ys.len(), 0);
    assert!(nlo < nhi);
    assert!(nhi < xs.len() + ys.len());
    let (mut xs, mut ys, nlo, nhi) = truncate_mul_middle_inputs(xs, ys, nlo, nhi);
    let len = nhi - nlo;
    if xs.len() < ys.len() {
        swap(&mut xs, &mut ys);
    }
    if ys.len() == 1 {
        integers_mul_scalar_to_out(out, &xs[nlo..nhi], &ys[0]);
        return;
    }
    let bits1 = vec_max_bits(xs).0;
    let bits2 = if ptr::eq(xs, ys) { bits1 } else { vec_max_bits(ys).0 };
    let len2 = u64::exact_from(ys.len());
    if fft_preferred(len2, bits1, bits2, 100, 200) && mul_middle_to_out_fft(out, xs, ys, nlo, nhi) {
        return;
    }
    let len = u64::exact_from(len);
    let short_enough = len2 < 50 || (len2 << 2 >= 3 * len && len < 150 + bits1 + bits2);
    match tiny_kernel(bits1, bits2, len2, short_enough) {
        Some(TinyKernel::OneWord) => mul_middle_to_out_tiny_1(out, xs, ys, nlo, nhi),
        Some(TinyKernel::TwoWord) => mul_middle_to_out_tiny_2(out, xs, ys, nlo, nhi),
        None if nhi <= 8 || classical_preferred(len2, bits1, bits2) || len <= 3 => {
            mul_middle_to_out_classical(out, xs, ys, nlo, nhi);
        }
        None if nlo == 0 && karatsuba_preferred(len2, bits1, bits2) => {
            mul_truncated_to_out_karatsuba(out, xs, ys);
        }
        None => {
            // Schönhage–Strassen, which FLINT chooses instead for some inputs of medium size,
            // has not been ported yet, so Kronecker substitution stands in for it.
            mul_middle_to_out_kronecker(out, xs, ys, nlo, nhi);
        }
    }
}}
