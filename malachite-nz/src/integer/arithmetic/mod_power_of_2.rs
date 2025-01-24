// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CeilingModPowerOf2, CeilingModPowerOf2Assign, ModPowerOf2, ModPowerOf2Assign, NegModPowerOf2,
    NegModPowerOf2Assign, RemPowerOf2, RemPowerOf2Assign,
};

impl ModPowerOf2 for Integer {
    type Output = Natural;

    /// Divides an [`Integer`] by $2^k$, taking it by value and returning just the remainder. The
    /// remainder is non-negative.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
    /// $$
    ///
    /// Unlike
    /// [`rem_power_of_2`](malachite_base::num::arithmetic::traits::RemPowerOf2::rem_power_of_2),
    /// this function always returns a non-negative number.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).mod_power_of_2(8), 4);
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!(Integer::from(-1611).mod_power_of_2(4), 5);
    /// ```
    fn mod_power_of_2(self, pow: u64) -> Natural {
        if self.sign {
            self.abs.mod_power_of_2(pow)
        } else {
            self.abs.neg_mod_power_of_2(pow)
        }
    }
}

impl ModPowerOf2 for &Integer {
    type Output = Natural;

    /// Divides an [`Integer`] by $2^k$, taking it by reference and returning just the remainder.
    /// The remainder is non-negative.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
    /// $$
    ///
    /// Unlike
    /// [`rem_power_of_2`](malachite_base::num::arithmetic::traits::RemPowerOf2::rem_power_of_2),
    /// this function always returns a non-negative number.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Integer::from(260)).mod_power_of_2(8), 4);
    /// // -101 * 2^4 + 5 = -1611
    /// assert_eq!((&Integer::from(-1611)).mod_power_of_2(4), 5);
    /// ```
    fn mod_power_of_2(self, pow: u64) -> Natural {
        if self.sign {
            (&self.abs).mod_power_of_2(pow)
        } else {
            (&self.abs).neg_mod_power_of_2(pow)
        }
    }
}

impl ModPowerOf2Assign for Integer {
    /// Divides an [`Integer`] by $2^k$, replacing the [`Integer`] by the remainder. The remainder
    /// is non-negative.
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = q2^k + r$ and $0
    /// \leq r < 2^k$.
    ///
    /// $$
    /// x \gets x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
    /// $$
    ///
    /// Unlike [`rem_power_of_2_assign`](RemPowerOf2Assign::rem_power_of_2_assign), this function
    /// always assigns a non-negative number.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.mod_power_of_2_assign(8);
    /// assert_eq!(x, 4);
    ///
    /// // -101 * 2^4 + 5 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.mod_power_of_2_assign(4);
    /// assert_eq!(x, 5);
    /// ```
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        if self.sign {
            self.abs.mod_power_of_2_assign(pow);
        } else {
            self.sign = true;
            self.abs.neg_mod_power_of_2_assign(pow);
        }
    }
}

impl RemPowerOf2 for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by $2^k$, taking it by value and returning just the remainder. The
    /// remainder has the same sign as the first number.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq |r| < 2^k$.
    ///
    /// $$
    /// f(x, k) = x - 2^k\operatorname{sgn}(x)\left \lfloor \frac{|x|}{2^k} \right \rfloor.
    /// $$
    ///
    /// Unlike
    /// [`mod_power_of_2`](malachite_base::num::arithmetic::traits::ModPowerOf2::mod_power_of_2),
    /// this function always returns zero or a number with the same sign as `self`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Integer::from(260).rem_power_of_2(8), 4);
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).rem_power_of_2(4), -11);
    /// ```
    fn rem_power_of_2(self, pow: u64) -> Integer {
        let abs_rem = self.abs.mod_power_of_2(pow);
        Integer {
            sign: self.sign || abs_rem == 0,
            abs: abs_rem,
        }
    }
}

impl RemPowerOf2 for &Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by $2^k$, taking it by reference and returning just the remainder.
    /// The remainder has the same sign as the first number.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq |r| < 2^k$.
    ///
    /// $$
    /// f(x, k) = x - 2^k\operatorname{sgn}(x)\left \lfloor \frac{|x|}{2^k} \right \rfloor.
    /// $$
    ///
    /// Unlike
    /// [`mod_power_of_2`](malachite_base::num::arithmetic::traits::ModPowerOf2::mod_power_of_2),
    /// this function always returns zero or a number with the same sign as `self`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Integer::from(260)).rem_power_of_2(8), 4);
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!((&Integer::from(-1611)).rem_power_of_2(4), -11);
    /// ```
    fn rem_power_of_2(self, pow: u64) -> Integer {
        let abs_rem = (&self.abs).mod_power_of_2(pow);
        Integer {
            sign: self.sign || abs_rem == 0,
            abs: abs_rem,
        }
    }
}

impl RemPowerOf2Assign for Integer {
    /// Divides an [`Integer`] by $2^k$, replacing the [`Integer`] by the remainder. The remainder
    /// has the same sign as the [`Integer`].
    ///
    /// If the quotient were computed, he quotient and remainder would satisfy $x = q2^k + r$ and $0
    /// \leq r < 2^k$.
    ///
    /// $$
    /// x \gets x - 2^k\operatorname{sgn}(x)\left \lfloor \frac{|x|}{2^k} \right \rfloor.
    /// $$
    ///
    /// Unlike [`mod_power_of_2_assign`](ModPowerOf2Assign::mod_power_of_2_assign), this function
    /// does never changes the sign of `self`, except possibly to set `self` to 0.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Integer::from(260);
    /// x.rem_power_of_2_assign(8);
    /// assert_eq!(x, 4);
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.rem_power_of_2_assign(4);
    /// assert_eq!(x, -11);
    /// ```
    fn rem_power_of_2_assign(&mut self, pow: u64) {
        self.abs.mod_power_of_2_assign(pow);
        if self.abs == 0 {
            self.sign = true;
        }
    }
}

impl CeilingModPowerOf2 for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by $2^k$, taking it by value and returning just the remainder. The
    /// remainder is non-positive.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq -r < 2^k$.
    ///
    /// $$
    /// f(x, y) =  x - 2^k\left \lceil \frac{x}{2^k} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!(Integer::from(260).ceiling_mod_power_of_2(8), -252);
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!(Integer::from(-1611).ceiling_mod_power_of_2(4), -11);
    /// ```
    fn ceiling_mod_power_of_2(self, pow: u64) -> Integer {
        let abs_mod = if self.sign {
            self.abs.neg_mod_power_of_2(pow)
        } else {
            self.abs.mod_power_of_2(pow)
        };
        Integer {
            sign: abs_mod == 0,
            abs: abs_mod,
        }
    }
}

impl CeilingModPowerOf2 for &Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by $2^k$, taking it by reference and returning just the remainder.
    /// The remainder is non-positive.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq -r < 2^k$.
    ///
    /// $$
    /// f(x, y) =  x - 2^k\left \lceil \frac{x}{2^k} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// assert_eq!((&Integer::from(260)).ceiling_mod_power_of_2(8), -252);
    /// // -100 * 2^4 + -11 = -1611
    /// assert_eq!((&Integer::from(-1611)).ceiling_mod_power_of_2(4), -11);
    /// ```
    fn ceiling_mod_power_of_2(self, pow: u64) -> Integer {
        let abs_mod = if self.sign {
            (&self.abs).neg_mod_power_of_2(pow)
        } else {
            (&self.abs).mod_power_of_2(pow)
        };
        Integer {
            sign: abs_mod == 0,
            abs: abs_mod,
        }
    }
}

impl CeilingModPowerOf2Assign for Integer {
    /// Divides an [`Integer`] by $2^k$, replacing the [`Integer`] by the remainder. The remainder
    /// is non-positive.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq -r < 2^k$.
    ///
    /// $$
    /// x \gets x - 2^k\left \lceil\frac{x}{2^k} \right \rceil.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CeilingModPowerOf2Assign;
    /// use malachite_nz::integer::Integer;
    ///
    /// // 2 * 2^8 + -252 = 260
    /// let mut x = Integer::from(260);
    /// x.ceiling_mod_power_of_2_assign(8);
    /// assert_eq!(x, -252);
    ///
    /// // -100 * 2^4 + -11 = -1611
    /// let mut x = Integer::from(-1611);
    /// x.ceiling_mod_power_of_2_assign(4);
    /// assert_eq!(x, -11);
    /// ```
    fn ceiling_mod_power_of_2_assign(&mut self, pow: u64) {
        if self.sign {
            self.abs.neg_mod_power_of_2_assign(pow);
        } else {
            self.abs.mod_power_of_2_assign(pow);
        };
        self.sign = self.abs == 0;
    }
}
