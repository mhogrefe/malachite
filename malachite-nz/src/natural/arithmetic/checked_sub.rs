// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::sub::{
    limbs_sub, limbs_sub_greater_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place,
    limbs_vec_sub_in_place_right,
};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::traits::Zero;

impl Natural {
    pub(crate) fn checked_sub_limb(mut self, other: Limb) -> Option<Natural> {
        if self.sub_assign_limb_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }

    pub(crate) fn checked_sub_limb_ref(&self, other: Limb) -> Option<Natural> {
        match (self, other) {
            (_, 0) => Some(self.clone()),
            (Natural(Small(small)), other) => small.checked_sub(other).map(Natural::from),
            (Natural(Large(ref limbs)), other) => {
                if *self < other {
                    None
                } else {
                    Some(Natural::from_owned_limbs_asc(
                        limbs_sub_limb(limbs, other).0,
                    ))
                }
            }
        }
    }

    // self -= other, return borrow
    fn sub_assign_limb_no_panic(&mut self, other: Limb) -> bool {
        match (&mut *self, other) {
            (_, 0) => false,
            (Natural(Small(ref mut x)), y) => match x.checked_sub(y) {
                Some(diff) => {
                    *x = diff;
                    false
                }
                None => true,
            },
            (Natural(Large(ref mut xs)), y) => {
                let borrow = limbs_sub_limb_in_place(xs, y);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    // self -= other, return borrow
    pub(crate) fn sub_assign_no_panic(&mut self, other: Natural) -> bool {
        match (&mut *self, other) {
            (_, Natural::ZERO) => false,
            (x, Natural(Small(y))) => x.sub_assign_limb_no_panic(y),
            (Natural(Small(_)), _) => true,
            (&mut Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                let borrow = xs.len() < ys.len() || limbs_sub_greater_in_place_left(xs, ys);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    // self -= &other, return borrow
    pub(crate) fn sub_assign_ref_no_panic(&mut self, other: &Natural) -> bool {
        match (&mut *self, other) {
            (_, &Natural::ZERO) => false,
            (x, y) if core::ptr::eq(x, y) => {
                *self = Natural::ZERO;
                false
            }
            (x, &Natural(Small(y))) => x.sub_assign_limb_no_panic(y),
            (Natural(Small(_)), _) => true,
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                let borrow = xs.len() < ys.len() || limbs_sub_greater_in_place_left(xs, ys);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    // self = &other - self, return borrow
    pub(crate) fn sub_right_assign_no_panic(&mut self, other: &Natural) -> bool {
        match (&mut *self, other) {
            (&mut Natural::ZERO, y) => {
                *self = y.clone();
                false
            }
            (x, y) if core::ptr::eq(x, y) => {
                *self = Natural::ZERO;
                false
            }
            (Natural(Small(x)), y) => y.checked_sub_limb_ref(*x).map_or(true, |result| {
                *self = result;
                false
            }),
            (_, Natural(Small(_))) => true,
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                let borrow = xs.len() > ys.len() || limbs_vec_sub_in_place_right(ys, xs);
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }
}

impl CheckedSub<Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking both by value and returning `None` if
    /// the result is negative.
    ///
    /// $$
    /// f(x, y) = \\begin{cases}
    ///     \operatorname{Some}(x - y) & \text{if} \\quad x \geq y, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedSub, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO
    ///         .checked_sub(Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(123u32)
    ///         .checked_sub(Natural::ZERO)
    ///         .to_debug_string(),
    ///     "Some(123)"
    /// );
    /// assert_eq!(
    ///     Natural::from(456u32)
    ///         .checked_sub(Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "Some(333)"
    /// );
    /// assert_eq!(
    ///     (Natural::from(10u32).pow(12) * Natural::from(3u32))
    ///         .checked_sub(Natural::from(10u32).pow(12))
    ///         .to_debug_string(),
    ///     "Some(2000000000000)"
    /// );
    /// ```
    fn checked_sub(mut self, other: Natural) -> Option<Natural> {
        if self.sub_assign_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSub<&'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning `None` if the result is negative.
    ///
    /// $$
    /// f(x, y) = \\begin{cases}
    ///     \operatorname{Some}(x - y) & \text{if} \\quad x \geq y, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedSub, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO
    ///         .checked_sub(&Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(123u32)
    ///         .checked_sub(&Natural::ZERO)
    ///         .to_debug_string(),
    ///     "Some(123)"
    /// );
    /// assert_eq!(
    ///     Natural::from(456u32)
    ///         .checked_sub(&Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "Some(333)"
    /// );
    /// assert_eq!(
    ///     (Natural::from(10u32).pow(12) * Natural::from(3u32))
    ///         .checked_sub(&Natural::from(10u32).pow(12))
    ///         .to_debug_string(),
    ///     "Some(2000000000000)"
    /// );
    /// ```
    fn checked_sub(mut self, other: &'a Natural) -> Option<Natural> {
        if self.sub_assign_ref_no_panic(other) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSub<Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning `None` if the result is negative.
    ///
    /// $$
    /// f(x, y) = \\begin{cases}
    ///     \operatorname{Some}(x - y) & \text{if} \\quad x \geq y, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
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
    /// use malachite_base::num::arithmetic::traits::{CheckedSub, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO)
    ///         .checked_sub(Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(123u32))
    ///         .checked_sub(Natural::ZERO)
    ///         .to_debug_string(),
    ///     "Some(123)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(456u32))
    ///         .checked_sub(Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "Some(333)"
    /// );
    /// assert_eq!(
    ///     (&(Natural::from(10u32).pow(12) * Natural::from(3u32)))
    ///         .checked_sub(Natural::from(10u32).pow(12))
    ///         .to_debug_string(),
    ///     "Some(2000000000000)"
    /// );
    /// ```
    fn checked_sub(self, mut other: Natural) -> Option<Natural> {
        if other.sub_right_assign_no_panic(self) {
            None
        } else {
            Some(other)
        }
    }
}

impl<'a, 'b> CheckedSub<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking both by reference and returning
    /// `None` if the result is negative.
    ///
    /// $$
    /// f(x, y) = \\begin{cases}
    ///     \operatorname{Some}(x - y) & \text{if} \\quad x \geq y, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
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
    /// use malachite_base::num::arithmetic::traits::{CheckedSub, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO)
    ///         .checked_sub(&Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(123u32))
    ///         .checked_sub(&Natural::ZERO)
    ///         .to_debug_string(),
    ///     "Some(123)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(456u32))
    ///         .checked_sub(&Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "Some(333)"
    /// );
    /// assert_eq!(
    ///     (&(Natural::from(10u32).pow(12) * Natural::from(3u32)))
    ///         .checked_sub(&Natural::from(10u32).pow(12))
    ///         .to_debug_string(),
    ///     "Some(2000000000000)"
    /// );
    /// ```
    fn checked_sub(self, other: &'a Natural) -> Option<Natural> {
        match (self, other) {
            (x, y) if core::ptr::eq(x, y) => Some(Natural::ZERO),
            (x, &Natural::ZERO) => Some(x.clone()),
            (x, &Natural(Small(y))) => x.checked_sub_limb_ref(y),
            (&Natural(Small(_)), _) => None,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                if self < other {
                    None
                } else {
                    Some(Natural::from_owned_limbs_asc(limbs_sub(xs, ys).0))
                }
            }
        }
    }
}
