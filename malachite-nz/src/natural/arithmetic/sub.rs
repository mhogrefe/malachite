// Copyright © 2026 Mikhail Hogrefe
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
use malachite_base::num::arithmetic::traits::CheckedSub;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

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
pub_crate_test! {limbs_sub_limb_in_place<T: PrimitiveUnsigned>(xs: &mut [T], mut y: T) -> bool {
    for x in &mut *xs {
        if x.overflowing_sub_assign(y) {
            y = T::ONE;
        } else {
            return false;
        }
    }
    y != T::ZERO
}}

// A subtract-with-borrow with a `bool` borrow. Written with `overflowing_sub` so that LLVM
// recognizes the subtract-with-borrow idiom; in the unrolled kernels below this compiles to
// flag-chained subtracts rather than rematerializing the borrow in a register every limb (see the
// analogous `add_with_carry` in add.rs and perf/README.md).
#[inline]
pub(crate) fn sub_with_borrow(x: Limb, y: Limb, borrow: bool) -> (Limb, bool) {
    let (diff, borrow_1) = x.overflowing_sub(y);
    let (diff, borrow_2) = diff.overflowing_sub(Limb::from(borrow));
    (diff, borrow_1 | borrow_2)
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
    // Filling a zeroed Vec and using the unrolled same-length kernel is faster than pushing one
    // limb at a time.
    let mut out = vec![0; xs_len];
    let mut borrow = limbs_sub_same_length_to_out(&mut out, &xs[..ys_len], ys);
    if xs_len != ys_len {
        out[ys_len..].copy_from_slice(&xs[ys_len..]);
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
    limbs_sub_same_length_with_borrow_in_to_out(out, xs, ys, false)
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
    limbs_sub_same_length_with_borrow_in_in_place_left(xs, ys, false)
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
    limbs_sub_same_length_with_borrow_in_in_place_right(xs, ys, false)
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
    if right_start >= len {
        // The read and write ranges are disjoint, so the unrolled kernel can be used.
        let (xs_lo, xs_hi) = xs.split_at_mut(right_start);
        limbs_sub_same_length_in_place_left(&mut xs_lo[..len], &xs_hi[..len])
    } else {
        // The ranges overlap. Each read at i + right_start happens before the write at that index,
        // so reading and writing in ascending order is correct.
        let mut borrow = false;
        for i in 0..len {
            (xs[i], borrow) = sub_with_borrow(xs[i], xs[i + right_start], borrow);
        }
        borrow
    }
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
    if right_start >= ys_len {
        // The read and write ranges are disjoint, so the unrolled kernel can be used.
        let (xs_lo, xs_hi) = xs.split_at_mut(right_start);
        limbs_sub_same_length_to_out(xs_lo, xs_hi, ys)
    } else {
        // The ranges overlap. Each read at i + right_start happens before the write at that index,
        // so reading and writing in ascending order is correct.
        let mut borrow = false;
        for i in 0..ys_len {
            (xs[i], borrow) = sub_with_borrow(xs[i + right_start], ys[i], borrow);
        }
        borrow
    }
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
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    let mut borrow = borrow_in;
    // 4x-unrolled so that LLVM chains the borrows in flags within each block.
    let mut out_chunks = out[..len].chunks_exact_mut(4);
    let mut xs_chunks = xs.chunks_exact(4);
    let mut ys_chunks = ys.chunks_exact(4);
    for ((o, x), y) in (&mut out_chunks).zip(&mut xs_chunks).zip(&mut ys_chunks) {
        for i in 0..4 {
            (o[i], borrow) = sub_with_borrow(x[i], y[i], borrow);
        }
    }
    for ((o, &x), &y) in out_chunks
        .into_remainder()
        .iter_mut()
        .zip(xs_chunks.remainder().iter())
        .zip(ys_chunks.remainder().iter())
    {
        (*o, borrow) = sub_with_borrow(x, y, borrow);
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
    assert_eq!(xs.len(), ys.len());
    let mut borrow = borrow_in;
    // 4x-unrolled so that LLVM chains the borrows in flags within each block.
    let mut xs_chunks = xs.chunks_exact_mut(4);
    let mut ys_chunks = ys.chunks_exact(4);
    for (x, y) in (&mut xs_chunks).zip(&mut ys_chunks) {
        for i in 0..4 {
            (x[i], borrow) = sub_with_borrow(x[i], y[i], borrow);
        }
    }
    for (x, &y) in xs_chunks
        .into_remainder()
        .iter_mut()
        .zip(ys_chunks.remainder().iter())
    {
        (*x, borrow) = sub_with_borrow(*x, y, borrow);
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
    assert_eq!(xs.len(), ys.len());
    let mut borrow = borrow_in;
    // 4x-unrolled so that LLVM chains the borrows in flags within each block.
    let mut xs_chunks = xs.chunks_exact(4);
    let mut ys_chunks = ys.chunks_exact_mut(4);
    for (x, y) in (&mut xs_chunks).zip(&mut ys_chunks) {
        for i in 0..4 {
            (y[i], borrow) = sub_with_borrow(x[i], y[i], borrow);
        }
    }
    for (&x, y) in xs_chunks
        .remainder()
        .iter()
        .zip(ys_chunks.into_remainder().iter_mut())
    {
        (*y, borrow) = sub_with_borrow(x, *y, borrow);
    }
    borrow
}}

fn sub_panic<S: Display, T: Display>(x: S, y: T) -> ! {
    panic!("Cannot subtract a number from a smaller number. self: {x}, other: {y}");
}

impl Natural {
    pub(crate) fn sub_limb(self, other: Limb) -> Self {
        self.checked_sub_limb(other)
            .expect("Cannot subtract a Limb from a smaller Natural")
    }

    pub(crate) fn sub_limb_ref(&self, other: Limb) -> Self {
        self.checked_sub_limb_ref(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }

    #[cfg(feature = "float_helpers")]
    pub fn sub_assign_at_limb(&mut self, i: usize, y: Limb) {
        if i == 0 {
            *self -= Self::from(y);
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

impl Sub<Self> for Natural {
    type Output = Self;

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
    fn sub(self, other: Self) -> Self {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl Sub<&Self> for Natural {
    type Output = Self;

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
    fn sub(self, other: &Self) -> Self {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl Sub<Natural> for &Natural {
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

impl Sub<&Natural> for &Natural {
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
    fn sub(self, other: &Natural) -> Natural {
        self.checked_sub(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

impl SubAssign<Self> for Natural {
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
    fn sub_assign(&mut self, other: Self) {
        assert!(
            !self.sub_assign_no_panic(other),
            "Cannot subtract a Natural from a smaller Natural"
        );
    }
}

impl SubAssign<&Self> for Natural {
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
    fn sub_assign(&mut self, other: &Self) {
        assert!(
            !self.sub_assign_ref_no_panic(other),
            "Cannot subtract a Natural from a smaller Natural"
        );
    }
}
