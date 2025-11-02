// Copyright © 2025 William Youmans
//
// Uses code adapted from the FLINT Library.
//
//      Copyright © 2014, 2016, 2020 William Hart
//      Copyright © 2020 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    Gcd, JacobiSymbol, ModInverse, ModMul, ModPow, ModSub, Parity, PowerOf2,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::factorization::traits::IsPrime;
use malachite_base::num::logic::traits::SignificantBits;

/// Tests whether a [`Natural`] is a strong probable prime (Miller-Rabin test) with a given base.
///
/// This is equivalent to `fmpz_is_strong_probabprime` from `fmpz/is_strong_probabprime.c`, FLINT
/// 3.1.2.
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::factorization::is_prime::is_strong_probable_prime;
///
/// assert!(is_strong_probable_prime(&Natural::from(7u32), &Natural::from(2u32)));
/// assert!(!is_strong_probable_prime(&Natural::from(9u32), &Natural::from(2u32)));
/// ```
pub fn is_strong_probable_prime(n: &Natural, base: &Natural) -> bool {
    if n <= &Natural::ONE {
        return false;
    }

    let nm1 = n - Natural::ONE;

    // Reduce base modulo n if needed
    let a = if base >= n { base % n } else { base.clone() };

    // Special cases: base is 0, 1, or n-1
    if a == Natural::ZERO || a == Natural::ONE || a == nm1 {
        return true;
    }

    // Find s such that n-1 = 2^s * d where d is odd
    let s = nm1.trailing_zeros().unwrap();
    let d = &nm1 >> s;

    // Compute y = a^d mod n
    let mut y = a.mod_pow(&d, n);

    // If y = 1, then n is a probable prime
    if y == Natural::ONE {
        return true;
    }

    // Check if y = n-1 for any of the s-1 squarings
    for _ in 0..s {
        if y == nm1 {
            return true;
        }
        // y = y^2 mod n
        y = (&y).mod_pow(&Natural::TWO, n);
    }

    false
}

/// Computes a Lucas chain for the Lucas probable prime test.
///
/// This is adapted from `lchain2_preinv` from `ulong_extras/is_probabprime.c`, FLINT 3.1.2.
fn lucas_chain(m: &Natural, a: &Natural, n: &Natural) -> (Natural, Natural) {
    let mut x = Natural::TWO;
    let mut y = a.clone();
    let bits = m.significant_bits();
    
    if bits == 0 {
        return (x, y);
    }

    let mut power = Natural::power_of_2(bits - 1);

    while power != Natural::ZERO {
        // xy = x * y - a (mod n)
        let xy = (&x).mod_mul(&y, n).mod_sub(a, n);

        if m & &power != 0 {
            // x = xy, y = y^2 - 2 (mod n)
            x = xy;
            y = (&y).mod_pow(&Natural::TWO, n).mod_sub(&Natural::TWO, n);
        } else {
            // x = x^2 - 2 (mod n), y = xy
            x = (&x).mod_pow(&Natural::TWO, n).mod_sub(&Natural::TWO, n);
            y = xy;
        }

        power >>= 1;
    }

    (x, y)
}

/// Tests whether a [`Natural`] is a Lucas probable prime.
///
/// This is adapted from `n_is_probabprime_lucas` from `ulong_extras/is_probabprime.c`, FLINT
/// 3.1.2.
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::factorization::is_prime::is_probable_prime_lucas;
///
/// assert!(is_probable_prime_lucas(&Natural::from(7u32)));
/// assert!(!is_probable_prime_lucas(&Natural::from(9u32)));
/// ```
pub fn is_probable_prime_lucas(n: &Natural) -> bool {
    if n <= &Natural::TWO {
        return n == &Natural::TWO;
    }

    if n.even() {
        return false;
    }
    
    // Handle small odd numbers
    if n == &Natural::from(3u32) {
        return true;
    }
    if n == &Natural::from(5u32) {
        return true;
    }

    // Find D such that (D/n) = -1
    let mut d = Natural::from(5u32);
    let mut neg_d = false;
    let mut j = 0;

    for i in 0..100 {
        d = Natural::from(5u32 + (i << 1));
        neg_d = false;

        // Check gcd(d, n) = 1
        if (&d).gcd(n % &d) == Natural::ONE {
            if i % 2 == 1 {
                neg_d = true;
            }

            // Compute Jacobi symbol
            let jacobi = if neg_d {
                // For negative d, we need to compute (-d | n)
                // Using the property that (-1 | n) = (-1)^((n-1)/2)
                let sign_factor = if (n - Natural::ONE) / Natural::TWO % Natural::TWO == Natural::ZERO {
                    1
                } else {
                    -1
                };
                sign_factor * (&d).jacobi_symbol(n)
            } else {
                (&d).jacobi_symbol(n)
            };

            if jacobi == -1 {
                break;
            }
        } else if n != &d {
            return false;
        }

        j += 1;
    }

    if j == 100 {
        return true;
    }

    // Compute Q = (1 - D) / 4 mod n
    // Since D >= 5 typically, we need to compute (1 - D) mod n first
    // (1 - D) mod n = (n + 1 - D) mod n
    let four = Natural::from(4u32);
    
    let one_minus_d_mod_n = if neg_d {
        // 1 - (-D) = 1 + D
        (Natural::ONE + &d) % n
    } else {
        // 1 - D, but D > 1, so compute as (n + 1 - D) mod n
        if &d >= n {
            // D >= n, so compute (1 - D) mod n = (1 - (D mod n)) mod n
            let d_mod_n = &d % n;
            if d_mod_n == Natural::ZERO {
                Natural::ONE
            } else {
                (n + Natural::ONE - d_mod_n) % n
            }
        } else if d == Natural::ONE {
            Natural::ZERO
        } else {
            // D < n and D > 1, so compute n + 1 - D
            (n + Natural::ONE - &d) % n
        }
    };
    
    // Q = (1 - D) / 4 mod n
    // We need to find Q such that 4Q ≡ (1-D) (mod n)
    // This means Q ≡ (1-D) * 4^(-1) (mod n)
    let four_mod_n = &four % n;
    let four_inv = match four_mod_n.mod_inverse(n) {
        Some(inv) => inv,
        None => return false, // gcd(4, n) != 1
    };
    
    let q = one_minus_d_mod_n.mod_mul(&four_inv, n);
    
    // Compute a = Q^(-1) - 2 (mod n)
    let q_inv = match q.mod_inverse(n) {
        Some(inv) => inv,
        None => return false,
    };
    
    // Use modular subtraction: a = (q_inv - 2) mod n
    // We compute this as (q_inv + n - 2) mod n to handle the case when q_inv < 2
    let a = (q_inv + n - Natural::TWO) % n;

    // Compute Lucas chain for m = n + 1
    let m = n + Natural::ONE;
    let (x, y) = lucas_chain(&m, &a, n);

    // Check if a * x ≡ 2 * y (mod n)
    let left = a.mod_mul(&x, n);
    let right = Natural::TWO.mod_mul(&y, n);

    left == right
}

/// Tests whether a [`Natural`] passes the Baillie-PSW primality test.
///
/// This test combines a strong probable prime test (base 2) with a Lucas probable prime test.
/// There are no known composite numbers that pass this test.
///
/// This is adapted from `n_is_probabprime_BPSW` and `fmpz_is_probabprime_BPSW` from FLINT 3.1.2.
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural::factorization::is_prime::is_probable_prime_bpsw;
///
/// assert!(is_probable_prime_bpsw(&Natural::from(7u32)));
/// assert!(!is_probable_prime_bpsw(&Natural::from(9u32)));
/// assert!(is_probable_prime_bpsw(&Natural::from(1000000007u32)));
/// ```
pub fn is_probable_prime_bpsw(n: &Natural) -> bool {
    // Handle small cases
    if n <= &Natural::ONE {
        return false;
    }
    if n == &Natural::TWO {
        return true;
    }
    if n.even() {
        return false;
    }

    // Strong probable prime test with base 2
    if !is_strong_probable_prime(n, &Natural::TWO) {
        return false;
    }

    // Lucas probable prime test
    is_probable_prime_lucas(n)
}

impl IsPrime for Natural {
    /// Tests whether a [`Natural`] is prime.
    ///
    /// For single-limb values (less than 2^64), this delegates to
    /// [`u64::is_prime`](malachite_base::num::factorization::traits::IsPrime::is_prime), which uses
    /// deterministic Miller-Rabin tests and the Baillie-PSW test.
    ///
    /// For multi-limb values, this uses the Baillie-PSW primality test, which combines a strong
    /// probable prime test (Miller-Rabin with base 2) and a Lucas probable prime test. There are
    /// no known composite numbers that pass the BPSW test, making it effectively deterministic for
    /// practical purposes.

    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPrime;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(2u32).is_prime());
    /// assert!(!Natural::from(4u32).is_prime());
    /// assert!(Natural::from(1000000007u32).is_prime());
    ///
    /// // Works for large numbers too
    /// let m127: Natural = (Natural::from(1u32) << 127) - Natural::from(1u32); // 2^127 - 1
    /// assert!(m127.is_prime());
    /// ```
    fn is_prime(&self) -> bool {
        // Delegate to u64::is_prime() for single-limb values
        if self.significant_bits() <= 64 {
            return u64::wrapping_from(self).is_prime();
        }

        // For multi-limb values, use BPSW test
        is_probable_prime_bpsw(self)
    }
}

