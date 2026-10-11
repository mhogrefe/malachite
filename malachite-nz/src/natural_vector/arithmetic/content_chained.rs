// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::arithmetic::content_chained::content_chained_helper;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::ContentChained;

impl ContentChained<Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalVector`], taking the vector by value
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.clone().content_chained(Natural::from(4u32)), 2u32);
    /// assert_eq!(v.content_chained(Natural::ZERO), 2u32);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.content_chained(Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x)
    }
}

impl ContentChained<&Natural> for NaturalVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalVector`], taking the vector by value
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.clone().content_chained(&Natural::from(4u32)), 2u32);
    /// assert_eq!(v.content_chained(&Natural::ZERO), 2u32);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!(v.content_chained(&Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: &Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x.clone())
    }
}

impl ContentChained<Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalVector`], taking the vector by
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).content_chained(Natural::from(4u32)), 2u32);
    /// assert_eq!((&v).content_chained(Natural::ZERO), 2u32);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!((&v).content_chained(Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x)
    }
}

impl ContentChained<&Natural> for &NaturalVector {
    type Output = Natural;

    /// Computes the GCD of `x` and the elements of a [`NaturalVector`], taking the vector by
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).content_chained(&Natural::from(4u32)), 2u32);
    /// assert_eq!((&v).content_chained(&Natural::ZERO), 2u32);
    /// let v = NaturalVector::from_str("()").unwrap();
    /// assert_eq!((&v).content_chained(&Natural::from(6u32)), 6u32);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: &Natural) -> Natural {
        content_chained_helper(&self.elements, |c| c, x.clone())
    }
}
