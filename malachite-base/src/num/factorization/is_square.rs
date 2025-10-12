// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL as published by the Free Software Foundation; either version
// 3 of the License, or (at your option any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedSqrt, FloorSqrt, Square};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::conversion::traits::{SplitInHalf, WrappingFrom};
use crate::num::factorization::traits::IsSquare;

const IS_SQUARE_MOD64: [bool; 64] = [
    true, true, false, false, true, false, false, false, false, true, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, true, false, false,
    false, false, false, false, false, true, false, false, true, false, false, false, false, true,
    false, false, false, false, false, false, false, true, false, false, false, false, false,
    false, false, true, false, false, false, false, false, false,
];

const IS_SQUARE_MOD65: [bool; 65] = [
    true, true, false, false, true, false, false, false, false, true, true, false, false, false,
    true, false, true, false, false, false, false, false, false, false, false, true, true, false,
    false, true, true, false, false, false, false, true, true, false, false, true, true, false,
    false, false, false, false, false, false, false, true, false, true, false, false, false, true,
    true, false, false, false, false, true, false, false, true,
];

const IS_SQUARE_MOD63: [bool; 63] = [
    true, true, false, false, true, false, false, true, false, true, false, false, false, false,
    true, false, true, false, true, false, false, false, true, false, false, true, false, false,
    true, false, false, false, false, false, false, true, true, true, false, false, false, false,
    false, true, false, false, true, false, false, true, false, false, false, false, false, false,
    true, false, true, false, false, false, false,
];

// This is n_is_square when FLINT64 is false, from ulong_extras/is_square.c, FLINT 3.1.2.
fn is_square_u64(x: u64) -> bool {
    IS_SQUARE_MOD64[(x % 64) as usize]
        && IS_SQUARE_MOD63[(x % 63) as usize]
        && IS_SQUARE_MOD65[(x % 65) as usize]
        && x.floor_sqrt().square() == x
}

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl IsSquare for $t {
            /// Determines whether an integer is a perfect square.
            ///
            /// $f(x) = (\exists b \in \Z : b^2 = x)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_square#is_square).
            #[inline]
            fn is_square(&self) -> bool {
                is_square_u64(u64::wrapping_from(*self))
            }
        }
    };
}
impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(usize);

// From mpn/generic/mod_34lsub1.c
const B1: u64 = u64::WIDTH >> 2;
const B2: u64 = B1 * 2;
const B3: u64 = B1 * 3;

const M2: u64 = (1 << B2) - 1;
const M3: u64 = (1 << B3) - 1;

const fn low0(n: u64) -> u64 {
    n & M3
}

const fn high0(n: u64) -> u64 {
    n >> B3
}

const fn low1(n: u64) -> u64 {
    (n & M2) << B1
}

const fn high1(n: u64) -> u64 {
    n >> B2
}

// This is mpn_mod_34lsub1 from mpn/generic/mod_34lsub1.c, GMP 6.3.0.
//
// Calculate a remainder from `limbs` divided by 2^(u64::WIDTH*3/4)-1. The remainder is not fully
// reduced, it's any limb value congruent to `limbs` modulo that divisor.
//
// Check gen-psqr.c. mpn_mod_34lsub1 preferred over mpn_mod_1 (plus a PERFSQR_PP modulus) with 32
// and 64 bit limb.
const fn mod_34lsub1(x_hi: u64, x_lo: u64) -> u64 {
    low0(x_lo) + high0(x_lo) + low1(x_hi) + high1(x_hi)
}

const MOD34_BITS: u64 = (u64::WIDTH >> 2) * 3;
const MOD34_MASK: u64 = (1 << MOD34_BITS) - 1;

const fn perfsqr_mod_34(x_hi: u64, x_lo: u64) -> u64 {
    let r = mod_34lsub1(x_hi, x_lo);
    (r & MOD34_MASK) + (r >> MOD34_BITS)
}

// This is PERFSQR_MOD_BITS from mpn/perfsqr.h, GMP 6.3.0. Either 49 on 64 bit limb or 25 on 32 bit
// limb. 2^48-1 = 3^2 * 5 * 7 * 13 * 17 * 97 ... 2^24-1 = 3^2 * 5 * 7 * 13 * 17 ...
const SQR_MOD_BITS: u64 = MOD34_BITS + 1;
const SQR_MOD_MASK: u64 = (1 << SQR_MOD_BITS) - 1;

const fn perfsqr_mod_idx(r: u64, d: u64, inv: u64) -> u64 {
    assert!(r <= SQR_MOD_MASK);
    assert!(inv.wrapping_mul(d) & SQR_MOD_MASK == 1);
    assert!(u64::MAX / d >= SQR_MOD_MASK);
    let q = r.wrapping_mul(inv) & SQR_MOD_MASK;
    assert!(r == (q.wrapping_mul(d) & SQR_MOD_MASK));
    q.wrapping_mul(d) >> SQR_MOD_BITS
}

// Single limb. Check precomputed bitmasks to see if remainder is a quadratic residue
fn perfsqr_mod_1(r: u64, d: u64, inv: u64, mask: u64) -> bool {
    assert!(d <= u64::WIDTH);
    let idx = perfsqr_mod_idx(r, d, inv);
    if (mask >> idx) & 1 == 0 {
        // non-square
        return false;
    }
    true
}

// Double limb. Check precomputed bitmasks to see if remainder is a quadratic residue
fn perfsqr_mod_2(r: u64, d: u64, inv: u64, mhi: u64, mlo: u64) -> bool {
    assert!(d <= const { 2 * u64::WIDTH });
    let mut idx = perfsqr_mod_idx(r, d, inv);
    let m = if idx < u64::WIDTH { mlo } else { mhi };
    idx %= u64::WIDTH;
    if (m >> idx) & 1 == 0 {
        // non-square
        return false;
    }
    true
}

// This test identifies 97.81% as non-squares. Grand total sq_res_0x100 and PERFSQR_MOD_TEST, 99.62%
// non-squares.
fn perfsqr_mod_test(x: u128) -> bool {
    let (x_hi, x_lo) = x.split_in_half();
    let r = perfsqr_mod_34(x_hi, x_lo);
    perfsqr_mod_2(r, 91, 0xfd2fd2fd2fd3, 0x2191240, 0x8850a206953820e1) // 69.23%
        && perfsqr_mod_2(r, 85, 0xfcfcfcfcfcfd, 0x82158, 0x10b48c4b4206a105) // 68.24%
        && perfsqr_mod_1(r, 9, 0xe38e38e38e39, 0x93)  // 55.56%
        && perfsqr_mod_2(r, 97, 0xfd5c5f02a3a1, 0x1eb628b47, 0x6067981b8b451b5f) // 49.48%
}

// This is sq_res0x100 from mpn/perfsqr.h when generated for 64 bit limb, GMP 6.3.0. Non-zero bit
// indicates a quadratic residue mod 0x100. This test identifies 82.81% as non-squares (212/256).
const SQR_MOD256: [u64; 4] =
    [0x202021202030213, 0x202021202020213, 0x202021202030212, 0x202021202020212];

impl IsSquare for u128 {
    /// Determines whether an integer is a perfect square.
    ///
    /// $f(x) = (\exists b \in \Z : b^2 = x)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::is_square#is_square).
    #[inline]
    fn is_square(&self) -> bool {
        let idx = self % 0x100; // mod 256

        // The first test excludes 212/256 (82.8%) of the perfect square candidates in O(1) time.
        //
        // This just checks the particular bit in the bitmask SQR_MOD256_U64 encoding where the
        // input can be a perfect square mod 256.
        if (SQR_MOD256[(idx >> u64::LOG_WIDTH) as usize]
            >> (idx & const { u64::WIDTH_MASK as Self }))
            & 1
            == 0
        {
            return false;
        }
        // The second test uses mpn_mod_34lsub1 to detect non-squares according to their residues
        // modulo small primes (or powers of primes). See mpn/perfsqr.h, GMP 6.3.0.
        if !perfsqr_mod_test(*self) {
            return false;
        }
        // For the third and last test, we finally compute the square root, to make sure we've
        // really got a perfect square.
        self.checked_sqrt().is_some()
    }
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl IsSquare for $t {
            /// Determines whether an integer is a perfect square.
            ///
            /// $f(x) = (\exists b \in \Z : b^2 = x)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_square#is_square).
            #[inline]
            fn is_square(&self) -> bool {
                if *self < 0 {
                    false
                } else {
                    self.unsigned_abs().is_square()
                }
            }
        }
    };
}
apply_to_signeds!(impl_signed);
