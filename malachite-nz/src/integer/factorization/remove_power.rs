// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::factorization::traits::{RemovePower, RemovePowerAssign};

// How many times a factor divides a value depends only on the magnitudes, and the quotient is the
// exact division by the signed power: it is negative when the value is, and flips again when the
// factor is negative and the power is odd.
fn remove_power_helper(x: &Integer, y: &Integer) -> (Integer, u64) {
    let (abs, k) = x.unsigned_abs_ref().remove_power(y.unsigned_abs_ref());
    let negative = (*x < 0) != (*y < 0 && k.odd());
    (Integer::from_sign_and_abs(!negative, abs), k)
}

impl RemovePower<Self> for Integer {
    type Output = Self;

    /// Removes the largest power of a factor from an [`Integer`], returning the reduced [`Integer`]
    /// together with the exponent of that power, and taking both [`Integer`]s by value.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$, so a negative factor raised to an odd power flips the sign of the
    /// quotient. The factor need not be prime. Zero is left alone, with an exponent of 0, since
    /// every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// See [here](super::remove_power#remove_power).
    #[inline]
    fn remove_power(self, other: Self) -> (Self, u64) {
        remove_power_helper(&self, &other)
    }
}

impl RemovePower<&Self> for Integer {
    type Output = Self;

    /// Removes the largest power of a factor from an [`Integer`], returning the reduced [`Integer`]
    /// together with the exponent of that power, and taking the first [`Integer`] by value and the
    /// second by reference.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$, so a negative factor raised to an odd power flips the sign of the
    /// quotient. The factor need not be prime. Zero is left alone, with an exponent of 0, since
    /// every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// See [here](super::remove_power#remove_power).
    #[inline]
    fn remove_power(self, other: &Self) -> (Self, u64) {
        remove_power_helper(&self, other)
    }
}

impl RemovePower<Integer> for &Integer {
    type Output = Integer;

    /// Removes the largest power of a factor from an [`Integer`], returning the reduced [`Integer`]
    /// together with the exponent of that power, and taking the first [`Integer`] by reference and
    /// the second by value.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$, so a negative factor raised to an odd power flips the sign of the
    /// quotient. The factor need not be prime. Zero is left alone, with an exponent of 0, since
    /// every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// See [here](super::remove_power#remove_power).
    #[inline]
    fn remove_power(self, other: Integer) -> (Integer, u64) {
        remove_power_helper(self, &other)
    }
}

impl RemovePower<&Integer> for &Integer {
    type Output = Integer;

    /// Removes the largest power of a factor from an [`Integer`], returning the reduced [`Integer`]
    /// together with the exponent of that power, and taking both [`Integer`]s by reference.
    ///
    /// If $f^k$ is the largest power of `other` that divides `self`, this returns
    /// $(\text{self}/f^k, k)$, so a negative factor raised to an odd power flips the sign of the
    /// quotient. The factor need not be prime. Zero is left alone, with an exponent of 0, since
    /// every power of the factor divides it.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// See [here](super::remove_power#remove_power).
    #[inline]
    fn remove_power(self, other: &Integer) -> (Integer, u64) {
        remove_power_helper(self, other)
    }
}

impl RemovePowerAssign<Self> for Integer {
    /// Divides an [`Integer`] by the largest power of a factor that divides it, in place, returning
    /// the exponent of that power. The factor is taken by value.
    ///
    /// The factor need not be prime. Zero is left alone, with an exponent of 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// See [here](super::remove_power#remove_power_assign).
    #[inline]
    fn remove_power_assign(&mut self, other: Self) -> u64 {
        let (q, k) = remove_power_helper(self, &other);
        *self = q;
        k
    }
}

impl RemovePowerAssign<&Self> for Integer {
    /// Divides an [`Integer`] by the largest power of a factor that divides it, in place, returning
    /// the exponent of that power. The factor is taken by reference.
    ///
    /// The factor need not be prime. Zero is left alone, with an exponent of 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is 0, 1, or -1.
    ///
    /// # Examples
    /// See [here](super::remove_power#remove_power_assign).
    #[inline]
    fn remove_power_assign(&mut self, other: &Self) -> u64 {
        let (q, k) = remove_power_helper(self, other);
        *self = q;
        k
    }
}
