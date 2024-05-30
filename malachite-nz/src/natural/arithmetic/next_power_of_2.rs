// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, NextPowerOf2, NextPowerOf2Assign,
};
use malachite_base::slices::{slice_set_zero, slice_test_zero};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the smallest integer power of 2 greater than or equal to the `Natural`.
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty.
pub_test! {limbs_next_power_of_2(xs: &[Limb]) -> Vec<Limb> {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    let mut out;
    if let Some(x) = xs_last.checked_next_power_of_two() {
        out = vec![0; xs_init.len()];
        if x == *xs_last && !slice_test_zero(xs_init) {
            if let Some(x) = x.arithmetic_checked_shl(1) {
                out.push(x);
            } else {
                out.push(0);
                out.push(1);
            }
        } else {
            out.push(x);
        }
    } else {
        out = vec![0; xs.len()];
        out.push(1);
    }
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the smallest integer power of 2 greater than or equal to the `Natural` to the input
// slice. If the input slice is too small to hold the result, the limbs are all set to zero and the
// carry bit, `true`, is returned. Otherwise, `false` is returned.
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty.
pub_test! {limbs_slice_next_power_of_2_in_place(xs: &mut [Limb]) -> bool {
    let (xs_last, xs_init) = xs.split_last_mut().unwrap();
    if let Some(x) = xs_last.checked_next_power_of_two() {
        if x == *xs_last && !slice_test_zero(xs_init) {
            slice_set_zero(xs_init);
            if let Some(x) = x.arithmetic_checked_shl(1) {
                *xs_last = x;
                false
            } else {
                *xs_last = 0;
                true
            }
        } else {
            slice_set_zero(xs_init);
            *xs_last = x;
            false
        }
    } else {
        slice_set_zero(xs_init);
        *xs_last = 0;
        true
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the smallest integer power of 2 greater than or equal to the `Natural` to the input
// `Vec`.
//
// This function assumes that `xs` is nonempty and the last (most significant) limb is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty.
pub_test! {limbs_vec_next_power_of_2_in_place(xs: &mut Vec<Limb>) {
    if limbs_slice_next_power_of_2_in_place(xs) {
        xs.push(1);
    }
}}

impl NextPowerOf2 for Natural {
    type Output = Natural;

    /// Finds the smallest power of 2 greater than or equal to a [`Natural`]. The [`Natural`] is
    /// taken by value.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{NextPowerOf2, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.next_power_of_2(), 1);
    /// assert_eq!(Natural::from(123u32).next_power_of_2(), 128);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).next_power_of_2(),
    ///     1099511627776u64
    /// );
    /// ```
    #[inline]
    fn next_power_of_2(mut self) -> Natural {
        self.next_power_of_2_assign();
        self
    }
}

impl<'a> NextPowerOf2 for &'a Natural {
    type Output = Natural;

    /// Finds the smallest power of 2 greater than or equal to a [`Natural`]. The [`Natural`] is
    /// taken by reference.
    ///
    /// $f(x) = 2^{\lceil \log_2 x \rceil}$.
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
    /// use malachite_base::num::arithmetic::traits::{NextPowerOf2, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).next_power_of_2(), 1);
    /// assert_eq!((&Natural::from(123u32)).next_power_of_2(), 128);
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).next_power_of_2(),
    ///     1099511627776u64
    /// );
    /// ```
    fn next_power_of_2(self) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => {
                if let Some(result) = small.checked_next_power_of_two() {
                    Small(result)
                } else {
                    Large(vec![0, 1])
                }
            }
            Natural(Large(ref limbs)) => Large(limbs_next_power_of_2(limbs)),
        })
    }
}

impl NextPowerOf2Assign for Natural {
    /// Replaces a [`Natural`] with the smallest power of 2 greater than or equal to it.
    ///
    /// $x \gets 2^{\lceil \log_2 x \rceil}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{NextPowerOf2Assign, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.next_power_of_2_assign();
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.next_power_of_2_assign();
    /// assert_eq!(x, 128);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.next_power_of_2_assign();
    /// assert_eq!(x, 1099511627776u64);
    /// ```
    fn next_power_of_2_assign(&mut self) {
        match *self {
            Natural(Small(ref mut small)) => {
                if let Some(pow) = small.checked_next_power_of_two() {
                    *small = pow;
                } else {
                    *self = Natural(Large(vec![0, 1]));
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_next_power_of_2_in_place(limbs);
            }
        }
    }
}
