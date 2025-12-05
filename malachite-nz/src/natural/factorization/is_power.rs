// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{CheckedRoot, DivAssignMod, DivMod, GcdAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::factorization::traits::{
    ExpressAsPower, Factor, IsPower, IsPrime, Primes,
};
use malachite_base::num::logic::traits::SignificantBits;

const PRIMES: [u32; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

// Find ONE perfect power representation for a Natural (not necessarily the smallest base).
//
// This function does NOT recurse - it just checks if n can be expressed as some base^exp.
fn get_perfect_power_natural(n: &Natural) -> Option<(Natural, u64)> {
    // Find largest power of 2 dividing n
    let mut pow_2 = n.trailing_zeros().unwrap();
    // Two divides exactly once - not a perfect power
    if pow_2 == 1 {
        return None;
    }
    // If pow_2 is prime, just check if n is a perfect pow_2-th power
    if pow_2.is_prime() {
        return n.checked_root(pow_2).map(|root| (root, pow_2));
    }
    // Divide out 2^pow_2 to get the odd part
    let mut q = n >> pow_2;
    // Factor out powers of small primes
    for &prime in PRIMES.iter().skip(1) {
        let prime = Natural::from(prime);
        let (new_q, r) = (&q).div_mod(&prime);
        if r == 0u32 {
            q = new_q;
            if q.div_assign_mod(&prime) != 0u32 {
                return None; // prime divides exactly once, reject
            }
            let mut pow_p = 2u64;
            loop {
                let (new_q, r) = (&q).div_mod(&prime);
                if r == 0 {
                    q = new_q;
                    pow_p += 1;
                } else {
                    break;
                }
            }
            pow_2.gcd_assign(pow_p);
            if pow_2 == 1 {
                return None; // we have multiplicity 1 of some factor
            }
            // As soon as pow_2 becomes prime, stop factoring
            if q == 1u32 || pow_2.is_prime() {
                return n.checked_root(pow_2).map(|root| (root, pow_2));
            }
        }
    }
    // After factoring, check remaining cases
    if pow_2 == 0 {
        // No factors found above; exhaustively check all prime exponents
        let bits = n.significant_bits();
        for nth in u64::primes() {
            // Terminate if exponent exceeds bit length (n ^ (1 / nth) < 2 for nth > bits)
            if nth > bits {
                return None;
            }
            if let Some(root) = n.checked_root(nth) {
                return Some((root, nth));
            }
        }
    } else {
        // Found some factors; only check prime divisors of pow_2
        for (nth, _) in pow_2.factor() {
            if let Some(root) = n.checked_root(nth) {
                return Some((root, nth));
            }
        }
    }
    None
}

// Boolean check: is n a perfect power?
//
// Note: This function computes roots the same number of times as get_perfect_power_natural, but
// discards the root values, only returning whether a perfect power representation exists.
//
// Left here in case we can find a way to optimize out root computations later.
fn get_perfect_power_natural_bool(n: &Natural) -> bool {
    // Find largest power of 2 dividing n
    let mut pow_2 = n.trailing_zeros().unwrap();
    // Two divides exactly once - not a perfect power
    if pow_2 == 1 {
        return false;
    }
    // If pow_2 is prime, check if n is a perfect pow_2-th power
    if pow_2.is_prime() {
        return n.checked_root(pow_2).is_some();
    }
    // Divide out 2^pow_2 to get the odd part
    let mut q = n >> pow_2;
    // Factor out powers of small primes
    for &prime in PRIMES.iter().skip(1) {
        let prime = Natural::from(prime);
        let (new_q, r) = (&q).div_mod(&prime);
        if r == 0 {
            q = new_q;
            if q.div_assign_mod(&prime) != 0u32 {
                return false; // prime divides exactly once, reject
            }
            let mut pow_p = 2u64;
            loop {
                let (new_q, r) = (&q).div_mod(&prime);
                if r == 0 {
                    q = new_q;
                    pow_p += 1;
                } else {
                    break;
                }
            }
            pow_2.gcd_assign(pow_p);
            if pow_2 == 1 {
                return false; // we have multiplicity 1 of some factor
            }
            // As soon as pow_2 becomes prime, stop factoring
            if q == Natural::ONE || pow_2.is_prime() {
                return n.checked_root(pow_2).is_some();
            }
        }
    }
    // After factoring, check remaining cases
    if pow_2 == 0 {
        // No factors found above; exhaustively check all prime exponents
        let bits = n.significant_bits();
        for nth in u64::primes() {
            // Terminate if exponent exceeds bit length (n ^ (1 / nth) < 2 for nth > bits)
            if nth > bits {
                return false;
            }
            if n.checked_root(nth).is_some() {
                return true;
            }
        }
    } else {
        // Found some factors; only check prime divisors of pow_2
        for (nth, _) in pow_2.factor() {
            if n.checked_root(nth).is_some() {
                return true;
            }
        }
    }
    false
}

// Express Natural as a power with the smallest possible base
//
// Note: This function is only called for multi-limb numbers (Large variant).
fn express_as_power_natural(n: &Natural) -> Option<(Natural, u64)> {
    // Get initial representation
    let (mut base, mut exp) = get_perfect_power_natural(n)?;
    // Continue until we have the smallest possible base
    while base > 3u32 {
        match get_perfect_power_natural(&base) {
            Some((base2, exp2)) => {
                base = base2;
                exp *= exp2;
            }
            None => break,
        }
    }
    Some((base, exp))
}

// Is Natural a perfect power?
//
// Note: This function is only called for multi-limb numbers
#[inline]
fn is_power_natural(n: &Natural) -> bool {
    get_perfect_power_natural_bool(n)
}

impl ExpressAsPower for Natural {
    /// Expresses a [`Natural`] as a perfect power if possible.
    ///
    /// Returns `Some((root, exponent))` where `root ^ exponent = self` and `exponent > 1`, or
    /// `None` if the number cannot be expressed as a perfect power.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::ExpressAsPower;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(8u32).express_as_power(),
    ///     Some((Natural::from(2u32), 3))
    /// );
    /// assert_eq!(
    ///     Natural::from(16u32).express_as_power(),
    ///     Some((Natural::from(2u32), 4))
    /// );
    /// assert_eq!(Natural::from(6u32).express_as_power(), None);
    /// ```
    fn express_as_power(&self) -> Option<(Self, u64)> {
        match self {
            // use the single-limb express_as_power impl for primitive integers
            Self(Small(small)) => small
                .express_as_power()
                .map(|(root, exp)| (Self::from(root), exp)),
            Self(Large(_)) => express_as_power_natural(self),
        }
    }
}

impl IsPower for Natural {
    /// Determines whether a [`Natural`] is a perfect power.
    ///
    /// A perfect power is any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both
    /// integers. In particular, 0 and 1 are considered perfect powers.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPower;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(0u32).is_power(), true);
    /// assert_eq!(Natural::from(1u32).is_power(), true);
    /// assert_eq!(Natural::from(4u32).is_power(), true);
    /// assert_eq!(Natural::from(6u32).is_power(), false);
    /// assert_eq!(Natural::from(8u32).is_power(), true);
    /// ```
    fn is_power(&self) -> bool {
        match self {
            // use the single-limb is_power impl for primitive integers
            Self(Small(small)) => small.is_power(),
            Self(Large(_)) => is_power_natural(self),
        }
    }
}
