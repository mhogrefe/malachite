// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993-1997, 1999-2016, 2020 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::{self, *};
use core::mem::swap;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::slices::{slice_leading_zeros, slice_test_zero};

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, compares the two `Natural`s.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_cmp` from `gmp.h`, GMP 6.2.1.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
pub_crate_test! {limbs_cmp_same_length(xs: &[Limb], ys: &[Limb]) -> Ordering {
    assert_eq!(xs.len(), ys.len());
    xs.iter().rev().cmp(ys.iter().rev())
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, compares
// the two `Natural`s. Neither limb slice can contain trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// # Panics
// Panics if the last element of `xs` or `ys` is zero.
pub_crate_test! {limbs_cmp(xs: &[Limb], ys: &[Limb]) -> Ordering {
    assert_ne!(xs.last(), Some(&0));
    assert_ne!(ys.last(), Some(&0));
    xs.len()
        .cmp(&ys.len())
        .then_with(|| limbs_cmp_same_length(xs, ys))
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
// their normalized comparison. See `Natural::cmp_normalized` for details.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// # Panics
// Panics if either `xs` or `ys` is empty, or if the last element of `xs` or `ys` is zero.
pub_test! {limbs_cmp_normalized(xs: &[Limb], ys: &[Limb]) -> Ordering {
    let mut xs = &xs[slice_leading_zeros(xs)..];
    let mut ys = &ys[slice_leading_zeros(ys)..];
    let mut xs_leading = LeadingZeros::leading_zeros(*xs.last().unwrap());
    assert_ne!(xs_leading, Limb::WIDTH);
    let mut ys_leading = LeadingZeros::leading_zeros(*ys.last().unwrap());
    assert_ne!(ys_leading, Limb::WIDTH);
    let mut xs_len = xs.len();
    let mut ys_len = ys.len();
    let mut swapped = false;
    match xs_leading.cmp(&ys_leading) {
        Equal => {
            return match xs_len.cmp(&ys_len) {
                Equal => limbs_cmp_same_length(xs, ys),
                Less => {
                    let leading_cmp = limbs_cmp_same_length(xs, &ys[ys_len - xs_len..]);
                    if leading_cmp == Greater {
                        Greater
                    } else {
                        Less
                    }
                }
                Greater => {
                    let leading_cmp = limbs_cmp_same_length(&xs[xs_len - ys_len..], ys);
                    if leading_cmp == Less {
                        Less
                    } else {
                        Greater
                    }
                }
            };
        }
        Less => {
            swap(&mut xs, &mut ys);
            swap(&mut xs_leading, &mut ys_leading);
            swap(&mut xs_len, &mut ys_len);
            swapped = true;
        }
        _ => {}
    }
    let xs_shift = xs_leading - ys_leading;
    let comp_xs_shift = Limb::WIDTH - xs_shift;
    let mut xs_i = xs_len - 1;
    let mut ys_i = ys_len - 1;
    loop {
        let y = ys[ys_i];
        let xs_hi = xs[xs_i];
        let xs_lo = if xs_i == 0 { 0 } else { xs[xs_i - 1] };
        let x = (xs_hi << xs_shift) | (xs_lo >> comp_xs_shift);
        let cmp = x.cmp(&y);
        if cmp != Equal {
            return if swapped { cmp.reverse() } else { cmp };
        }
        if xs_i == 0 {
            return if ys_i == 0 {
                Equal
            } else if swapped {
                Greater
            } else {
                Less
            };
        } else if ys_i == 0 {
            return if xs_lo << xs_shift == 0 && slice_test_zero(&xs[..xs_i - 1]) {
                Equal
            } else if swapped {
                Less
            } else {
                Greater
            };
        }
        xs_i -= 1;
        ys_i -= 1;
    }
}}

impl PartialOrd for Natural {
    /// Compares two [`Natural`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Natural {
    /// Compares two [`Natural`]s.
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
    /// assert!(Natural::from(123u32) > Natural::from(122u32));
    /// assert!(Natural::from(123u32) >= Natural::from(122u32));
    /// assert!(Natural::from(123u32) < Natural::from(124u32));
    /// assert!(Natural::from(123u32) <= Natural::from(124u32));
    /// ```
    fn cmp(&self, other: &Natural) -> Ordering {
        if core::ptr::eq(self, other) {
            return Equal;
        }
        match (self, other) {
            (&Natural(Small(ref x)), &Natural(Small(ref y))) => x.cmp(y),
            (&Natural(Small(_)), &Natural(Large(_))) => Less,
            (&Natural(Large(_)), &Natural(Small(_))) => Greater,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => limbs_cmp(xs, ys),
        }
    }
}

impl Natural {
    /// Returns a result of a comparison between two [`Natural`]s as if each had been multiplied by
    /// some power of 2 to bring it into the interval $[1, 2)$.
    ///
    /// That is, the comparison is equivalent to a comparison between $f(x)$ and $f(y)$, where
    /// $$
    /// f(n) = n2^{\lfloor\log_2 n \rfloor}.
    /// $$
    ///
    /// The multiplication is not actually performed.
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
    /// Panics if either argument is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 == 1.0 * 2^0, 4 == 1.0 * 2^2
    /// // 1.0 == 1.0
    /// assert_eq!(
    ///     Natural::from(1u32).cmp_normalized(&Natural::from(4u32)),
    ///     Equal
    /// );
    ///
    /// // 5 == 1.25 * 2^2, 6 == 1.5 * 2^2
    /// // 1.25 < 1.5
    /// assert_eq!(
    ///     Natural::from(5u32).cmp_normalized(&Natural::from(6u32)),
    ///     Less
    /// );
    ///
    /// // 3 == 1.5 * 2^1, 17 == 1.0625 * 2^4
    /// // 1.5 > 1.0625
    /// assert_eq!(
    ///     Natural::from(3u32).cmp_normalized(&Natural::from(17u32)),
    ///     Greater
    /// );
    ///
    /// // 9 == 1.125 * 2^3, 36 == 1.125 * 2^5
    /// // 1.125 == 1.125
    /// assert_eq!(
    ///     Natural::from(9u32).cmp_normalized(&Natural::from(36u32)),
    ///     Equal
    /// );
    /// ```
    pub fn cmp_normalized(&self, other: &Natural) -> Ordering {
        assert_ne!(*self, 0);
        assert_ne!(*other, 0);
        if core::ptr::eq(self, other) {
            return Equal;
        }
        match (self, other) {
            (&Natural(Small(x)), &Natural(Small(y))) => {
                let leading_x = x.leading_zeros();
                let leading_y = y.leading_zeros();
                match leading_x.cmp(&leading_y) {
                    Equal => x.cmp(&y),
                    Less => x.cmp(&(y << (leading_y - leading_x))),
                    Greater => (x << (leading_x - leading_y)).cmp(&y),
                }
            }
            (&Natural(Small(x)), &Natural(Large(ref ys))) => limbs_cmp_normalized(&[x], ys),
            (&Natural(Large(ref xs)), &Natural(Small(y))) => limbs_cmp_normalized(xs, &[y]),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => limbs_cmp_normalized(xs, ys),
        }
    }

    #[cfg(feature = "float_helpers")]
    pub fn cmp_normalized_no_shift(&self, other: &Natural) -> Ordering {
        assert_ne!(*self, 0);
        assert_ne!(*other, 0);
        if core::ptr::eq(self, other) {
            return Equal;
        }
        match (self, other) {
            (&Natural(Small(x)), &Natural(Small(y))) => x.cmp(&y),
            (Natural(Small(x)), &Natural(Large(ref ys))) => {
                let (ys_last, ys_init) = ys.split_last().unwrap();
                let c = x.cmp(ys_last);
                if c != Equal {
                    c
                } else if slice_test_zero(ys_init) {
                    Equal
                } else {
                    Less
                }
            }
            (&Natural(Large(ref xs)), Natural(Small(y))) => {
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let c = xs_last.cmp(y);
                if c != Equal {
                    c
                } else if slice_test_zero(xs_init) {
                    Equal
                } else {
                    Greater
                }
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                let xs_len = xs.len();
                let ys_len = ys.len();
                match xs_len.cmp(&ys_len) {
                    Equal => xs.iter().rev().cmp(ys.iter().rev()),
                    Less => {
                        let (ys_lo, ys_hi) = ys.split_at(ys_len - xs_len);
                        let c = xs.iter().rev().cmp(ys_hi.iter().rev());
                        if c != Equal {
                            c
                        } else if slice_test_zero(ys_lo) {
                            Equal
                        } else {
                            Less
                        }
                    }
                    Greater => {
                        let (xs_lo, xs_hi) = xs.split_at(xs_len - ys_len);
                        let c = xs_hi.iter().rev().cmp(ys.iter().rev());
                        if c != Equal {
                            c
                        } else if slice_test_zero(xs_lo) {
                            Equal
                        } else {
                            Greater
                        }
                    }
                }
            }
        }
    }
}
