// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Mod, ModAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use core::ops::{Rem, RemAssign};

impl<T: PrimitiveUnsigned> Rem<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedMatrix`] by a `T`, keeping the remainders, taking the
    /// matrix by value.
    ///
    /// $A \\% m$ is the matrix whose $(i, j)$th element is $A_{ij} \\% m$. The height and width are
    /// unchanged, and the elements that reduce to zero stay, so $[4, 3]$ modulo 4 is $[0, 3]$.
    ///
    /// The result is reduced modulo $m$, which is to say that
    /// [`mod_is_reduced`](crate::num::arithmetic::traits::ModIsReduced::mod_is_reduced) returns
    /// `true` for it.
    ///
    /// $$
    /// f(A, m) = B, \\quad \text{where} \\quad B_{ij} = A_{ij} - m \left \lfloor
    /// \frac{A_{ij}}{m} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[5, 4, 1]").unwrap();
    /// assert_eq!((a % 3).to_string(), "[2, 1, 1]");
    ///
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// let a = UnsignedMatrix::<u64>::from_str("[4, 3]").unwrap();
    /// assert_eq!((a % 4).to_string(), "[0, 3]");
    /// ```
    #[inline]
    fn rem(mut self, m: T) -> Self {
        self %= m;
        self
    }
}

impl<T: PrimitiveUnsigned> Rem<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Divides every element of an [`UnsignedMatrix`] by a `T`, keeping the remainders, taking the
    /// matrix by reference.
    ///
    /// See the documentation for the [`Rem`] implementation on [`UnsignedMatrix`] that takes the
    /// matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[5, 4, 1]").unwrap();
    /// assert_eq!((&a % 3).to_string(), "[2, 1, 1]");
    ///
    /// // An element that reduces to zero stays, so the shape is unchanged.
    /// let a = UnsignedMatrix::<u64>::from_str("[4, 3]").unwrap();
    /// assert_eq!((&a % 4).to_string(), "[0, 3]");
    /// // The matrix is left alone.
    /// assert_eq!(a.to_string(), "[4, 3]");
    /// ```
    fn rem(self, m: T) -> UnsignedMatrix<T> {
        assert_ne!(m, T::ZERO, "division by zero");
        UnsignedMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements.iter().map(|&x| x % m).collect(),
        )
    }
}

impl<T: PrimitiveUnsigned> RemAssign<T> for UnsignedMatrix<T> {
    /// Divides every element of an [`UnsignedMatrix`] by a `T`, replacing each element by the
    /// remainder.
    ///
    /// See the documentation for the [`Rem`] implementation on [`UnsignedMatrix`] that takes the
    /// matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[5, 4, 1]").unwrap();
    /// a %= 3;
    /// assert_eq!(a.to_string(), "[2, 1, 1]");
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[4, 3]").unwrap();
    /// a %= 4;
    /// assert_eq!(a.to_string(), "[0, 3]");
    /// ```
    fn rem_assign(&mut self, m: T) {
        assert_ne!(m, T::ZERO, "division by zero");
        for x in &mut self.elements {
            *x %= m;
        }
    }
}

impl<T: PrimitiveUnsigned> Mod<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedMatrix`] by a `T`, keeping the remainders, taking the
    /// matrix by value.
    ///
    /// An [`UnsignedMatrix`]'s elements are never negative, so this agrees with `%` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`UnsignedMatrix`] that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[4, 4, 5]").unwrap();
    /// assert_eq!(a.mod_op(3).to_string(), "[1, 1, 2]");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> Self {
        self % m
    }
}

impl<T: PrimitiveUnsigned> Mod<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Divides every element of an [`UnsignedMatrix`] by a `T`, keeping the remainders, taking the
    /// matrix by reference.
    ///
    /// An [`UnsignedMatrix`]'s elements are never negative, so this agrees with `%` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`UnsignedMatrix`] that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[4, 4, 5]").unwrap();
    /// assert_eq!((&a).mod_op(3).to_string(), "[1, 1, 2]");
    /// ```
    #[inline]
    fn mod_op(self, m: T) -> UnsignedMatrix<T> {
        self % m
    }
}

impl<T: PrimitiveUnsigned> ModAssign<T> for UnsignedMatrix<T> {
    /// Divides every element of an [`UnsignedMatrix`] by a `T`, replacing each element by the
    /// remainder.
    ///
    /// An [`UnsignedMatrix`]'s elements are never negative, so this agrees with `%=` everywhere; it
    /// is the same operation under the name the mod-family traits use. See the documentation for
    /// the [`Rem`] implementation on [`UnsignedMatrix`] that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.elements.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u64>::from_str("[4, 4, 5]").unwrap();
    /// a.mod_assign(3);
    /// assert_eq!(a.to_string(), "[1, 1, 2]");
    /// ```
    #[inline]
    fn mod_assign(&mut self, m: T) {
        *self %= m;
    }
}
