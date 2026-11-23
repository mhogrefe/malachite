// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::number_theory::zeta_helpers::{
    euler_number_abs, euler_number_size, multi_mod_vec,
};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::modular::traits::{ModInverse, ModMulAssign, ModNegAssign};
use malachite_base::unsigned_polynomial::series::mod_inverse_truncated::mod_inverse_truncated_vec;

// The even indices below which a single Euler number is taken from a table rather than from
// Dirichlet's beta function, whose Euler product would need too many primes there.
const EULER_NUMBER_BETA_LIMIT: u64 = 36;

// E_0, E_2, ..., E_34.
const EULER_NUMBERS_SMALL: [i128; 18] = [
    1,
    -1,
    5,
    -61,
    1385,
    -50521,
    2702765,
    -199360981,
    19391512145,
    -2404879675441,
    370371188237525,
    -69348874393137901,
    15514534163557086905,
    -4087072509293123892361,
    1252259641403629865468285,
    -441543893249023104553682821,
    177519391579539289436664789665,
    -80723299235887898062168247453281,
];

// |E_0|, |E_2|, ..., |E_{2(len - 1)}| modulo the prime p, from the series sech(x) = 1 / cosh(x) =
// sum_{k >= 0} E_{2k} x^(2k) / (2k)!. The prime must be greater than 2 len.
//
// This is __euler_number_vec_mod_p from arith/euler_number_vec.c, FLINT 3.6.0.
fn euler_numbers_abs_mod(len: usize, p: u64) -> Vec<u64> {
    let m = u64::exact_from(len);
    // The coefficients of cosh(x), 1 / (2k)!
    let mut factorial = 1u64;
    for i in 2..=(m - 1) << 1 {
        factorial.mod_mul_assign(i, p);
    }
    let mut c = factorial.mod_inverse(p).unwrap();
    let mut coefficients = vec![0u64; len];
    for k in (0..m).rev() {
        coefficients[usize::exact_from(k)] = c;
        if k != 0 {
            c.mod_mul_assign((k << 1) * ((k << 1) - 1), p);
        }
    }
    let mut result = mod_inverse_truncated_vec(&coefficients, m, p).unwrap();
    result.resize(len, 0);
    // Multiply by the factorials, and make the signs positive.
    let mut c = 1u64;
    for (k, x) in (0..m).zip(result.iter_mut()) {
        x.mod_mul_assign(c, p);
        c.mod_mul_assign(((k << 1) + 1) * ((k << 1) + 2), p);
        c.mod_neg_assign(p);
    }
    result
}

crate_test_fn! {
    // E_0, E_2, ..., E_{2(len - 1)}, by the multimodular algorithm.
    //
    // This is __euler_number_vec_multi_mod from arith/euler_number_vec.c, FLINT 3.6.0, for the even
    // indices, with each entry recombined over just the primes its size needs.
    even_euler_numbers_multi_mod(len: usize) -> Vec<Integer> {
        if len == 0 {
            return Vec::new();
        }
        multi_mod_vec(
            len,
            euler_number_size(u64::exact_from(len << 1)),
            |k| euler_number_size(u64::exact_from(k << 1)),
            |p| euler_numbers_abs_mod(len, p),
        )
        .into_iter()
        .enumerate()
        // E_{2k} is negative for an odd k.
        .map(|(k, x)| if k.odd() { -Integer::from(x) } else { Integer::from(x) })
        .collect()
    }
}

impl Integer {
    /// Returns the Euler number $E_n$.
    ///
    /// The Euler numbers are the integers defined by the generating function
    /// $$
    /// \operatorname{sech} x = \frac{1}{\cosh x} = \sum_{n=0}^\infty E_n \frac{x^n}{n!}.
    /// $$
    /// They begin $1, 0, -1, 0, 5, 0, -61, \ldots$, and $E_n = 0$ for every odd $n$.
    ///
    /// For an even $n \geq 36$, $|E_n|$ is the integer nearest to an enclosure of $2^{n+2} n!\,
    /// \beta(n+1) / \pi^{n+1}$, where $\beta$ is Dirichlet's beta function, enclosed by its Euler
    /// product, and $\pi$ is enclosed by the Chudnovsky series; a precision at which the enclosure
    /// contains only one integer is found by increasing it as needed.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::euler_number(0).to_string(), "1");
    /// assert_eq!(Integer::euler_number(1).to_string(), "0");
    /// assert_eq!(Integer::euler_number(2).to_string(), "-1");
    /// assert_eq!(Integer::euler_number(4).to_string(), "5");
    /// assert_eq!(Integer::euler_number(6).to_string(), "-61");
    /// assert_eq!(Integer::euler_number(10).to_string(), "-50521");
    /// assert_eq!(
    ///     Integer::euler_number(40).to_string(),
    ///     "14851150718114980017877156781405826684425"
    /// );
    /// ```
    ///
    /// This is arith_euler_number from arith/euler_number.c, FLINT 3.6.0.
    pub fn euler_number(n: u64) -> Self {
        if n.odd() {
            Self::ZERO
        } else if n < EULER_NUMBER_BETA_LIMIT {
            Self::from(EULER_NUMBERS_SMALL[usize::wrapping_from(n >> 1)])
        } else {
            let x = Self::from(euler_number_abs(n));
            if n % 4 == 2 { -x } else { x }
        }
    }
}

/// Returns the first `len` Euler numbers, $E_0, E_1, \ldots, E_{\mathrm{len}-1}$.
///
/// See [`Integer::euler_number`] for the definition. The even ones are found modulo enough
/// word-sized primes, from the power series of $\operatorname{sech} x$, and recombined by the
/// Chinese remainder theorem.
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
/// use malachite_nz::integer::number_theory::euler_number::euler_numbers_prefix;
///
/// assert_eq!(
///     euler_numbers_prefix(11)
///         .iter()
///         .map(|e| e.to_string())
///         .collect::<Vec<_>>(),
///     ["1", "0", "-1", "0", "5", "0", "-61", "0", "1385", "0", "-50521"]
/// );
/// assert!(euler_numbers_prefix(0).is_empty());
/// ```
///
/// This is arith_euler_number_vec from arith/euler_number_vec.c, FLINT 3.6.0.
pub fn euler_numbers_prefix(len: u64) -> Vec<Integer> {
    let len = usize::exact_from(len);
    let mut result = Vec::with_capacity(len);
    let mut evens = even_euler_numbers_multi_mod(len.div_ceil(2)).into_iter();
    for n in 0..len {
        result.push(if n.odd() {
            Integer::ZERO
        } else {
            evens.next().unwrap()
        });
    }
    result
}
