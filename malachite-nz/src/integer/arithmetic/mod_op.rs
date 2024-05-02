// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign,
};

impl Mod<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning just the
    /// remainder. The remainder has the same sign as the second [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23).mod_op(Integer::from(10)), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(Integer::from(23).mod_op(Integer::from(-10)), -7);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23).mod_op(Integer::from(10)), 7);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23).mod_op(Integer::from(-10)), -3);
    /// ```
    #[inline]
    fn mod_op(mut self, other: Integer) -> Integer {
        self.mod_assign(other);
        self
    }
}

impl<'a> Mod<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning just the remainder. The remainder has the same sign as the second
    /// [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23).mod_op(&Integer::from(10)), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(Integer::from(23).mod_op(&Integer::from(-10)), -7);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23).mod_op(&Integer::from(10)), 7);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23).mod_op(&Integer::from(-10)), -3);
    /// ```
    #[inline]
    fn mod_op(mut self, other: &'a Integer) -> Integer {
        self.mod_assign(other);
        self
    }
}

impl<'a> Mod<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning just the remainder. The remainder has the same sign as the second
    /// [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23)).mod_op(Integer::from(10)), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((&Integer::from(23)).mod_op(Integer::from(-10)), -7);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((&Integer::from(-23)).mod_op(Integer::from(10)), 7);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23)).mod_op(Integer::from(-10)), -3);
    /// ```
    fn mod_op(self, other: Integer) -> Integer {
        Integer::from_sign_and_abs(
            other.sign,
            if self.sign == other.sign {
                &self.abs % other.abs
            } else {
                (&self.abs).neg_mod(other.abs)
            },
        )
    }
}

impl<'a, 'b> Mod<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning just
    /// the remainder. The remainder has the same sign as the second [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
    ///
    /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Mod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23)).mod_op(&Integer::from(10)), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((&Integer::from(23)).mod_op(&Integer::from(-10)), -7);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((&Integer::from(-23)).mod_op(&Integer::from(10)), 7);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23)).mod_op(&Integer::from(-10)), -3);
    /// ```
    fn mod_op(self, other: &'b Integer) -> Integer {
        Integer::from_sign_and_abs(
            other.sign,
            if self.sign == other.sign {
                &self.abs % &other.abs
            } else {
                (&self.abs).neg_mod(&other.abs)
            },
        )
    }
}

impl ModAssign<Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the second [`Integer`] by value and
    /// replacing the first by the remainder. The remainder has the same sign as the second
    /// [`Integer`].
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x.mod_assign(Integer::from(10));
    /// assert_eq!(x, 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x.mod_assign(Integer::from(-10));
    /// assert_eq!(x, -7);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.mod_assign(Integer::from(10));
    /// assert_eq!(x, 7);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x.mod_assign(Integer::from(-10));
    /// assert_eq!(x, -3);
    /// ```
    fn mod_assign(&mut self, other: Integer) {
        if self.sign == other.sign {
            self.abs %= other.abs;
        } else {
            self.abs.neg_mod_assign(other.abs);
        };
        self.sign = other.sign || self.abs == 0;
    }
}

impl<'a> ModAssign<&'a Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the second [`Integer`] by reference
    /// and replacing the first by the remainder. The remainder has the same sign as the second
    /// [`Integer`].
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x.mod_assign(&Integer::from(10));
    /// assert_eq!(x, 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x.mod_assign(&Integer::from(-10));
    /// assert_eq!(x, -7);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.mod_assign(&Integer::from(10));
    /// assert_eq!(x, 7);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x.mod_assign(&Integer::from(-10));
    /// assert_eq!(x, -3);
    /// ```
    fn mod_assign(&mut self, other: &'a Integer) {
        if self.sign == other.sign {
            self.abs %= &other.abs;
        } else {
            self.abs.neg_mod_assign(&other.abs);
        };
        self.sign = other.sign || self.abs == 0;
    }
}

impl Rem<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning just the
    /// remainder. The remainder has the same sign as the first [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23) % Integer::from(10), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(Integer::from(23) % Integer::from(-10), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23) % Integer::from(10), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23) % Integer::from(-10), -3);
    /// ```
    #[inline]
    fn rem(mut self, other: Integer) -> Integer {
        self %= other;
        self
    }
}

impl<'a> Rem<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning just the remainder. The remainder has the same sign as the first
    /// [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23) % &Integer::from(10), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(Integer::from(23) % &Integer::from(-10), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23) % &Integer::from(10), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23) % &Integer::from(-10), -3);
    /// ```
    #[inline]
    fn rem(mut self, other: &'a Integer) -> Integer {
        self %= other;
        self
    }
}

impl<'a> Rem<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning just the remainder. The remainder has the same sign as the first
    /// [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(&Integer::from(23) % Integer::from(10), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(&Integer::from(23) % Integer::from(-10), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(&Integer::from(-23) % Integer::from(10), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(&Integer::from(-23) % Integer::from(-10), -3);
    /// ```
    #[inline]
    fn rem(self, other: Integer) -> Integer {
        Integer::from_sign_and_abs(self.sign, &self.abs % other.abs)
    }
}

impl<'a, 'b> Rem<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning just
    /// the remainder. The remainder has the same sign as the first [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) = x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(&Integer::from(23) % &Integer::from(10), 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(&Integer::from(23) % &Integer::from(-10), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(&Integer::from(-23) % &Integer::from(10), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(&Integer::from(-23) % &Integer::from(-10), -3);
    /// ```
    #[inline]
    fn rem(self, other: &'b Integer) -> Integer {
        Integer::from_sign_and_abs(self.sign, &self.abs % &other.abs)
    }
}

impl RemAssign<Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the second [`Integer`] by value and
    /// replacing the first by the remainder. The remainder has the same sign as the first
    /// [`Integer`].
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x %= Integer::from(10);
    /// assert_eq!(x, 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x %= Integer::from(-10);
    /// assert_eq!(x, 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x %= Integer::from(10);
    /// assert_eq!(x, -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x %= Integer::from(-10);
    /// assert_eq!(x, -3);
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Integer) {
        self.abs %= other.abs;
        self.sign = self.sign || self.abs == 0;
    }
}

impl<'a> RemAssign<&'a Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the second [`Integer`] by reference
    /// and replacing the first by the remainder. The remainder has the same sign as the first
    /// [`Integer`].
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y \operatorname{sgn}(xy)
    ///     \left \lfloor \left | \frac{x}{y} \right | \right \rfloor.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x %= &Integer::from(10);
    /// assert_eq!(x, 3);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x %= &Integer::from(-10);
    /// assert_eq!(x, 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x %= &Integer::from(10);
    /// assert_eq!(x, -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x %= &Integer::from(-10);
    /// assert_eq!(x, -3);
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &'a Integer) {
        self.abs %= &other.abs;
        self.sign = self.sign || self.abs == 0;
    }
}

impl CeilingMod<Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by value and returning just the
    /// remainder. The remainder has the opposite sign as the second [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23).ceiling_mod(Integer::from(10)), -7);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(Integer::from(23).ceiling_mod(Integer::from(-10)), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23).ceiling_mod(Integer::from(10)), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23).ceiling_mod(Integer::from(-10)), 7);
    /// ```
    #[inline]
    fn ceiling_mod(mut self, other: Integer) -> Integer {
        self.ceiling_mod_assign(other);
        self
    }
}

impl<'a> CeilingMod<&'a Integer> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by value and the second by
    /// reference and returning just the remainder. The remainder has the opposite sign as the
    /// second [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Integer::from(23).ceiling_mod(&Integer::from(10)), -7);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!(Integer::from(23).ceiling_mod(&Integer::from(-10)), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!(Integer::from(-23).ceiling_mod(&Integer::from(10)), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!(Integer::from(-23).ceiling_mod(&Integer::from(-10)), 7);
    /// ```
    #[inline]
    fn ceiling_mod(mut self, other: &'a Integer) -> Integer {
        self.ceiling_mod_assign(other);
        self
    }
}

impl<'a> CeilingMod<Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking the first by reference and the second
    /// by value and returning just the remainder. The remainder has the opposite sign as the second
    /// [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23)).ceiling_mod(Integer::from(10)), -7);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((&Integer::from(23)).ceiling_mod(Integer::from(-10)), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((&Integer::from(-23)).ceiling_mod(Integer::from(10)), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23)).ceiling_mod(Integer::from(-10)), 7);
    /// ```
    fn ceiling_mod(self, other: Integer) -> Integer {
        Integer::from_sign_and_abs(
            !other.sign,
            if self.sign == other.sign {
                (&self.abs).neg_mod(other.abs)
            } else {
                &self.abs % other.abs
            },
        )
    }
}

impl<'a, 'b> CeilingMod<&'b Integer> for &'a Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], taking both by reference and returning just
    /// the remainder. The remainder has the opposite sign as the second [`Integer`].
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingMod;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Integer::from(23)).ceiling_mod(&Integer::from(10)), -7);
    ///
    /// // -3 * -10 + -7 = 23
    /// assert_eq!((&Integer::from(23)).ceiling_mod(&Integer::from(-10)), 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// assert_eq!((&Integer::from(-23)).ceiling_mod(&Integer::from(10)), -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// assert_eq!((&Integer::from(-23)).ceiling_mod(&Integer::from(-10)), 7);
    /// ```
    fn ceiling_mod(self, other: &'b Integer) -> Integer {
        Integer::from_sign_and_abs(
            !other.sign,
            if self.sign == other.sign {
                (&self.abs).neg_mod(&other.abs)
            } else {
                &self.abs % &other.abs
            },
        )
    }
}

impl CeilingModAssign<Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the [`Integer`] on the right-hand side
    /// by value and replacing the first number by the remainder. The remainder has the opposite
    /// sign as the second number.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lceil\frac{x}{y} \right \rceil.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x.ceiling_mod_assign(Integer::from(10));
    /// assert_eq!(x, -7);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x.ceiling_mod_assign(Integer::from(-10));
    /// assert_eq!(x, 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.ceiling_mod_assign(Integer::from(10));
    /// assert_eq!(x, -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x.ceiling_mod_assign(Integer::from(-10));
    /// assert_eq!(x, 7);
    /// ```
    fn ceiling_mod_assign(&mut self, other: Integer) {
        if self.sign == other.sign {
            self.abs.neg_mod_assign(other.abs);
        } else {
            self.abs %= other.abs;
        };
        self.sign = !other.sign || self.abs == 0;
    }
}

impl<'a> CeilingModAssign<&'a Integer> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], taking the [`Integer`] on the right-hand side
    /// by reference and replacing the first number by the remainder. The remainder has the opposite
    /// sign as the second number.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq |r| < |y|$.
    ///
    /// $$
    /// x \gets x - y\left \lceil\frac{x}{y} \right \rceil.
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
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Integer::from(23);
    /// x.ceiling_mod_assign(&Integer::from(10));
    /// assert_eq!(x, -7);
    ///
    /// // -3 * -10 + -7 = 23
    /// let mut x = Integer::from(23);
    /// x.ceiling_mod_assign(&Integer::from(-10));
    /// assert_eq!(x, 3);
    ///
    /// // -3 * 10 + 7 = -23
    /// let mut x = Integer::from(-23);
    /// x.ceiling_mod_assign(&Integer::from(10));
    /// assert_eq!(x, -3);
    ///
    /// // 2 * -10 + -3 = -23
    /// let mut x = Integer::from(-23);
    /// x.ceiling_mod_assign(&Integer::from(-10));
    /// assert_eq!(x, 7);
    /// ```
    fn ceiling_mod_assign(&mut self, other: &'a Integer) {
        if self.sign == other.sign {
            self.abs.neg_mod_assign(&other.abs);
        } else {
            self.abs %= &other.abs;
        };
        self.sign = !other.sign || self.abs == 0;
    }
}
