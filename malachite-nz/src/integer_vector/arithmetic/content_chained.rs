// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ContentChained, Gcd, GcdAssign};
use malachite_base::num::basic::traits::One;

// Returns the GCD of `gcd` and the absolute values of the elements of `xs`, where `abs` gives the
// absolute value of an element.
//
// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT 3.6.0.
pub(crate) fn content_chained_helper<T>(
    xs: &[T],
    abs: impl Fn(&T) -> &Natural,
    mut gcd: Natural,
) -> Natural {
    // Zeros make no difference, so they are skipped at both ends.
    let Some(start) = xs.iter().position(|c| *abs(c) != 0u32) else {
        return gcd;
    };
    let end = xs.iter().rposition(|c| *abs(c) != 0u32).unwrap();
    let mut xs = &xs[start..=end];
    // An element of absolute value 1 makes the GCD 1. Only the ends are checked, since for the
    // coefficients of a polynomial they are the likeliest to be ±1.
    if let [first, .., last] = xs
        && (*abs(first) == 1u32 || *abs(last) == 1u32)
    {
        return Natural::ONE;
    }
    // The elements are taken in pairs from both ends, stopping as soon as the GCD reaches 1.
    while gcd != 1u32
        && let [first, middle @ .., last] = xs
    {
        gcd.gcd_assign(abs(first).gcd(abs(last)));
        xs = middle;
    }
    if gcd != 1u32
        && let [middle] = xs
    {
        gcd.gcd_assign(abs(middle));
    }
    gcd
}

// Returns the GCD of `x` and the absolute values of the elements of `xs`.
//
// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT 3.6.0.
#[doc(hidden)]
#[inline]
pub fn integers_content_chained(xs: &[Integer], x: &Natural) -> Natural {
    content_chained_helper(xs, Integer::unsigned_abs_ref, x.clone())
}

impl ContentChained<Natural> for IntegerVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of an [`IntegerVector`], taking the vector by value
    /// and `x` by value.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!(v.clone().content_chained(Natural::from(4u32)), 2u32);
    /// assert_eq!(v.content_chained(Natural::ZERO), 2u32);
    /// let v = IntegerVector::from_str("()").unwrap();
    /// assert_eq!(v.content_chained(Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: Natural) -> Natural {
        content_chained_helper(&self.elements, Integer::unsigned_abs_ref, x)
    }
}

impl ContentChained<&Natural> for IntegerVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of an [`IntegerVector`], taking the vector by value
    /// and `x` by reference.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!(v.clone().content_chained(&Natural::from(4u32)), 2u32);
    /// assert_eq!(v.content_chained(&Natural::ZERO), 2u32);
    /// let v = IntegerVector::from_str("()").unwrap();
    /// assert_eq!(v.content_chained(&Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: &Natural) -> Natural {
        content_chained_helper(&self.elements, Integer::unsigned_abs_ref, x.clone())
    }
}

impl ContentChained<Natural> for &IntegerVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of an [`IntegerVector`], taking the vector by
    /// reference and `x` by value.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!((&v).content_chained(Natural::from(4u32)), 2u32);
    /// assert_eq!((&v).content_chained(Natural::ZERO), 2u32);
    /// let v = IntegerVector::from_str("()").unwrap();
    /// assert_eq!((&v).content_chained(Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: Natural) -> Natural {
        content_chained_helper(&self.elements, Integer::unsigned_abs_ref, x)
    }
}

impl ContentChained<&Natural> for &IntegerVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of an [`IntegerVector`], taking the vector by
    /// reference and `x` by reference.
    ///
    /// The GCD of `x` and the elements of the 0-dimensional vector, or of a vector of zeros, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(v, x) = \gcd(x, v_0, v_1, \ldots, v_{n-1}),
    /// $$
    ///
    /// where $n$ is the dimension of $v$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural::Natural;
    ///
    /// let v = IntegerVector::from_str("(-6, 4, -10)").unwrap();
    /// assert_eq!((&v).content_chained(&Natural::from(4u32)), 2u32);
    /// assert_eq!((&v).content_chained(&Natural::ZERO), 2u32);
    /// let v = IntegerVector::from_str("()").unwrap();
    /// assert_eq!((&v).content_chained(&Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: &Natural) -> Natural {
        content_chained_helper(&self.elements, Integer::unsigned_abs_ref, x.clone())
    }
}
