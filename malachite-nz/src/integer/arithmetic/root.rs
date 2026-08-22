// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, FloorRoot, FloorRootAssign, Parity, Pow,
    RootAssignRem, RootRem, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Zero};

impl FloorRoot<u64> for Integer {
    type Output = Self;

    /// Returns the floor of the $n$th root of an [`Integer`], taking the [`Integer`] by value.
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorRoot;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(999).floor_root(3), 9);
    /// assert_eq!(Integer::from(1000).floor_root(3), 10);
    /// assert_eq!(Integer::from(1001).floor_root(3), 10);
    /// assert_eq!(Integer::from(100000000000i64).floor_root(5), 158);
    /// assert_eq!(Integer::from(-100000000000i64).floor_root(5), -159);
    /// ```
    #[inline]
    fn floor_root(mut self, exp: u64) -> Self {
        self.floor_root_assign(exp);
        self
    }
}

impl FloorRoot<u64> for &Integer {
    type Output = Integer;

    /// Returns the floor of the $n$th root of an [`Integer`], taking the [`Integer`] by reference.
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorRoot;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(999)).floor_root(3), 9);
    /// assert_eq!((&Integer::from(1000)).floor_root(3), 10);
    /// assert_eq!((&Integer::from(1001)).floor_root(3), 10);
    /// assert_eq!((&Integer::from(100000000000i64)).floor_root(5), 158);
    /// assert_eq!((&Integer::from(-100000000000i64)).floor_root(5), -159);
    /// ```
    #[inline]
    fn floor_root(self, exp: u64) -> Integer {
        if *self >= 0 {
            Integer::from(self.unsigned_abs_ref().floor_root(exp))
        } else if exp.odd() {
            -self.unsigned_abs_ref().ceiling_root(exp)
        } else {
            panic!("Cannot take even root of {self}")
        }
    }
}

impl FloorRootAssign<u64> for Integer {
    /// Replaces an [`Integer`] with the floor of its $n$th root.
    ///
    /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorRootAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(999);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Integer::from(1000);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(1001);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(100000000000i64);
    /// x.floor_root_assign(5);
    /// assert_eq!(x, 158);
    ///
    /// let mut x = Integer::from(-100000000000i64);
    /// x.floor_root_assign(5);
    /// assert_eq!(x, -159);
    /// ```
    #[inline]
    fn floor_root_assign(&mut self, exp: u64) {
        if *self >= 0 {
            self.mutate_unsigned_abs(|n| n.floor_root_assign(exp));
        } else if exp.odd() {
            self.mutate_unsigned_abs(|n| n.ceiling_root_assign(exp));
        } else {
            panic!("Cannot take even root of {self}")
        }
    }
}

impl CeilingRoot<u64> for Integer {
    type Output = Self;

    /// Returns the ceiling of the $n$th root of an [`Integer`], taking the [`Integer`] by value.
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingRoot;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(999).ceiling_root(3), 10);
    /// assert_eq!(Integer::from(1000).ceiling_root(3), 10);
    /// assert_eq!(Integer::from(1001).ceiling_root(3), 11);
    /// assert_eq!(Integer::from(100000000000i64).ceiling_root(5), 159);
    /// assert_eq!(Integer::from(-100000000000i64).ceiling_root(5), -158);
    /// ```
    #[inline]
    fn ceiling_root(mut self, exp: u64) -> Self {
        self.ceiling_root_assign(exp);
        self
    }
}

impl CeilingRoot<u64> for &Integer {
    type Output = Integer;

    /// Returns the ceiling of the $n$th root of an [`Integer`], taking the [`Integer`] by
    /// reference.
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingRoot;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(999).ceiling_root(3), 10);
    /// assert_eq!(Integer::from(1000).ceiling_root(3), 10);
    /// assert_eq!(Integer::from(1001).ceiling_root(3), 11);
    /// assert_eq!(Integer::from(100000000000i64).ceiling_root(5), 159);
    /// assert_eq!(Integer::from(-100000000000i64).ceiling_root(5), -158);
    /// ```
    #[inline]
    fn ceiling_root(self, exp: u64) -> Integer {
        if *self >= 0 {
            Integer::from(self.unsigned_abs_ref().ceiling_root(exp))
        } else if exp.odd() {
            -self.unsigned_abs_ref().floor_root(exp)
        } else {
            panic!("Cannot take even root of {self}")
        }
    }
}

impl CeilingRootAssign<u64> for Integer {
    /// Replaces an [`Integer`] with the ceiling of its $n$th root.
    ///
    /// $x \gets \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingRootAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(999);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(1000);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(1001);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 11);
    ///
    /// let mut x = Integer::from(100000000000i64);
    /// x.ceiling_root_assign(5);
    /// assert_eq!(x, 159);
    ///
    /// let mut x = Integer::from(-100000000000i64);
    /// x.ceiling_root_assign(5);
    /// assert_eq!(x, -158);
    /// ```
    #[inline]
    fn ceiling_root_assign(&mut self, exp: u64) {
        if *self >= 0 {
            self.mutate_unsigned_abs(|n| n.ceiling_root_assign(exp));
        } else if exp.odd() {
            self.mutate_unsigned_abs(|n| n.floor_root_assign(exp));
        } else {
            panic!("Cannot take even root of {self}")
        }
    }
}

impl CheckedRoot<u64> for Integer {
    type Output = Self;

    /// Returns the the $n$th root of an [`Integer`], or `None` if the [`Integer`] is not a perfect
    /// $n$th power. The [`Integer`] is taken by value.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(999).checked_root(3).to_debug_string(), "None");
    /// assert_eq!(
    ///     Integer::from(1000).checked_root(3).to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     Integer::from(1001).checked_root(3).to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Integer::from(100000000000i64)
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Integer::from(-100000000000i64)
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Integer::from(10000000000i64)
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "Some(100)"
    /// );
    /// assert_eq!(
    ///     Integer::from(-10000000000i64)
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "Some(-100)"
    /// );
    /// ```
    #[inline]
    fn checked_root(self, exp: u64) -> Option<Self> {
        if self >= 0 {
            self.unsigned_abs().checked_root(exp).map(Self::from)
        } else if exp.odd() {
            self.unsigned_abs().checked_root(exp).map(Natural::neg)
        } else {
            panic!("Cannot take even root of {self}")
        }
    }
}

impl CheckedRoot<u64> for &Integer {
    type Output = Integer;

    /// Returns the the $n$th root of an [`Integer`], or `None` if the [`Integer`] is not a perfect
    /// $n$th power. The [`Integer`] is taken by reference.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(999)).checked_root(3).to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1000)).checked_root(3).to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1001)).checked_root(3).to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(100000000000i64))
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-100000000000i64))
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10000000000i64))
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "Some(100)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10000000000i64))
    ///         .checked_root(5)
    ///         .to_debug_string(),
    ///     "Some(-100)"
    /// );
    /// ```
    #[inline]
    fn checked_root(self, exp: u64) -> Option<Integer> {
        if *self >= 0 {
            self.unsigned_abs_ref().checked_root(exp).map(Integer::from)
        } else if exp.odd() {
            self.unsigned_abs_ref().checked_root(exp).map(Natural::neg)
        } else {
            panic!("Cannot take even root of {self}")
        }
    }
}

// The floor root of a negative value is the negated ceiling root of its absolute value, so the
// remainder is what the absolute value falls short of that ceiling root's power.
fn root_rem_neg(abs: Natural, exp: u64) -> (Integer, Integer) {
    let (floor, rem) = (&abs).root_rem(exp);
    if rem == 0u32 {
        (-Integer::from(floor), Integer::ZERO)
    } else {
        let ceiling = floor + Natural::ONE;
        let rem = (&ceiling).pow(exp) - abs;
        (-Integer::from(ceiling), Integer::from(rem))
    }
}

impl RootRem<u64> for Integer {
    type RootOutput = Self;
    type RemOutput = Self;

    /// Returns the floor of the $n$th root of an [`Integer`], and the remainder (the difference
    /// between the [`Integer`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^n)$.
    ///
    /// The remainder is always non-negative, because the root is rounded toward negative infinity.
    /// GMP's `mpz_rootrem` truncates toward zero instead, so on a negative operand its root is this
    /// one plus 1, unless the root is exact, and its remainder is negative; see [`CeilingRoot`] for
    /// that convention.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
    ///
    /// The [`Integer`] is taken by value.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootRem;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(999).root_rem(3),
    ///     (Integer::from(9), Integer::from(270))
    /// );
    /// assert_eq!(
    ///     Integer::from(1000).root_rem(3),
    ///     (Integer::from(10), Integer::ZERO)
    /// );
    /// // the root is rounded down, so the remainder stays non-negative
    /// assert_eq!(
    ///     Integer::from(-999).root_rem(3),
    ///     (Integer::from(-10), Integer::ONE)
    /// );
    /// assert_eq!(
    ///     Integer::from(-1000).root_rem(3),
    ///     (Integer::from(-10), Integer::ZERO)
    /// );
    /// ```
    fn root_rem(self, exp: u64) -> (Self, Self) {
        if self >= 0 {
            let (root, rem) = self.unsigned_abs().root_rem(exp);
            (Self::from(root), Self::from(rem))
        } else if exp.odd() {
            root_rem_neg(self.unsigned_abs(), exp)
        } else {
            panic!("Cannot take even root of a negative Integer")
        }
    }
}

impl RootRem<u64> for &Integer {
    type RootOutput = Integer;
    type RemOutput = Integer;

    /// Returns the floor of the $n$th root of an [`Integer`], and the remainder (the difference
    /// between the [`Integer`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^n)$.
    ///
    /// The remainder is always non-negative, because the root is rounded toward negative infinity.
    /// GMP's `mpz_rootrem` truncates toward zero instead, so on a negative operand its root is this
    /// one plus 1, unless the root is exact, and its remainder is negative; see [`CeilingRoot`] for
    /// that convention.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
    ///
    /// The [`Integer`] is taken by reference.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootRem;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(999)).root_rem(3),
    ///     (Integer::from(9), Integer::from(270))
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1000)).root_rem(3),
    ///     (Integer::from(10), Integer::ZERO)
    /// );
    /// // the root is rounded down, so the remainder stays non-negative
    /// assert_eq!(
    ///     (&Integer::from(-999)).root_rem(3),
    ///     (Integer::from(-10), Integer::ONE)
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-1000)).root_rem(3),
    ///     (Integer::from(-10), Integer::ZERO)
    /// );
    /// ```
    fn root_rem(self, exp: u64) -> (Integer, Integer) {
        if *self >= 0 {
            let (root, rem) = self.unsigned_abs_ref().root_rem(exp);
            (Integer::from(root), Integer::from(rem))
        } else if exp.odd() {
            root_rem_neg(self.unsigned_abs(), exp)
        } else {
            panic!("Cannot take even root of a negative Integer")
        }
    }
}

impl RootAssignRem<u64> for Integer {
    type RemOutput = Self;

    /// Replaces an [`Integer`] with the floor of its $n$th root, and returns the remainder (the
    /// difference between the original [`Integer`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = x - \lfloor\sqrt\[n\]{x}\rfloor^n$,
    ///
    /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// The remainder is always non-negative; see [`RootRem`] for how that compares with GMP.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RootAssignRem;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(999);
    /// assert_eq!(x.root_assign_rem(3), 270);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Integer::from(-999);
    /// assert_eq!(x.root_assign_rem(3), 1);
    /// assert_eq!(x, -10);
    /// ```
    fn root_assign_rem(&mut self, exp: u64) -> Self {
        let (root, rem) = core::mem::take(self).root_rem(exp);
        *self = root;
        rem
    }
}
