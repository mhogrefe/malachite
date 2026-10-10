// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use core::ops::{Div, DivAssign};

impl<T: PrimitiveUnsigned> Div<T> for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by a `T`, taking it by value.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded down.
    ///
    /// $$
    /// f(v, c)_i = \left\lfloor\frac{v_i}{c}\right\rfloor.
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
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div).
    #[inline]
    fn div(mut self, c: T) -> Self {
        self /= c;
        self
    }
}

impl<T: PrimitiveUnsigned> Div<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by a `T`, taking it by reference.
    ///
    /// The quotient is taken element by element, so the result has the same dimension as the
    /// vector.
    ///
    /// Each quotient is rounded down.
    ///
    /// $$
    /// f(v, c)_i = \left\lfloor\frac{v_i}{c}\right\rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div).
    fn div(self, c: T) -> UnsignedVector<T> {
        assert!(c != T::ZERO, "division by zero");
        UnsignedVector {
            elements: if c == T::ONE {
                self.elements.clone()
            } else {
                self.elements.iter().map(|&x| x / c).collect()
            },
        }
    }
}

impl<T: PrimitiveUnsigned> DivAssign<T> for UnsignedVector<T> {
    /// Divides every element of an [`UnsignedVector`] by a `T`, in place.
    ///
    /// Every element is divided by `c`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `c` is zero.
    ///
    /// # Examples
    /// See [here](super::scalar_div#div_assign).
    fn div_assign(&mut self, c: T) {
        assert!(c != T::ZERO, "division by zero");
        if c != T::ONE {
            for x in &mut self.elements {
                *x /= c;
            }
        }
    }
}
