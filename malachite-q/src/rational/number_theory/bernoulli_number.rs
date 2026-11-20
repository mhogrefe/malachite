// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    DivExact, DivExactAssign, ModInverse, ModMul, ModMulAssign, ModNegAssign, NegAssign, Parity,
    SubMulAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, OneHalf, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::number_theory::traits::Primorial;
use malachite_base::unsigned_polynomial::series::mod_inverse_truncated::mod_inverse_truncated_vec;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::zeta_helpers::{
    bernoulli_number_numerator_abs, bernoulli_number_size, multi_mod_vec,
};

// The even indices below which the numerators of the Bernoulli numbers fit in an `i64`.
const BERNOULLI_SMALL_NUMERATOR_LIMIT: u64 = 35;

// The numerators of B_0, B_2, B_4, ..., B_34. This is _bernoulli_numer_small from arith.h, FLINT
// 3.6.0, at its 64-bit size, used on every platform.
const BERNOULLI_SMALL_NUMERATORS: [i64; 18] = [
    1,
    1,
    -1,
    1,
    -1,
    5,
    -691,
    7,
    -3617,
    43867,
    -174611,
    854513,
    -236364091,
    8553103,
    -23749461029,
    8615841276005,
    -7709321041217,
    2577687858367,
];

// The number of even indices below which the Bernoulli numbers are computed by Ramanujan's
// recursion rather than by the multimodular algorithm: B_0, B_2, ..., B_698.
const BERNOULLI_RECURSIVE_LIMIT: usize = 350;

// The largest number of even-index Bernoulli numbers that Ramanujan's recursion computes: B_0, B_2,
// ..., B_1442.
const MAX_RAMANUJAN_LEN: usize = 722;

// B_n for an even n, from the absolute value of its numerator and its denominator, which are
// coprime. B_0 and B_2, B_6, B_10, ... are positive; B_4, B_8, B_12, ... are negative.
const fn even_bernoulli_from_abs(n: u64, numerator: Natural, denominator: Natural) -> Rational {
    Rational {
        sign: n == 0 || n % 4 == 2,
        numerator,
        denominator,
    }
}

// B_n for an even n, from its signed numerator and its denominator, which are coprime.
fn even_bernoulli_from_signed(n: u64, numerator: Integer, denominator: Natural) -> Rational {
    debug_assert_eq!(numerator >= 0u32, n == 0 || n % 4 == 2);
    even_bernoulli_from_abs(n, numerator.unsigned_abs(), denominator)
}

// B_n for an even n, by itself.
//
// This is _bernoulli_fmpq_ui_zeta from bernoulli/fmpq_ui_zeta.c, FLINT 3.6.0, for an n of at least
// 2; FLINT's global cache of small Bernoulli numbers is not kept.
fn even_bernoulli_number(n: u64) -> Rational {
    let denominator = Natural::bernoulli_number_denominator(n);
    if n < BERNOULLI_SMALL_NUMERATOR_LIMIT {
        let numerator = Integer::from(BERNOULLI_SMALL_NUMERATORS[usize::wrapping_from(n >> 1)]);
        return even_bernoulli_from_signed(n, numerator, denominator);
    }
    let numerator = bernoulli_number_numerator_abs(n, &denominator);
    even_bernoulli_from_abs(n, numerator, denominator)
}

impl Rational {
    /// Returns the Bernoulli number $B_n$.
    ///
    /// The Bernoulli numbers are the rationals defined by the generating function
    /// $$
    /// \frac{x}{e^x - 1} = \sum_{n=0}^\infty B_n \frac{x^n}{n!},
    /// $$
    /// so $B_1 = -1/2$. They begin $1, -1/2, 1/6, 0, -1/30, 0, 1/42, \ldots$, and $B_n = 0$ for
    /// every odd $n \geq 3$.
    ///
    /// The denominator is given by the von Staudt–Clausen theorem (see
    /// [`Natural::bernoulli_number_denominator`]). For an even $n \geq 36$, the numerator is the
    /// integer nearest to an enclosure of $2 n!\, \zeta(n) / (2\pi)^n$ times the denominator, where
    /// $\zeta(n)$ is enclosed by its Euler product and $\pi$ by the Chudnovsky series; a precision
    /// at which the enclosure contains only one integer is found by increasing it as needed.
    ///
    /// To compute all of $B_0, B_1, \ldots, B_{n-1}$, [`bernoulli_numbers_prefix`] is faster than
    /// calling this function repeatedly.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::bernoulli_number(0).to_string(), "1");
    /// assert_eq!(Rational::bernoulli_number(1).to_string(), "-1/2");
    /// assert_eq!(Rational::bernoulli_number(2).to_string(), "1/6");
    /// assert_eq!(Rational::bernoulli_number(3).to_string(), "0");
    /// assert_eq!(Rational::bernoulli_number(4).to_string(), "-1/30");
    /// assert_eq!(Rational::bernoulli_number(12).to_string(), "-691/2730");
    /// assert_eq!(
    ///     Rational::bernoulli_number(50).to_string(),
    ///     "495057205241079648212477525/66"
    /// );
    /// ```
    ///
    /// This is arith_bernoulli_number from arith/bernoulli_number.c, FLINT 3.6.0.
    pub fn bernoulli_number(n: u64) -> Self {
        match n {
            0 => Self::ONE,
            1 => -Self::ONE_HALF,
            _ if n.odd() => Self::ZERO,
            _ => even_bernoulli_number(n),
        }
    }
}

// The numerators of B_0, B_2, ..., B_{2(len - 1)} over the common denominator primorial(2 len - 1),
// by Ramanujan's congruences, from the numerators of the first `start` of them, which are given
// over their own denominators `den`; then the numerators are converted back to the separate
// denominators.
//
// This is __ramanujan_even_common_denom from arith/bernoulli_number_vec_recursive.c, FLINT 3.6.0,
// indexed by n / 2 instead of n.
fn ramanujan_even_common_denominator(num: &mut [Integer], den: &mut [Natural], start: usize) {
    let len = num.len();
    // Every factor of the products of 6 below is at most m + 6, so they fit in a u64 when m < 1444.
    // FLINT also has products of 3 and of 2 for larger m, but the recursion is only used below 700.
    assert!(
        len <= MAX_RAMANUJAN_LEN,
        "Ramanujan's recursion is only implemented for at most {MAX_RAMANUJAN_LEN} values"
    );
    if start >= len {
        return;
    }
    // The indices are below 2 len - 1, so every denominator divides the primorial of 2 len.
    let common_denominator = Natural::primorial(u64::exact_from(len << 1));
    // Convert the initial values to the common denominator.
    for k in 0..start {
        num[k] *= Integer::from((&common_denominator).div_exact(&den[k]));
    }
    // Ramanujan's recursive formula
    for half_m in start..len {
        let m = u64::exact_from(half_m << 1);
        // The common denominator is divisible by 6.
        let mut x = Integer::from(&common_denominator / const { Natural::const_from(3) })
            * Integer::from(m + 3);
        if m % 6 == 4 {
            x.neg_assign();
            x >>= 1u32;
        }
        // c = t = binomial(m + 3, m)
        let t = Natural::from((m + 1) * (m + 2) * (m + 3) / 6);
        let mut c = t.clone();
        let mut j = 6;
        while j <= m {
            let r = m - j;
            // c = binomial(m + 3, m - j)
            c *= Natural::from((r + 6) * (r + 5) * (r + 4) * (r + 3) * (r + 2) * (r + 1));
            c.div_exact_assign(Natural::from(
                j * (j + 3) * (j - 2) * (j + 2) * (j - 1) * (j + 1),
            ));
            x.sub_mul_assign(Integer::from(&c), &num[usize::exact_from((m - j) >> 1)]);
            j += 6;
        }
        num[half_m] = x.div_exact(Integer::from(&t));
    }
    // Convert to separate denominators.
    for (k, (x, d)) in num.iter_mut().zip(den.iter_mut()).enumerate() {
        *d = Natural::bernoulli_number_denominator(u64::exact_from(k << 1));
        x.div_exact_assign(Integer::from((&common_denominator).div_exact(&*d)));
    }
}

crate_test_fn! {
    // B_0, B_2, ..., B_{2(len - 1)}, by Ramanujan's recursion.
    //
    // This is _arith_bernoulli_number_vec_recursive from arith/bernoulli_number_vec_recursive.c,
    // FLINT 3.6.0, for the even indices.
    even_bernoulli_numbers_recursive(len: usize) -> Vec<Rational> {
        let start = len.min(BERNOULLI_SMALL_NUMERATORS.len());
        let mut num = vec![Integer::ZERO; len];
        let mut den = vec![Natural::ONE; len];
        for k in 0..start {
            num[k] = Integer::from(BERNOULLI_SMALL_NUMERATORS[k]);
            den[k] = Natural::bernoulli_number_denominator(u64::exact_from(k << 1));
        }
        ramanujan_even_common_denominator(&mut num, &mut den, start);
        num.into_iter()
            .zip(den)
            .enumerate()
            .map(|(k, (x, d))| even_bernoulli_from_signed(u64::exact_from(k << 1), x, d))
            .collect()
    }
}

// The absolute values of the numerators of B_0, B_2, ..., B_{2(len - 1)} modulo the prime p, given
// their denominators `den`, from the series x^2 / (cosh(x) - 1) = sum_{k >= 0} 2 (1 - 2k) B_{2k}
// x^(2k) / (2k)!. The prime must be greater than 2 len.
//
// This is __bernoulli_number_vec_mod_p from arith/bernoulli_number_vec_multi_mod.c, FLINT 3.6.0.
fn even_bernoulli_numerators_abs_mod(den: &[Natural], p: u64) -> Vec<u64> {
    let len = den.len();
    let m = u64::exact_from(len);
    // The coefficients of (cosh(x) - 1) / x^2, 1 / (2k + 2)!
    let mut factorial = 1u64;
    for i in 2..=m << 1 {
        factorial.mod_mul_assign(i, p);
    }
    let mut c = factorial.mod_inverse(p).unwrap();
    let mut coefficients = vec![0u64; len];
    for k in (0..m).rev() {
        coefficients[usize::exact_from(k)] = c;
        c.mod_mul_assign(((k << 1) + 1) * ((k << 1) + 2), p);
    }
    let mut result = mod_inverse_truncated_vec(&coefficients, m, p).unwrap();
    result.resize(len, 0);
    result[0] = 1;
    // N_{2k} = -D_{2k} (2k)! / (2 (2k - 1)) times the k-th coefficient
    let mut c = p - 1;
    for k in 1..m {
        let i = usize::exact_from(k);
        let d = u64::exact_from(&(&den[i] % Natural::from(p)));
        result[i].mod_mul_assign(c.mod_mul(d, p), p);
        c.mod_mul_assign(((k + 1) << 1) * ((k << 1) - 1), p);
    }
    // the absolute values: negate B_4, B_8, ...
    for x in result.iter_mut().skip(2).step_by(2) {
        x.mod_neg_assign(p);
    }
    result
}

crate_test_fn! {
    // B_0, B_2, ..., B_{2(len - 1)}, by the multimodular algorithm.
    //
    // This is _arith_bernoulli_number_vec_multi_mod from arith/bernoulli_number_vec_multi_mod.c,
    // FLINT 3.6.0, for the even indices, with each numerator recombined from the residues of its
    // absolute value over just the primes its size needs.
    even_bernoulli_numbers_multi_mod(len: usize) -> Vec<Rational> {
        if len == 0 {
            return Vec::new();
        }
        let den: Vec<Natural> = (0..len)
            .map(|k| Natural::bernoulli_number_denominator(u64::exact_from(k << 1)))
            .collect();
        let max_den_bits = den.iter().map(SignificantBits::significant_bits).max().unwrap();
        let numerators = multi_mod_vec(
            len,
            bernoulli_number_size(u64::exact_from(len << 1)) + max_den_bits as f64 + 2.0,
            |k| {
                bernoulli_number_size(u64::exact_from(k << 1))
                    + den[k].significant_bits() as f64
                    + 2.0
            },
            |p| even_bernoulli_numerators_abs_mod(&den, p),
        );
        numerators
            .into_iter()
            .zip(den)
            .enumerate()
            .map(|(k, (x, d))| even_bernoulli_from_abs(u64::exact_from(k << 1), x, d))
            .collect()
    }
}

// B_0, B_2, ..., B_{2(len - 1)}.
//
// This is _arith_bernoulli_number_vec from arith/bernoulli_number_vec.c, FLINT 3.6.0, for the even
// indices.
pub(crate) fn even_bernoulli_numbers(len: usize) -> Vec<Rational> {
    if len < BERNOULLI_RECURSIVE_LIMIT {
        even_bernoulli_numbers_recursive(len)
    } else {
        even_bernoulli_numbers_multi_mod(len)
    }
}

/// Returns the first `len` Bernoulli numbers, $B_0, B_1, \ldots, B_{\mathrm{len}-1}$.
///
/// See [`Rational::bernoulli_number`] for the definition. Below index 700 the even ones are
/// computed by Ramanujan's congruences over a common denominator, and from there by the
/// multimodular algorithm: they are found modulo enough word-sized primes, from the power series of
/// $x^2 / (\cosh x - 1)$, and recombined by the Chinese remainder theorem.
///
/// # Worst-case complexity
/// $T(n) = O(n^3 \log n)$
///
/// $M(n) = O(n^2 \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_q::rational::number_theory::bernoulli_number::bernoulli_numbers_prefix;
///
/// assert_eq!(
///     bernoulli_numbers_prefix(10)
///         .iter()
///         .map(|b| b.to_string())
///         .collect::<Vec<_>>(),
///     ["1", "-1/2", "1/6", "0", "-1/30", "0", "1/42", "0", "-1/30", "0"]
/// );
/// assert!(bernoulli_numbers_prefix(0).is_empty());
/// ```
///
/// This is arith_bernoulli_number_vec from arith/bernoulli_number_vec.c, FLINT 3.6.0.
pub fn bernoulli_numbers_prefix(len: u64) -> Vec<Rational> {
    let len = usize::exact_from(len);
    let mut result = Vec::with_capacity(len);
    let mut evens = even_bernoulli_numbers(len.div_ceil(2)).into_iter();
    for n in 0..len {
        result.push(match n {
            1 => -Rational::ONE_HALF,
            _ if n.odd() => Rational::ZERO,
            _ => evens.next().unwrap(),
        });
    }
    result
}

/// Returns the Bernoulli numbers of the even indices from 2 to $2\,\mathrm{len}$: $B_2, B_4,
/// \ldots, B_{2\,\mathrm{len}}$.
///
/// These are the nonzero Bernoulli numbers after $B_0$ and $B_1$, the coefficients of asymptotic
/// series such as Stirling's. See [`bernoulli_numbers_prefix`] for the algorithms.
///
/// # Worst-case complexity
/// $T(n) = O(n^3 \log n)$
///
/// $M(n) = O(n^2 \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
///
/// # Examples
/// ```
/// use malachite_q::rational::number_theory::bernoulli_number::even_bernoulli_numbers_prefix;
///
/// assert_eq!(
///     even_bernoulli_numbers_prefix(6)
///         .iter()
///         .map(|b| b.to_string())
///         .collect::<Vec<_>>(),
///     ["1/6", "-1/30", "1/42", "-1/30", "5/66", "-691/2730"]
/// );
/// assert!(even_bernoulli_numbers_prefix(0).is_empty());
/// ```
///
/// This is equivalent to `bernoulliEvens` from `AzRat/Bernoulli.lean`, Azurite.
pub fn even_bernoulli_numbers_prefix(len: u64) -> Vec<Rational> {
    let len = usize::exact_from(len);
    let mut result = even_bernoulli_numbers(len + 1);
    result.remove(0);
    result
}
