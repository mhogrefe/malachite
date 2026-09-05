// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::mul_shr_round::mul_shr_round_ref_ref;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{MulShrRound, MulShrRoundAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::{self, *};

// Rounding a negative value with `rm` is rounding its magnitude with `-rm`, and the magnitude's
// `Ordering` flips on the way back. Exactness is checked here rather than left to the `Natural`
// core so that the panic message shows the signed operands.
fn mul_shr_round_integers(
    x: &Integer,
    y: &Integer,
    bits: u64,
    rm: RoundingMode,
) -> (Integer, Ordering) {
    if *x == 0u32 || *y == 0u32 {
        return (Integer::ZERO, Equal);
    }
    if rm == Exact {
        let exact = x.unsigned_abs_ref().trailing_zeros().unwrap()
            + y.unsigned_abs_ref().trailing_zeros().unwrap()
            >= bits;
        assert!(
            exact,
            "Product right shift is not exact: {x} * {y} >> {bits}"
        );
    }
    let negative = (*x < 0u32) != (*y < 0u32);
    let (mag, o) = mul_shr_round_ref_ref(
        x.unsigned_abs_ref(),
        y.unsigned_abs_ref(),
        bits,
        if negative { -rm } else { rm },
    );
    if negative {
        (Integer::from_sign_and_abs(false, mag), o.reverse())
    } else {
        (Integer::from(mag), o)
    }
}

impl MulShrRound<Self, u64> for Integer {
    type Output = Self;

    /// Multiplies two [`Integer`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking both [`Integer`]s by value. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// When most of the product is discarded, the product's low portion is never computed: a short
    /// product determines the surviving bits at roughly half the cost of a full multiplication.
    /// `Floor` rounds toward negative infinity and `Down` toward zero, so they differ when the
    /// product is negative.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     Integer::from(-100).mul_shr_round(Integer::from(200), 8, Floor),
    ///     (Integer::from(-79), Less)
    /// );
    /// assert_eq!(
    ///     Integer::from(-100).mul_shr_round(Integer::from(200), 8, Down),
    ///     (Integer::from(-78), Greater)
    /// );
    /// ```
    #[inline]
    fn mul_shr_round(self, y: Self, bits: u64, rm: RoundingMode) -> (Self, Ordering) {
        mul_shr_round_integers(&self, &y, bits, rm)
    }
}

impl MulShrRound<&Self, u64> for Integer {
    type Output = Self;

    /// Multiplies two [`Integer`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking the first [`Integer`] by value and
    /// the second by reference. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See the by-value documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     Integer::from(-100).mul_shr_round(&Integer::from(200), 8, Nearest),
    ///     (Integer::from(-78), Greater)
    /// );
    /// ```
    #[inline]
    fn mul_shr_round(self, y: &Self, bits: u64, rm: RoundingMode) -> (Self, Ordering) {
        mul_shr_round_integers(&self, y, bits, rm)
    }
}

impl MulShrRound<Integer, u64> for &Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking the first [`Integer`] by reference
    /// and the second by value. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See the by-value documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     (&Integer::from(-96)).mul_shr_round(Integer::from(8), 8, Exact),
    ///     (Integer::from(-3), Equal)
    /// );
    /// ```
    #[inline]
    fn mul_shr_round(self, y: Integer, bits: u64, rm: RoundingMode) -> (Integer, Ordering) {
        mul_shr_round_integers(self, &y, bits, rm)
    }
}

impl MulShrRound<&Integer, u64> for &Integer {
    type Output = Integer;

    /// Multiplies two [`Integer`]s and right-shifts the product (divides it by a power of 2),
    /// rounding according to a specified rounding mode, taking both [`Integer`]s by reference. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal to,
    /// or greater than the exact value.
    ///
    /// See the by-value documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRound;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(
    ///     (&Integer::from(100)).mul_shr_round(&Integer::from(-200), 8, Ceiling),
    ///     (Integer::from(-78), Greater)
    /// );
    /// ```
    #[inline]
    fn mul_shr_round(self, y: &Integer, bits: u64, rm: RoundingMode) -> (Integer, Ordering) {
        mul_shr_round_integers(self, y, bits, rm)
    }
}

impl MulShrRoundAssign<Self, u64> for Integer {
    /// Multiplies two [`Integer`]s and right-shifts the product (divides it by a power of 2) in
    /// place, rounding according to a specified rounding mode, taking the [`Integer`] on the
    /// right-hand side by value. An [`Ordering`] is returned, indicating whether the assigned value
    /// is less than, equal to, or greater than the exact value.
    ///
    /// See the [`MulShrRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Integer::from(-100);
    /// assert_eq!(x.mul_shr_round_assign(Integer::from(200), 8, Floor), Less);
    /// assert_eq!(x, -79);
    /// ```
    #[inline]
    fn mul_shr_round_assign(&mut self, y: Self, bits: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = mul_shr_round_integers(self, &y, bits, rm);
        o
    }
}

impl MulShrRoundAssign<&Self, u64> for Integer {
    /// Multiplies two [`Integer`]s and right-shifts the product (divides it by a power of 2) in
    /// place, rounding according to a specified rounding mode, taking the [`Integer`] on the
    /// right-hand side by reference. An [`Ordering`] is returned, indicating whether the assigned
    /// value is less than, equal to, or greater than the exact value.
    ///
    /// See the [`MulShrRound`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the shift is not exact.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::MulShrRoundAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Integer::from(-100);
    /// assert_eq!(
    ///     x.mul_shr_round_assign(&Integer::from(200), 8, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x, -78);
    /// ```
    #[inline]
    fn mul_shr_round_assign(&mut self, y: &Self, bits: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = mul_shr_round_integers(self, y, bits, rm);
        o
    }
}
