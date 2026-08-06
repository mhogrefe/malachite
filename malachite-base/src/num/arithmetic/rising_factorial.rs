// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedRisingFactorial, RisingFactorial, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::SaturatingFrom;

// Computes the rising factorial by a checked product of consecutive factors. Partial products never
// exceed the final magnitude, since every remaining factor has magnitude at least 1, so `None`
// means exactly that the result is unrepresentable.
//
// This is the loop of rfac from fmpz/rfac.c, FLINT 3.6.0, with overflow reported rather than
// assumed away.
private_test_fn! {checked_rising_factorial_unsigned<T: PrimitiveUnsigned>(
    x: T,
    n: u64,
) -> Option<T> {
    if n == 0 {
        return Some(T::ONE);
    }
    if x == T::ZERO {
        return Some(T::ZERO);
    }
    let mut f = x;
    let mut factor = x;
    for _ in 1..n {
        factor = factor.checked_add(T::ONE)?;
        f = f.checked_mul(factor)?;
    }
    Some(f)
}}

// The signed case must detect a factor sequence that reaches or crosses zero before multiplying:
// the result is then an exactly representable zero, but the partial products leading up to the zero
// factor may not be. This mirrors the negative-base analysis of fmpz_rfac_ui from fmpz/rfac.c,
// FLINT 3.6.0, where the span check picks between a zero and a negated positive rising factorial;
// here the remaining all-negative product runs directly in the signed type, whose checked
// operations reach even the most negative value when the result is representable.
private_test_fn! {checked_rising_factorial_signed<
    U: PrimitiveUnsigned + SaturatingFrom<u64>,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    n: u64,
) -> Option<S> {
    if n == 0 {
        return Some(S::ONE);
    }
    if x <= S::ZERO && x.unsigned_abs() <= U::saturating_from(n - 1) {
        return Some(S::ZERO);
    }
    let mut f = x;
    let mut factor = x;
    for _ in 1..n {
        factor = factor.checked_add(S::ONE)?;
        f = f.checked_mul(factor)?;
    }
    Some(f)
}}

macro_rules! impl_rising_factorial {
    ($t:ident) => {
        impl RisingFactorial for $t {
            type Output = $t;

            /// Computes the rising factorial of a number: the product of the `n` consecutive
            /// numbers starting at `self`, or 1 when `n` is 0.
            ///
            /// If the result is too large to be represented, the function panics. For a function
            /// that returns `None` instead, try
            /// [`checked_rising_factorial`](CheckedRisingFactorial::checked_rising_factorial).
            ///
            /// $$
            /// f(x, n) = x^{(n)} = x (x + 1) \cdots (x + n - 1).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `Self::WIDTH`: once its
            /// factors reach 2, a nonzero product at least doubles per factor, so the loop runs
            /// $O(n)$ times before overflowing or finishing.
            ///
            /// # Panics
            /// Panics if the result is not representable.
            ///
            /// # Examples
            /// See [here](super::rising_factorial#rising_factorial).
            #[inline]
            fn rising_factorial(self, n: u64) -> $t {
                self.checked_rising_factorial(n).unwrap()
            }
        }

        impl CheckedRisingFactorial for $t {
            /// Computes the rising factorial of a number: the product of the `n` consecutive
            /// numbers starting at `self`, or 1 when `n` is 0. Returns `None` if the result cannot
            /// be represented.
            ///
            /// $$
            /// f(x, n) = \operatorname{Some}(x^{(n)}) = \operatorname{Some}(x (x + 1) \cdots
            /// (x + n - 1)),
            /// $$
            /// if the product is representable.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `Self::WIDTH`: once its
            /// factors reach 2, a nonzero product at least doubles per factor, so the loop runs
            /// $O(n)$ times before overflowing or finishing.
            ///
            /// # Examples
            /// See [here](super::rising_factorial#checked_rising_factorial).
            #[inline]
            fn checked_rising_factorial(self, n: u64) -> Option<$t> {
                checked_rising_factorial_unsigned(self, n)
            }
        }
    };
}
apply_to_unsigneds!(impl_rising_factorial);

macro_rules! impl_rising_factorial_signed {
    ($u:ident, $s:ident) => {
        impl RisingFactorial for $s {
            type Output = $s;

            /// Computes the rising factorial of a number: the product of the `n` consecutive
            /// numbers starting at `self`, or 1 when `n` is 0.
            ///
            /// If the result is too large to be represented, the function panics. For a function
            /// that returns `None` instead, try
            /// [`checked_rising_factorial`](CheckedRisingFactorial::checked_rising_factorial).
            ///
            /// $$
            /// f(x, n) = x^{(n)} = x (x + 1) \cdots (x + n - 1).
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `Self::WIDTH`: once its
            /// factors reach 2, a nonzero product at least doubles per factor, so the loop runs
            /// $O(n)$ times before overflowing or finishing.
            ///
            /// # Panics
            /// Panics if the result is not representable.
            ///
            /// # Examples
            /// See [here](super::rising_factorial#rising_factorial).
            #[inline]
            fn rising_factorial(self, n: u64) -> $s {
                self.checked_rising_factorial(n).unwrap()
            }
        }

        impl CheckedRisingFactorial for $s {
            /// Computes the rising factorial of a number: the product of the `n` consecutive
            /// numbers starting at `self`, or 1 when `n` is 0. Returns `None` if the result cannot
            /// be represented.
            ///
            /// A factor sequence that reaches or crosses zero has a product of exactly zero, which
            /// is always representable.
            ///
            /// $$
            /// f(x, n) = \operatorname{Some}(x^{(n)}) = \operatorname{Some}(x (x + 1) \cdots
            /// (x + n - 1)),
            /// $$
            /// if the product is representable.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `Self::WIDTH`: once its
            /// factors reach 2, a nonzero product at least doubles per factor, so the loop runs
            /// $O(n)$ times before overflowing or finishing.
            ///
            /// # Examples
            /// See [here](super::rising_factorial#checked_rising_factorial).
            #[inline]
            fn checked_rising_factorial(self, n: u64) -> Option<$s> {
                checked_rising_factorial_signed::<$u, $s>(self, n)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_rising_factorial_signed);
