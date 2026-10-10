// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{DivExact, DivExactAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> DivExact<T> for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by a `T`, taking it by value. Every element
    /// must be exactly divisible by the `T`.
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $f(v, c) = v/c$.
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
    /// See [here](super::div_exact#div_exact).
    #[inline]
    fn div_exact(mut self, c: T) -> Self {
        self.div_exact_assign(c);
        self
    }
}

impl<T: PrimitiveUnsigned> DivExact<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by a `T`, taking it by reference. Every
    /// element must be exactly divisible by the `T`.
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $f(v, c) = v/c$.
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
    /// See [here](super::div_exact#div_exact).
    fn div_exact(self, c: T) -> UnsignedVector<T> {
        assert!(c != T::ZERO, "division by zero");
        UnsignedVector {
            elements: if c == T::ONE {
                self.elements.clone()
            } else {
                self.elements.iter().map(|&x| x.div_exact(c)).collect()
            },
        }
    }
}

impl<T: PrimitiveUnsigned> DivExactAssign<T> for UnsignedVector<T> {
    /// Divides every element of an [`UnsignedVector`] by a `T`, in place. Every element must be
    /// exactly divisible by the `T`.
    ///
    /// If some element is not exactly divisible by `c`, this function may panic or return a
    /// meaningless result. Dividing by 1 returns the vector unchanged.
    ///
    /// $v \gets v/c$.
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
    /// See [here](super::div_exact#div_exact_assign).
    fn div_exact_assign(&mut self, c: T) {
        assert!(c != T::ZERO, "division by zero");
        if c != T::ONE {
            for x in &mut self.elements {
                x.div_exact_assign(c);
            }
        }
    }
}
