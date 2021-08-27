use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, DivRound, FloorRoot, FloorRootAssign, Pow,
    PowerOf2, ShrRound, RootRem, RootAssignRem
};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;
use std::cmp::Ordering;

#[doc(hidden)]
pub fn floor_inverse_binary<F: Fn(&Natural) -> Natural>(
    f: F,
    x: &Natural,
    mut low: Natural,
    mut high: Natural,
) -> Natural {
    loop {
        if high <= low {
            return low;
        }
        let mid = (&low + &high).shr_round(1, RoundingMode::Ceiling);
        match f(&mid).cmp(x) {
            Ordering::Equal => return mid,
            Ordering::Less => low = mid,
            Ordering::Greater => high = mid - Natural::ONE,
        }
    }
}

#[doc(hidden)]
pub fn _floor_root_binary(x: &Natural, exp: u64) -> Natural {
    if exp == 0 {
        panic!("Cannot take 0th root");
    } else if exp == 1 || x < &Natural::TWO {
        x.clone()
    } else {
        let p = Natural::power_of_2(x.significant_bits().div_round(exp, RoundingMode::Ceiling));
        floor_inverse_binary(|x| x.pow(exp), x, &p >> 1, p)
    }
}

#[doc(hidden)]
pub fn _ceiling_root_binary(x: &Natural, exp: u64) -> Natural {
    let floor_root = _floor_root_binary(x, exp);
    if &(&floor_root).pow(exp) == x {
        floor_root
    } else {
        floor_root + Natural::ONE
    }
}

#[doc(hidden)]
pub fn _checked_root_binary(x: &Natural, exp: u64) -> Option<Natural> {
    let floor_root = _floor_root_binary(x, exp);
    if &(&floor_root).pow(exp) == x {
        Some(floor_root)
    } else {
        None
    }
}

#[doc(hidden)]
pub fn _root_rem_binary(x: &Natural, exp: u64) -> (Natural, Natural) {
    let floor_root = _floor_root_binary(x, exp);
    let rem = x - (&floor_root).pow(exp);
    (floor_root, rem)
}

impl FloorRootAssign for Natural {
    /// Replaces a `Natural` with the floor of its $n$th root.
    ///
    /// $x \gets \lfloor\root{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorRootAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(999u16);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(1000u16);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1001u16);
    /// x.floor_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(100000000000u64);
    /// x.floor_root_assign(5);
    /// assert_eq!(x, 158);
    /// ```
    #[inline]
    fn floor_root_assign(&mut self, exp: u64) {
        *self = (&*self).floor_root(exp);
    }
}

impl FloorRoot for Natural {
    type Output = Natural;

    /// Returns the floor of the $n$th root of a `Natural`, taking the `Natural` by value.
    ///
    /// $f(x) = \lfloor\root{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).floor_root(3), 9);
    /// assert_eq!(Natural::from(1000u16).floor_root(3), 10);
    /// assert_eq!(Natural::from(1001u16).floor_root(3), 10);
    /// assert_eq!(Natural::from(100000000000u64).floor_root(5), 158);
    /// ```
    #[inline]
    fn floor_root(self, exp: u64) -> Natural {
        (&self).floor_root(exp)
    }
}

impl<'a> FloorRoot for &'a Natural {
    type Output = Natural;

    /// Returns the floor of the $n$th root of a `Natural`, taking the `Natural` by reference.
    ///
    /// $f(x, n) = \lfloor\root\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(999u16)).floor_root(3), 9);
    /// assert_eq!((&Natural::from(1000u16)).floor_root(3), 10);
    /// assert_eq!((&Natural::from(1001u16)).floor_root(3), 10);
    /// assert_eq!((&Natural::from(100000000000u64)).floor_root(5), 158);
    /// ```
    #[inline]
    fn floor_root(self, exp: u64) -> Natural {
        _floor_root_binary(self, exp)
    }
}

impl CeilingRootAssign for Natural {
    /// Replaces a `Natural` with the ceiling of its $n$th root.
    ///
    /// $x \gets \lceil\root\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingRootAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(999u16);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1000u16);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1001u16);
    /// x.ceiling_root_assign(3);
    /// assert_eq!(x, 11);
    ///
    /// let mut x = Natural::from(100000000000u64);
    /// x.ceiling_root_assign(5);
    /// assert_eq!(x, 159);
    /// ```
    #[inline]
    fn ceiling_root_assign(&mut self, exp: u64) {
        *self = (&*self).ceiling_root(exp);
    }
}

impl CeilingRoot for Natural {
    type Output = Natural;

    /// Returns the ceiling of the $n$th root of a `Natural`, taking the `Natural` by value.
    ///
    /// $f(x, n) = \lceil\root\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1000u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1001u16).ceiling_root(3), 11);
    /// assert_eq!(Natural::from(100000000000u64).ceiling_root(5), 159);
    /// ```
    #[inline]
    fn ceiling_root(self, exp: u64) -> Natural {
        (&self).ceiling_root(exp)
    }
}

impl<'a> CeilingRoot for &'a Natural {
    type Output = Natural;

    /// Returns the ceiling of the $n$th root of a `Natural`, taking the `Natural` by reference.
    ///
    /// $f(x) = \lceil\root\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingRoot;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1000u16).ceiling_root(3), 10);
    /// assert_eq!(Natural::from(1001u16).ceiling_root(3), 11);
    /// assert_eq!(Natural::from(100000000000u64).ceiling_root(5), 159);
    /// ```
    #[inline]
    fn ceiling_root(self, exp: u64) -> Natural {
        _ceiling_root_binary(self, exp)
    }
}

impl CheckedRoot for Natural {
    type Output = Natural;

    /// Returns the the $n$th root of a `Natural`, or `None` if the `Natural` is not a perfect
    /// $n$th power. The `Natural` is taken by value.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(root\[n\]{x}) & \root\[n\]{x} \in \Z \\\\
    ///     \operatorname{None} & \textrm{otherwise},
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).checked_root(3).to_debug_string(), "None");
    /// assert_eq!(Natural::from(1000u16).checked_root(3).to_debug_string(), "Some(10)");
    /// assert_eq!(Natural::from(1001u16).checked_root(3).to_debug_string(), "None");
    /// assert_eq!(Natural::from(100000000000u64).checked_root(5).to_debug_string(), "None");
    /// assert_eq!(Natural::from(10000000000u64).checked_root(5).to_debug_string(), "Some(100)");
    /// ```
    #[inline]
    fn checked_root(self, exp: u64) -> Option<Natural> {
        (&self).checked_root(exp)
    }
}

impl<'a> CheckedRoot for &'a Natural {
    type Output = Natural;

    /// Returns the the $n$th root of a `Natural`, or `None` if the `Natural` is not a perfect
    /// square. The `Natural` is taken by reference.
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
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(999u16)).checked_root(3).to_debug_string(), "None");
    /// assert_eq!((&Natural::from(1000u16)).checked_root(3).to_debug_string(), "Some(10)");
    /// assert_eq!((&Natural::from(1001u16)).checked_root(3).to_debug_string(), "None");
    /// assert_eq!((&Natural::from(100000000000u64)).checked_root(5).to_debug_string(), "None");
    /// assert_eq!(
    ///     (&Natural::from(10000000000u64)).checked_root(5).to_debug_string(),
    ///     "Some(100)"
    /// );
    /// ```
    #[inline]
    fn checked_root(self, exp: u64) -> Option<Natural> {
        _checked_root_binary(self, exp)
    }
}

impl RootAssignRem for Natural {
    type RemOutput = Natural;

    /// Replaces a `Natural` with the floor of its square root, and returns the remainder (the
    /// difference between the original `Natural` and the square of the floor).
    ///
    /// $f(x) = x - \lfloor\root{x}\rfloor^2$,
    ///
    /// $x \gets \lfloor\root{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RootAssignRem;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(999u16);
    /// assert_eq!(x.root_assign_rem(3), 270);
    /// assert_eq!(x, 9);
    ///
    /// let mut x = Natural::from(1000u16);
    /// assert_eq!(x.root_assign_rem(3), 0);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(1001u16);
    /// assert_eq!(x.root_assign_rem(3), 1);
    /// assert_eq!(x, 10);
    ///
    /// let mut x = Natural::from(100000000000u64);
    /// assert_eq!(x.root_assign_rem(5), 1534195232);
    /// assert_eq!(x, 158);
    /// ```
    #[inline]
    fn root_assign_rem(&mut self, exp: u64) -> Natural {
        let (root, rem) = (&*self).root_rem(exp);
        *self = root;
        rem
    }
}

impl RootRem for Natural {
    type RootOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a `Natural`, and the remainder (the difference
    /// between the `Natural` and the square of the floor). The `Natural` is taken by value.
    ///
    /// $f(x) = (\lfloor\root{x}\rfloor, x - \lfloor\root{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RootRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(999u16).root_rem(3).to_debug_string(), "(9, 270)");
    /// assert_eq!(Natural::from(1000u16).root_rem(3).to_debug_string(), "(10, 0)");
    /// assert_eq!(Natural::from(1001u16).root_rem(3).to_debug_string(), "(10, 1)");
    /// assert_eq!(
    ///     Natural::from(100000000000u64).root_rem(5).to_debug_string(),
    ///     "(158, 1534195232)"
    /// );
    /// ```
    #[inline]
    fn root_rem(self, exp: u64) -> (Natural, Natural) {
        (&self).root_rem(exp)
    }
}

impl<'a> RootRem for &'a Natural {
    type RootOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a `Natural`, and the remainder (the difference
    /// between the `Natural` and the square of the floor). The `Natural` is taken by reference.
    ///
    /// $f(x) = (\lfloor\root{x}\rfloor, x - \lfloor\root{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::RootRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(999u16)).root_rem(3).to_debug_string(), "(9, 270)");
    /// assert_eq!((&Natural::from(1000u16)).root_rem(3).to_debug_string(), "(10, 0)");
    /// assert_eq!((&Natural::from(1001u16)).root_rem(3).to_debug_string(), "(10, 1)");
    /// assert_eq!(
    ///     (&Natural::from(100000000000u64)).root_rem(5).to_debug_string(),
    ///     "(158, 1534195232)"
    /// );
    /// ```
    #[inline]
    fn root_rem(self, exp: u64) -> (Natural, Natural) {
        _root_rem_binary(self, exp)
    }
}
