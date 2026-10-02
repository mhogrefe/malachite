// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedFallingFactorial, FallingFactorial};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Computes the falling factorial by a checked product of consecutive factors. When n > x, one of
// the factors is 0, and the result is an exactly representable zero, although the partial products
// leading up to that factor may not be; so that case is detected before multiplying. Otherwise
// every factor is at least 1, partial products never exceed the final product, and `None` means
// exactly that the result is unrepresentable.
fn checked_falling_factorial_unsigned<T: PrimitiveUnsigned>(x: T, n: u64) -> Option<T> {
    if n == 0 {
        return Some(T::ONE);
    }
    if x <= T::saturating_from(n - 1) {
        return Some(T::ZERO);
    }
    let mut f = x;
    let mut factor = x;
    for _ in 1..n {
        factor -= T::ONE;
        f = f.checked_mul(factor)?;
    }
    Some(f)
}

macro_rules! impl_falling_factorial {
    ($t:ident) => {
        impl FallingFactorial for $t {
            type Output = $t;

            /// Computes the falling factorial of a number: the product of the `n` consecutive
            /// numbers counting down from `self`, or 1 when `n` is 0.
            ///
            /// If the result is too large to be represented, the function panics. For a function
            /// that returns `None` instead, try
            /// [`checked_falling_factorial`](CheckedFallingFactorial::checked_falling_factorial).
            ///
            /// $$
            /// f(x, n) = x^{\underline{n}} = x (x - 1) \cdots (x - n + 1).
            /// $$
            ///
            /// When `n` exceeds `self`, one of the factors is 0, and so is the result.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `Self::WIDTH`: every factor
            /// but possibly the last is at least 2, so the product at least doubles per factor, and
            /// the loop runs $O(n)$ times before overflowing or finishing.
            ///
            /// # Panics
            /// Panics if the result is not representable.
            ///
            /// # Examples
            /// See [here](super::falling_factorial#falling_factorial).
            #[inline]
            fn falling_factorial(self, n: u64) -> $t {
                self.checked_falling_factorial(n).unwrap()
            }
        }

        impl CheckedFallingFactorial for $t {
            /// Computes the falling factorial of a number: the product of the `n` consecutive
            /// numbers counting down from `self`, or 1 when `n` is 0. Returns `None` if the result
            /// cannot be represented.
            ///
            /// When `n` exceeds `self`, one of the factors is 0, and so is the result, which is
            /// always representable.
            ///
            /// $$
            /// f(x, n) = \operatorname{Some}(x^{\underline{n}}) = \operatorname{Some}(x (x - 1)
            /// \cdots (x - n + 1)),
            /// $$
            /// if the product is representable.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `Self::WIDTH`: every factor
            /// but possibly the last is at least 2, so the product at least doubles per factor, and
            /// the loop runs $O(n)$ times before overflowing or finishing.
            ///
            /// # Examples
            /// See [here](super::falling_factorial#checked_falling_factorial).
            #[inline]
            fn checked_falling_factorial(self, n: u64) -> Option<$t> {
                checked_falling_factorial_unsigned(self, n)
            }
        }
    };
}
apply_to_unsigneds!(impl_falling_factorial);
