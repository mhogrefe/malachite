// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright 1991, 1993, 1994, 1996, 2001, 2003, 2012, 2015 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::ops::Not;
use malachite_base::num::logic::traits::NotAssign;

// Returns the bitwise not of a slice of limbs.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_com` from `mpn/generic/com.c`, GMP 6.2.1, where `rp` is returned.
pub_test! {limbs_not(xs: &[Limb]) -> Vec<Limb> {
    xs.iter().map(|x| !x).collect()
}}

// Writes the bitwise not of a slice of limbs to the lowest `x.len()` limbs of `out`. For this to
// work, `out` must be at least as long as `xs`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_com` from `mpn/generic/com.c`, GMP 6.2.1, where `rp != up`.
//
// # Panics
// Panics if `out` is shorter than `xs`.
pub_crate_test! {limbs_not_to_out(out: &mut [Limb], xs: &[Limb]) {
    assert!(out.len() >= xs.len());
    for (x, y) in out.iter_mut().zip(xs.iter()) {
        *x = !y;
    }
}}

// Takes the bitwise not of a slice of limbs in place.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_com` from `mpn/generic/com.c`, GMP 6.2.1, where `rp == up`.
pub_crate_test! {limbs_not_in_place(xs: &mut [Limb]) {
    for x in &mut *xs {
        x.not_assign();
    }
}}

impl Not for Natural {
    type Output = Integer;

    /// Returns the bitwise negation of a [`Natural`], taking it by value and returning an
    /// [`Integer`].
    ///
    /// The [`Natural`] is bitwise-negated as if it were represented in two's complement.
    ///
    /// $$
    /// f(n) = -n - 1.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(!Natural::ZERO, -1);
    /// assert_eq!(!Natural::from(123u32), -124);
    /// ```
    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self.add_limb(1),
        }
    }
}

impl<'a> Not for &'a Natural {
    type Output = Integer;

    /// Returns the bitwise negation of a [`Natural`], taking it by reference and returning an
    /// [`Integer`].
    ///
    /// The [`Natural`] is bitwise-negated as if it were represented in two's complement.
    ///
    /// $$
    /// f(n) = -n - 1.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(!&Natural::ZERO, -1);
    /// assert_eq!(!&Natural::from(123u32), -124);
    /// ```
    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self.add_limb_ref(1),
        }
    }
}
