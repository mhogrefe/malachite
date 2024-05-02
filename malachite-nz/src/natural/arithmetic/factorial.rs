// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `limb_apprsqrt`, `mpz_2multiswing_1`, `mpz_oddfac_1`, `mpz_fac_ui`, and `mpz_2fac_ui`
//      contributed to the GNU project by Marco Bodrato.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mul::product_of_limbs::limbs_product;
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::Natural;
use crate::platform::{
    Limb, NTH_ROOT_NUMB_MASK_TABLE, ODD_DOUBLEFACTORIAL_TABLE_LIMIT, ODD_DOUBLEFACTORIAL_TABLE_MAX,
    ODD_FACTORIAL_TABLE_LIMIT, ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE, ONE_LIMB_ODD_FACTORIAL_TABLE,
    TABLE_2N_MINUS_POPC_2N, TABLE_LIMIT_2N_MINUS_POPC_2N,
};
use alloc::vec::Vec;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DoubleFactorial, Factorial, Gcd, Multifactorial, Parity, Pow, PowerOf2, Square, Subfactorial,
    XMulYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, WrappingFrom};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u32;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::factorization::prime_sieve::limbs_prime_sieve_u64;
use malachite_base::num::factorization::prime_sieve::{id_to_n, limbs_prime_sieve_size, n_to_bit};
use malachite_base::num::logic::traits::{BitAccess, CountOnes, NotAssign, SignificantBits};

pub_test! {subfactorial_naive(n: u64) -> Natural {
    let mut f = Natural::ONE;
    let mut b = true;
    for i in 1..=n {
        f *= Natural::from(i);
        if b {
            f -= Natural::ONE;
        } else {
            f += Natural::ONE;
        }
        b.not_assign();
    }
    f
}}

// Returns an approximation of the square root of x.
//
// It gives:
// ```
// limb_apprsqrt(x) ^ 2 <= x < (limb_apprsqrt(x) + 1) ^ 2
// ```
// or
// ```
// x <= limb_apprsqrt(x) ^ 2 <= x * 9 / 8
// ```
//
// This is equivalent to `limb_apprsqrt` in `mpz/oddfac_1.c`, GMP 6.2.1.
fn limbs_approx_sqrt(x: u64) -> u64 {
    assert!(x > 2);
    let s = x.significant_bits() >> 1;
    (u64::power_of_2(s) + (x >> s)) >> 1
}

pub(crate) const fn bit_to_n(bit: u64) -> u64 {
    (bit * 3 + 4) | 1
}

// `limbs_2_multiswing_odd` computes the odd part of the 2-multiswing factorial of the parameter n.
// The result x is an odd positive integer so that multiswing(n, 2) = x * 2 ^ a.
//
// The algorithm is described by Peter Luschny in "Divide, Swing and Conquer the Factorial!".
//
// The pointer sieve points to `limbs_prime_sieve_size(n)` limbs containing a bit array where primes
// are marked as 0. Enough limbs must be pointed by `factors`.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpz_2multiswing_1` from `mpz/oddfac_1.c`, GMP 6.2.1, where `x_and_sieve`
// is provided as a single slice, allowing the sieve to be overwritten.
#[allow(clippy::useless_conversion)]
fn limbs_2_multiswing_odd(
    x_and_sieve: &mut [Limb],
    x_len: usize,
    mut n: Limb,
    factors: &mut [Limb],
) -> usize {
    assert!(n > 25);
    let mut prod = if n.odd() { n } else { 1 };
    n.clear_bit(0);
    let max_prod = Limb::MAX / (n - 1);
    // Handle prime = 3 separately
    let mut j = 0;
    if prod > max_prod {
        // not triggered by the first billion inputs
        fail_on_untested_path("limbs_2_multiswing_odd, prod > max_prod for prime == 3");
        factors[j] = prod;
        j += 1;
        prod = 1;
    }
    let mut q = n;
    while q >= 3 {
        q /= 3;
        if q.odd() {
            prod *= 3;
        }
    }
    let limb_n = n;
    let n = u64::exact_from(n);
    // Swing primes from 5 to n / 3
    let mut s = limbs_approx_sqrt(n);
    assert!(s >= 5);
    s = n_to_bit(s);
    assert!(bit_to_n(s + 1).square() > n);
    assert!(s < n_to_bit(n / 3));
    let start = n_to_bit(5);
    let mut index = usize::exact_from(start >> Limb::LOG_WIDTH);
    let mut mask = Limb::power_of_2(start & Limb::WIDTH_MASK);
    let sieve = &mut x_and_sieve[x_len..];
    for i in start + 1..=s + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            if prod > max_prod {
                factors[j] = prod;
                j += 1;
                prod = 1;
            }
            let mut q = limb_n;
            while q >= prime {
                q /= prime;
                if q.odd() {
                    prod *= prime;
                }
            }
        }
        mask <<= 1;
        if mask == 0 {
            mask = 1;
            index += 1;
        }
    }
    assert!(max_prod <= Limb::MAX / 3);
    let l_max_prod = max_prod * 3;
    for i in s + 2..=n_to_bit(n / 3) + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            if (limb_n / prime).odd() {
                if prod > l_max_prod {
                    factors[j] = prod;
                    j += 1;
                    prod = prime;
                } else {
                    prod *= prime;
                }
            }
        }
        mask <<= 1;
        if mask == 0 {
            mask = 1;
            index += 1;
        }
    }
    // Store primes from (n + 1) / 2 to n
    let start = n_to_bit(n >> 1) + 1;
    let mut index = usize::exact_from(start >> Limb::LOG_WIDTH);
    let mut mask = Limb::power_of_2(start & Limb::WIDTH_MASK);
    for i in start + 1..=n_to_bit(n) + 1 {
        if sieve[index] & mask == 0 {
            let prime = Limb::exact_from(id_to_n(i));
            if prod > max_prod {
                factors[j] = prod;
                j += 1;
                prod = prime;
            } else {
                prod *= prime;
            }
        }
        mask <<= 1;
        if mask == 0 {
            mask = 1;
            index += 1;
        }
    }
    if j != 0 {
        factors[j] = prod;
        j += 1;
        limbs_product(x_and_sieve, &mut factors[..j])
    } else {
        // not triggered by the first billion inputs
        fail_on_untested_path("limbs_2_multiswing_odd, j == 0");
        x_and_sieve[0] = prod;
        1
    }
}

pub(crate) const FAC_DSC_THRESHOLD: usize = 236;

const FACTORS_PER_LIMB: usize =
    (Limb::WIDTH / ((usize::WIDTH - (FAC_DSC_THRESHOLD - 1).leading_zeros() as u64) + 1)) as usize;

// n ^ log <= Limb::MAX: a limb can store log factors less than n.
//
// This is equivalent to log_n_max, `gmp-impl.h`, GMP 6.2.1.
pub(crate) fn log_n_max(n: Limb) -> u64 {
    // NTH_ROOT_NUMB_MASK_TABLE[0] is Limb::MAX, so a match will always be found
    u64::wrapping_from(
        NTH_ROOT_NUMB_MASK_TABLE
            .iter()
            .rposition(|&x| n <= x)
            .unwrap(),
    ) + 1
}

// `limbs_odd_factorial` computes the odd part of the factorial of the parameter n, i.e. n! = x * 2
// ^ a, where x is the returned value: an odd positive integer.
//
// If `double` is `true`, a square is skipped in the DSC part, e.g. if n is odd, n >
// FAC_DSC_THRESHOLD and `double` is true, x is set to n!!.
//
// If n is too small, `double` is ignored, and an assert can be triggered.
//
// TODO: FAC_DSC_THRESHOLD is used here with two different roles:
// - to decide when prime factorisation is needed,
// - to stop the recursion, once sieving is done.
// Maybe two thresholds can do a better job.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
//
// This is equivalent to `mpz_oddfac_1` from `mpz/oddfac_1.c`, GMP 6.2.1.
pub_crate_test! {
#[allow(clippy::redundant_comparisons)]
limbs_odd_factorial(n: usize, double: bool) -> Vec<Limb> {
    assert!(Limb::convertible_from(n));
    if double {
        assert!(n > ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 1 && n >= FAC_DSC_THRESHOLD);
    }
    if n <= ODD_FACTORIAL_TABLE_LIMIT {
        vec![ONE_LIMB_ODD_FACTORIAL_TABLE[n]]
    } else if n <= ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 1 {
        let (hi, lo) = Limb::x_mul_y_to_zz(
            ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE[(n - 1) >> 1],
            ONE_LIMB_ODD_FACTORIAL_TABLE[n >> 1],
        );
        vec![lo, hi]
    } else {
        // Compute the number of recursive steps for the DSC algorithm
        let mut m = n;
        let mut s = 0;
        while m >= FAC_DSC_THRESHOLD {
            m >>= 1;
            s += 1;
        }
        let mut factors = vec![0; m / FACTORS_PER_LIMB + 1];
        assert!(m >= FACTORS_PER_LIMB);
        assert!(m > ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 1);
        let mut j = 0;
        let mut prod = 1;
        let mut max_prod = Limb::MAX / Limb::wrapping_from(FAC_DSC_THRESHOLD);
        while m > ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 1 {
            let mut i = ODD_DOUBLEFACTORIAL_TABLE_LIMIT + 2;
            factors[j] = ODD_DOUBLEFACTORIAL_TABLE_MAX;
            j += 1;
            while i <= m {
                if prod > max_prod {
                    factors[j] = prod;
                    j += 1;
                    prod = Limb::wrapping_from(i);
                } else {
                    prod *= Limb::wrapping_from(i);
                }
                i += 2;
            }
            max_prod <<= 1;
            m >>= 1;
        }
        factors[j] = prod;
        j += 1;
        factors[j] = ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE[(m - 1) >> 1];
        j += 1;
        factors[j] = ONE_LIMB_ODD_FACTORIAL_TABLE[m >> 1];
        j += 1;
        let mut out = vec![0; j];
        let size = limbs_product(&mut out, &mut factors[..j]);
        out.truncate(size);
        if s != 0 {
            // Use the algorithm described by Peter Luschny in "Divide, Swing and Conquer the
            // Factorial!".
            let mut size = (n >> Limb::LOG_WIDTH) + 4;
            let n_m_1 = u64::exact_from(n - 1);
            assert!(limbs_prime_sieve_size::<Limb>(n_m_1) < size - (size >> 1));
            // 2-multiswing(n) < 2^(n - 1) * sqrt(n / pi) < 2 ^ (n + Limb::WIDTH); One more can be
            // overwritten by mul, another for the sieve.
            let mut swing_and_sieve = vec![0; size];
            // Put the sieve on the second half; it will be overwritten by the last
            // `limbs_2_multiswing_odd`.
            let sieve_offset = (size >> 1) + 1;
            #[cfg(feature = "32_bit_limbs")]
            let count = limbs_prime_sieve_u32(&mut swing_and_sieve[sieve_offset..], n_m_1);
            #[cfg(not(feature = "32_bit_limbs"))]
            let count = limbs_prime_sieve_u64(&mut swing_and_sieve[sieve_offset..], n_m_1);
            size = usize::exact_from(
                (count + 1)
                    / log_n_max(Limb::exact_from(n))
                    + 1,
            );
            let mut factors = vec![0; size];
            let mut out_len = out.len();
            for i in (0..s).rev() {
                let ns = limbs_2_multiswing_odd(
                    &mut swing_and_sieve,
                    sieve_offset,
                    Limb::exact_from(n >> i),
                    &mut factors,
                );
                let mut square;
                if double && i == 0 {
                    size = out_len;
                    square = vec![0; size];
                    square[..out_len].copy_from_slice(&out[..out_len]);
                } else {
                    size = out_len << 1;
                    square = vec![0; size];
                    let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(out_len)];
                    limbs_square_to_out(&mut square, &out[..out_len], &mut square_scratch);
                    if square[size - 1] == 0 {
                        size -= 1;
                    }
                }
                out_len = size + ns;
                out.resize(out_len, 0);
                assert!(ns <= size);
                // n != n$ * floor(n / 2)! ^ 2
                let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(size, ns)];
                if limbs_mul_greater_to_out(
                    &mut out,
                    &square[..size],
                    &swing_and_sieve[..ns],
                    &mut mul_scratch,
                ) == 0
                {
                    out_len -= 1;
                }
            }
        }
        if *out.last().unwrap() == 0 {
            out.pop();
        }
        out
    }
}}

const FAC_ODD_THRESHOLD: Limb = 24;

#[cfg(feature = "32_bit_limbs")]
const SMALL_FACTORIAL_LIMIT: u64 = 13;
#[cfg(not(feature = "32_bit_limbs"))]
const SMALL_FACTORIAL_LIMIT: u64 = 21;

impl Factorial for Natural {
    /// Computes the factorial of a number.
    ///
    /// $$
    /// f(n) = n! = 1 \times 2 \times 3 \times \cdots \times n.
    /// $$
    ///
    /// $n! = O(\sqrt{n}(n/e)^n)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Factorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::factorial(0), 1);
    /// assert_eq!(Natural::factorial(1), 1);
    /// assert_eq!(Natural::factorial(2), 2);
    /// assert_eq!(Natural::factorial(3), 6);
    /// assert_eq!(Natural::factorial(4), 24);
    /// assert_eq!(Natural::factorial(5), 120);
    /// assert_eq!(
    ///     Natural::factorial(100).to_string(),
    ///     "9332621544394415268169923885626670049071596826438162146859296389521759999322991560894\
    ///     1463976156518286253697920827223758251185210916864000000000000000000000000"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_fac_ui` from `mpz/fac_ui.c`, GMP 6.2.1.
    #[allow(clippy::useless_conversion)]
    fn factorial(n: u64) -> Natural {
        assert!(Limb::convertible_from(n));
        if n < SMALL_FACTORIAL_LIMIT {
            Natural::from(Limb::factorial(n))
        } else if n < u64::from(FAC_ODD_THRESHOLD) {
            let mut factors =
                vec![0; usize::wrapping_from(n - SMALL_FACTORIAL_LIMIT) / FACTORS_PER_LIMB + 2];
            factors[0] = Limb::factorial(SMALL_FACTORIAL_LIMIT - 1);
            let mut j = 1;
            let n = Limb::wrapping_from(n);
            let mut prod = n;
            const MAX_PROD: Limb = Limb::MAX / (FAC_ODD_THRESHOLD | 1);
            const LIMB_SMALL_FACTORIAL_LIMIT: Limb = SMALL_FACTORIAL_LIMIT as Limb;
            for i in (LIMB_SMALL_FACTORIAL_LIMIT..n).rev() {
                if prod > MAX_PROD {
                    factors[j] = prod;
                    j += 1;
                    prod = i;
                } else {
                    prod *= i;
                }
            }
            factors[j] = prod;
            j += 1;
            let mut xs = vec![0; j];
            let size = limbs_product(&mut xs, &mut factors[..j]);
            xs.truncate(size);
            Natural::from_owned_limbs_asc(xs)
        } else {
            let count = if n <= TABLE_LIMIT_2N_MINUS_POPC_2N {
                u64::from(TABLE_2N_MINUS_POPC_2N[usize::exact_from((n >> 1) - 1)])
            } else {
                n - CountOnes::count_ones(n)
            };
            Natural::from_owned_limbs_asc(limbs_odd_factorial(usize::exact_from(n), false)) << count
        }
    }
}

const FAC_2DSC_THRESHOLD: Limb = ((FAC_DSC_THRESHOLD << 1) | (FAC_DSC_THRESHOLD & 1)) as Limb;

impl DoubleFactorial for Natural {
    /// Computes the double factorial of a number.
    ///
    /// $$
    /// f(n) = n!! = n \times (n - 2) \times (n - 4) \times \cdots \times i,
    /// $$
    /// where $i$ is 1 if $n$ is odd and $2$ if $n$ is even.
    ///
    /// $n!! = O(\sqrt{n}(n/e)^{n/2})$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DoubleFactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::double_factorial(0), 1);
    /// assert_eq!(Natural::double_factorial(1), 1);
    /// assert_eq!(Natural::double_factorial(2), 2);
    /// assert_eq!(Natural::double_factorial(3), 3);
    /// assert_eq!(Natural::double_factorial(4), 8);
    /// assert_eq!(Natural::double_factorial(5), 15);
    /// assert_eq!(Natural::double_factorial(6), 48);
    /// assert_eq!(Natural::double_factorial(7), 105);
    /// assert_eq!(
    ///     Natural::double_factorial(99).to_string(),
    ///     "2725392139750729502980713245400918633290796330545803413734328823443106201171875"
    /// );
    /// assert_eq!(
    ///     Natural::double_factorial(100).to_string(),
    ///     "34243224702511976248246432895208185975118675053719198827915654463488000000000000"
    /// );
    /// ```
    ///
    /// This is equivalent to `mpz_2fac_ui` from `mpz/2fac_ui.c`, GMP 6.2.1.
    fn double_factorial(n: u64) -> Natural {
        assert!(Limb::convertible_from(n));
        if n.even() {
            // n is even, n = 2k, (2k)!! = k! 2^k
            let half_n = usize::wrapping_from(n >> 1);
            let count = if n <= TABLE_LIMIT_2N_MINUS_POPC_2N && n != 0 {
                u64::from(TABLE_2N_MINUS_POPC_2N[half_n - 1])
            } else {
                n - CountOnes::count_ones(n)
            };
            Natural::from_owned_limbs_asc(limbs_odd_factorial(half_n, false)) << count
        } else if n <= u64::wrapping_from(ODD_DOUBLEFACTORIAL_TABLE_LIMIT) {
            Natural::from(ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE[usize::wrapping_from(n >> 1)])
        } else if n < u64::wrapping_from(FAC_2DSC_THRESHOLD) {
            let mut factors = vec![0; usize::exact_from(n) / (FACTORS_PER_LIMB << 1) + 1];
            factors[0] = ODD_DOUBLEFACTORIAL_TABLE_MAX;
            let mut j = 1;
            let mut n = Limb::wrapping_from(n);
            let mut prod = n;
            let max_prod = Limb::MAX / FAC_2DSC_THRESHOLD;
            const LIMIT: Limb = ODD_DOUBLEFACTORIAL_TABLE_LIMIT as Limb + 2;
            while n > LIMIT {
                n -= 2;
                if prod > max_prod {
                    factors[j] = prod;
                    j += 1;
                    prod = n;
                } else {
                    prod *= n;
                }
            }
            factors[j] = prod;
            j += 1;
            let mut xs = vec![0; j];
            let size = limbs_product(&mut xs, &mut factors[..j]);
            xs.truncate(size);
            Natural::from_owned_limbs_asc(xs)
        } else {
            Natural::from_owned_limbs_asc(limbs_odd_factorial(usize::exact_from(n), true))
        }
    }
}

impl Multifactorial for Natural {
    /// Computes a multifactorial of a number.
    ///
    /// $$
    /// f(n, m) = n!^{(m)} = n \times (n - m) \times (n - 2m) \times \cdots \times i.
    /// $$
    /// If $n$ is divisible by $m$, then $i$ is $m$; otherwise, $i$ is the remainder when $n$ is
    /// divided by $m$.
    ///
    /// $n!^{(m)} = O(\sqrt{n}(n/e)^{n/m})$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n, m) = O(n \log n)$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Multifactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::multifactorial(0, 1), 1);
    /// assert_eq!(Natural::multifactorial(1, 1), 1);
    /// assert_eq!(Natural::multifactorial(2, 1), 2);
    /// assert_eq!(Natural::multifactorial(3, 1), 6);
    /// assert_eq!(Natural::multifactorial(4, 1), 24);
    /// assert_eq!(Natural::multifactorial(5, 1), 120);
    ///
    /// assert_eq!(Natural::multifactorial(0, 2), 1);
    /// assert_eq!(Natural::multifactorial(1, 2), 1);
    /// assert_eq!(Natural::multifactorial(2, 2), 2);
    /// assert_eq!(Natural::multifactorial(3, 2), 3);
    /// assert_eq!(Natural::multifactorial(4, 2), 8);
    /// assert_eq!(Natural::multifactorial(5, 2), 15);
    /// assert_eq!(Natural::multifactorial(6, 2), 48);
    /// assert_eq!(Natural::multifactorial(7, 2), 105);
    ///
    /// assert_eq!(Natural::multifactorial(0, 3), 1);
    /// assert_eq!(Natural::multifactorial(1, 3), 1);
    /// assert_eq!(Natural::multifactorial(2, 3), 2);
    /// assert_eq!(Natural::multifactorial(3, 3), 3);
    /// assert_eq!(Natural::multifactorial(4, 3), 4);
    /// assert_eq!(Natural::multifactorial(5, 3), 10);
    /// assert_eq!(Natural::multifactorial(6, 3), 18);
    /// assert_eq!(Natural::multifactorial(7, 3), 28);
    /// assert_eq!(Natural::multifactorial(8, 3), 80);
    /// assert_eq!(Natural::multifactorial(9, 3), 162);
    ///
    /// assert_eq!(
    ///     Natural::multifactorial(100, 3).to_string(),
    ///     "174548867015437739741494347897360069928419328000000000"
    /// );
    /// ```
    fn multifactorial(mut n: u64, mut m: u64) -> Natural {
        assert_ne!(m, 0);
        assert!(Limb::convertible_from(n));
        assert!(Limb::convertible_from(m));
        if n < 3 || n - 3 < m - 1 {
            // n < 3 || n - 1 <= m
            if n == 0 {
                Natural::ONE
            } else {
                Natural::from(n)
            }
        } else {
            // 0 < m < n - 1 < Limb::MAX
            let gcd = n.gcd(m);
            if gcd > 1 {
                n /= gcd;
                m /= gcd;
            }
            if m <= 2 {
                // fac or 2fac
                if m == 1 {
                    match gcd {
                        gcd if gcd > 2 => Natural::from(gcd).pow(n) * Natural::factorial(n),
                        2 => Natural::double_factorial(n << 1),
                        _ => Natural::factorial(n),
                    }
                } else if gcd > 1 {
                    // m == 2
                    Natural::from(gcd).pow((n >> 1) + 1) * Natural::double_factorial(n)
                } else {
                    Natural::double_factorial(n)
                }
            } else {
                // m >= 3, gcd(n,m) = 1
                let reduced_n = n / m + 1;
                let mut n = Limb::exact_from(n);
                let m = Limb::exact_from(m);
                let mut j = 0;
                let mut prod = n;
                n -= m;
                let max_prod = Limb::MAX / n;
                let mut factors = vec![0; usize::exact_from(reduced_n / log_n_max(n) + 2)];
                while n > m {
                    if prod > max_prod {
                        factors[j] = prod;
                        j += 1;
                        prod = n;
                    } else {
                        prod *= n;
                    }
                    n -= m;
                }
                factors[j] = n;
                j += 1;
                factors[j] = prod;
                j += 1;
                let mut xs = vec![0; j];
                let size = limbs_product(&mut xs, &mut factors[..j]);
                xs.truncate(size);
                let x = Natural::from_owned_limbs_asc(xs);
                if gcd == 1 {
                    x
                } else {
                    Natural::from(gcd).pow(reduced_n) * x
                }
            }
        }
    }
}

impl Subfactorial for Natural {
    /// Computes the subfactorial of a number.
    ///
    /// The subfactorial of $n$ counts the number of derangements of a set of size $n$; a
    /// derangement is a permutation with no fixed points.
    ///
    /// $$
    /// f(n) = \\ !n = \lfloor n!/e \rfloor.
    /// $$
    ///
    /// $!n = O(n!) = O(\sqrt{n}(n/e)^n)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Subfactorial;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::subfactorial(0), 1);
    /// assert_eq!(Natural::subfactorial(1), 0);
    /// assert_eq!(Natural::subfactorial(2), 1);
    /// assert_eq!(Natural::subfactorial(3), 2);
    /// assert_eq!(Natural::subfactorial(4), 9);
    /// assert_eq!(Natural::subfactorial(5), 44);
    /// assert_eq!(
    ///     Natural::subfactorial(100).to_string(),
    ///     "3433279598416380476519597752677614203236578380537578498354340028268518079332763243279\
    ///     1396429850988990237345920155783984828001486412574060553756854137069878601"
    /// );
    /// ```
    #[inline]
    fn subfactorial(n: u64) -> Natural {
        subfactorial_naive(n)
    }
}
