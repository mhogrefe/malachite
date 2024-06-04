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

use crate::natural::arithmetic::shl::{limbs_shl, limbs_vec_shl_in_place};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::iter::Sum;
use core::ops::{Add, AddAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the sum of the `Natural` and a `Limb`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_add_1` from `gmp.h`, GMP 6.2.1, where the result is returned.
pub_crate_test! {limbs_add_limb(xs: &[Limb], mut y: Limb) -> Vec<Limb> {
    let len = xs.len();
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let (sum, overflow) = xs[i].overflowing_add(y);
        out.push(sum);
        if overflow {
            y = 1;
        } else {
            y = 0;
            out.extend_from_slice(&xs[i + 1..]);
            break;
        }
    }
    if y != 0 {
        out.push(y);
    }
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the sum of the `Natural` and a `Limb` to an output slice. The output slice must be at
// least as long as the input slice. Returns whether there is a carry.
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
// This is equivalent to `mpn_add_1` from `gmp.h`, GMP 6.2.1.
pub_crate_test! {limbs_add_limb_to_out(out: &mut [Limb], xs: &[Limb], mut y: Limb) -> bool {
    let len = xs.len();
    assert!(out.len() >= len);
    for i in 0..len {
        let overflow;
        (out[i], overflow) = xs[i].overflowing_add(y);
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

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the sum of the `Natural` and a `Limb` to the input slice. Returns whether there is a
// carry.
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
pub_crate_test! {limbs_slice_add_limb_in_place<T: PrimitiveUnsigned>(
    xs: &mut [T],
    mut y: T
) -> bool {
    for x in &mut *xs {
        if x.overflowing_add_assign(y) {
            y = T::ONE;
        } else {
            return false;
        }
    }
    y != T::ZERO
}}

// Interpreting a nonempty `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes
// the limbs of the sum of the `Natural` and a `Limb` to the input `Vec`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$ (only if the `Vec` reallocates)
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty.
//
// This is equivalent to `mpz_add_ui` from `mpz/aors_ui.h`, GMP 6.2.1, where the input is
// non-negative.
pub_crate_test! {limbs_vec_add_limb_in_place(xs: &mut Vec<Limb>, y: Limb) {
    assert!(!xs.is_empty());
    if limbs_slice_add_limb_in_place(xs, y) {
        xs.push(1);
    }
}}

#[inline]
fn add_with_carry_limb(x: Limb, y: Limb, carry: Limb) -> (Limb, Limb) {
    let result_no_carry = x.wrapping_add(y);
    let result = result_no_carry.wrapping_add(carry);
    let carry = Limb::from((result_no_carry < x) || (result < result_no_carry));
    (result, carry)
}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where the
// first slice is at least as long as the second, returns a `Vec` of the limbs of the sum of the
// `Natural`s.
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
// This is equivalent to `mpn_add` from `gmp.h`, GMP 6.2.1, where the first input is at least as
// long as the second, and the output is returned.
pub_crate_test! {limbs_add_greater(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    if core::ptr::eq(xs, ys) {
        return limbs_shl(xs, 1);
    }
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let mut out = Vec::with_capacity(xs_len);
    let mut carry = 0;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        let o;
        (o, carry) = add_with_carry_limb(x, y, carry);
        out.push(o);
    }
    if xs_len == ys_len {
        if carry != 0 {
            out.push(1);
        }
    } else {
        out.extend_from_slice(&xs[ys_len..]);
        if carry != 0 && limbs_slice_add_limb_in_place(&mut out[ys_len..], 1) {
            out.push(1);
        }
    }
    out
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns a
// `Vec` of the limbs of the sum of the `Natural`s.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpn_add` from `gmp.h`, GMP 6.2.1, where the output is returned.
pub_crate_test! {limbs_add(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_add_greater(xs, ys)
    } else {
        limbs_add_greater(ys, xs)
    }
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s to an
// output slice. The output must be at least as long as one of the input slices. Returns whether
// there is a carry.
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
// This is equivalent to `mpn_add_n` from `gmp.h`, GMP 6.2.1.
pub_crate_test! {limbs_add_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    let mut carry = 0;
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        (*out, carry) = add_with_carry_limb(x, y, carry);
    }
    carry != 0
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where the
// first slice is at least as long as the second, writes the `xs.len()` least-significant limbs of
// the sum of the `Natural`s to an output slice. The output must be at least as long as `xs`.
// Returns whether there is a carry.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys` or if `out` is too short.
//
// This is equivalent to `mpn_add` from `gmp.h`, GMP 6.2.1, where the first input is at least as
// long as the second.
pub_crate_test! {limbs_add_greater_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert!(out.len() >= xs_len);
    let carry = limbs_add_same_length_to_out(out, &xs[..ys_len], ys);
    if xs_len == ys_len {
        carry
    } else if carry {
        limbs_add_limb_to_out(&mut out[ys_len..], &xs[ys_len..], 1)
    } else {
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
        false
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `max(xs.len(), ys.len())` least-significant limbs of the sum of the `Natural`s to an output
// slice. The output must be at least as long as the longer input slice. Returns whether there is a
// carry.
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
// This is equivalent to `mpn_add` from `gmp.h`, GMP 6.2.1.
pub_crate_test! {limbs_add_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_add_greater_to_out(out, xs, ys)
    } else {
        limbs_add_greater_to_out(out, ys, xs)
    }
}}

// Given two slices of `Limb`s as the limbs `xs` and `ys`, where `xs` is at least as long as `ys`
// and `xs_len` is no greater than `ys.len()`, writes the `ys.len()` lowest limbs of the sum of
// `xs[..xs_len]` and `ys` to `xs`. Returns whether there is a carry.
//
// For example, `limbs_add_to_out_aliased(&mut xs[..12], 7, &ys[..10])` would be equivalent to
// `limbs_add_to_out(&mut xs[..12], &xs[..7], &ys[..10])` although the latter expression is not
// allowed because `xs` cannot be borrowed in that way.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` is shorter than `ys` or `xs_len` is greater than `ys.len()`.
//
// This is equivalent to `mpn_add` from `gmp.h`, GMP 6.2.1, where the second argument is at least as
// long as the first and the output pointer is the same as the first input pointer.
pub_crate_test! {limbs_add_to_out_aliased(xs: &mut [Limb], xs_len: usize, ys: &[Limb]) -> bool {
    let ys_len = ys.len();
    assert!(xs.len() >= ys_len);
    assert!(xs_len <= ys_len);
    let (ys_lo, ys_hi) = ys.split_at(xs_len);
    xs[xs_len..ys_len].copy_from_slice(ys_hi);
    limbs_slice_add_greater_in_place_left(&mut xs[..ys_len], ys_lo)
}}

// For example, `limbs_add_to_out_aliased_2(&mut xs[..15], 5, &ys[..10])` would be equivalent to
// `limbs_add_to_out(&mut xs[..10], &xs[5..15], &ys[..10])` although the latter expression is not
// allowed because `xs` cannot be borrowed in that way.
pub_crate_test! {
    limbs_add_to_out_aliased_2(xs: &mut [Limb], xs_offset: usize, ys: &[Limb]) -> bool {
    let len = ys.len();
    assert_eq!(xs.len(), len + xs_offset);
    let mut carry = 0;
    for i in 0..len {
        (xs[i], carry) = add_with_carry_limb(xs[i + xs_offset], ys[i], carry);
    }
    carry != 0
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s to the
// first (left) slice. Returns whether there is a carry.
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
// This is equivalent to `mpn_add_n` from `gmp.h`, GMP 6.2.1, where the output is written to the
// first input.
pub_crate_test! {limbs_slice_add_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    assert_eq!(xs_len, ys.len());
    let mut carry = 0;
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        (*x, carry) = add_with_carry_limb(*x, y, carry);
    }
    carry != 0
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, where the
// length of the first slice is greater than or equal to the length of the second, writes the
// `xs.len()` least-significant limbs of the sum of the `Natural`s to the first (left) slice.
// Returns whether there is a carry.
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
// This is equivalent to `mpn_add` from `gmp.h`, GMP 6.2.1, where the first input is at least as
// long as the second, and the output is written to the first input.
pub_crate_test! {limbs_slice_add_greater_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
    let carry = limbs_slice_add_same_length_in_place_left(xs_lo, ys);
    if xs_len == ys_len {
        carry
    } else if carry {
        limbs_slice_add_limb_in_place(xs_hi, 1)
    } else {
        false
    }
}}

// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the limbs of the sum of the `Natural`s to the first (left) slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is `max(1,
// ys.len() - xs.len())`.
//
// This is equivalent to `mpz_add` from `mpz/aors.h`, GMP 6.2.1, where both inputs are non-negative
// and the output is written to the first input.
pub_crate_test! {limbs_vec_add_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    if core::ptr::eq(xs.as_slice(), ys) {
        limbs_vec_shl_in_place(xs, 1);
        return;
    }
    let xs_len = xs.len();
    let ys_len = ys.len();
    let carry = if xs_len >= ys_len {
        limbs_slice_add_greater_in_place_left(xs, ys)
    } else {
        let (ys_lo, ys_hi) = ys.split_at(xs_len);
        let mut carry = limbs_slice_add_same_length_in_place_left(xs, ys_lo);
        xs.extend_from_slice(ys_hi);
        if carry {
            carry = limbs_slice_add_limb_in_place(&mut xs[xs_len..], 1);
        }
        carry
    };
    if carry {
        xs.push(1);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the `max(xs.len(), ys.len())` least-significant limbs of the sum of the `Natural`s to the longer
// slice (or the first one, if they are equally long). Returns a pair of `bool`s. The first is
// `false` when the output is to the first slice and `true` when it's to the second slice, and the
// second is whether there is a carry.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// This is equivalent to `mpn_add` from `gmp.h`, GMP 6.2.1, where the output is written to the
// longer input.
pub_test! {limbs_slice_add_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> (bool, bool) {
    if xs.len() >= ys.len() {
        (false, limbs_slice_add_greater_in_place_left(xs, ys))
    } else {
        (true, limbs_slice_add_greater_in_place_left(ys, xs))
    }
}}

// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
// the limbs of the sum of the `Natural`s to the longer slice (or the first one, if they are equally
// long). Returns a `bool` which is `false` when the output is to the first `Vec` and `true` when
// it's to the second `Vec`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$ (only if the `Vec` reallocates)
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_add` from `mpz/aors.h`, GMP 6.2.1, where both inputs are non-negative
// and the output is written to the longer input.
pub_test! {limbs_vec_add_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> bool {
    if xs.len() >= ys.len() {
        if limbs_slice_add_greater_in_place_left(xs, ys) {
            xs.push(1);
        }
        false
    } else {
        if limbs_slice_add_greater_in_place_left(ys, xs) {
            ys.push(1);
        }
        true
    }
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s and a
// carry (`false` is 0, `true` is 1) to an output slice. The output must be at least as long as one
// of the input slices. Returns whether there is a carry.
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
// This is equivalent to `mpn_add_nc` from `gmp-impl.h`, GMP 6.2.1, where `rp` and `up` are
// disjoint.
pub_crate_test! {limbs_add_same_length_with_carry_in_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    carry_in: bool,
) -> bool {
    let mut carry = limbs_add_same_length_to_out(out, xs, ys);
    if carry_in {
        carry |= limbs_slice_add_limb_in_place(&mut out[..xs.len()], 1);
    }
    carry
}}

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, writes the `xs.len()` least-significant limbs of the sum of the `Natural`s and a
// carry (`false` is 0, `true` is 1) to the first (left) slice. Returns whether there is a carry.
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
// This is equivalent to `mpn_add_nc` from `gmp-impl.h`, GMP 6.2.1, where `rp` is the same as `up`.
pub_crate_test! {limbs_add_same_length_with_carry_in_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    carry_in: bool,
) -> bool {
    let mut carry = limbs_slice_add_same_length_in_place_left(xs, ys);
    if carry_in {
        carry |= limbs_slice_add_limb_in_place(xs, 1);
    }
    carry
}}

impl Natural {
    #[inline]
    pub(crate) fn add_limb(mut self, other: Limb) -> Natural {
        self.add_assign_limb(other);
        self
    }

    pub(crate) fn add_limb_ref(&self, other: Limb) -> Natural {
        match (self, other) {
            (x, 0) => x.clone(),
            (Natural(Small(small)), other) => match small.overflowing_add(other) {
                (sum, false) => Natural::from(sum),
                (sum, true) => Natural(Large(vec![sum, 1])),
            },
            (Natural(Large(ref limbs)), other) => Natural(Large(limbs_add_limb(limbs, other))),
        }
    }

    fn add_assign_limb(&mut self, other: Limb) {
        match (&mut *self, other) {
            (_, 0) => {}
            (&mut Natural::ZERO, _) => *self = Natural::from(other),
            (&mut Natural(Small(ref mut small)), other) => {
                let (sum, overflow) = small.overflowing_add(other);
                if overflow {
                    *self = Natural(Large(vec![sum, 1]));
                } else {
                    *small = sum;
                }
            }
            (&mut Natural(Large(ref mut limbs)), other) => {
                limbs_vec_add_limb_in_place(limbs, other);
            }
        }
    }

    #[cfg(feature = "float_helpers")]
    pub fn add_assign_at_limb(&mut self, i: usize, y: Limb) {
        if i == 0 {
            *self += Natural::from(y);
            return;
        }
        let xs = self.promote_in_place();
        if xs.len() <= i {
            xs.resize(i + 1, 0);
        }
        if limbs_slice_add_limb_in_place(&mut xs[i..], y) {
            xs.push(1);
        }
    }
}

impl Add<Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO + Natural::from(123u32), 123);
    /// assert_eq!(Natural::from(123u32) + Natural::ZERO, 123);
    /// assert_eq!(Natural::from(123u32) + Natural::from(456u32), 579);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12) + (Natural::from(10u32).pow(12) << 1),
    ///     3000000000000u64
    /// );
    /// ```
    fn add(mut self, other: Natural) -> Natural {
        self += other;
        self
    }
}

impl<'a> Add<&'a Natural> for Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO + &Natural::from(123u32), 123);
    /// assert_eq!(Natural::from(123u32) + &Natural::ZERO, 123);
    /// assert_eq!(Natural::from(123u32) + &Natural::from(456u32), 579);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12) + &(Natural::from(10u32).pow(12) << 1),
    ///     3000000000000u64
    /// );
    /// ```
    #[inline]
    fn add(mut self, other: &'a Natural) -> Natural {
        self += other;
        self
    }
}

impl<'a> Add<Natural> for &'a Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::ZERO + Natural::from(123u32), 123);
    /// assert_eq!(&Natural::from(123u32) + Natural::ZERO, 123);
    /// assert_eq!(&Natural::from(123u32) + Natural::from(456u32), 579);
    /// assert_eq!(
    ///     &Natural::from(10u32).pow(12) + (Natural::from(10u32).pow(12) << 1),
    ///     3000000000000u64
    /// );
    /// ```
    #[inline]
    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
}

impl<'a, 'b> Add<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Adds two [`Natural`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(&Natural::ZERO + &Natural::from(123u32), 123);
    /// assert_eq!(&Natural::from(123u32) + &Natural::ZERO, 123);
    /// assert_eq!(&Natural::from(123u32) + &Natural::from(456u32), 579);
    /// assert_eq!(
    ///     &Natural::from(10u32).pow(12) + &(Natural::from(10u32).pow(12) << 1),
    ///     3000000000000u64
    /// );
    /// ```
    fn add(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Natural(Small(y))) => x.add_limb_ref(y),
            (&Natural(Small(x)), y) => y.add_limb_ref(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => Natural(Large(limbs_add(xs, ys))),
        }
    }
}

impl AddAssign<Natural> for Natural {
    /// Adds a [`Natural`] to a [`Natural`] in place, taking the [`Natural`] on the right-hand side
    /// by value.
    ///
    /// $$
    /// x \gets x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$ (only if the underlying [`Vec`] needs to reallocate)
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x += Natural::from(10u32).pow(12);
    /// x += Natural::from(10u32).pow(12) * Natural::from(2u32);
    /// x += Natural::from(10u32).pow(12) * Natural::from(3u32);
    /// x += Natural::from(10u32).pow(12) * Natural::from(4u32);
    /// assert_eq!(x, 10000000000000u64);
    /// ```
    fn add_assign(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (x, &mut Natural(Small(y))) => x.add_assign_limb(y),
            (&mut Natural(Small(x)), y) => *self = y.add_limb_ref(x),
            (&mut Natural(Large(ref mut xs)), &mut Natural(Large(ref mut ys))) => {
                if limbs_vec_add_in_place_either(xs, ys) {
                    *self = other;
                }
            }
        }
    }
}

impl<'a> AddAssign<&'a Natural> for Natural {
    /// Adds a [`Natural`] to a [`Natural`] in place, taking the [`Natural`] on the right-hand side
    /// by reference.
    ///
    /// $$
    /// x \gets x + y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x += &Natural::from(10u32).pow(12);
    /// x += &(Natural::from(10u32).pow(12) * Natural::from(2u32));
    /// x += &(Natural::from(10u32).pow(12) * Natural::from(3u32));
    /// x += &(Natural::from(10u32).pow(12) * Natural::from(4u32));
    /// assert_eq!(x, 10000000000000u64);
    /// ```
    fn add_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (x, &Natural(Small(y))) => x.add_assign_limb(y),
            (&mut Natural(Small(x)), y) => *self = y.add_limb_ref(x),
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                limbs_vec_add_in_place_left(xs, ys);
            }
        }
    }
}

impl Sum for Natural {
    /// Adds up all the [`Natural`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Natural::sum(xs.map(Natural::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::sum(vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().into_iter()),
    ///     17
    /// );
    /// ```
    fn sum<I>(xs: I) -> Natural
    where
        I: Iterator<Item = Natural>,
    {
        let mut s = Natural::ZERO;
        for x in xs {
            s += x;
        }
        s
    }
}

impl<'a> Sum<&'a Natural> for Natural {
    /// Adds up all the [`Natural`]s in an iterator of [`Natural`] references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \sum_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `Natural::sum(xs.map(Natural::significant_bits))`.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Sum;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::sum(vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().iter()),
    ///     17
    /// );
    /// ```
    fn sum<I>(xs: I) -> Natural
    where
        I: Iterator<Item = &'a Natural>,
    {
        let mut s = Natural::ZERO;
        for x in xs {
            s += x;
        }
        s
    }
}
