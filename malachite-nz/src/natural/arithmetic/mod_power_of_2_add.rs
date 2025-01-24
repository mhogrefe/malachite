// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_limb, limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left, limbs_vec_add_in_place_left,
};
use crate::natural::logic::bit_access::limbs_clear_bit;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2Shl, ModPowerOf2ShlAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the sum of the `Natural` and a `Limb`, mod `2 ^ pow`. Assumes the input is already
// reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_mod_power_of_2_add_limb(xs: &[Limb], y: Limb, pow: u64) -> Vec<Limb> {
    if xs.len() < usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0) {
        limbs_add_limb(xs, y)
    } else {
        let mut out = xs.to_vec();
        if !limbs_slice_add_limb_in_place(&mut out, y) {
            limbs_clear_bit(&mut out, pow);
        }
        out
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the sum of the `Natural` and a `Limb`, mod `2 ^ pow`, to the input slice. Returns
// whether there is a carry. Assumes the input is already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_slice_mod_power_of_2_add_limb_in_place(
    xs: &mut [Limb],
    y: Limb,
    pow: u64
) -> bool {
    if xs.len() < usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0) {
        limbs_slice_add_limb_in_place(xs, y)
    } else {
        if !limbs_slice_add_limb_in_place(xs, y) {
            limbs_clear_bit(xs, pow);
        }
        false
    }
}}

// Interpreting a nonempty `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes
// the limbs of the sum of the `Natural` and a `Limb`, mod `2 ^ pow`, to the input `Vec`. Assumes
// the input is already reduced mod `2 ^ pow`.
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
pub_crate_test! {limbs_vec_mod_power_of_2_add_limb_in_place(xs: &mut Vec<Limb>, y: Limb, pow: u64) {
    assert!(!xs.is_empty());
    if limbs_slice_mod_power_of_2_add_limb_in_place(xs, y, pow) {
        xs.push(1);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where the
// first slice is at least as long as the second, returns a `Vec` of the limbs of the sum of the
// `Natural`s mod `2 ^ pow`. Assumes the inputs are already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys`.
pub_test! {limbs_mod_power_of_2_add_greater(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    let mut out = xs.to_vec();
    if limbs_slice_mod_power_of_2_add_greater_in_place_left(&mut out, ys, pow) {
        out.push(1);
    }
    out
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns a
// `Vec` of the limbs of the sum of the `Natural`s mod `2 ^ pow`. Assumes the inputs are already
// reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
pub_test! {limbs_mod_power_of_2_add(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_mod_power_of_2_add_greater(xs, ys, pow)
    } else {
        limbs_mod_power_of_2_add_greater(ys, xs, pow)
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where the
// length of the first slice is greater than or equal to the length of the second, writes the
// `xs.len()` least-significant limbs of the sum of the `Natural`s, mod `2 ^ pow`, to the first
// (left) slice. Returns whether there is a carry. Assumes the inputs are already reduced mod `2 ^
// pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys`.
pub_test! {limbs_slice_mod_power_of_2_add_greater_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    pow: u64,
) -> bool {
    if xs.len() < usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0) {
        limbs_slice_add_greater_in_place_left(xs, ys)
    } else {
        if !limbs_slice_add_greater_in_place_left(xs, ys) {
            limbs_clear_bit(xs, pow);
        }
        false
    }
}}

// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the sum of the `Natural`s, mod `2 ^ pow`, to the first (left)
// slice. Assumes the inputs are already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is `max(1,
// ys.len() - xs.len())`.
pub_test! {limbs_vec_mod_power_of_2_add_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], pow: u64) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    if xs_len < max_len && ys_len < max_len {
        limbs_vec_add_in_place_left(xs, ys);
    } else {
        let carry = if xs_len >= ys_len {
            limbs_slice_mod_power_of_2_add_greater_in_place_left(xs, ys, pow)
        } else {
            let (ys_lo, ys_hi) = ys.split_at(xs_len);
            let mut carry = limbs_slice_add_same_length_in_place_left(xs, ys_lo);
            xs.extend_from_slice(ys_hi);
            if carry {
                carry = limbs_slice_add_limb_in_place(&mut xs[xs_len..], 1);
            }
            carry
        };
        if !carry {
            limbs_clear_bit(xs, pow);
        }
    }
}}

// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the limbs of the sum of the `Natural`s, mod `2 ^ pow`, to the longer slice (or the first one, if
// they are equally long). Returns a `bool` which is `false` when the output is to the first `Vec`
// and `true` when it's to the second `Vec`. Assumes the inputs are already reduced mod `2 ^ pow`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
pub_test! {limbs_mod_power_of_2_add_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    pow: u64,
) -> bool {
    if xs.len() >= ys.len() {
        if limbs_slice_mod_power_of_2_add_greater_in_place_left(xs, ys, pow) {
            xs.push(1);
        }
        false
    } else {
        if limbs_slice_mod_power_of_2_add_greater_in_place_left(ys, xs, pow) {
            ys.push(1);
        }
        true
    }
}}

impl Natural {
    fn mod_power_of_2_add_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (self, y, pow) {
            (_, 0, _) => self.clone(),
            (&Natural::ZERO, _, _) => Natural(Small(y)),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(small.mod_power_of_2_add(other, pow)))
            }
            (&Natural(Small(small)), other, _) => {
                let (sum, overflow) = small.overflowing_add(other);
                if overflow {
                    Natural(Large(vec![sum, 1]))
                } else {
                    Natural(Small(sum))
                }
            }
            (&Natural(Large(ref limbs)), other, pow) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_add_limb(limbs, other, pow))
            }
        }
    }

    fn mod_power_of_2_add_assign_limb(&mut self, y: Limb, pow: u64) {
        match (&mut *self, y, pow) {
            (_, 0, _) => {}
            (&mut Natural::ZERO, _, _) => *self = Natural(Small(y)),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                small.mod_power_of_2_add_assign(other, pow);
            }
            (&mut Natural(Small(ref mut small)), other, _) => {
                let (sum, overflow) = small.overflowing_add(other);
                if overflow {
                    *self = Natural(Large(vec![sum, 1]));
                } else {
                    *small = sum;
                }
            }
            (&mut Natural(Large(ref mut limbs)), y, pow) => {
                limbs_vec_mod_power_of_2_add_limb_in_place(limbs, y, pow);
                self.trim();
            }
        }
    }
}

impl ModPowerOf2Add<Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$. Both
    /// [`Natural`]s are taken by value.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_2_add(Natural::from(2u32), 5), 2);
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_add(Natural::from(14u32), 4),
    ///     8
    /// );
    /// ```
    fn mod_power_of_2_add(mut self, other: Natural, pow: u64) -> Natural {
        self.mod_power_of_2_add_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOf2Add<&'a Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$. The
    /// first [`Natural`] is taken by value and the second by reference.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_2_add(&Natural::from(2u32), 5), 2);
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_add(&Natural::from(14u32), 4),
    ///     8
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add(mut self, other: &'a Natural, pow: u64) -> Natural {
        self.mod_power_of_2_add_assign(other, pow);
        self
    }
}

impl ModPowerOf2Add<Natural> for &Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$. The
    /// first [`Natural`] is taken by reference and the second by value.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_power_of_2_add(Natural::from(2u32), 5),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_2_add(Natural::from(14u32), 4),
    ///     8
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add(self, mut other: Natural, pow: u64) -> Natural {
        other.mod_power_of_2_add_assign(self, pow);
        other
    }
}

impl ModPowerOf2Add<&Natural> for &Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$. Both
    /// [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_power_of_2_add(&Natural::from(2u32), 5),
    ///     2
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_2_add(&Natural::from(14u32), 4),
    ///     8
    /// );
    /// ```
    fn mod_power_of_2_add(self, other: &Natural, pow: u64) -> Natural {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (self, other) {
            (x, y) if core::ptr::eq(x, y) => self.mod_power_of_2_shl(1, pow),
            (x, &Natural(Small(y))) => x.mod_power_of_2_add_limb_ref(y, pow),
            (&Natural(Small(x)), y) => y.mod_power_of_2_add_limb_ref(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_add(xs, ys, pow))
            }
        }
    }
}

impl ModPowerOf2AddAssign<Natural> for Natural {
    /// Adds two [`Natural`]s modulo $2^k$, in place. The inputs must be already reduced modulo
    /// $2^k$. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// $x \gets z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_power_of_2_add_assign(Natural::from(2u32), 5);
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_add_assign(Natural::from(14u32), 4);
    /// assert_eq!(x, 8);
    /// ```
    fn mod_power_of_2_add_assign(&mut self, mut other: Natural, pow: u64) {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (&mut *self, &mut other) {
            (x, &mut Natural(Small(y))) => x.mod_power_of_2_add_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => *self = y.mod_power_of_2_add_limb_ref(x, pow),
            (&mut Natural(Large(ref mut xs)), _) => {
                if let Natural(Large(mut ys)) = other {
                    if limbs_mod_power_of_2_add_in_place_either(xs, &mut ys, pow) {
                        *xs = ys;
                    }
                    self.trim();
                }
            }
        }
    }
}

impl<'a> ModPowerOf2AddAssign<&'a Natural> for Natural {
    /// Adds two [`Natural`]s modulo $2^k$, in place. The inputs must be already reduced modulo
    /// $2^k$. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// $x \gets z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_power_of_2_add_assign(&Natural::from(2u32), 5);
    /// assert_eq!(x, 2);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_add_assign(&Natural::from(14u32), 4);
    /// assert_eq!(x, 8);
    /// ```
    fn mod_power_of_2_add_assign(&mut self, other: &'a Natural, pow: u64) {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (&mut *self, other) {
            (x, y) if core::ptr::eq(x, y) => {
                self.mod_power_of_2_shl_assign(pow, 1);
            }
            (x, &Natural(Small(y))) => x.mod_power_of_2_add_assign_limb(y, pow),
            (&mut Natural(Small(x)), y) => *self = y.mod_power_of_2_add_limb_ref(x, pow),
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                limbs_vec_mod_power_of_2_add_in_place_left(xs, ys, pow);
                self.trim();
            }
        }
    }
}
