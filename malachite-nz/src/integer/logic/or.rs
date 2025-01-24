// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993, 1994, 1996, 1997, 2000, 2001, 2005, 2012, 2013, 2015-2018 Free
//      Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::logic::not::{limbs_not_in_place, limbs_not_to_out};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::{max, Ordering::*};
use core::ops::{BitOr, BitOrAssign};
use itertools::repeat_n;
use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, returns the limbs of the bitwise or of the `Integer` and a `Limb`. `xs` cannot be
// empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if `xs` is empty or only contains zeros.
pub_test! {limbs_neg_or_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    if y == 0 {
        return xs.to_vec();
    }
    let mut out = vec![0; xs.len()];
    let i = slice_leading_zeros(xs);
    if i == 0 {
        out[0] = (xs[0].wrapping_neg() | y).wrapping_neg();
        out[1..].copy_from_slice(&xs[1..]);
    } else {
        out[0] = y.wrapping_neg();
        for x in &mut out[1..i] {
            *x = Limb::MAX;
        }
        out[i] = xs[i] - 1;
        out[i + 1..].copy_from_slice(&xs[i + 1..]);
    }
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, writes the limbs of the bitwise or of the `Integer` and a `Limb` to an output slice.
// The output slice must be at least as long as the input slice. `xs` cannot be empty or only
// contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if `xs` is empty or only contains zeros, or if `out` is shorter than `xs`.
pub_test! {limbs_neg_or_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    let len = xs.len();
    assert!(out.len() >= len);
    if y == 0 {
        out[..len].copy_from_slice(xs);
        return;
    }
    let i = slice_leading_zeros(xs);
    if i == 0 {
        out[0] = (xs[0].wrapping_neg() | y).wrapping_neg();
        out[1..len].copy_from_slice(&xs[1..]);
    } else {
        out[0] = y.wrapping_neg();
        for x in &mut out[1..i] {
            *x = Limb::MAX;
        }
        out[i] = xs[i] - 1;
        out[i + 1..len].copy_from_slice(&xs[i + 1..]);
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, writes the limbs of the bitwise or of the `Integer`, writes the limbs of the bitwise
// or of the `Integer` and a `Limb` to the input slice. `xs` cannot be empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// May panic if `xs` is empty or only contains zeros.
pub_test! {limbs_neg_or_limb_in_place(xs: &mut [Limb], y: Limb) {
    if y == 0 {
        return;
    }
    let i = slice_leading_zeros(xs);
    if i == 0 {
        xs[0] = (xs[0].wrapping_neg() | y).wrapping_neg();
    } else {
        xs[0] = y.wrapping_neg();
        for x in &mut xs[1..i] {
            *x = Limb::MAX;
        }
        xs[i] -= 1;
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
// negative of the bitwise or of the `Integer` and a negative number whose lowest limb is given by
// `y` and whose other limbs are full of `true` bits. The slice cannot be empty or only contain
// zeros.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `xs` is empty.
pub_const_test! {limbs_pos_or_neg_limb(xs: &[Limb], y: Limb) -> Limb {
    (xs[0] | y).wrapping_neg()
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, returns the negative of the bitwise or of the `Integer` and a negative number whose
// lowest limb is given by `y` and whose other limbs are full of `true` bits. The slice cannot be
// empty or only contain zeros.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `xs` is empty.
pub_const_test! {limbs_neg_or_neg_limb(xs: &[Limb], y: Limb) -> Limb {
    (xs[0].wrapping_neg() | y).wrapping_neg()
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, returns the limbs of the bitwise or of the `Integer`s. `xs` and `ys` may not
// be empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is
// `ys.len()`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where `res` is returned, the first
// input is positive, and the second is negative.
pub_test! {limbs_or_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        let mut out = vec![0; x_i];
        out.push(xs[x_i].wrapping_neg());
        out.extend(xs[x_i + 1..].iter().map(|x| !x));
        out.extend(repeat_n(Limb::MAX, y_i - xs_len));
        out.push(ys[y_i] - 1);
        out.extend_from_slice(&ys[y_i + 1..]);
        out
    } else if x_i >= ys_len {
        ys.to_vec()
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        let mut out = vec![0; min_i];
        match x_i.cmp(&y_i) {
            Equal => {
                out.push((!xs[x_i] & (ys[y_i] - 1)) + 1);
            }
            Less => {
                out.push(xs[x_i].wrapping_neg());
                out.extend(xs[x_i + 1..y_i].iter().map(|x| !x));
                out.push(!xs[y_i] & (ys[y_i] - 1));
            }
            Greater => {
                out.extend_from_slice(&ys[y_i..x_i]);
                out.push(!xs[x_i] & ys[x_i]);
            }
        }
        out.extend(
            xs[max_i + 1..]
                .iter()
                .zip(ys[max_i + 1..].iter())
                .map(|(x, y)| !x & y),
        );
        if xs_len < ys_len {
            out.extend_from_slice(&ys[xs_len..]);
        }
        out
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise or of the `Integer`s to an output slice.
// `xs` and `ys` may not be empty or only contain zeros. The output slice must be at least as long
// as the second input slice. `ys.len()` limbs will be written; if the number of significant limbs
// of the result is lower, some of the written limbs will be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than `ys`.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where the first input is positive
// and the second is negative.
pub_test! {limbs_or_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        slice_set_zero(&mut out[..x_i]);
        out[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut out[x_i + 1..xs_len], &xs[x_i + 1..]);
        for x in &mut out[xs_len..y_i] {
            *x = Limb::MAX;
        }
        out[y_i] = ys[y_i] - 1;
        out[y_i + 1..ys_len].copy_from_slice(&ys[y_i + 1..]);
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        slice_set_zero(&mut out[..min_i]);
        match x_i.cmp(&y_i) {
            Equal => {
                out[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Less => {
                out[x_i] = xs[x_i].wrapping_neg();
                limbs_not_to_out(&mut out[x_i + 1..y_i], &xs[x_i + 1..y_i]);
                out[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Greater => {
                out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                out[x_i] = !xs[x_i] & ys[x_i];
            }
        }
        for (out, (x, y)) in out[max_i + 1..]
            .iter_mut()
            .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
        {
            *out = !x & y;
        }
        if xs_len < ys_len {
            out[xs_len..ys_len].copy_from_slice(&ys[xs_len..]);
        }
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise or of the `Integer`s to the first (left)
// slice. `xs` and `ys` may not be empty or only contain zeros. Returns whether the result is too
// large to be contained in the first slice; if it is, only the lowest `xs.len()` limbs are written.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where `res == op1`, the first input
// is positive and the second is negative, and the length of `op1` is not changed; instead, a carry
// is returned.
pub_test! {limbs_slice_or_pos_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..]);
        true
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        slice_set_zero(&mut xs[ys_len..]);
        false
    } else {
        let max_i = max(x_i, y_i);
        match x_i.cmp(&y_i) {
            Equal => {
                xs[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Less => {
                xs[x_i].wrapping_neg_assign();
                limbs_not_in_place(&mut xs[x_i + 1..y_i]);
                xs[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Greater => {
                xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                xs[x_i] = !xs[x_i] & ys[x_i];
            }
        }
        if xs_len < ys_len {
            for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..xs_len].iter()) {
                *x = !*x & y;
            }
            true
        } else {
            for (x, y) in xs[max_i + 1..ys_len].iter_mut().zip(ys[max_i + 1..].iter()) {
                *x = !*x & y;
            }
            slice_set_zero(&mut xs[ys_len..]);
            false
        }
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise or of the `Integer`s to the first (left)
// slice. `xs` and `ys` may not be empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is `max(1,
// ys.len() - xs.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where `res == op1` and the first
// input is positive and the second is negative.
pub_test! {limbs_vec_or_pos_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..]);
        xs.extend(repeat_n(Limb::MAX, y_i - xs_len));
        xs.push(ys[y_i] - 1);
        xs.extend_from_slice(&ys[y_i + 1..]);
    } else if x_i >= ys_len {
        *xs = ys.to_vec();
    } else {
        let max_i = max(x_i, y_i);
        match x_i.cmp(&y_i) {
            Equal => {
                xs[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Less => {
                xs[x_i].wrapping_neg_assign();
                limbs_not_in_place(&mut xs[x_i + 1..y_i]);
                xs[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Greater => {
                xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                xs[x_i] = !xs[x_i] & ys[x_i];
            }
        }
        if xs_len < ys_len {
            for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..xs_len].iter()) {
                *x = !*x & y;
            }
            xs.extend_from_slice(&ys[xs_len..]);
        } else {
            for (x, y) in xs[max_i + 1..ys_len].iter_mut().zip(ys[max_i + 1..].iter()) {
                *x = !*x & y;
            }
            xs.truncate(ys_len);
        }
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise or of the `Integer`s to the second (right)
// slice. `xs` and `ys` may not be empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where `res == op2` and the first
// input is positive and the second is negative.
pub_test! {limbs_or_pos_neg_in_place_right(xs: &[Limb], ys: &mut [Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        ys[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut ys[x_i + 1..xs_len], &xs[x_i + 1..]);
        for y in &mut ys[xs_len..y_i] {
            *y = Limb::MAX;
        }
        ys[y_i] -= 1;
    } else if x_i < ys_len {
        let max_i = max(x_i, y_i);
        match x_i.cmp(&y_i) {
            Equal => {
                ys[y_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Less => {
                ys[x_i] = xs[x_i].wrapping_neg();
                limbs_not_to_out(&mut ys[x_i + 1..y_i], &xs[x_i + 1..y_i]);
                ys[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Greater => {
                ys[x_i] &= !xs[x_i];
            }
        }
        if xs_len < ys_len {
            for (x, y) in xs[max_i + 1..].iter().zip(ys[max_i + 1..xs_len].iter_mut()) {
                *y &= !x;
            }
        } else {
            for (x, y) in xs[max_i + 1..ys_len].iter().zip(ys[max_i + 1..].iter_mut()) {
                *y &= !x;
            }
        }
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, returns the limbs of the bitwise or of the `Integer`s. `xs` and `ys` may not be empty
// or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is
// `min(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where `res` is returned and both
// inputs are negative.
pub_test! {limbs_or_neg_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs.to_vec()
    } else if x_i >= ys_len {
        ys.to_vec()
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        let mut out = vec![0; min_i];
        let x = match x_i.cmp(&y_i) {
            Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Less => {
                out.extend_from_slice(&xs[x_i..y_i]);
                xs[y_i] & (ys[y_i] - 1)
            }
            Greater => {
                out.extend_from_slice(&ys[y_i..x_i]);
                (xs[x_i] - 1) & ys[x_i]
            }
        };
        out.push(x);
        out.extend(
            xs[max_i + 1..]
                .iter()
                .zip(ys[max_i + 1..].iter())
                .map(|(x, y)| x & y),
        );
        out
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the max(`xs.len()`, `ys.len()`) limbs of the bitwise or of the `Integer`s to
// an output slice. `xs` and `ys` may not be empty or only contain zeros. The output slice must be
// at least as long as the shorter input slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than the shorter
// of `xs` and `ys`.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where both inputs are negative.
pub_test! {limbs_or_neg_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len || out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        out[..xs_len].copy_from_slice(xs);
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        slice_set_zero(&mut out[..min_i]);
        let x = match x_i.cmp(&y_i) {
            Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Less => {
                out[x_i..y_i].copy_from_slice(&xs[x_i..y_i]);
                xs[y_i] & (ys[y_i] - 1)
            }
            Greater => {
                out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                (xs[x_i] - 1) & ys[x_i]
            }
        };
        out[max_i] = x;
        for (out, (x, y)) in out[max_i + 1..]
            .iter_mut()
            .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
        {
            *out = x & y;
        }
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the limbs of the bitwise or of the `Integer`s to the first (left) slice. `xs`
// and `ys` may not be empty or only contain zeros. If the result has fewer significant limbs than
// the left slice, the remaining limbs in the left slice are set to `Limb::MAX`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where `res == op1`, both inputs are
// negative, and the length of `op1` is not changed.
pub_test! {limbs_slice_or_neg_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        slice_set_zero(&mut xs[ys_len..]);
    } else {
        let max_i = max(x_i, y_i);
        if x_i > y_i {
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        }
        xs[max_i] = match x_i.cmp(&y_i) {
            Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Less => xs[y_i] & (ys[y_i] - 1),
            Greater => (xs[x_i] - 1) & ys[x_i],
        };
        for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
            *x &= y;
        }
        if xs_len > ys_len {
            slice_set_zero(&mut xs[ys_len..]);
        }
    }
}}

// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of the
// negatives of two `Integer`s, writes the limbs of the bitwise or of the `Integer`s to the `Vec`.
// `xs` and `ys` may not be empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where `res == op1` and both inputs
// are negative.
pub_test! {limbs_vec_or_neg_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
    } else if x_i >= ys_len {
        xs.truncate(ys_len);
        xs.copy_from_slice(ys);
    } else {
        let max_i = max(x_i, y_i);
        if x_i > y_i {
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        }
        xs[max_i] = match x_i.cmp(&y_i) {
            Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Less => xs[y_i] & (ys[y_i] - 1),
            Greater => (xs[x_i] - 1) & ys[x_i],
        };
        for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
            *x &= y;
        }
        xs.truncate(ys_len);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the lower min(`xs.len()`, `ys.len()`) limbs of the bitwise or of the
// `Integer`s to the shorter slice (or the first one, if they are equally long). `xs` and `ys` may
// not be empty or only contain zeros. Returns a `bool` which is `false` when the output is to the
// first slice and `true` when it's to the second slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_ior` from `mpz/ior.c`, GMP 6.2.1, where both inputs are negative and
// the result is written to the shorter input slice.
pub_test! {limbs_or_neg_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        false
    } else if x_i >= ys_len {
        true
    } else {
        let max_i = max(x_i, y_i);
        let boundary = match x_i.cmp(&y_i) {
            Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Less => xs[y_i] & (ys[y_i] - 1),
            Greater => (xs[x_i] - 1) & ys[x_i],
        };
        if xs_len > ys_len {
            if y_i > x_i {
                ys[x_i..y_i].copy_from_slice(&xs[x_i..y_i]);
            }
            ys[max_i] = boundary;
            for (y, x) in ys[max_i + 1..].iter_mut().zip(xs[max_i + 1..].iter()) {
                *y &= x;
            }
            true
        } else {
            if x_i > y_i {
                xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            }
            xs[max_i] = boundary;
            for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
                *x &= y;
            }
            false
        }
    }
}}

impl Natural {
    fn or_assign_pos_limb_neg(&mut self, other: Limb) {
        *self = self.or_pos_limb_neg(other);
    }

    fn or_pos_limb_neg(&self, other: Limb) -> Natural {
        Natural(Small(match *self {
            Natural(Small(small)) => (small | other).wrapping_neg(),
            Natural(Large(ref limbs)) => limbs_pos_or_neg_limb(limbs, other),
        }))
    }

    fn or_assign_neg_limb_neg(&mut self, other: Limb) {
        *self = self.or_neg_limb_neg(other);
    }

    fn or_neg_limb_neg(&self, other: Limb) -> Natural {
        Natural(Small(match *self {
            Natural(Small(small)) => (small.wrapping_neg() | other).wrapping_neg(),
            Natural(Large(ref limbs)) => limbs_neg_or_neg_limb(limbs, other),
        }))
    }

    fn or_assign_neg_limb_pos(&mut self, other: Limb) {
        match *self {
            Natural(Small(ref mut small)) => {
                *small = (small.wrapping_neg() | other).wrapping_neg();
            }
            Natural(Large(ref mut limbs)) => {
                limbs_neg_or_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }

    fn or_neg_limb_pos(&self, other: Limb) -> Natural {
        match *self {
            Natural(Small(ref small)) => {
                Natural(Small((small.wrapping_neg() | other).wrapping_neg()))
            }
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_neg_or_limb(limbs, other))
            }
        }
    }

    fn or_assign_pos_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.or_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => *self = other.or_neg_limb_pos(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_or_pos_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn or_assign_pos_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.or_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => {
                other.or_assign_neg_limb_pos(*x);
                *self = other;
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                limbs_or_pos_neg_in_place_right(xs, ys);
                *self = other;
                self.trim();
            }
        }
    }

    fn or_assign_neg_pos_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_pos(*y),
            (Natural(Small(x)), _) => *self = other.or_pos_limb_neg(x.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_or_pos_neg_in_place_right(ys, xs);
                self.trim();
            }
        }
    }

    fn or_assign_neg_pos(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_pos(*y),
            (Natural(Small(x)), _) => {
                other.or_assign_pos_limb_neg(x.wrapping_neg());
                *self = other;
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                limbs_or_pos_neg_in_place_right(ys, xs);
                self.trim();
            }
        }
    }

    fn or_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.or_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.or_neg_limb_pos(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_or_pos_neg(xs, ys))
            }
        }
    }

    fn or_assign_neg_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => *self = other.or_neg_limb_neg(x.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_or_neg_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn or_assign_neg_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => {
                other.or_assign_neg_limb_neg(x.wrapping_neg());
                *self = other;
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_or_neg_neg_in_place_either(xs, ys) {
                    *self = other;
                }
                self.trim();
            }
        }
    }

    fn or_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.or_neg_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.or_neg_limb_neg(x.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_or_neg_neg(xs, ys))
            }
        }
    }
}

impl BitOr<Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise or of two [`Integer`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x \vee y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-123) | Integer::from(-456), -67);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) | -(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -999999995905i64
    /// );
    /// ```
    #[inline]
    fn bitor(mut self, other: Integer) -> Integer {
        self |= other;
        self
    }
}

impl<'a> BitOr<&'a Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise or of two [`Integer`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = x \vee y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-123) | &Integer::from(-456), -67);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) | &-(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -999999995905i64
    /// );
    /// ```
    #[inline]
    fn bitor(mut self, other: &'a Integer) -> Integer {
        self |= other;
        self
    }
}

impl BitOr<Integer> for &Integer {
    type Output = Integer;

    /// Takes the bitwise or of two [`Integer`]s, taking the first by reference and the second by
    /// value.
    ///
    /// $$
    /// f(x, y) = x \vee y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::from(-123) | Integer::from(-456), -67);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) | -(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -999999995905i64
    /// );
    /// ```
    #[inline]
    fn bitor(self, mut other: Integer) -> Integer {
        other |= self;
        other
    }
}

impl BitOr<&Integer> for &Integer {
    type Output = Integer;

    /// Takes the bitwise or of two [`Integer`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x \vee y.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::from(-123) | &Integer::from(-456), -67);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) | &-(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -999999995905i64
    /// );
    /// ```
    fn bitor(self, other: &Integer) -> Integer {
        match (self.sign, other.sign) {
            (true, true) => Integer {
                sign: true,
                abs: &self.abs | &other.abs,
            },
            (true, false) => Integer {
                sign: false,
                abs: self.abs.or_pos_neg(&other.abs),
            },
            (false, true) => Integer {
                sign: false,
                abs: other.abs.or_pos_neg(&self.abs),
            },
            (false, false) => Integer {
                sign: false,
                abs: self.abs.or_neg_neg(&other.abs),
            },
        }
    }
}

impl BitOrAssign<Integer> for Integer {
    /// Bitwise-ors an [`Integer`] with another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x |= Integer::from(0x0000000f);
    /// x |= Integer::from(0x00000f00);
    /// x |= Integer::from(0x000f_0000);
    /// x |= Integer::from(0x0f000000);
    /// assert_eq!(x, 0x0f0f_0f0f);
    /// ```
    fn bitor_assign(&mut self, other: Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitor_assign(other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.or_assign_pos_neg(other.abs);
            }
            (false, true) => self.abs.or_assign_neg_pos(other.abs),
            (false, false) => self.abs.or_assign_neg_neg(other.abs),
        }
    }
}

impl<'a> BitOrAssign<&'a Integer> for Integer {
    /// Bitwise-ors an [`Integer`] with another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x |= &Integer::from(0x0000000f);
    /// x |= &Integer::from(0x00000f00);
    /// x |= &Integer::from(0x000f_0000);
    /// x |= &Integer::from(0x0f000000);
    /// assert_eq!(x, 0x0f0f_0f0f);
    /// ```
    fn bitor_assign(&mut self, other: &'a Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitor_assign(&other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.or_assign_pos_neg_ref(&other.abs);
            }
            (false, true) => self.abs.or_assign_neg_pos_ref(&other.abs),
            (false, false) => self.abs.or_assign_neg_neg_ref(&other.abs),
        }
    }
}
