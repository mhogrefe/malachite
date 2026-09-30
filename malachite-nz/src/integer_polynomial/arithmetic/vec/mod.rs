// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::InnerNatural::Small;
use crate::natural::{LIMB_MAX_QUARTER, Natural, TWICE_WIDTH, WIDTH_MINUS_2};
use crate::platform::{Limb, SignedLimb};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::cmp::min;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;

pub mod dot_general;
pub mod max_bits;
pub mod max_limbs;

// The largest absolute value of a small FLINT `fmpz`, which is stored in a single word with two
// bits to spare; larger values are stored as GMP integers. Some of FLINT's algorithms take a fast
// path for small values that depends on the spare bits, so a faithful translation takes it for the
// same values.
pub(crate) const COEFF_MAX: Limb = LIMB_MAX_QUARTER;

// The largest number of bits that FLINT stores in a small `fmpz`: `SMALL_FMPZ_BITCOUNT_MAX` from
// `flint.h`, FLINT 3.6.0.
pub(crate) const SMALL_FMPZ_BITCOUNT_MAX: u64 = WIDTH_MINUS_2;

// The value of `x` as a signed word, if FLINT would store `x` as a small `fmpz`; that is, if
// `COEFF_IS_MPZ` from `flint.h`, FLINT 3.6.0, would be false.
pub(crate) fn small_value(x: &Integer) -> Option<SignedLimb> {
    match x.abs {
        Natural(Small(small)) if small <= COEFF_MAX => {
            let value = SignedLimb::wrapping_from(small);
            Some(if x.sign { value } else { -value })
        }
        _ => None,
    }
}

// The values of the elements of `xs`, each of which FLINT must store as a small `fmpz`, as signed
// words or as a wider signed type.
pub(crate) fn small_values<T: From<SignedLimb>>(xs: &[Integer]) -> Vec<T> {
    xs.iter()
        .map(|x| T::from(small_value(x).unwrap()))
        .collect()
}

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

// Sets each element of `out` to the sum of the elements of `xs` and `ys` at the same index.
//
// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0, with the output
// separate from the inputs.
pub(crate) fn vec_add<T: Borrow<Integer>>(out: &mut [Integer], xs: &[T], ys: &[T]) {
    for ((o, x), y) in out.iter_mut().zip(xs).zip(ys) {
        *o = x.borrow() + y.borrow();
    }
}

// Adds each element of `ys` to the element of `xs` at the same index.
//
// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0, with the output the
// same as the first input.
pub(crate) fn vec_add_assign(xs: &mut [Integer], ys: &[Integer]) {
    for (x, y) in xs.iter_mut().zip(ys) {
        *x += y;
    }
}

// Subtracts each element of `ys` from the element of `xs` at the same index.
//
// This is equivalent to `_fmpz_vec_sub` from `fmpz_vec/sub.c`, FLINT 3.6.0, with the output the
// same as the first input.
pub(crate) fn vec_sub_assign(xs: &mut [Integer], ys: &[Integer]) {
    for (x, y) in xs.iter_mut().zip(ys) {
        *x -= y;
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
