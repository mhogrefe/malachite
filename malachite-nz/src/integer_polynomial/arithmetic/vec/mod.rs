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
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;

pub mod dot_general;
pub mod max_bits;

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
