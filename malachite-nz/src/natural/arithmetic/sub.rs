// Copyright © 2024 Mikhail Hogrefe
//
// Some optimizations contributed by florian1345.
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018, 2020 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::fmt::Display;
use core::ops::{Sub, SubAssign};
use malachite_base::num::arithmetic::traits::{CheckedSub, OverflowingSubAssign};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
// `Limb` from the `Natural`. Returns a pair consisting of the limbs of the result, and whether
// there was a borrow left over; that is, whether the `Limb` was greater than the `Natural`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_sub_1` from `gmp.h`, GMP 6.2.1, where the result is returned.
pub_crate_test! {limbs_sub_limb(xs: &[Limb], mut y: Limb) -> (Vec<Limb>, bool) {
    let len = xs.len();
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let (diff, overflow) = xs[i].overflowing_sub(y);
        out.push(diff);
        if overflow {
            y = 1;
        } else {
            y = 0;
            out.extend_from_slice(&xs[i + 1..]);
            break;
        }
    }
    (out, y != 0)
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
// `Limb` from the `Natural`, writing the `xs.len()` limbs of the result to an output slice. Returns
// whether there was a borrow left over; that is, whether the `Limb` was greater than the `Natural`.
// The output slice must be at least as long as the input slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is shorter than `xs`.
//
// This is equivalent to `mpn_sub_1` from `gmp.h`, GMP 6.2.1.
pub_crate_test! {limbs_sub_limb_to_out(out: &mut [Limb], xs: &[Limb], mut y: Limb) -> bool {
    let len = xs.len();
    assert!(out.len() >= len);
    for i in 0..len {
        let overflow;
        (out[i], overflow) = xs[i].overflowing_sub(y);
        if overflow {
            y = 1;
        } else {
            y = 0;
            let copy_index = i + 1;
            out[copy_index..len].copy_from_slice(&xs[copy_index..]);
            break;
        }
    }
    y != 0
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
// `Limb` from the `Natural` and writes the limbs of the result to the input slice. Returns whether
// there was a borrow left over; that is, whether the `Limb` was greater than the `Natural`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_add_1` from `gmp.h`, GMP 6.2.1, where the result is written to the
// input slice.
pub_crate_test! {limbs_sub_limb_in_place(xs: &mut [Limb], mut y: Limb) -> bool {
    for x in &mut *xs {
        if x.overflowing_sub_assign(y) {
            y = 1;
        } else {
            return false;
        }
    }
    y != 0
}}

#[inline]
pub(crate) fn sub_with_carry(x: Limb, y: Limb, carry: Limb) -> (Limb, Limb) {
    let result_no_carry = x.wrapping_sub(y);
    let result = result_no_carry.wrapping_sub(carry);
    let carry = Limb::from((result_no_carry > x) || (result > result_no_carry));
    (result, carry)
}

// Interpreting a two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
// subtracts the second from the first. Returns a pair consisting of the limbs of the result, and
// whether there was a borrow left over; that is, whether the second `Natural` was greater than the
// first `Natural`. The first slice must be at least as long as the second.
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
//
// This is equivalent to `mpn_sub` from `gmp.h`, GMP 6.2.1, where the output is returned.
pub_crate_test! {limbs_sub(xs: &[Limb], ys: &[Limb]) -> (Vec<Limb>, bool) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let mut out = Vec::with_capacity(xs_len);
    let mut carry = 0;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        let o;
        (o, carry) = sub_with_carry(x, y, carry);
        out.push(o);
    }
    let mut borrow = carry != 0;
    if xs_len != ys_len {
        out.extend_from_slice(&xs[ys_len..]);
        if borrow {
            borrow = limbs_sub_limb_in_place(&mut out[ys_len..], 1);
        }
    }
    (out, borrow)
}}

// Interpreting a two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to an
// output slice. Returns whether there was a borrow left over; that is, whether the second `Natural`
// was greater than the first `Natural`. The output slice must be at least as long as either input
// slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is shorter than `xs` or if `xs` and `ys` have different lengths.
//
// This is equivalent to `mpn_sub_n` from `gmp.h`, GMP 6.2.1.
pub_crate_test! {limbs_sub_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    let mut carry = 0;

    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        (*out, carry) = sub_with_carry(x, y, carry);
    }

    carry > 0
}}

// Interpreting a two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
// subtracts the second from the first, writing the `xs.len()` limbs of the result to an output
// slice. Returns whether there was a borrow left over; that is, whether the second `Natural` was
// greater than the first `Natural`. The output slice must be at least as long as the first input
// slice and the first input slice must be at least as long as the second.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is shorter than `xs` or if `xs` is shorter than `ys`.
//
// This is equivalent to `mpn_sub` from `gmp.h`, GMP 6.2.1.
pub_crate_test! {limbs_sub_greater_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    let (xs_lo, xs_hi) = xs.split_at(ys_len);
    let borrow = limbs_sub_same_length_to_out(out, xs_lo, ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_to_out(&mut out[ys_len..], xs_hi, 1)
    } else {
        out[ys_len..xs_len].copy_from_slice(xs_hi);
        false
    }
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
// the first (left) slice. Returns whether there was a borrow left over; that is, whether the second
// `Natural` was greater than the first `Natural`.
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
// This is equivalent to `mpn_sub_n` from `gmp.h`, GMP 6.2.1, where the output is written to the
// first input.
pub_crate_test! {limbs_sub_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    assert_eq!(xs.len(), ys.len());
    let mut carry = 0;

    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        (*x, carry) = sub_with_carry(*x, y, carry);
    }

    carry > 0
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, subtracts
// the second from the first, writing the `xs.len()` limbs of the result to the first (left) slice.
// Returns whether there was a borrow left over; that is, whether the second `Natural` was greater
// than the first `Natural`. The first slice must be at least as long as the second.
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
//
// This is equivalent to `mpn_sub` from `gmp.h`, GMP 6.2.1, where the output is written to the first
// input.
pub_crate_test! {limbs_sub_greater_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
    let borrow = limbs_sub_same_length_in_place_left(xs_lo, ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_in_place(xs_hi, 1)
    } else {
        false
    }
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
// the second (right) slice. Returns whether there was a borrow left over; that is, whether the
// second `Natural` was greater than the first `Natural`.
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
// This is equivalent to `mpn_sub_n` from `gmp.h`, GMP 6.2.1, where the output is written to the
// second input.
pub_crate_test! {limbs_sub_same_length_in_place_right(xs: &[Limb], ys: &mut [Limb]) -> bool {
    assert_eq!(xs.len(), ys.len());
    let mut carry = 0;

    for (&x, y) in xs.iter().zip(ys.iter_mut()) {
        (*y, carry) = sub_with_carry(x, *y, carry);
    }

    carry > 0
}}

// Given two equal-length slices `xs` and `ys`, computes the difference between the `Natural`s whose
// limbs are `xs` and `&ys[..len]`, and writes the limbs of the result to `ys`. Returns whether
// there was a borrow left over; that is, whether the second `Natural` was greater than the first
// `Natural`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` and `ys` have different lengths or if `len` is greater than `xs.len()`.
//
// This is equivalent to `mpn_sub_n` from `gmp.h`, GMP 6.2.1, where the output is written to the
// second input (which has `len` limbs) and the second input has enough space past `len` to
// accomodate the output.
pub_crate_test! {limbs_slice_sub_in_place_right(xs: &[Limb], ys: &mut [Limb], len: usize) -> bool {
    let xs_len = xs.len();
    assert_eq!(xs_len, ys.len());
    let (xs_lo, xs_hi) = xs.split_at(len);
    let (ys_lo, ys_hi) = ys.split_at_mut(len);
    let borrow = limbs_sub_same_length_in_place_right(xs_lo, ys_lo);
    if xs_len == len {
        borrow
    } else if borrow {
        limbs_sub_limb_to_out(ys_hi, xs_hi, 1)
    } else {
        ys_hi.copy_from_slice(xs_hi);
        false
    }
}}

// Interpreting a of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
// the `Vec`, possibly extending the `Vec`'s length. Returns whether there was a borrow left over;
// that is, whether the second `Natural` was greater than the first `Natural`. The first slice must
// be at least as long as the second.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `xs.len()` -
// `ys.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys`.
pub_crate_test! {limbs_vec_sub_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let (xs_lo, xs_hi) = xs.split_at(ys_len);
    let borrow = limbs_sub_same_length_in_place_right(xs_lo, ys);
    if xs_len == ys_len {
        borrow
    } else {
        ys.extend_from_slice(xs_hi);
        if borrow {
            limbs_sub_limb_in_place(&mut ys[ys_len..], 1)
        } else {
            false
        }
    }
}}

// Given a slice `xs`, computes the difference between the `Natural`s whose limbs are
// `&xs[..xs.len() - right_start]` and `&xs[right_start..]`, and writes the limbs of the result to
// `&xs[..xs.len() - right_start]`. Returns whether there was a borrow left over; that is, whether
// the second `Natural` was greater than the first `Natural`. As implied by the name, the input
// slices may overlap.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len() - right_start`.
//
// # Panics
// Panics if `right_start` is greater than `xs.len()`.
//
// This is equivalent to `mpn_sub_n` from `gmp.h`, GMP 6.2.1, where the output is written to the
// first input, and the two inputs are possibly-overlapping subslices of a single slice.
pub_crate_test! {limbs_sub_same_length_in_place_with_overlap(
    xs: &mut [Limb],
    right_start: usize
) -> bool {
    let len = xs.len() - right_start;
    let mut carry = 0;
    for i in 0..len {
        (xs[i], carry) = sub_with_carry(xs[i], xs[i + right_start], carry);
    }
    carry != 0
}}

// Given two slices `xs` and `ys`, computes the difference between the `Natural`s whose limbs are
// `&xs[xs.len() - ys.len()..]` and `&ys`, and writes the limbs of the result to `&xs[..ys.len()]`.
// Returns whether there was a borrow left over; that is, whether the second `Natural` was greater
// than the first `Natural`. As implied by the name, the input and output ranges may overlap. `xs`
// must be at least as long as `ys`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ys.len()`.
//
// # Panics
// Panics if `xs.len()` is shorter than `ys.len()`.
//
// This is equivalent to `mpn_sub_n` from `gmp.h`, GMP 6.2.1, where the output is a prefix of a
// slice and the left operand of the subtraction is a suffix of the same slice, and the prefix and
// suffix may overlap.
pub_crate_test! {limbs_sub_same_length_to_out_with_overlap(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let right_start = xs_len - ys_len;
    let mut carry = 0;
    for i in 0..ys_len {
        (xs[i], carry) = sub_with_carry(xs[i + right_start], ys[i], carry);
    }
    carry != 0
}}

// Interpreting a two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, subtracts the second from the first, and then subtracts a borrow (`false` is 0,
// `true` is 1), writing the `xs.len()` limbs of the result to an output slice. Returns whether
// there was a borrow left over. The output slice must be at least as long as either input slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `out` is shorter than `xs` or if `xs` and `ys` have different lengths.
//
// This is equivalent to `mpn_sub_nc` from `gmp-impl.h`, GMP 6.2.1, where `rp`, `up`, and `vp` are
// disjoint.
pub_crate_test! {limbs_sub_same_length_with_borrow_in_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    borrow_in: bool,
) -> bool {
    let mut borrow = limbs_sub_same_length_to_out(out, xs, ys);
    if borrow_in {
        borrow |= limbs_sub_limb_in_place(&mut out[..xs.len()], 1);
    }
    borrow
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, subtracts the second from the first, and then subtracts a borrow (`false` is 0,
// `true` is 1), writing the `xs.len()` limbs of the result to the first (left) slice. Return
// whether there was a borrow left over.
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
// This is equivalent to `mpn_sub_nc` from `gmp-impl.h`, GMP 6.2.1, where `rp` is the same as `up`.
pub_crate_test! {limbs_sub_same_length_with_borrow_in_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    borrow_in: bool,
) -> bool {
    let mut borrow = limbs_sub_same_length_in_place_left(xs, ys);
    if borrow_in {
        borrow |= limbs_sub_limb_in_place(xs, 1);
    }
    borrow
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, subtracts the second from the first, and then subtracts a borrow (`false` is 0,
// `true` is 1), writing the `xs.len()` limbs of the result to the second (right) slice. Returns
// whether there was a borrow left over.
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
// This is equivalent to `mpn_sub_nc` from `gmp-impl.h`, GMP 6.2.1, where `rp` is the same as `vp`.
pub_crate_test! {limbs_sub_same_length_with_borrow_in_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    borrow_in: bool,
) -> bool {
    let mut borrow = limbs_sub_same_length_in_place_right(xs, ys);
    if borrow_in {
        borrow |= limbs_sub_limb_in_place(ys, 1);
    }
    borrow
}}

fn sub_panic<S: Display, T: Display>(x: S, y: T) -> ! {
    panic!("Cannot subtract a number from a smaller number. self: {x}, other: {y}");
}

impl Natural {
    pub(crate) fn sub_limb(self, other: Limb) -> Natural {
        self.checked_sub_limb(other)
            .expect("Cannot subtract a Limb from a smaller Natural")
    }

    pub(crate) fn sub_limb_ref(&self, other: Limb) -> Natural {
        self.checked_sub_limb_ref(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }

    #[cfg(feature = "float_helpers")]
    pub fn sub_assign_at_limb(&mut self, i: usize, y: Limb) {
        if i == 0 {
            *self -= Natural::from(y);
            return;
        }
        let xs = self.promote_in_place();
        if xs.len() <= i {
            xs.resize(i + 1, 0);
        }
        assert!(!limbs_sub_limb_in_place(&mut xs[i..], y));
        self.trim();
    }
}

impl Sub<Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking both by value.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32) - Natural::ZERO, 123);
    /// assert_eq!(Natural::from(456u32) - Natural::from(123u32), 333);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12) * Natural::from(3u32) - Natural::from(10u32).pow(12),
    ///     2000000000000u64
    /// );
    /// ```
    fn sub(self, other: Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl<'a> Sub<&'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32) - &Natural::ZERO, 123);
    /// assert_eq!(Natural::from(456u32) - &Natural::from(123u32), 333);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12) * Natural::from(3u32) - &Natural::from(10u32).pow(12),
    ///     2000000000000u64
    /// );
    /// ```
    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl<'a> Sub<Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::from(123u32) - Natural::ZERO, 123);
    /// assert_eq!(&Natural::from(456u32) - Natural::from(123u32), 333);
    /// assert_eq!(
    ///     &(Natural::from(10u32).pow(12) * Natural::from(3u32)) - Natural::from(10u32).pow(12),
    ///     2000000000000u64
    /// );
    /// ```
    fn sub(self, other: Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl<'a, 'b> Sub<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking both by reference.
    ///
    /// $$
    /// f(x, y) = x - y.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::from(123u32) - &Natural::ZERO, 123);
    /// assert_eq!(&Natural::from(456u32) - &Natural::from(123u32), 333);
    /// assert_eq!(
    ///     &(Natural::from(10u32).pow(12) * Natural::from(3u32)) - &Natural::from(10u32).pow(12),
    ///     2000000000000u64
    /// );
    /// ```
    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

impl SubAssign<Natural> for Natural {
    /// Subtracts a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32).pow(12) * Natural::from(10u32);
    /// x -= Natural::from(10u32).pow(12);
    /// x -= Natural::from(10u32).pow(12) * Natural::from(2u32);
    /// x -= Natural::from(10u32).pow(12) * Natural::from(3u32);
    /// x -= Natural::from(10u32).pow(12) * Natural::from(4u32);
    /// assert_eq!(x, 0);
    /// ```
    fn sub_assign(&mut self, other: Natural) {
        assert!(
            !self.sub_assign_no_panic(other),
            "Cannot subtract a Natural from a smaller Natural"
        );
    }
}

impl<'a> SubAssign<&'a Natural> for Natural {
    /// Subtracts a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets x - y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32).pow(12) * Natural::from(10u32);
    /// x -= &Natural::from(10u32).pow(12);
    /// x -= &(Natural::from(10u32).pow(12) * Natural::from(2u32));
    /// x -= &(Natural::from(10u32).pow(12) * Natural::from(3u32));
    /// x -= &(Natural::from(10u32).pow(12) * Natural::from(4u32));
    /// assert_eq!(x, 0);
    /// ```
    fn sub_assign(&mut self, other: &'a Natural) {
        assert!(
            !self.sub_assign_ref_no_panic(other),
            "Cannot subtract a Natural from a smaller Natural"
        );
    }
}
