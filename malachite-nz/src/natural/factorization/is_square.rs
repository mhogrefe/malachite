// Copyright © 2025 William Youmans
//
// Uses code adopted from the GMP Library.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::natural::arithmetic::sqrt::limbs_checked_sqrt;
use crate::platform::Limb;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::factorization::traits::IsSquare;

const MOD34_BITS: Limb = ((Limb::WIDTH as Limb) / 4) * 3;
const MOD34_MASK: Limb = (1 << MOD34_BITS) - 1;

// This is PERFSQR_MOD_BITS from mpn/perfsqr.h, GMP 6.3.0. Either 49 on 64 bit limb or 25 on 32 bit
// limb. 2^48-1 = 3^2 * 5 * 7 * 13 * 17 * 97 ... 2^24-1 = 3^2 * 5 * 7 * 13 * 17 ...
const SQR_MOD_BITS: Limb = MOD34_BITS + 1;
const SQR_MOD_MASK: Limb = (1 << SQR_MOD_BITS) - 1;

// From mpn/generic/mod_34lsub1.c
const B1: Limb = (Limb::WIDTH as Limb) / 4;
const B2: Limb = B1 * 2;
const B3: Limb = B1 * 3;

const M1: Limb = (1 << B1) - 1;
const M2: Limb = (1 << B2) - 1;
const M3: Limb = (1 << B3) - 1;

const fn low0(n: Limb) -> Limb {
    n & M3
}
const fn high0(n: Limb) -> Limb {
    n >> B3
}

const fn low1(n: Limb) -> Limb {
    (n & M2) << B1
}
const fn high1(n: Limb) -> Limb {
    n >> B2
}

const fn low2(n: Limb) -> Limb {
    (n & M1) << B2
}
const fn high2(n: Limb) -> Limb {
    n >> B1
}

const fn parts0(n: Limb) -> Limb {
    low0(n) + high0(n)
}
const fn parts1(n: Limb) -> Limb {
    low1(n) + high1(n)
}
const fn parts2(n: Limb) -> Limb {
    low2(n) + high2(n)
}

// This is mpn_mod_34lsub1 from mpn/generic/mod_34lsub1.c, GMP 6.3.0.
//
// Calculate a remainder from `limbs` divided by 2^(Limb::WIDTH*3/4)-1. The remainder is not fully
// reduced, it's any limb value congruent to `limbs` modulo that divisor.
//
// Check gen-psqr.c. mpn_mod_34lsub1 preferred over mpn_mod_1 (plus a PERFSQR_PP modulus) with 32
// and 64 bit limb.
pub(crate) fn mod_34lsub1(limbs: &[Limb]) -> Limb {
    // Process in chunks of 3, chunks lets us cleanly handle remainder
    let (sums, carries) = limbs.chunks(3).fold(
        ([Limb::ZERO; 3], [Limb::ZERO; 3]),
        |(mut sums, mut carries), chunk| {
            for (i, &limb) in chunk.iter().enumerate() {
                let (sum, overflow) = sums[i].overflowing_add(limb);
                sums[i] = sum;
                if overflow {
                    carries[i] += 1;
                }
            }
            (sums, carries)
        },
    );
    parts0(sums[0])
        + parts1(sums[1])
        + parts2(sums[2])
        + parts1(carries[0])
        + parts2(carries[1])
        + parts0(carries[2])
}

// reduce mod 2^(WIDTH*3/4) - 1 and ensure result is in [0, modulus)
fn perfsqr_mod_34(limbs: &[Limb]) -> Limb {
    let r = mod_34lsub1(limbs);
    (r & MOD34_MASK) + (r >> MOD34_BITS)
}

const fn perfsqr_mod_idx(r: Limb, d: Limb, inv: Limb) -> Limb {
    assert!(r <= SQR_MOD_MASK);
    assert!(inv.wrapping_mul(d) & SQR_MOD_MASK == 1);
    assert!(Limb::MAX / d >= SQR_MOD_MASK);
    let q = r.wrapping_mul(inv) & SQR_MOD_MASK;
    assert!(r == (q.wrapping_mul(d) & SQR_MOD_MASK));
    q.wrapping_mul(d) >> SQR_MOD_BITS
}

// Single limb. Check precomputed bitmasks to see if remainder is a quadratic residue
fn perfsqr_mod_1(r: Limb, d: Limb, inv: Limb, mask: Limb) -> bool {
    //   CNST_LIMB(0x202021202020213),
    assert!(d <= Limb::WIDTH as Limb);
    let idx = perfsqr_mod_idx(r, d, inv);
    if (mask >> idx) & 1 == 0 {
        // non-square
        return false;
    }
    true
}

// Double limb. Check precomputed bitmasks to see if remainder is a quadratic residue
fn perfsqr_mod_2(r: Limb, d: Limb, inv: Limb, mhi: Limb, mlo: Limb) -> bool {
    assert!(d <= 2 * Limb::WIDTH as Limb);
    let mut idx = perfsqr_mod_idx(r, d, inv);
    let m = if idx < Limb::WIDTH as Limb { mlo } else { mhi };
    idx %= Limb::WIDTH as Limb;
    if (m >> idx) & 1 == 0 {
        // non-square
        return false;
    }
    true
}

// This test identifies 97.81% as non-squares. Grand total sq_res_0x100 and PERFSQR_MOD_TEST, 99.62%
// non-squares.
#[cfg(not(feature = "32_bit_limbs"))]
fn perfsqr_mod_test(limbs: &[Limb]) -> bool {
    let r = perfsqr_mod_34(limbs);

    perfsqr_mod_2(r, 91, 0xfd2fd2fd2fd3, 0x2191240, 0x8850a206953820e1) // 69.23%
        && perfsqr_mod_2(r, 85, 0xfcfcfcfcfcfd, 0x82158, 0x10b48c4b4206a105) // 68.24%
        && perfsqr_mod_1(r, 9, 0xe38e38e38e39, 0x93)  // 55.56%
        && perfsqr_mod_2(r, 97, 0xfd5c5f02a3a1, 0x1eb628b47, 0x6067981b8b451b5f) // 49.48%
}

// This test identifies 95.66% as non-squares. Grand total sq_res_0x100 and PERFSQR_MOD_TEST, 99.25%
// non-squares.
#[cfg(feature = "32_bit_limbs")]
fn perfsqr_mod_test(limbs: &[Limb]) -> bool {
    let r = perfsqr_mod_34(limbs);

    perfsqr_mod_2(r, 45, 0xfa4fa5, 0x920, 0x1a442481) // 73.33%
        && perfsqr_mod_1(r, 17, 0xf0f0f1, 0x1a317) // 47.06 %
        && perfsqr_mod_1(r, 13, 0xec4ec5, 0x9e5) // 46.15 %
        && perfsqr_mod_1(r, 7, 0xdb6db7, 0x69) // 42.86 %
}

// This is sq_res0x100 from mpn/perfsqr.h when generated for 64 bit limb, GMP 6.3.0. Non-zero bit
// indicates a quadratic residue mod 0x100. This test identifies 82.81% as non-squares (212/256).
#[cfg(not(feature = "32_bit_limbs"))]
const SQR_MOD256: [u64; 4] =
    [0x202021202030213, 0x202021202020213, 0x202021202030212, 0x202021202020212];

// This is sq_res0x100 from mpn/perfsqr.h when generated for 32 bit limb, GMP 6.3.0. Non-zero bit
// indicates a quadratic residue mod 0x100. This test identifies 82.81% as non-squares (212/256).
#[cfg(feature = "32_bit_limbs")]
const SQR_MOD256: [u32; 8] =
    [0x2030213, 0x2020212, 0x2020213, 0x2020212, 0x2030212, 0x2020212, 0x2020212, 0x2020212];

fn limbs_is_square(limbs: &[Limb]) -> bool {
    assert!(!limbs.is_empty());
    let idx = limbs[0] % 0x100; // mod 256

    // The first test excludes 212/256 (82.8%) of the perfect square candidates in O(1) time.
    //
    // This just checks the particular bit in the bitmask SQR_MOD256_U64 encoding where the input
    // can be a perfect square mod 256.
    if (SQR_MOD256[(idx >> Limb::LOG_WIDTH) as usize] >> (idx & const { Limb::WIDTH_MASK as Limb }))
        & 1
        == 0
    {
        return false;
    }
    // The second test uses mpn_mod_34lsub1 to detect non-squares according to their residues modulo
    // small primes (or powers of primes). See mpn/perfsqr.h, GMP 6.3.0.
    if !perfsqr_mod_test(limbs) {
        return false;
    }
    // For the third and last test, we finally compute the square root, to make sure we've really
    // got a perfect square.
    limbs_checked_sqrt(limbs).is_some()
}

impl IsSquare for Natural {
    /// Determine whether a [`Natural`] is a perfect square. Rules out > 99% of non-squares in
    /// $O(1)$ time using quadratic residue tests, then falls back to computing the square root. The
    /// [`Natural`] is taken by reference.
    ///
    /// $f(x) = (\exists b \in \Z : b^2 = x)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// let x = Natural::from(12345u64);
    /// let mut y = &x * &x;
    ///
    /// assert!((&y).is_square());
    ///
    /// y += Natural::from(1u64);
    /// assert!(!(&y).is_square());
    /// ```
    fn is_square(&self) -> bool {
        match self {
            // use the FLINT n_is_square impl for primitive integers TODO: is the FLINT n_is_square
            // better than the GMP algorithm in this file for word size integers?
            Natural(Small(small)) => small.is_square(),
            Natural(Large(limbs)) => limbs_is_square(limbs),
        }
    }
}
