// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Mod, ModAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::ops::{Rem, RemAssign};

impl<T: PrimitiveUnsigned> Rem<T> for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by a `T`, keeping the remainders, taking the
    /// vector by value.
    ///
    /// $v \\% m$ is the vector whose $i$th element is $v_i \\% m$. The dimension is unchanged:
    /// unlike a polynomial, a vector keeps the elements that reduce to zero, so $(4, 3)$ modulo 4
    /// is $(0, 3)$.
    ///
    /// The result is reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](crate::num::arithmetic::traits::ModIsReduced::mod_is_reduced) returns
    /// `true` for it.
    ///
    /// $$
    /// f(v, m) = w, \\quad \text{where} \\quad w_i = v_i - m \left \lfloor \frac{v_i}{m}
    /// \right \rfloor.
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
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(5, 4, 1)").unwrap();
    /// assert_eq!((v % 3).to_string(), "(2, 1, 1)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = UnsignedVector::<u64>::from_str("(4, 3)").unwrap();
    /// assert_eq!((v % 4).to_string(), "(0, 3)");
    /// ```
    #[inline]
    fn rem(mut self, m: T) -> Self {
        self %= m;
        self
    }
}

impl<T: PrimitiveUnsigned> Rem<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by a `T`, keeping the remainders, taking the
    /// vector by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`UnsignedVector`] that takes the
    /// vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(5, 4, 1)").unwrap();
    /// assert_eq!((&v % 3).to_string(), "(2, 1, 1)");
    ///
    /// // An element that reduces to zero stays, so the dimension is unchanged.
    /// let v = UnsignedVector::<u64>::from_str("(4, 3)").unwrap();
    /// assert_eq!((&v % 4).to_string(), "(0, 3)");
    /// // The vector is left alone.
    /// assert_eq!(v.to_string(), "(4, 3)");
    /// ```
    fn rem(self, m: T) -> UnsignedVector<T> {
        assert_ne!(m, T::ZERO, "division by zero");
        UnsignedVector {
            elements: self.elements.iter().map(|&x| x % m).collect(),
        }
    }
}

impl<T: PrimitiveUnsigned> RemAssign<T> for UnsignedVector<T> {
    /// Divides every element of an [`UnsignedVector`] by a `T`, replacing each element by the
    /// remainder.
    ///
    /// See the documentation for the [`Rem`] implementation on [`UnsignedVector`] that takes the
    /// vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u64>::from_str("(5, 4, 1)").unwrap();
    /// v %= 3;
    /// assert_eq!(v.to_string(), "(2, 1, 1)");
    ///
    /// let mut v = UnsignedVector::<u64>::from_str("(4, 3)").unwrap();
    /// v %= 4;
    /// assert_eq!(v.to_string(), "(0, 3)");
    /// ```
    fn rem_assign(&mut self, m: T) {
        assert_ne!(m, T::ZERO, "division by zero");
        for x in &mut self.elements {
            *x %= m;
        }
    }
}

impl<T: PrimitiveUnsigned> Mod<T> for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by a `T`, keeping the remainders, taking the
    /// vector by value.
    ///
    /// An [`UnsignedVector`]'s elements are never negative, so this agrees with `%` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`UnsignedVector`] that takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(4, 4, 5)").unwrap();
    /// assert_eq!(v.mod_op(3).to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> Self {
        self % m
    }
}

impl<T: PrimitiveUnsigned> Mod<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by a `T`, keeping the remainders, taking the
    /// vector by reference.
    ///
    /// An [`UnsignedVector`]'s elements are never negative, so this agrees with `%` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`UnsignedVector`] that takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u64>::from_str("(4, 4, 5)").unwrap();
    /// assert_eq!((&v).mod_op(3).to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> UnsignedVector<T> {
        self % m
    }
}

impl<T: PrimitiveUnsigned> ModAssign<T> for UnsignedVector<T> {
    /// Divides every element of an [`UnsignedVector`] by a `T`, replacing each element by the
    /// remainder.
    ///
    /// An [`UnsignedVector`]'s elements are never negative, so this agrees with `%=` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`UnsignedVector`] that takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u64>::from_str("(4, 4, 5)").unwrap();
    /// v.mod_assign(3);
    /// assert_eq!(v.to_string(), "(1, 1, 2)");
    /// ```
    #[inline]
    fn mod_assign(&mut self, m: T) {
        *self %= m;
    }
}
