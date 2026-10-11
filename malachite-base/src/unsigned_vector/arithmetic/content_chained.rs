// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ContentChained;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

// Returns the GCD of `gcd` and the elements of `xs`. Zeros at either end are skipped, an element of
// 1 at either end gives 1 at once, and the rest are taken in pairs from both ends, stopping as soon
// as the GCD reaches 1.
//
// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT 3.6.0.
pub(crate) fn content_chained<T: PrimitiveUnsigned>(xs: &[T], mut gcd: T) -> T {
    let Some(start) = xs.iter().position(|&c| c != T::ZERO) else {
        return gcd;
    };
    let end = xs.iter().rposition(|&c| c != T::ZERO).unwrap();
    let mut xs = &xs[start..=end];
    if let [first, .., last] = xs
        && (*first == T::ONE || *last == T::ONE)
    {
        return T::ONE;
    }
    while gcd != T::ONE
        && let [first, middle @ .., last] = xs
    {
        gcd.gcd_assign(first.gcd(*last));
        xs = middle;
    }
    if gcd != T::ONE
        && let [middle] = xs
    {
        gcd.gcd_assign(*middle);
    }
    gcd
}

impl<T: PrimitiveUnsigned> ContentChained<T> for UnsignedVector<T> {
    type Output = T;

    /// Computes the GCD of `x` and the elements of an [`UnsignedVector`], taking the vector by
    /// value.
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!(v.clone().content_chained(4), 2);
    /// assert_eq!(v.content_chained(0), 2);
    /// let v = UnsignedVector::<u8>::from_str("()").unwrap();
    /// assert_eq!(v.content_chained(6), 6);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: T) -> T {
        content_chained(&self.elements, x)
    }
}

impl<T: PrimitiveUnsigned> ContentChained<T> for &UnsignedVector<T> {
    type Output = T;

    /// Computes the GCD of `x` and the elements of an [`UnsignedVector`], taking the vector by
    /// reference.
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
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(6, 4, 10)").unwrap();
    /// assert_eq!((&v).content_chained(4), 2);
    /// assert_eq!((&v).content_chained(0), 2);
    /// let v = UnsignedVector::<u8>::from_str("()").unwrap();
    /// assert_eq!((&v).content_chained(6), 6);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_content_chained` from `fmpz_vec/content_chained.c`, FLINT
    /// 3.6.0.
    #[inline]
    fn content_chained(self, x: T) -> T {
        content_chained(&self.elements, x)
    }
}
