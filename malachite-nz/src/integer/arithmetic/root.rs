use integer::Integer;
use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, FloorRoot, FloorRootAssign, Parity, UnsignedAbs,
};
use natural::Natural;
use std::ops::Neg;

impl FloorRootAssign for Integer {
    /// Replaces an `Integer` with the floor of its square root.
    ///
    /// $x \gets \lfloor\root{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            self.unsigned_abs_mut(|n| n.floor_root_assign(exp));
        } else if exp.odd() {
            self.unsigned_abs_mut(|n| n.ceiling_root_assign(exp));
        } else {
            panic!("Cannot take even root of {}", self)
        }
    }
}

impl FloorRoot for Integer {
    type Output = Integer;

    /// Returns the floor of the square root of an `Integer`, taking the `Integer` by value.
    ///
    /// $f(x) = \lfloor\root{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
    fn floor_root(mut self, exp: u64) -> Integer {
        self.floor_root_assign(exp);
        self
    }
}

impl<'a> FloorRoot for &'a Integer {
    type Output = Integer;

    /// Returns the floor of the square root of a `Integer`, taking the `Integer` by reference.
    ///
    /// $f(x) = \lfloor\root{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl CeilingRootAssign for Integer {
    /// Replaces an `Integer` with the ceiling of its square root.
    ///
    /// $x \gets \lceil\root{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            self.unsigned_abs_mut(|n| n.ceiling_root_assign(exp));
        } else if exp.odd() {
            self.unsigned_abs_mut(|n| n.floor_root_assign(exp));
        } else {
            panic!("Cannot take even root of {}", self)
        }
    }
}

impl CeilingRoot for Integer {
    type Output = Integer;

    /// Returns the ceiling of the square root of an `Integer`, taking the `Integer` by value.
    ///
    /// $f(x) = \lceil\root{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
    fn ceiling_root(mut self, exp: u64) -> Integer {
        self.ceiling_root_assign(exp);
        self
    }
}

impl<'a> CeilingRoot for &'a Integer {
    type Output = Integer;

    /// Returns the ceiling of the square root of an `Integer`, taking the `Integer` by reference.
    ///
    /// $f(x) = \lceil\root{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl CheckedRoot for Integer {
    type Output = Integer;

    /// Returns the the square root of an `Integer`, or `None` if the `Integer` is not a perfect
    /// square. The `Integer` is taken by value.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(root{x}) & \root{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(999).checked_root(3).to_debug_string(), "None");
    /// assert_eq!(Integer::from(1000).checked_root(3).to_debug_string(), "Some(10)");
    /// assert_eq!(Integer::from(1001).checked_root(3).to_debug_string(), "None");
    /// assert_eq!(Integer::from(100000000000i64).checked_root(5).to_debug_string(), "None");
    /// assert_eq!(Integer::from(-100000000000i64).checked_root(5).to_debug_string(), "None");
    /// assert_eq!(Integer::from(10000000000i64).checked_root(5).to_debug_string(), "Some(100)");
    /// assert_eq!(Integer::from(-10000000000i64).checked_root(5).to_debug_string(), "Some(-100)");
    /// ```
    #[inline]
    fn checked_root(self, exp: u64) -> Option<Integer> {
        if self >= 0 {
            self.unsigned_abs().checked_root(exp).map(Integer::from)
        } else if exp.odd() {
            self.unsigned_abs().checked_root(exp).map(Natural::neg)
        } else {
            panic!("Cannot take even root of {}", self)
        }
    }
}

impl<'a> CheckedRoot for &'a Integer {
    type Output = Integer;

    /// Returns the the square root of an `Integer`, or `None` if the `Integer` is not a perfect
    /// square. The `Integer` is taken by reference.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(root{x}) & \root{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(999)).checked_root(3).to_debug_string(), "None");
    /// assert_eq!((&Integer::from(1000)).checked_root(3).to_debug_string(), "Some(10)");
    /// assert_eq!((&Integer::from(1001)).checked_root(3).to_debug_string(), "None");
    /// assert_eq!((&Integer::from(100000000000i64)).checked_root(5).to_debug_string(), "None");
    /// assert_eq!((&Integer::from(-100000000000i64)).checked_root(5).to_debug_string(), "None");
    /// assert_eq!((&Integer::from(10000000000i64)).checked_root(5).to_debug_string(), "Some(100)");
    /// assert_eq!(
    ///     (&Integer::from(-10000000000i64)).checked_root(5).to_debug_string(),
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
            panic!("Cannot take even root of {}", self)
        }
    }
}
