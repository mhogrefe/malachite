use integer::Integer;
use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, SqrtRem,
    SqrtRemAssign, UnsignedAbs,
};
use natural::Natural;

impl FloorSqrtAssign for Integer {
    /// Replaces an `Integer` with the floor of its square root.
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            self.unsigned_abs_mut(Natural::floor_sqrt_assign);
        } else {
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl FloorSqrt for Integer {
    type Output = Integer;

    /// Returns the floor of the square root of an `Integer`, taking the `Integer` by value.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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

    /// Returns the floor of the square root of a `Integer`, taking the `Integer` by reference.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl CeilingSqrtAssign for Integer {
    /// Replaces an `Integer` with the ceiling of its square root.
    ///
    /// $x \gets \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            self.unsigned_abs_mut(Natural::ceiling_sqrt_assign);
        } else {
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl CeilingSqrt for Integer {
    type Output = Integer;

    /// Returns the ceiling of the square root of an `Integer`, taking the `Integer` by value.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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

    /// Returns the ceiling of the square root of an `Integer`, taking the `Integer` by reference.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
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
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl CheckedSqrt for Integer {
    type Output = Integer;

    /// Returns the the square root of an `Integer`, or `None` if the `Integer` is not a perfect
    /// square. The `Integer` is taken by value.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(99u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Integer::from(100u8).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!(Integer::from(101u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Integer::from(1000000000u32).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Integer::from(10000000000u64).checked_sqrt().to_debug_string(), "Some(100000)");
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Integer> {
        if self >= 0 {
            self.unsigned_abs().checked_sqrt().map(Integer::from)
        } else {
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl<'a> CheckedSqrt for &'a Integer {
    type Output = Integer;

    /// Returns the the square root of an `Integer`, or `None` if the `Integer` is not a perfect
    /// square. The `Integer` is taken by reference.
    ///
    /// $$
    /// f(x) = \\begin{cases}
    ///     \operatorname{Some}(sqrt{x}) & \sqrt{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(99u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Integer::from(100u8)).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!((&Integer::from(101u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Integer::from(1000000000u32)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(
    ///     (&Integer::from(10000000000u64)).checked_sqrt().to_debug_string(),
    ///     "Some(100000)"
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Integer> {
        if *self >= 0 {
            self.unsigned_abs_ref().checked_sqrt().map(Integer::from)
        } else {
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl SqrtRemAssign for Integer {
    type RemOutput = Natural;

    /// Replaces an `Integer` with the floor of its square root, and returns the remainder (the
    /// difference between the original `Integer` and the square of the floor).
    ///
    /// $f(x) = x - \lfloor\sqrt{x}\rfloor^2$,
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRemAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(99);
    /// assert_eq!(x.sqrt_rem_assign(), 18);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Integer::from(100);
    /// assert_eq!(x.sqrt_rem_assign(), 0);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(101);
    /// assert_eq!(x.sqrt_rem_assign(), 1);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Integer::from(1000000000);
    /// assert_eq!(x.sqrt_rem_assign(), 49116);
    /// assert_eq!(x, 31622);
    ///
    /// let mut x = Integer::from(10000000000u64);
    /// assert_eq!(x.sqrt_rem_assign(), 0);
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn sqrt_rem_assign(&mut self) -> Natural {
        if *self >= 0 {
            self.unsigned_abs_mut(Natural::sqrt_rem_assign)
        } else {
            panic!("Cannot take square root of {}", self)
        }
    }
}

impl SqrtRem for Integer {
    type SqrtOutput = Integer;
    type RemOutput = Natural;

    /// Returns the floor of the square root of an `Integer`, and the remainder (the difference
    /// between the `Integer` and the square of the floor). The `Integer` is taken by value.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(99).sqrt_rem().to_debug_string(), "(9, 18)");
    /// assert_eq!(Integer::from(100).sqrt_rem().to_debug_string(), "(10, 0)");
    /// assert_eq!(Integer::from(101).sqrt_rem().to_debug_string(), "(10, 1)");
    /// assert_eq!(Integer::from(1000000000).sqrt_rem().to_debug_string(), "(31622, 49116)");
    /// assert_eq!(Integer::from(10000000000u64).sqrt_rem().to_debug_string(), "(100000, 0)");
    /// ```
    #[inline]
    fn sqrt_rem(mut self) -> (Integer, Natural) {
        let rem = self.sqrt_rem_assign();
        (self, rem)
    }
}

impl<'a> SqrtRem for &'a Integer {
    type SqrtOutput = Integer;
    type RemOutput = Natural;

    /// Returns the floor of the square root of an `Integer`, and the remainder (the difference
    /// between the `Integer` and the square of the floor). The `Integer` is taken by reference.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(99)).sqrt_rem().to_debug_string(), "(9, 18)");
    /// assert_eq!((&Integer::from(100)).sqrt_rem().to_debug_string(), "(10, 0)");
    /// assert_eq!((&Integer::from(101)).sqrt_rem().to_debug_string(), "(10, 1)");
    /// assert_eq!((&Integer::from(1000000000)).sqrt_rem().to_debug_string(), "(31622, 49116)");
    /// assert_eq!((&Integer::from(10000000000u64)).sqrt_rem().to_debug_string(), "(100000, 0)");
    /// ```
    #[inline]
    fn sqrt_rem(self) -> (Integer, Natural) {
        if *self >= 0 {
            let (sqrt, rem) = self.unsigned_abs_ref().sqrt_rem();
            (Integer::from(sqrt), rem)
        } else {
            panic!("Cannot take square root of {}", self)
        }
    }
}
