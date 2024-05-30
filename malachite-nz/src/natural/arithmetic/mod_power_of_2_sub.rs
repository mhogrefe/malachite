// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use crate::natural::arithmetic::mod_power_of_2::{
    limbs_neg_mod_power_of_2, limbs_neg_mod_power_of_2_in_place,
    limbs_slice_mod_power_of_2_in_place,
};
use crate::natural::arithmetic::mod_power_of_2_add::limbs_vec_mod_power_of_2_add_limb_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place,
    limbs_sub_same_length_in_place_right, limbs_vec_sub_in_place_right,
};
use crate::natural::logic::low_mask::limbs_low_mask;
use crate::natural::logic::not::limbs_not_in_place;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Neg, ModPowerOf2NegAssign, ModPowerOf2Sub, ModPowerOf2SubAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
fn extend_with_ones(xs: &mut Vec<Limb>, pow: u64) {
    xs.resize(
        usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0),
        Limb::MAX,
    );
}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
// `Natural` from a `Limb`, mod `2 ^ pow`. Assumes the input is already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// # Panics
// Panics if `pow` is zero.
pub_test! {limbs_mod_power_of_2_limb_sub_limbs(x: Limb, ys: &[Limb], pow: u64) -> Vec<Limb> {
    let mut diff = limbs_neg_mod_power_of_2(ys, pow);
    limbs_vec_mod_power_of_2_add_limb_in_place(&mut diff, x, pow);
    diff
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
// `Natural` from a `Limb`, mod `2 ^ pow`, and writes the limbs of the difference to the input
// slice. Assumes the input is already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// # Panics
// Panics if `pow` is zero.
pub_test! {limbs_mod_power_of_2_limb_sub_limbs_in_place(x: Limb, ys: &mut Vec<Limb>, pow: u64) {
    limbs_neg_mod_power_of_2_in_place(ys, pow);
    limbs_vec_mod_power_of_2_add_limb_in_place(ys, x, pow);
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, subtracts
// the second `Natural` from the first, mod `2 ^ pow`, and returns a `Vec` of the limbs of the
// difference. Assumes the inputs are already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
pub_test! {limbs_mod_power_of_2_sub(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    let ys_len = ys.len();
    let mut out_limbs = xs.to_vec();
    if ys_len > xs.len() {
        out_limbs.resize(ys_len, 0);
    }
    if limbs_sub_greater_in_place_left(&mut out_limbs, ys) {
        extend_with_ones(&mut out_limbs, pow);
        limbs_slice_mod_power_of_2_in_place(&mut out_limbs, pow);
    }
    out_limbs
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, subtracts
// the second `Natural` from the first, mod `2 ^ pow`, and writes the limbs of the difference to the
// first (left) slice. Assumes the inputs are already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
pub_test! {limbs_mod_power_of_2_sub_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], pow: u64) {
    let ys_len = ys.len();
    if ys_len > xs.len() {
        xs.resize(ys_len, 0);
    }
    if limbs_sub_greater_in_place_left(xs, ys) {
        extend_with_ones(xs, pow);
        limbs_slice_mod_power_of_2_in_place(xs, pow);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, subtracts
// the second `Natural` from the first, mod `2 ^ pow`, and writes the limbs of the difference to the
// second (right) slice. Assumes the inputs are already reduced mod `2 ^ pow`.
//
// Neither input slice may have trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
pub_test! {limbs_mod_power_of_2_sub_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, pow: u64) {
    let xs_len = xs.len();
    if xs_len >= ys.len() {
        if limbs_vec_sub_in_place_right(xs, ys) {
            extend_with_ones(ys, pow);
            limbs_slice_mod_power_of_2_in_place(ys, pow);
        }
    } else {
        let (ys_lo, ys_hi) = ys.split_at_mut(xs_len);
        if limbs_sub_same_length_in_place_right(xs, ys_lo) {
            limbs_not_in_place(ys_hi);
        } else {
            limbs_twos_complement_in_place(ys_hi);
        }
        extend_with_ones(ys, pow);
        limbs_slice_mod_power_of_2_in_place(ys, pow);
    }
}}

// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, subtracts
// the second `Natural` from the first, mod `2 ^ pow`, and writes the limbs of the difference to to
// the longer slice (or the first one, if they are equally long). Returns a `bool` which is `false`
// when the output is to the first `Vec` and `true` when it's to the second `Vec`. Assumes the
// inputs are already reduced mod `2 ^ pow`.
//
// Neither input slice may have trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
pub_test! {limbs_mod_power_of_2_sub_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    pow: u64,
) -> bool {
    if xs.len() >= ys.len() {
        limbs_mod_power_of_2_sub_in_place_left(xs, ys, pow);
        false
    } else {
        limbs_mod_power_of_2_sub_in_place_right(xs, ys, pow);
        true
    }
}}

impl Natural {
    fn mod_power_of_2_sub_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (self, y, pow) {
            (x, 0, _) => x.clone(),
            (&Natural::ZERO, _, _) => Natural(Small(y)).mod_power_of_2_neg(pow),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(small.mod_power_of_2_sub(other, pow)))
            }
            (&Natural(Small(small)), other, _) => {
                let (diff, overflow) = small.overflowing_sub(other);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    Natural(Large(out))
                } else {
                    Natural(Small(diff))
                }
            }
            (&Natural(Large(ref limbs)), other, _) => {
                Natural::from_owned_limbs_asc(limbs_sub_limb(limbs, other).0)
            }
        }
    }

    // other - self
    fn mod_power_of_2_right_sub_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (self, y, pow) {
            (_, 0, _) => self.mod_power_of_2_neg(pow),
            (&Natural::ZERO, _, _) => Natural(Small(y)),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(other.mod_power_of_2_sub(small, pow)))
            }
            (&Natural(Small(small)), other, _) => {
                let (diff, overflow) = other.overflowing_sub(small);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    Natural(Large(out))
                } else {
                    Natural(Small(diff))
                }
            }
            (&Natural(Large(ref limbs)), other, _) => Natural::from_owned_limbs_asc(
                limbs_mod_power_of_2_limb_sub_limbs(other, limbs, pow),
            ),
        }
    }

    fn mod_power_of_2_sub_assign_limb(&mut self, y: Limb, pow: u64) {
        match (&mut *self, y, pow) {
            (_, 0, _) => {}
            (&mut Natural::ZERO, _, _) => *self = Natural(Small(y)).mod_power_of_2_neg(pow),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                small.mod_power_of_2_sub_assign(other, pow);
            }
            (&mut Natural(Small(ref mut small)), other, _) => {
                let (diff, overflow) = small.overflowing_sub(other);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    *self = Natural(Large(out));
                } else {
                    *small = diff;
                }
            }
            (&mut Natural(Large(ref mut limbs)), other, _) => {
                limbs_sub_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }

    // other -= self
    fn mod_power_of_2_right_sub_assign_limb(&mut self, other: Limb, pow: u64) {
        match (&mut *self, other, pow) {
            (_, 0, _) => self.mod_power_of_2_neg_assign(pow),
            (&mut Natural::ZERO, _, _) => *self = Natural(Small(other)),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                *small = other.mod_power_of_2_sub(*small, pow);
            }
            (&mut Natural(Small(ref mut small)), other, _) => {
                let (diff, overflow) = other.overflowing_sub(*small);
                if overflow {
                    let mut out = limbs_low_mask(pow);
                    out[0] = diff;
                    *self = Natural(Large(out));
                } else {
                    *small = diff;
                }
            }
            (&mut Natural(Large(ref mut limbs)), other, _) => {
                limbs_mod_power_of_2_limb_sub_limbs_in_place(other, limbs, pow);
                self.trim();
            }
        }
    }
}

impl ModPowerOf2Sub<Natural> for Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// Both [`Natural`]s are taken by value.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x - y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).mod_power_of_2_sub(Natural::TWO, 4), 8);
    /// assert_eq!(
    ///     Natural::from(56u32).mod_power_of_2_sub(Natural::from(123u32), 9),
    ///     445
    /// );
    /// ```
    fn mod_power_of_2_sub(mut self, other: Natural, pow: u64) -> Natural {
        self.mod_power_of_2_sub_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOf2Sub<&'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// The first [`Natural`] is taken by value and the second by reference.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x - y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).mod_power_of_2_sub(&Natural::TWO, 4), 8);
    /// assert_eq!(
    ///     Natural::from(56u32).mod_power_of_2_sub(&Natural::from(123u32), 9),
    ///     445
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub(mut self, other: &'a Natural, pow: u64) -> Natural {
        self.mod_power_of_2_sub_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOf2Sub<Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// The first [`Natural`] is taken by reference and the second by value.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x - y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_2_sub(Natural::TWO, 4),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(56u32)).mod_power_of_2_sub(Natural::from(123u32), 9),
    ///     445
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub(self, mut other: Natural, pow: u64) -> Natural {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (self, &mut other) {
            (x, Natural(Small(y))) => x.mod_power_of_2_sub_limb_ref(*y, pow),
            (&Natural(Small(x)), y) => {
                y.mod_power_of_2_right_sub_assign_limb(x, pow);
                other
            }
            (&Natural(Large(ref xs)), &mut Natural(Large(ref mut ys))) => {
                limbs_mod_power_of_2_sub_in_place_right(xs, ys, pow);
                other.trim();
                other
            }
        }
    }
}

impl<'a, 'b> ModPowerOf2Sub<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Subtracts two [`Natural`] modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// Both [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x - y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_2_sub(&Natural::TWO, 4),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(56u32)).mod_power_of_2_sub(&Natural::from(123u32), 9),
    ///     445
    /// );
    /// ```
    fn mod_power_of_2_sub(self, other: &'a Natural, pow: u64) -> Natural {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (self, other) {
            (x, y) if core::ptr::eq(x, y) => Natural::ZERO,
            (x, &Natural(Small(y))) => x.mod_power_of_2_sub_limb_ref(y, pow),
            (&Natural(Small(x)), y) => y.mod_power_of_2_right_sub_limb_ref(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_sub(xs, ys, pow))
            }
        }
    }
}

impl ModPowerOf2SubAssign<Natural> for Natural {
    /// Subtracts two [`Natural`] modulo $2^k$, in place. The inputs must be already reduced modulo
    /// $2^k$. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// $x \gets z$, where $x, y, z < 2^k$ and $x - y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_sub_assign(Natural::TWO, 4);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(56u32);
    /// x.mod_power_of_2_sub_assign(Natural::from(123u32), 9);
    /// assert_eq!(x, 445);
    /// ```
    fn mod_power_of_2_sub_assign(&mut self, mut other: Natural, pow: u64) {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (&mut *self, &mut other) {
            (x, &mut Natural(Small(y))) => x.mod_power_of_2_sub_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => {
                y.mod_power_of_2_right_sub_assign_limb(x, pow);
                *self = other;
            }
            (&mut Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_mod_power_of_2_sub_in_place_either(xs, ys, pow) {
                    swap(xs, ys);
                }
                self.trim();
            }
        }
    }
}

impl<'a> ModPowerOf2SubAssign<&'a Natural> for Natural {
    /// Subtracts two [`Natural`] modulo $2^k$, in place. The inputs must be already reduced modulo
    /// $2^k$. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// $x \gets z$, where $x, y, z < 2^k$ and $x - y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_sub_assign(&Natural::TWO, 4);
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(56u32);
    /// x.mod_power_of_2_sub_assign(&Natural::from(123u32), 9);
    /// assert_eq!(x, 445);
    /// ```
    fn mod_power_of_2_sub_assign(&mut self, other: &'a Natural, pow: u64) {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (&mut *self, other) {
            (x, y) if core::ptr::eq(x, y) => *self = Natural::ZERO,
            (x, &Natural(Small(y))) => x.mod_power_of_2_sub_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => *self = y.mod_power_of_2_right_sub_limb_ref(x, pow),
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                limbs_mod_power_of_2_sub_in_place_left(xs, ys, pow);
                self.trim();
            }
        }
    }
}
