// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
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
use core::cmp::Ordering::*;
use core::mem::swap;
use core::ops::{BitAnd, BitAndAssign};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::slices::slice_set_zero;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// bitwise and of the `Natural` and a `Limb`. The slice cannot be empty.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `xs` is empty.
pub_const_test! {limbs_and_limb(xs: &[Limb], y: Limb) -> Limb {
    xs[0] & y
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns a
// `Vec` of the limbs of the bitwise and of the `Natural`s. The length of the result is the length
// of the shorter input slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res` is returned and both
// inputs are non-negative.
pub_test! {limbs_and(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    xs.iter().zip(ys.iter()).map(|(x, y)| x & y).collect()
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to a specified slice. The
// output slice must be at least as long as the length of one of the input slices.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` and `ys` have different lengths or if `out` is too short.
//
// This is equivalent to `mpn_and_n` from `gmp-impl.h`, GMP 6.2.1.
pub_test! {limbs_and_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        *out = x & y;
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the limbs of the bitwise and of the `Natural`s to a specified slice. The output slice must be at
// least as long as the longer input slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `out` is too short.
//
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where both inputs are non-negative.
pub_test! {limbs_and_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        assert!(out.len() >= xs_len);
        limbs_and_same_length_to_out(out, &xs[..ys_len], ys);
        slice_set_zero(&mut out[ys_len..xs_len]);
    } else {
        assert!(out.len() >= ys_len);
        limbs_and_same_length_to_out(out, xs, &ys[..xs_len]);
        slice_set_zero(&mut out[xs_len..ys_len]);
    }
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to the first (left) slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
//
// This is equivalent to `mpn_and_n` from `gmp-impl.h`, GMP 6.2.1, where `rp == up`.
pub_test! {limbs_slice_and_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    assert_eq!(xs.len(), ys.len());
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        *x &= y;
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the limbs of the bitwise and of the `Natural`s to the first (left) slice. If the second slice is
// shorter than the first, then some of the most-significant bits of the first slice should become
// zero. Rather than setting them to zero, this function optionally returns the length of the
// significant part of the slice. The caller can decide whether to zero the rest. If `None` is
// returned, the entire slice remains significant.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res == op1` and both inputs
// are non-negative.
pub_test! {limbs_slice_and_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> Option<usize> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys.len()) {
        Equal => {
            limbs_slice_and_same_length_in_place_left(xs, ys);
            None
        }
        Greater => {
            limbs_slice_and_same_length_in_place_left(&mut xs[..ys_len], ys);
            Some(ys_len)
        }
        Less => {
            limbs_slice_and_same_length_in_place_left(xs, &ys[..xs_len]);
            None
        }
    }
}}

// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the bitwise and of the `Natural`s to the `Vec`. If the slice is
// shorter than the `Vec`, then some of the most-significant bits of the `Vec` should become zero.
// Rather than setting them to zero, this function truncates the `Vec`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res == op1` and both inputs
// are non-negative and have the same length, and `res` is truncated afterwards to remove the
// `max(0, xs.len() - ys.len())` trailing zero limbs.
pub_test! {limbs_vec_and_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    if let Some(truncate_size) = limbs_slice_and_in_place_left(xs, ys) {
        xs.truncate(truncate_size);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, takes the
// limbs of the bitwise and of the `Natural`s and writes them to the shorter slice (or the first
// one, if they are equally long). If the function writes to the first slice, it returns `false`;
// otherwise, it returns `true`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where both inputs are non-negative
// and the result is written to the shorter input slice.
pub_test! {limbs_and_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Equal => {
            limbs_slice_and_same_length_in_place_left(xs, ys);
            false
        }
        Less => {
            limbs_slice_and_same_length_in_place_left(xs, &ys[..xs_len]);
            false
        }
        Greater => {
            limbs_slice_and_same_length_in_place_left(ys, &xs[..ys_len]);
            true
        }
    }
}}

impl Natural {
    fn and_limb(self, other: Limb) -> Limb {
        Limb::wrapping_from(&self) & other
    }

    fn and_limb_ref(&self, other: Limb) -> Limb {
        Limb::wrapping_from(self) & other
    }

    fn and_assign_limb(&mut self, other: Limb) {
        *self = Natural(Small(self.and_limb_ref(other)));
    }
}

impl BitAnd<Natural> for Natural {
    type Output = Natural;

    /// Takes the bitwise and of two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x \wedge y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32) & Natural::from(456u32), 72);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12) & (Natural::from(10u32).pow(12) - Natural::ONE),
    ///     999999995904u64
    /// );
    /// ```
    #[inline]
    fn bitand(mut self, other: Natural) -> Natural {
        self &= other;
        self
    }
}

impl<'a> BitAnd<&'a Natural> for Natural {
    type Output = Natural;

    /// Takes the bitwise and of two [`Natural`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = x \wedge y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32) & &Natural::from(456u32), 72);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12) & &(Natural::from(10u32).pow(12) - Natural::ONE),
    ///     999999995904u64
    /// );
    /// ```
    #[inline]
    fn bitand(mut self, other: &'a Natural) -> Natural {
        self &= other;
        self
    }
}

impl<'a> BitAnd<Natural> for &'a Natural {
    type Output = Natural;

    /// Takes the bitwise and of two [`Natural`]s, taking the first by reference and the seocnd by
    /// value.
    ///
    /// $$
    /// f(x, y) = x \wedge y.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::from(123u32) & Natural::from(456u32), 72);
    /// assert_eq!(
    ///     &Natural::from(10u32).pow(12) & (Natural::from(10u32).pow(12) - Natural::ONE),
    ///     999999995904u64
    /// );
    /// ```
    #[inline]
    fn bitand(self, mut other: Natural) -> Natural {
        other &= self;
        other
    }
}

impl<'a, 'b> BitAnd<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Takes the bitwise and of two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x \wedge y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::from(123u32) & &Natural::from(456u32), 72);
    /// assert_eq!(
    ///     &Natural::from(10u32).pow(12) & &(Natural::from(10u32).pow(12) - Natural::ONE),
    ///     999999995904u64
    /// );
    /// ```
    fn bitand(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Natural(Small(y))) => Natural(Small(x.and_limb_ref(y))),
            (&Natural(Small(x)), y) => Natural(Small(y.and_limb_ref(x))),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_and(xs, ys))
            }
        }
    }
}

impl BitAndAssign<Natural> for Natural {
    /// Bitwise-ands a [`Natural`] with another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets x \wedge y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(u32::MAX);
    /// x &= Natural::from(0xf0ffffffu32);
    /// x &= Natural::from(0xfff0_ffffu32);
    /// x &= Natural::from(0xfffff0ffu32);
    /// x &= Natural::from(0xfffffff0u32);
    /// assert_eq!(x, 0xf0f0_f0f0u32);
    /// ```
    fn bitand_assign(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.and_assign_limb(*y),
            (Natural(Small(ref mut x)), _) => *x = other.and_limb(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_and_in_place_either(xs, ys) {
                    swap(xs, ys);
                }
                self.trim();
            }
        }
    }
}

impl<'a> BitAndAssign<&'a Natural> for Natural {
    /// Bitwise-ands a [`Natural`] with another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets x \wedge y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(u32::MAX);
    /// x &= &Natural::from(0xf0ffffffu32);
    /// x &= &Natural::from(0xfff0_ffffu32);
    /// x &= &Natural::from(0xfffff0ffu32);
    /// x &= &Natural::from(0xfffffff0u32);
    /// assert_eq!(x, 0xf0f0_f0f0u32);
    /// ```
    fn bitand_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.and_assign_limb(*y),
            (Natural(Small(ref mut x)), _) => *x = other.and_limb_ref(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_and_in_place_left(xs, ys);
                self.trim();
            }
        }
    }
}
