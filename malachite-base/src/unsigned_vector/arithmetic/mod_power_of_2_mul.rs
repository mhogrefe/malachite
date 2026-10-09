// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, c: T, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        c.mod_power_of_2_is_reduced(pow),
        "c must be reduced mod 2^pow, but {c} >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<T> for UnsignedVector<T> {
    type Output = Self;

    /// Multiplies every element of an [`UnsignedVector`] by a scalar modulo $2^k$, taking the
    /// vector by value. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// Each product is reduced modulo $2^k$, and the result has the same dimension as the vector.
    ///
    /// $$
    /// f(v, c, k) = cv \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self`, or `c`, is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!(v.mod_power_of_2_mul(3, 3).to_string(), "(7, 3, 1)");
    /// ```
    #[inline]
    fn mod_power_of_2_mul(mut self, c: T, pow: u64) -> Self {
        self.mod_power_of_2_mul_assign(c, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Mul<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Multiplies every element of an [`UnsignedVector`] by a scalar modulo $2^k$, taking the
    /// vector by reference. The elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2Mul`] implementation that takes the vector by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self`, or `c`, is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_mul(3, 3).to_string(), "(7, 3, 1)");
    /// ```
    fn mod_power_of_2_mul(self, c: T, pow: u64) -> UnsignedVector<T> {
        assert_reduced(self, c, pow);
        UnsignedVector {
            elements: self
                .elements
                .iter()
                .map(|&x| x.mod_power_of_2_mul(c, pow))
                .collect(),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2MulAssign<T> for UnsignedVector<T> {
    /// Multiplies every element of an [`UnsignedVector`] by a scalar modulo $2^k$, in place. The
    /// elements and the scalar must already be reduced modulo $2^k$.
    ///
    /// See the documentation for the [`ModPowerOf2Mul`] implementation that takes the vector by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self`, or `c`, is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// v.mod_power_of_2_mul_assign(3, 3);
    /// assert_eq!(v.to_string(), "(7, 3, 1)");
    /// ```
    fn mod_power_of_2_mul_assign(&mut self, c: T, pow: u64) {
        assert_reduced(self, c, pow);
        for x in &mut self.elements {
            x.mod_power_of_2_mul_assign(c, pow);
        }
    }
}
