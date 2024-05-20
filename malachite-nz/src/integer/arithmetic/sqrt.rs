// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, UnsignedAbs,
};

impl FloorSqrt for Integer {
    type Output = Integer;

    /// Returns the floor of the square root of an [`Integer`], taking it by value.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(99).floor_sqrt(), 9);
    /// assert_eq!(Integer::from(100).floor_sqrt(), 10);
    /// assert_eq!(Integer::from(101).floor_sqrt(), 10);
    /// assert_eq!(Integer::from(1000000000).floor_sqrt(), 31622);
    /// assert_eq!(Integer::from(10000000000u64).floor_sqrt(), 100000);
    /// ```
    #[inline]
    fn floor_sqrt(mut self) -> Integer {
        self.floor_sqrt_assign();
        self
    }
}

impl<'a> FloorSqrt for &'a Integer {
    type Output = Integer;

    /// Returns the floor of the square root of an [`Integer`], taking it by reference.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(99)).floor_sqrt(), 9);
    /// assert_eq!((&Integer::from(100)).floor_sqrt(), 10);
    /// assert_eq!((&Integer::from(101)).floor_sqrt(), 10);
    /// assert_eq!((&Integer::from(1000000000)).floor_sqrt(), 31622);
    /// assert_eq!((&Integer::from(10000000000u64)).floor_sqrt(), 100000);
    /// ```
    #[inline]
    fn floor_sqrt(self) -> Integer {
        if *self >= 0 {
            Integer::from(self.unsigned_abs_ref().floor_sqrt())
        } else {
            panic!("Cannot take square root of {self}")
        }
    }
}

impl FloorSqrtAssign for Integer {
    /// Replaces an [`Integer`] with the floor of its square root.
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::FloorSqrtAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(99);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Integer::from(100);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(101);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(1000000000);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 31622);
    ///
    /// let mut x = Integer::from(10000000000u64);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn floor_sqrt_assign(&mut self) {
        if *self >= 0 {
            self.mutate_unsigned_abs(Natural::floor_sqrt_assign);
        } else {
            panic!("Cannot take square root of {self}")
        }
    }
}

impl CeilingSqrt for Integer {
    type Output = Integer;

    /// Returns the ceiling of the square root of an [`Integer`], taking it by value.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(99).ceiling_sqrt(), 10);
    /// assert_eq!(Integer::from(100).ceiling_sqrt(), 10);
    /// assert_eq!(Integer::from(101).ceiling_sqrt(), 11);
    /// assert_eq!(Integer::from(1000000000).ceiling_sqrt(), 31623);
    /// assert_eq!(Integer::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt(mut self) -> Integer {
        self.ceiling_sqrt_assign();
        self
    }
}

impl<'a> CeilingSqrt for &'a Integer {
    type Output = Integer;

    /// Returns the ceiling of the square root of an [`Integer`], taking it by reference.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(99).ceiling_sqrt(), 10);
    /// assert_eq!(Integer::from(100).ceiling_sqrt(), 10);
    /// assert_eq!(Integer::from(101).ceiling_sqrt(), 11);
    /// assert_eq!(Integer::from(1000000000).ceiling_sqrt(), 31623);
    /// assert_eq!(Integer::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt(self) -> Integer {
        if *self >= 0 {
            Integer::from(self.unsigned_abs_ref().ceiling_sqrt())
        } else {
            panic!("Cannot take square root of {self}")
        }
    }
}

impl CeilingSqrtAssign for Integer {
    /// Replaces an [`Integer`] with the ceiling of its square root.
    ///
    /// $x \gets \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingSqrtAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(99u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(100);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(101);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 11);
    ///
    /// let mut x = Integer::from(1000000000);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 31623);
    ///
    /// let mut x = Integer::from(10000000000u64);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt_assign(&mut self) {
        if *self >= 0 {
            self.mutate_unsigned_abs(Natural::ceiling_sqrt_assign);
        } else {
            panic!("Cannot take square root of {self}")
        }
    }
}

impl CheckedSqrt for Integer {
    type Output = Integer;

    /// Returns the the square root of an [`Integer`], or `None` if it is not a perfect square. The
    /// [`Integer`] is taken by value.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(99u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(
    ///     Integer::from(100u8).checked_sqrt().to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     Integer::from(101u8).checked_sqrt().to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Integer::from(1000000000u32)
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Integer::from(10000000000u64)
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "Some(100000)"
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Integer> {
        if self >= 0 {
            self.unsigned_abs().checked_sqrt().map(Integer::from)
        } else {
            panic!("Cannot take square root of {self}")
        }
    }
}

impl<'a> CheckedSqrt for &'a Integer {
    type Output = Integer;

    /// Returns the the square root of an [`Integer`], or `None` if it is not a perfect square. The
    /// [`Integer`] is taken by reference.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \text{if} \\quad \sqrt{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(99u8)).checked_sqrt().to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(100u8)).checked_sqrt().to_debug_string(),
    ///     "Some(10)"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(101u8)).checked_sqrt().to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(1000000000u32))
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10000000000u64))
    ///         .checked_sqrt()
    ///         .to_debug_string(),
    ///     "Some(100000)"
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Integer> {
        if *self >= 0 {
            self.unsigned_abs_ref().checked_sqrt().map(Integer::from)
        } else {
            panic!("Cannot take square root of {self}")
        }
    }
}
