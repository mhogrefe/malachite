// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Contributed to the GNU project by Marco Bodrato.
//
//      Copyright © 2012, 2015, 2016 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::factorial::log_n_max;
use crate::natural::arithmetic::mul::product_of_limbs::limbs_product;
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{PowerOf2, Primorial, RotateLeftAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, WrappingFrom};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u32;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u64;
use malachite_base::num::factorization::prime_sieve::{id_to_n, limbs_prime_sieve_size, n_to_bit};
use malachite_base::num::factorization::traits::Primes;

#[cfg(feature = "32_bit_limbs")]
const SMALL_PRIMORIAL_LIMIT: u64 = 29;
#[cfg(not(feature = "32_bit_limbs"))]
const SMALL_PRIMORIAL_LIMIT: u64 = 53;

// This is equivalent to `mpz_primorial_ui` from `mpz/primorial_ui.c`, GMP 6.2.1, where n is too
// large for the primorial of n to fit in a single limb.
#[allow(clippy::useless_conversion)]
fn limbs_primorial(n: Limb) -> Vec<Limb> {
    let n_u64 = u64::from(n);
    let size = usize::exact_from(n >> Limb::LOG_WIDTH);
    let size = size + (size >> 1) + 1;
    assert!(size >= limbs_prime_sieve_size::<Limb>(n_u64));
    let mut sieve = vec![0; size];
    #[cfg(feature = "32_bit_limbs")]
    let count = limbs_prime_sieve_u32(&mut sieve, n_u64);
    #[cfg(not(feature = "32_bit_limbs"))]
    let count = limbs_prime_sieve_u64(&mut sieve, n);
    let size = usize::exact_from((count + 1) / log_n_max(n) + 1);
    let mut factors = vec![0; size];
    let mut j = 0;
    let mut prod = 6;
    // Store primes from 5 to n
    let max_prod = Limb::MAX / n;
    let i = n_to_bit(5);
    let mut index = usize::exact_from(i >> Limb::LOG_WIDTH);
    let mut mask = Limb::power_of_2(i & Limb::WIDTH_MASK);
    for i in i + 1..=n_to_bit(n_u64) + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::wrapping_from(id_to_n(i));
            if prod > max_prod {
                factors[j] = prod;
                j += 1;
                prod = prime;
            } else {
                prod *= prime;
            }
        }
        mask.rotate_left_assign(1);
        if mask == 1 {
            index += 1;
        }
    }
    // j != 0
    factors[j] = prod;
    j += 1;
    sieve.resize(j, 0);
    let out_len = limbs_product(&mut sieve, &mut factors[..j]);
    sieve.truncate(out_len);
    sieve
}

#[cfg(feature = "32_bit_limbs")]
const SMALL_PRODUCT_OF_FIRST_N_PRIMES_LIMIT: u64 = 10;
#[cfg(not(feature = "32_bit_limbs"))]
const SMALL_PRODUCT_OF_FIRST_N_PRIMES_LIMIT: u64 = 16;

fn limbs_product_of_first_n_primes(n: usize) -> Vec<Limb> {
    let mut prod: Limb = 1;
    let mut factors = Vec::new();
    for prime in Limb::primes().take(n) {
        if let Some(p) = prod.checked_mul(prime) {
            prod = p;
        } else {
            factors.push(prod);
            prod = prime;
        }
    }
    factors.push(prod);
    let mut out = vec![0; factors.len() + 1];
    let out_len = limbs_product(&mut out, &mut factors);
    out.truncate(out_len);
    out
}

impl Primorial for Natural {
    /// Computes the primorial of a [`Natural`]: the product of all primes less than or equal to it.
    ///
    /// The [`product_of_first_n_primes`](Natural::product_of_first_n_primes) function is similar;
    /// it computes the primorial of the $n$th prime.
    ///
    /// $$
    /// f(n) = n\\# =prod_{pleq natop p\\text {prime}} p.
    /// $$
    ///
    /// $n\\# = O(e^{(1+o(1))n})$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Primorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::primorial(0), 1);
    /// assert_eq!(Natural::primorial(1), 1);
    /// assert_eq!(Natural::primorial(2), 2);
    /// assert_eq!(Natural::primorial(3), 6);
    /// assert_eq!(Natural::primorial(4), 6);
    /// assert_eq!(Natural::primorial(5), 30);
    /// assert_eq!(
    ///     Natural::primorial(100).to_string(),
    ///     "2305567963945518424753102147331756070"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_primorial_ui` from `mpz/primorial_ui.c`, GMP 6.2.1.
    #[inline]
    fn primorial(n: u64) -> Natural {
        assert!(Limb::convertible_from(n));
        if n < SMALL_PRIMORIAL_LIMIT {
            Natural::from(Limb::primorial(n))
        } else {
            Natural::from_owned_limbs_asc(limbs_primorial(Limb::wrapping_from(n)))
        }
    }

    /// Computes the product of the first $n$ primes.
    ///
    /// The [`primorial`](Natural::primorial) function is similar; it computes the product of all
    /// primes less than or equal to $n$.
    ///
    /// $$
    /// f(n) = p_n\\# = \prod_{k=1}^n p_n,
    /// $$
    /// where $p_n$ is the $n$th prime number.
    ///
    /// $p_n\\# = O\left (\left (\frac{1}{e}k\log k\left (\frac{\log k}{e^2}k \right )^{1/\log
    /// k}\right )^k\omega(1)\right )$.
    ///
    /// This asymptotic approximation is due to [Bart
    /// Michels](https://math.stackexchange.com/a/1594930).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Primorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::product_of_first_n_primes(0), 1);
    /// assert_eq!(Natural::product_of_first_n_primes(1), 2);
    /// assert_eq!(Natural::product_of_first_n_primes(2), 6);
    /// assert_eq!(Natural::product_of_first_n_primes(3), 30);
    /// assert_eq!(Natural::product_of_first_n_primes(4), 210);
    /// assert_eq!(Natural::product_of_first_n_primes(5), 2310);
    /// assert_eq!(
    ///     Natural::product_of_first_n_primes(100).to_string(),
    ///     "4711930799906184953162487834760260422020574773409675520188634839616415335845034221205\
    ///     28925670554468197243910409777715799180438028421831503871944494399049257903072063599053\
    ///     8452312528339864352999310398481791730017201031090"
    /// );
    /// ```
    #[inline]
    fn product_of_first_n_primes(n: u64) -> Natural {
        assert!(Limb::convertible_from(n));
        if n < SMALL_PRODUCT_OF_FIRST_N_PRIMES_LIMIT {
            Natural::from(Limb::product_of_first_n_primes(n))
        } else {
            Natural::from_owned_limbs_asc(limbs_product_of_first_n_primes(usize::exact_from(n)))
        }
    }
}
