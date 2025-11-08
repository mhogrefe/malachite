// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CheckedRoot, DivExactAssign, DivisibleBy, Gcd, WrappingMulAssign,
};
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower, IsPrime};
use malachite_base::num::logic::traits::BitScan;

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

const SMALLEST_OMITTED_PRIME: u32 = 1009;

// Find ONE perfect power representation for a Natural (not necessarily the smallest base).
//
// This function does NOT recurse - it just checks if n can be expressed as some base^exp.
fn get_perfect_power_natural(n: &Natural) -> Option<(Natural, u32)> {
    use malachite_base::num::basic::traits::One;

    // Find largest power of 2 dividing n
    let mut pow_2 = n.index_of_next_true_bit(0)?;

    // Two divides exactly once - not a perfect power
    if pow_2 == 1 {
        return None;
    }

    // Divide out 2^pow_2 to get the odd part
    let mut q = if pow_2 > 0 { n >> pow_2 } else { n.clone() };

    // If pow_2 is prime, just check if n is a perfect pow_2-th power
    if pow_2.is_prime() {
        return n.checked_root(pow_2).map(|root| (root, pow_2 as u32));
    }

    // Factor out powers of small primes
    for &prime in PRIMES.iter().skip(1) {
        let prime_nat = Natural::from(prime);
        if (&q).divisible_by(&prime_nat) {
            let prime_squared = &prime_nat * &prime_nat;
            if !(&q).divisible_by(&prime_squared) {
                return None; // prime divides exactly once, reject
            }
            q.div_exact_assign(&prime_squared);

            let mut pow_p = 2u64;
            while (&q).divisible_by(&prime_nat) {
                q.div_exact_assign(&prime_nat);
                pow_p += 1;
            }

            pow_2 = pow_2.gcd(pow_p);
            if pow_2 == 1 {
                return None; // we have multiplicity 1 of some factor
            }

            if q == Natural::ONE {
                return n.checked_root(pow_2).map(|root| (root, pow_2 as u32));
            }

            // As soon as pow_2 becomes prime, stop factoring
            if pow_2.is_prime() {
                return n.checked_root(pow_2).map(|root| (root, pow_2 as u32));
            }
        }
    }

    // After factoring, check remaining cases
    if pow_2 == 0 {
        // No factors found above; exhaustively check all prime exponents
        for nth in 2u64.. {
            if !nth.is_prime() {
                continue;
            }

            if let Some(root) = n.checked_root(nth) {
                return Some((root, nth as u32));
            }

            // Early termination optimization
            if q < SMALLEST_OMITTED_PRIME {
                return None;
            }
        }
    } else {
        // Found some factors; only check prime divisors of pow_2
        for nth in 2u64..=pow_2 {
            if !nth.is_prime() {
                continue;
            }

            if pow_2 % nth != 0 {
                continue;
            }

            if let Some(root) = n.checked_root(nth) {
                return Some((root, nth as u32));
            }

            // Early termination optimization
            if q < SMALLEST_OMITTED_PRIME {
                return None;
            }
        }

        return None;
    }

    None
}

// Boolean check: is n a perfect power? (avoid computing roots where possible)
fn get_perfect_power_natural_bool(n: &Natural) -> bool {
    use malachite_base::num::basic::traits::One;

    // Find largest power of 2 dividing n
    let mut pow_2 = match n.index_of_next_true_bit(0) {
        Some(p) => p,
        None => return false, // Zero - caller should handle
    };

    // Two divides exactly once - not a perfect power
    if pow_2 == 1 {
        return false;
    }

    // Divide out 2^pow_2 to get the odd part
    let mut q = if pow_2 > 0 { n >> pow_2 } else { n.clone() };

    // If pow_2 is prime, check if n is a perfect pow_2-th power
    if pow_2.is_prime() {
        return n.checked_root(pow_2).is_some();
    }

    // Factor out powers of small primes
    for &prime in PRIMES.iter().skip(1) {
        let prime_nat = Natural::from(prime);
        if (&q).divisible_by(&prime_nat) {
            let prime_squared = &prime_nat * &prime_nat;
            if !(&q).divisible_by(&prime_squared) {
                return false; // prime divides exactly once, reject
            }
            q.div_exact_assign(&prime_squared);

            let mut pow_p = 2u64;
            while (&q).divisible_by(&prime_nat) {
                q.div_exact_assign(&prime_nat);
                pow_p += 1;
            }

            pow_2 = pow_2.gcd(pow_p);
            if pow_2 == 1 {
                return false; // we have multiplicity 1 of some factor
            }

            if q == Natural::ONE {
                return n.checked_root(pow_2).is_some();
            }

            // As soon as pow_2 becomes prime, stop factoring
            if pow_2.is_prime() {
                return n.checked_root(pow_2).is_some();
            }
        }
    }

    // After factoring, check remaining cases
    if pow_2 == 0 {
        // No factors found above; exhaustively check all prime exponents
        for nth in 2u64.. {
            if !nth.is_prime() {
                continue;
            }

            if n.checked_root(nth).is_some() {
                return true;
            }

            // Early termination optimization
            if q < SMALLEST_OMITTED_PRIME {
                return false;
            }
        }
    } else {
        // Found some factors; only check prime divisors of pow_2
        for nth in 2u64..=pow_2 {
            if !nth.is_prime() {
                continue;
            }

            if pow_2 % nth != 0 {
                continue;
            }

            if n.checked_root(nth).is_some() {
                return true;
            }

            // Early termination optimization
            if q < SMALLEST_OMITTED_PRIME {
                return false;
            }
        }

        return false;
    }

    false
}

// Express Natural as a power with the smallest possible base
fn express_as_power_natural(n: &Natural) -> Option<(Natural, u32)> {
    use malachite_base::num::basic::traits::{One, Zero};

    // Special case: zero is considered a perfect square (0 = 0^2)
    if n == &Natural::ZERO {
        return Some((Natural::ZERO, 2));
    }

    // Special case: 1 is a perfect square (1 = 1^2)
    if n == &Natural::ONE {
        return Some((Natural::ONE, 2));
    }

    // Get initial representation
    let (mut base, mut exp) = get_perfect_power_natural(n)?;

    // Continue until we have the smallest possible base
    while base > 3u32 {
        match get_perfect_power_natural(&base) {
            Some((base2, exp2)) => {
                base = base2;
                exp.wrapping_mul_assign(exp2);
            }
            None => break,
        }
    }

    Some((base, exp))
}

// Is Natural a perfect power?
fn is_power_natural(n: &Natural) -> bool {
    use malachite_base::num::basic::traits::One;

    n <= &Natural::ONE || get_perfect_power_natural_bool(n)
}

impl ExpressAsPower for Natural {
    /// Expresses a [`Natural`] as a perfect power if possible.
    ///
    /// Returns `Some((root, exponent))` where `root^exponent = self` and `exponent > 1`, or `None`
    /// if the number cannot be expressed as a perfect power.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::ExpressAsPower;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(8u32).express_as_power(), Some((Natural::from(2u32), 3)));
    /// assert_eq!(Natural::from(16u32).express_as_power(), Some((Natural::from(2u32), 4)));
    /// assert_eq!(Natural::from(6u32).express_as_power(), None);
    /// ```
    fn express_as_power(&self) -> Option<(Self, u64)> {
        express_as_power_natural(self).map(|(root, exp)| (root, u64::from(exp)))
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
        is_power_natural(self)
    }
}
