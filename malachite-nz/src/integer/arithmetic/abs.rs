// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};

impl Abs for Integer {
    type Output = Integer;

    /// Takes the absolute value of an [`Integer`], taking the [`Integer`] by value.
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.abs(), 0);
    /// assert_eq!(Integer::from(123).abs(), 123);
    /// assert_eq!(Integer::from(-123).abs(), 123);
    /// ```
    #[inline]
    fn abs(mut self) -> Integer {
        self.sign = true;
        self
    }
}

impl Abs for &Integer {
    type Output = Integer;

    /// Takes the absolute value of an [`Integer`], taking the [`Integer`] by reference.
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO).abs(), 0);
    /// assert_eq!((&Integer::from(123)).abs(), 123);
    /// assert_eq!((&Integer::from(-123)).abs(), 123);
    /// ```
    fn abs(self) -> Integer {
        Integer {
            sign: true,
            abs: self.abs.clone(),
        }
    }
}

impl AbsAssign for Integer {
    /// Replaces an [`Integer`] with its absolute value.
    ///
    /// $$
    /// x \gets |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.abs_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(123);
    /// x.abs_assign();
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Integer::from(-123);
    /// x.abs_assign();
    /// assert_eq!(x, 123);
    /// ```
    #[inline]
    fn abs_assign(&mut self) {
        self.sign = true;
    }
}

impl UnsignedAbs for Integer {
    type Output = Natural;

    /// Takes the absolute value of an [`Integer`], taking the [`Integer`] by value and converting
    /// the result to a [`Natural`].
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::UnsignedAbs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.unsigned_abs(), 0);
    /// assert_eq!(Integer::from(123).unsigned_abs(), 123);
    /// assert_eq!(Integer::from(-123).unsigned_abs(), 123);
    /// ```
    #[inline]
    fn unsigned_abs(self) -> Natural {
        self.abs
    }
}

impl UnsignedAbs for &Integer {
    type Output = Natural;

    /// Takes the absolute value of an [`Integer`], taking the [`Integer`] by reference and
    /// converting the result to a [`Natural`].
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::UnsignedAbs;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO).unsigned_abs(), 0);
    /// assert_eq!((&Integer::from(123)).unsigned_abs(), 123);
    /// assert_eq!((&Integer::from(-123)).unsigned_abs(), 123);
    /// ```
    #[inline]
    fn unsigned_abs(self) -> Natural {
        self.abs.clone()
    }
}

impl Integer {
    /// Finds the absolute value of an [`Integer`], taking the [`Integer`] by reference and
    /// returning a reference to the internal [`Natural`] absolute value.
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(*Integer::ZERO.unsigned_abs_ref(), 0);
    /// assert_eq!(*Integer::from(123).unsigned_abs_ref(), 123);
    /// assert_eq!(*Integer::from(-123).unsigned_abs_ref(), 123);
    /// ```
    #[inline]
    pub const fn unsigned_abs_ref(&self) -> &Natural {
        &self.abs
    }

    /// Mutates the absolute value of an [`Integer`] using a provided closure, and then returns
    /// whatever the closure returns.
    ///
    /// This function is similar to the [`unsigned_abs_ref`](Integer::unsigned_abs_ref) function,
    /// which returns a reference to the absolute value. A function that returns a _mutable_
    /// reference would be too dangerous, as it could leave the [`Integer`] in an invalid state
    /// (specifically, with a negative sign but a zero absolute value). So rather than returning a
    /// mutable reference, this function allows mutation of the absolute value using a closure.
    /// After the closure executes, this function ensures that the [`Integer`] remains valid.
    ///
    /// There is only constant time and memory overhead on top of the time and memory used by the
    /// closure.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::DivAssignMod;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut n = Integer::from(-123);
    /// let remainder = n.mutate_unsigned_abs(|x| x.div_assign_mod(Natural::TWO));
    /// assert_eq!(n, -61);
    /// assert_eq!(remainder, 1);
    ///
    /// let mut n = Integer::from(-123);
    /// n.mutate_unsigned_abs(|x| *x >>= 10);
    /// assert_eq!(n, 0);
    /// ```
    pub fn mutate_unsigned_abs<F: FnOnce(&mut Natural) -> T, T>(&mut self, f: F) -> T {
        let out = f(&mut self.abs);
        if !self.sign && self.abs == 0 {
            self.sign = true;
        }
        out
    }
}
