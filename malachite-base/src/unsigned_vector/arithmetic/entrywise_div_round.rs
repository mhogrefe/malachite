// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{EntrywiseDivRound, EntrywiseDivRoundAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::rounding_modes::RoundingMode;
use crate::unsigned_vector::UnsignedVector;

impl<T: PrimitiveUnsigned> EntrywiseDivRound<T> for UnsignedVector<T> {
    type Output = Self;

    /// Divides every element of an [`UnsignedVector`] by a `T`, taking it by value, and rounds
    /// every quotient according to the specified rounding mode.
    ///
    /// Each quotient is rounded as by [`DivRound`](crate::num::arithmetic::traits::DivRound), but
    /// no [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements
    /// may be rounded in different directions. Passing `Floor` or `Down` is equivalent to using
    /// `/`.
    ///
    /// $f(v, c, r)_i = \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according
    /// to `rm`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round).
    #[inline]
    fn entrywise_div_round(mut self, c: T, rm: RoundingMode) -> Self {
        self.entrywise_div_round_assign(c, rm);
        self
    }
}

impl<T: PrimitiveUnsigned> EntrywiseDivRound<T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Divides every element of an [`UnsignedVector`] by a `T`, taking it by reference, and rounds
    /// every quotient according to the specified rounding mode.
    ///
    /// Each quotient is rounded as by [`DivRound`](crate::num::arithmetic::traits::DivRound), but
    /// no [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements
    /// may be rounded in different directions. Passing `Floor` or `Down` is equivalent to using
    /// `/`.
    ///
    /// $f(v, c, r)_i = \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according
    /// to `rm`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round).
    fn entrywise_div_round(self, c: T, rm: RoundingMode) -> UnsignedVector<T> {
        assert!(c != T::ZERO, "division by zero");
        UnsignedVector {
            elements: if c == T::ONE {
                self.elements.clone()
            } else {
                self.elements
                    .iter()
                    .map(|&x| x.div_round(c, rm).0)
                    .collect()
            },
        }
    }
}

impl<T: PrimitiveUnsigned> EntrywiseDivRoundAssign<T> for UnsignedVector<T> {
    /// Divides every element of an [`UnsignedVector`] by a `T`, in place, and rounds every quotient
    /// according to the specified rounding mode.
    ///
    /// Each quotient is rounded as by [`DivRound`](crate::num::arithmetic::traits::DivRound), but
    /// no [`Ordering`](core::cmp::Ordering) is returned, since with `Nearest` different elements
    /// may be rounded in different directions. Passing `Floor` or `Down` is equivalent to using
    /// `/`.
    ///
    /// $v_i \gets \operatorname{round}(v_i/c)$, where $\operatorname{round}$ rounds according to
    /// `rm`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `c` is zero, or if `rm` is `Exact` and some element is not divisible by `c`.
    ///
    /// # Examples
    /// See [here](super::entrywise_div_round#entrywise_div_round_assign).
    fn entrywise_div_round_assign(&mut self, c: T, rm: RoundingMode) {
        assert!(c != T::ZERO, "division by zero");
        if c != T::ONE {
            for x in &mut self.elements {
                x.div_round_assign(c, rm);
            }
        }
    }
}
