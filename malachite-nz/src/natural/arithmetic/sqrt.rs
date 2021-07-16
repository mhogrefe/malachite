use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, PowerOf2, ShrRound,
    SqrtRem, SqrtRemAssign, Square,
};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;
use std::cmp::Ordering;

fn floor_inverse_binary<F: Fn(&Natural) -> Natural>(
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
pub fn _floor_sqrt_binary(x: &Natural) -> Natural {
    if x < &Natural::TWO {
        x.clone()
    } else {
        let p = Natural::power_of_2(x.significant_bits().shr_round(1, RoundingMode::Ceiling));
        floor_inverse_binary(|x| x.square(), x, &p >> 1, p)
    }
}

#[doc(hidden)]
pub fn _ceiling_sqrt_binary(x: &Natural) -> Natural {
    let floor_sqrt = _floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        floor_sqrt
    } else {
        floor_sqrt + Natural::ONE
    }
}

#[doc(hidden)]
pub fn _checked_sqrt_binary(x: &Natural) -> Option<Natural> {
    let floor_sqrt = _floor_sqrt_binary(x);
    if &(&floor_sqrt).square() == x {
        Some(floor_sqrt)
    } else {
        None
    }
}

#[doc(hidden)]
pub fn _sqrt_rem_binary(x: &Natural) -> (Natural, Natural) {
    let floor_sqrt = _floor_sqrt_binary(x);
    let rem = x - (&floor_sqrt).square();
    (floor_sqrt, rem)
}

//TODO use better algorithms

impl FloorSqrtAssign for Natural {
    /// Replaces a `Natural` with the floor of its square root.
    ///
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorSqrtAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 9);
    /// 
    /// let mut x = Natural::from(100u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    /// 
    /// let mut x = Natural::from(101u8);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 10);
    /// 
    /// let mut x = Natural::from(1000000000u32);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 31622);
    /// 
    /// let mut x = Natural::from(10000000000u64);
    /// x.floor_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn floor_sqrt_assign(&mut self) {
        *self = _floor_sqrt_binary(&*self);
    }
}

impl FloorSqrt for Natural {
    type Output = Natural;

    /// Returns the floor of the square root of a `Natural`, taking the `Natural` by value.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).floor_sqrt(), 9);
    /// assert_eq!(Natural::from(100u8).floor_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).floor_sqrt(), 10);
    /// assert_eq!(Natural::from(1000000000u32).floor_sqrt(), 31622);
    /// assert_eq!(Natural::from(10000000000u64).floor_sqrt(), 100000);
    /// ```
    #[inline]
    fn floor_sqrt(self) -> Natural {
        _floor_sqrt_binary(&self)
    }
}

impl<'a> FloorSqrt for &'a Natural {
    type Output = Natural;

    /// Returns the floor of the square root of a `Natural`, taking the `Natural` by reference.
    ///
    /// $f(x) = \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::FloorSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(99u8)).floor_sqrt(), 9);
    /// assert_eq!((&Natural::from(100u8)).floor_sqrt(), 10);
    /// assert_eq!((&Natural::from(101u8)).floor_sqrt(), 10);
    /// assert_eq!((&Natural::from(1000000000u32)).floor_sqrt(), 31622);
    /// assert_eq!((&Natural::from(10000000000u64)).floor_sqrt(), 100000);
    /// ```
    #[inline]
    fn floor_sqrt(self) -> Natural {
        _floor_sqrt_binary(self)
    }
}

impl CeilingSqrtAssign for Natural {
    /// Replaces a `Natural` with the ceiling of its square root.
    ///
    /// $x \gets \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingSqrtAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    /// 
    /// let mut x = Natural::from(100u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 10);
    /// 
    /// let mut x = Natural::from(101u8);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 11);
    /// 
    /// let mut x = Natural::from(1000000000u32);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 31623);
    /// 
    /// let mut x = Natural::from(10000000000u64);
    /// x.ceiling_sqrt_assign();
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt_assign(&mut self) {
        *self = _ceiling_sqrt_binary(&*self);
    }
}

impl CeilingSqrt for Natural {
    type Output = Natural;

    /// Returns the ceiling of the square root of a `Natural`, taking the `Natural` by value.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(100u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).ceiling_sqrt(), 11);
    /// assert_eq!(Natural::from(1000000000u32).ceiling_sqrt(), 31623);
    /// assert_eq!(Natural::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt(self) -> Natural {
        _ceiling_sqrt_binary(&self)
    }
}

impl<'a> CeilingSqrt for &'a Natural {
    type Output = Natural;

    /// Returns the ceiling of the square root of a `Natural`, taking the `Natural` by reference.
    ///
    /// $f(x) = \lceil\sqrt{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CeilingSqrt;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(100u8).ceiling_sqrt(), 10);
    /// assert_eq!(Natural::from(101u8).ceiling_sqrt(), 11);
    /// assert_eq!(Natural::from(1000000000u32).ceiling_sqrt(), 31623);
    /// assert_eq!(Natural::from(10000000000u64).ceiling_sqrt(), 100000);
    /// ```
    #[inline]
    fn ceiling_sqrt(self) -> Natural {
        _ceiling_sqrt_binary(self)
    }
}

impl CheckedSqrt for Natural {
    type Output = Natural;

    /// Returns the the square root of a `Natural`, or `None` if the `Natural` is not a perfect
    /// square. The `Natural` is taken by value.
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
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Natural::from(100u8).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!(Natural::from(101u8).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Natural::from(1000000000u32).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(Natural::from(10000000000u64).checked_sqrt().to_debug_string(), "Some(100000)");
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Natural> {
        _checked_sqrt_binary(&self)
    }
}

impl<'a> CheckedSqrt for &'a Natural {
    type Output = Natural;

    /// Returns the the square root of a `Natural`, or `None` if the `Natural` is not a perfect
    /// square. The `Natural` is taken by reference.
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
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedSqrt;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(99u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Natural::from(100u8)).checked_sqrt().to_debug_string(), "Some(10)");
    /// assert_eq!((&Natural::from(101u8)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!((&Natural::from(1000000000u32)).checked_sqrt().to_debug_string(), "None");
    /// assert_eq!(
    ///     (&Natural::from(10000000000u64)).checked_sqrt().to_debug_string(),
    ///     "Some(100000)"
    /// );
    /// ```
    #[inline]
    fn checked_sqrt(self) -> Option<Natural> {
        _checked_sqrt_binary(self)
    }
}

impl SqrtRemAssign for Natural {
    type RemOutput = Natural;

    /// Replaces a `Natural` with the floor of its square root, and returns the remainder (the
    /// difference between the original `Natural` and the square of the floor).
    ///
    /// $f(x) = x - \lfloor\sqrt{x}\rfloor^2$,
    /// 
    /// $x \gets \lfloor\sqrt{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRemAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(99u8);
    /// assert_eq!(x.sqrt_rem_assign(), 18);
    /// assert_eq!(x, 9);
    /// 
    /// let mut x = Natural::from(100u8);
    /// assert_eq!(x.sqrt_rem_assign(), 0);
    /// assert_eq!(x, 10);
    /// 
    /// let mut x = Natural::from(101u8);
    /// assert_eq!(x.sqrt_rem_assign(), 1);
    /// assert_eq!(x, 10);
    /// 
    /// let mut x = Natural::from(1000000000u32);
    /// assert_eq!(x.sqrt_rem_assign(), 49116);
    /// assert_eq!(x, 31622);
    /// 
    /// let mut x = Natural::from(10000000000u64);
    /// assert_eq!(x.sqrt_rem_assign(), 0);
    /// assert_eq!(x, 100000);
    /// ```
    #[inline]
    fn sqrt_rem_assign(&mut self) -> Natural {
        let (sqrt, rem) = _sqrt_rem_binary(&*self);
        *self = sqrt;
        rem
    }
}

impl SqrtRem for Natural {
    type SqrtOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a `Natural`, and the remainder (the difference
    /// between the `Natural` and the square of the floor). The `Natural` is taken by value.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(99u8).sqrt_rem().to_debug_string(), "(9, 18)");
    /// assert_eq!(Natural::from(100u8).sqrt_rem().to_debug_string(), "(10, 0)");
    /// assert_eq!(Natural::from(101u8).sqrt_rem().to_debug_string(), "(10, 1)");
    /// assert_eq!(Natural::from(1000000000u32).sqrt_rem().to_debug_string(), "(31622, 49116)");
    /// assert_eq!(Natural::from(10000000000u64).sqrt_rem().to_debug_string(), "(100000, 0)");
    /// ```
    #[inline]
    fn sqrt_rem(self) -> (Natural, Natural) {
        _sqrt_rem_binary(&self)
    }
}

impl<'a> SqrtRem for &'a Natural {
    type SqrtOutput = Natural;
    type RemOutput = Natural;

    /// Returns the floor of the square root of a `Natural`, and the remainder (the difference
    /// between the `Natural` and the square of the floor). The `Natural` is taken by reference.
    ///
    /// $f(x) = (\lfloor\sqrt{x}\rfloor, x - \lfloor\sqrt{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SqrtRem;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(99u8)).sqrt_rem().to_debug_string(), "(9, 18)");
    /// assert_eq!((&Natural::from(100u8)).sqrt_rem().to_debug_string(), "(10, 0)");
    /// assert_eq!((&Natural::from(101u8)).sqrt_rem().to_debug_string(), "(10, 1)");
    /// assert_eq!((&Natural::from(1000000000u32)).sqrt_rem().to_debug_string(), "(31622, 49116)");
    /// assert_eq!((&Natural::from(10000000000u64)).sqrt_rem().to_debug_string(), "(100000, 0)");
    /// ```
    #[inline]
    fn sqrt_rem(self) -> (Natural, Natural) {
        _sqrt_rem_binary(self)
    }
}
