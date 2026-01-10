// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::natural::arithmetic::sub_mul::{
    limbs_sub_mul, limbs_sub_mul_in_place_left, limbs_sub_mul_limb_greater,
    limbs_sub_mul_limb_greater_in_place_left,
};
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};
use malachite_base::num::basic::traits::Zero;

macro_rules! large_left {
    ($a_limbs: ident, $b_limbs: ident, $c_limbs: ident) => {
        (
            Natural(Large($a_limbs)),
            Natural(Large($b_limbs)),
            Natural(Large($c_limbs)),
        )
    };
}

macro_rules! large_right {
    ($self: ident, $a_limbs: ident, $b_limbs: ident, $c_limbs: ident) => {{
        let borrow = $a_limbs.len() < $b_limbs.len() + $c_limbs.len() - 1
            || limbs_sub_mul_in_place_left($a_limbs, &$b_limbs, &$c_limbs);
        if !borrow {
            $self.trim();
        }
        borrow
    }};
}

impl Natural {
    fn checked_sub_mul_limb_ref_ref(&self, b: &Self, c: Limb) -> Option<Self> {
        match (self, b, c) {
            (a, _, 0) | (a, &Self::ZERO, _) => Some(a.clone()),
            (a, b @ Self(Small(_)), c) => a.checked_sub(b * Self::from(c)),
            (Self(Small(_)), _, _) => None,
            (Self(Large(a_limbs)), Self(Large(b_limbs)), c) => {
                if a_limbs.len() >= b_limbs.len() {
                    limbs_sub_mul_limb_greater(a_limbs, b_limbs, c).map(Self::from_owned_limbs_asc)
                } else {
                    None
                }
            }
        }
    }

    fn sub_mul_assign_limb_no_panic(&mut self, b: Self, c: Limb) -> bool {
        match (&mut *self, b, c) {
            (_, _, 0) | (_, Self::ZERO, _) => false,
            (a, b @ Self(Small(_)), c) => a.sub_assign_no_panic(b * Self::from(c)),
            (Self(Small(_)), _, _) => true,
            (Self(Large(a_limbs)), Self(Large(b_limbs)), c) => {
                let borrow = a_limbs.len() < b_limbs.len()
                    || limbs_sub_mul_limb_greater_in_place_left(a_limbs, &b_limbs, c) != 0;
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    fn sub_mul_assign_limb_ref_no_panic(&mut self, b: &Self, c: Limb) -> bool {
        match (&mut *self, b, c) {
            (_, _, 0) | (_, &Self::ZERO, _) => false,
            (a, b @ Self(Small(_)), c) => a.sub_assign_no_panic(b * Self::from(c)),
            (Self(Small(_)), _, _) => true,
            (Self(Large(a_limbs)), Self(Large(b_limbs)), c) => {
                let borrow = a_limbs.len() < b_limbs.len()
                    || limbs_sub_mul_limb_greater_in_place_left(a_limbs, b_limbs, c) != 0;
                if !borrow {
                    self.trim();
                }
                borrow
            }
        }
    }

    pub(crate) fn sub_mul_assign_no_panic(&mut self, b: Self, c: Self) -> bool {
        match (&mut *self, b, c) {
            (a, Self(Small(small_b)), c) => a.sub_mul_assign_limb_no_panic(c, small_b),
            (a, b, Self(Small(small_c))) => a.sub_mul_assign_limb_no_panic(b, small_c),
            (Self(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }

    pub(crate) fn sub_mul_assign_val_ref_no_panic(&mut self, b: Self, c: &Self) -> bool {
        match (&mut *self, &b, c) {
            (a, Self(Small(small_b)), c) => a.sub_mul_assign_limb_ref_no_panic(c, *small_b),
            (a, _, Self(Small(small_c))) => a.sub_mul_assign_limb_no_panic(b, *small_c),
            (Self(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }

    pub(crate) fn sub_mul_assign_ref_val_no_panic(&mut self, b: &Self, c: Self) -> bool {
        match (&mut *self, b, &c) {
            (a, Self(Small(small_b)), _) => a.sub_mul_assign_limb_no_panic(c, *small_b),
            (a, b, Self(Small(small_c))) => a.sub_mul_assign_limb_ref_no_panic(b, *small_c),
            (Self(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }

    pub(crate) fn sub_mul_assign_ref_ref_no_panic(&mut self, b: &Self, c: &Self) -> bool {
        match (&mut *self, b, c) {
            (a, Self(Small(small_b)), c) => a.sub_mul_assign_limb_ref_no_panic(c, *small_b),
            (a, b, Self(Small(small_c))) => a.sub_mul_assign_limb_ref_no_panic(b, *small_c),
            (Self(Small(_)), _, _) => true,
            large_left!(a_limbs, b_limbs, c_limbs) => large_right!(self, a_limbs, b_limbs, c_limbs),
        }
    }
}

impl CheckedSubMul<Self, Self> for Natural {
    type Output = Self;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking all three by value
    /// and returning `None` if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \\begin{cases}
    ///     \operatorname{Some}(x - yz) & \text{if} \\quad x \geq yz, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedSubMul, Pow};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .checked_sub_mul(Natural::from(3u32), Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "Some(8)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .checked_sub_mul(Natural::from(3u32), Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .checked_sub_mul(Natural::from(0x10000u32), Natural::from(0x10000u32))
    ///         .to_debug_string(),
    ///     "Some(995705032704)"
    /// );
    /// ```
    fn checked_sub_mul(mut self, y: Self, z: Self) -> Option<Self> {
        if self.sub_mul_assign_no_panic(y, z) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSubMul<Self, &'a Self> for Natural {
    type Output = Self;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first two by
    /// value and the third by reference and returning `None` if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \\begin{cases}
    ///     \operatorname{Some}(x - yz) & \text{if} \\quad x \geq yz, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedSubMul, Pow};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .checked_sub_mul(Natural::from(3u32), &Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "Some(8)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .checked_sub_mul(Natural::from(3u32), &Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .checked_sub_mul(Natural::from(0x10000u32), &Natural::from(0x10000u32))
    ///         .to_debug_string(),
    ///     "Some(995705032704)"
    /// );
    /// ```
    fn checked_sub_mul(mut self, y: Self, z: &'a Self) -> Option<Self> {
        if self.sub_mul_assign_val_ref_no_panic(y, z) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> CheckedSubMul<&'a Self, Self> for Natural {
    type Output = Self;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first and third
    /// by value and the second by reference and returning `None` if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \\begin{cases}
    ///     \operatorname{Some}(x - yz) & \text{if} \\quad x \geq yz, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedSubMul, Pow};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .checked_sub_mul(&Natural::from(3u32), Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "Some(8)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .checked_sub_mul(&Natural::from(3u32), Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .checked_sub_mul(&Natural::from(0x10000u32), Natural::from(0x10000u32))
    ///         .to_debug_string(),
    ///     "Some(995705032704)"
    /// );
    /// ```
    fn checked_sub_mul(mut self, y: &'a Self, z: Self) -> Option<Self> {
        if self.sub_mul_assign_ref_val_no_panic(y, z) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, 'b> CheckedSubMul<&'a Self, &'b Self> for Natural {
    type Output = Self;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first by value
    /// and the second and third by reference and returning `None` if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \\begin{cases}
    ///     \operatorname{Some}(x - yz) & \text{if} \\quad x \geq yz, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedSubMul, Pow};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32)
    ///         .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "Some(8)"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .checked_sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32))
    ///         .to_debug_string(),
    ///     "Some(995705032704)"
    /// );
    /// ```
    fn checked_sub_mul(mut self, y: &'a Self, z: &'b Self) -> Option<Self> {
        if self.sub_mul_assign_ref_ref_no_panic(y, z) {
            None
        } else {
            Some(self)
        }
    }
}

impl CheckedSubMul<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking all three by
    /// reference and returning `None` if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \\begin{cases}
    ///     \operatorname{Some}(x - yz) & \text{if} \\quad x \geq yz, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedSubMul, Pow};
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(20u32))
    ///         .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "Some(8)"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32))
    ///         .checked_sub_mul(&Natural::from(3u32), &Natural::from(4u32))
    ///         .to_debug_string(),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12))
    ///         .checked_sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32))
    ///         .to_debug_string(),
    ///     "Some(995705032704)"
    /// );
    /// ```
    fn checked_sub_mul(self, y: &Natural, z: &Natural) -> Option<Natural> {
        match (self, y, z) {
            (x, Natural(Small(small_y)), z) => x.checked_sub_mul_limb_ref_ref(z, *small_y),
            (x, y, Natural(Small(small_z))) => x.checked_sub_mul_limb_ref_ref(y, *small_z),
            (Natural(Small(_)), _, _) => None,
            (Natural(Large(x_limbs)), Natural(Large(y_limbs)), Natural(Large(z_limbs))) => {
                if x_limbs.len() >= y_limbs.len() + z_limbs.len() - 1 {
                    limbs_sub_mul(x_limbs, y_limbs, z_limbs).map(Natural::from_owned_limbs_asc)
                } else {
                    None
                }
            }
        }
    }
}
