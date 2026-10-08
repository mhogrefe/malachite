// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::SMALL_FMPZ_BITCOUNT_MAX;
use crate::natural::TWICE_WIDTH;
use core::cmp::min;
use malachite_base::num::logic::traits::SignificantBits;

// The kernels that multiply polynomials with small coefficients using word arithmetic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TinyKernel {
    // Single-word arithmetic, as in `_fmpz_poly_mul_tiny1` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    OneWord,
    // Double-word arithmetic, as in `_fmpz_poly_mul_tiny2` from `fmpz_poly/mul.c`, FLINT 3.6.0.
    TwoWord,
}

// The tiny kernel that FLINT's multiplication dispatchers choose, if any, for factors whose
// coefficients have at most `bits1` and `bits2` bits, the shorter of which has length `len2`.
// `short_enough` is the dispatcher's own condition on the lengths. Every coefficient of the
// product, and every partial sum of the terms that make it up, has at most `bits1 + bits2 +
// len2.significant_bits()` bits, which must fit in the kernel's accumulator.
pub(crate) fn tiny_kernel(
    bits1: u64,
    bits2: u64,
    len2: u64,
    short_enough: bool,
) -> Option<TinyKernel> {
    if bits1 > SMALL_FMPZ_BITCOUNT_MAX || bits2 > SMALL_FMPZ_BITCOUNT_MAX || !short_enough {
        return None;
    }
    let rbits = bits1 + bits2 + len2.significant_bits();
    if rbits <= SMALL_FMPZ_BITCOUNT_MAX {
        Some(TinyKernel::OneWord)
    } else if rbits < TWICE_WIDTH {
        Some(TinyKernel::TwoWord)
    } else {
        None
    }
}

// Whether FLINT's multiplication dispatchers choose classical multiplication for factors the
// shorter of which has length `len2`, and whose coefficients have at most `bits1` and `bits2` bits,
// when no tiny kernel applies.
pub(crate) fn classical_preferred(len2: u64, bits1: u64, bits2: u64) -> bool {
    len2 <= 6 && min(bits1, bits2) <= 5000
}

// Whether FLINT's multiplication dispatchers choose Karatsuba multiplication for the same factors,
// when neither a tiny kernel nor classical multiplication applies.
pub(crate) fn karatsuba_preferred(len2: u64, bits1: u64, bits2: u64) -> bool {
    len2 <= 4 || (len2 <= 8 && (1500..=10000).contains(&(bits1 + bits2)))
}

// Whether the multiplication dispatchers choose Schönhage–Strassen multiplication over Kronecker
// substitution, for factors of lengths `len1 >= len2`, whose coefficients have at most `bits1` and
// `bits2` bits, when neither a tiny kernel, classical multiplication, nor Karatsuba multiplication
// applies. `max_len` bounds `len1 + len2`: once the transform is long enough, its coefficients must
// grow with the length, and Schönhage–Strassen falls behind. That happens once a product has
// more than 4096 coefficients, so products use 4097; a middle product of factors of lengths $2n -
// 1$ and $n$ uses a transform as long as the first factor, and uses 3071, so that $n \leq 1024$.
//
// The window was measured (see `tune_poly_mul_grid` in bin_util/tune.rs, on factors of equal length
// and coefficient size, and on the middle product of factors of lengths $2n - 1$ and $n$) and is
// wider than FLINT's single-threaded one, which takes `len2` from 8 to 75 and `bits1 + bits2` from
// 800 to 4000: Schönhage–Strassen is also faster than Kronecker substitution for longer factors
// with coefficients of several hundred to a few thousand bits. FLINT also chooses it for very long
// inputs with very large coefficients, but only when it may use at least 4 threads, and Malachite
// is single-threaded.
pub(crate) fn schonhage_strassen_preferred(
    len1: u64,
    len2: u64,
    bits1: u64,
    bits2: u64,
    max_len: u64,
) -> bool {
    let bits = bits1 + bits2;
    len1 + len2 <= max_len
        && match len2 {
            0..=7 => false,
            8..=15 => (1000..=4000).contains(&bits),
            16..=100 => (800..=4000).contains(&bits),
            _ => (1000..=5000).contains(&bits),
        }
}

// Whether FLINT's multiplication dispatchers try the small-prime FFT first, for factors the shorter
// of which has length `len2`, and whose coefficients have at most `bits1` and `bits2` bits: when
// the shorter factor has at least `min_len` coefficients, and either the product's coefficients are
// small or large enough, or it has at least `always_len`.
pub(crate) const fn fft_preferred(
    len2: u64,
    bits1: u64,
    bits2: u64,
    min_len: u64,
    always_len: u64,
) -> bool {
    let bits = bits1 + bits2;
    len2 >= min_len && (bits <= 40 || bits >= 128 || len2 >= always_len)
}
