// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993, 1994, 1996, 1997, 2000, 2001, 2005, 2012, 2015-2018 Free Software
//      Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::add::{
    limbs_add_limb, limbs_add_limb_to_out, limbs_slice_add_limb_in_place,
};
use crate::natural::arithmetic::sub::{
    limbs_sub, limbs_sub_greater_in_place_left, limbs_sub_greater_to_out, limbs_sub_limb,
    limbs_sub_limb_in_place, limbs_sub_limb_to_out, limbs_vec_sub_in_place_right,
};
use crate::natural::logic::not::limbs_not_in_place;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::{max, Ordering::*};
use core::ops::{BitXor, BitXorAssign};
use itertools::repeat_n;
use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero, slice_test_zero};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a `Limb`. `xs` cannot be
// empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_neg_xor_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    if y == 0 {
        return xs.to_vec();
    }
    let head = xs[0];
    let tail = &xs[1..];
    let mut out = Vec::with_capacity(xs.len());
    if head != 0 {
        let head = head.wrapping_neg() ^ y;
        if head == 0 {
            out.push(0);
            out.extend_from_slice(&limbs_add_limb(tail, 1));
        } else {
            out.push(head.wrapping_neg());
            out.extend_from_slice(tail);
        }
    } else {
        out.push(y.wrapping_neg());
        out.extend_from_slice(&limbs_sub_limb(tail, 1).0);
    }
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, writes the limbs of the bitwise and of the `Integer`, writes the limbs of the bitwise
// xor of the `Integer` and a `Limb` to an output slice. The output slice must be at least as long
// as the input slice. `xs` cannot be empty or only contain zeros. Returns whether a carry occurs.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_neg_xor_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) -> bool {
    let len = xs.len();
    assert!(out.len() >= len);
    if y == 0 {
        out[..len].copy_from_slice(xs);
        return false;
    }
    let head = xs[0];
    let tail = &xs[1..];
    if head != 0 {
        let head = head.wrapping_neg() ^ y;
        if head == 0 {
            out[0] = 0;
            limbs_add_limb_to_out(&mut out[1..len], tail, 1)
        } else {
            out[0] = head.wrapping_neg();
            out[1..len].copy_from_slice(tail);
            false
        }
    } else {
        out[0] = y.wrapping_neg();
        limbs_sub_limb_to_out(&mut out[1..len], tail, 1);
        false
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a `Limb` to the input slice.
// `xs` cannot be empty or only contain zeros. Returns whether a carry occurs.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_slice_neg_xor_limb_in_place(xs: &mut [Limb], y: Limb) -> bool {
    if y == 0 {
        return false;
    }
    let (head, tail) = xs.split_at_mut(1);
    let head = &mut head[0];
    if *head != 0 {
        *head = head.wrapping_neg() ^ y;
        if *head == 0 {
            limbs_slice_add_limb_in_place(tail, 1)
        } else {
            head.wrapping_neg_assign();
            false
        }
    } else {
        *head = y.wrapping_neg();
        limbs_sub_limb_in_place(tail, 1);
        false
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a `Limb` to the input slice.
// `xs` cannot be empty or only contain zeros. If a carry occurs, extends the `Vec`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_vec_neg_xor_limb_in_place(xs: &mut Vec<Limb>, y: Limb) {
    if limbs_slice_neg_xor_limb_in_place(xs, y) {
        xs.push(1);
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by `y`
// and whose other limbs are full of `true` bits. `xs` may not be empty.
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
pub_test! {limbs_pos_xor_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let (head, tail) = xs.split_first().unwrap();
    let lo = head ^ y;
    let mut out;
    if lo == 0 {
        out = limbs_add_limb(tail, 1);
        out.insert(0, 0);
    } else {
        out = xs.to_vec();
        out[0] = lo.wrapping_neg();
    }
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, writes the
// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by `y`
// and whose other limbs are full of `true` bits to an output slice. `xs` may not be empty or only
// contain zeros. The output slice must be at least as long as the input slice. Returns whether
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
// Panics if `xs` is empty or if `out` is shorter than `xs`.
pub_test! {limbs_pos_xor_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) -> bool {
    let (head, tail) = xs.split_first().unwrap();
    let (out_head, out_tail) = out[..xs.len()].split_first_mut().unwrap();
    let lo = head ^ y;
    if lo == 0 {
        *out_head = 0;
        limbs_add_limb_to_out(out_tail, tail, 1)
    } else {
        *out_head = lo.wrapping_neg();
        out_tail.copy_from_slice(tail);
        false
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, takes the
// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `y` and whose
// other limbs are full of `true` bits, in place. `xs` may not be empty. Returns whether there is a
// carry.
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
pub_test! {limbs_slice_pos_xor_limb_neg_in_place(xs: &mut [Limb], y: Limb) -> bool {
    let (head, tail) = xs.split_at_mut(1);
    let head = &mut head[0];
    *head ^= y;
    if *head == 0 {
        limbs_slice_add_limb_in_place(tail, 1)
    } else {
        *head = head.wrapping_neg();
        false
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of an `Integer`, takes the
// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `y` and whose
// other limbs are full of `true` bits, in place. `xs` may not be empty.
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
pub_test! {limbs_vec_pos_xor_limb_neg_in_place(xs: &mut Vec<Limb>, y: Limb) {
    if limbs_slice_pos_xor_limb_neg_in_place(xs, y) {
        xs.push(1);
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a negative number whose
// lowest limb is given by `y` and whose other limbs are full of `true` bits. `xs` may not be empty
// or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty or only contains zeros.
pub_test! {limbs_neg_xor_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut out;
    if xs[0] == 0 {
        let carry;
        (out, carry) = limbs_sub_limb(xs, 1);
        assert!(!carry);
        out[0] = y;
    } else {
        out = xs.to_vec();
        out[0] = xs[0].wrapping_neg() ^ y;
    }
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a negative number whose
// lowest limb is given by `y` and whose other limbs are full of `true` bits to an output slice.
// `xs` may not be empty or only contain zeros. The output slice must be at least as long as the
// input slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty or only contains zeros, or if `out` is shorter than `xs`.
pub_test! {limbs_neg_xor_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    let (head, tail) = xs.split_first().unwrap();
    let (out_head, out_tail) = out[..xs.len()].split_first_mut().unwrap();
    if *head == 0 {
        *out_head = y;
        assert!(!limbs_sub_limb_to_out(out_tail, tail, 1));
    } else {
        *out_head = xs[0].wrapping_neg() ^ y;
        out_tail.copy_from_slice(tail);
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, takes the bitwise xor of the `Integer` and a negative number whose lowest limb is
// given by `y` and whose other limbs are full of `true` bits, in place. `xs` may not be empty or
// only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is empty or only contains zeros.
pub_test! {limbs_neg_xor_limb_neg_in_place(xs: &mut [Limb], y: Limb) {
    let (head, tail) = xs.split_first_mut().unwrap();
    if *head == 0 {
        assert!(!limbs_sub_limb_in_place(tail, 1));
        *head = y;
    } else {
        head.wrapping_neg_assign();
        *head ^= y;
    }
}}

fn limbs_xor_pos_neg_helper(x: Limb, boundary_seen: &mut bool) -> Limb {
    if *boundary_seen {
        !x
    } else if x == 0 {
        0
    } else {
        *boundary_seen = true;
        x.wrapping_neg()
    }
}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, returns the limbs of the bitwise xor of the `Integer`s. `xs` and `ys` may
// not be empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where `res` is returned, the first
// input is positive, and the second is negative.
pub_test! {limbs_xor_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
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
        return out;
    } else if x_i >= ys_len {
        let mut out = ys.to_vec();
        out.extend_from_slice(&xs[ys_len..]);
        return out;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut out = vec![0; min_i];
    let mut boundary_seen = false;
    let x = match x_i.cmp(&y_i) {
        Equal => {
            limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen)
        }
        Less => {
            boundary_seen = true;
            out.push(xs[x_i].wrapping_neg());
            out.extend(xs[x_i + 1..y_i].iter().map(|x| !x));
            xs[y_i] ^ (ys[y_i] - 1)
        }
        Greater => {
            boundary_seen = true;
            out.extend_from_slice(&ys[y_i..x_i]);
            xs[x_i] ^ ys[x_i]
        }
    };
    out.push(x);
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter());
    if boundary_seen {
        out.extend(xys.map(|(x, y)| x ^ y));
    } else {
        for (&x, &y) in xys {
            out.push(limbs_xor_pos_neg_helper(x ^ !y, &mut boundary_seen));
        }
    }
    if xs_len != ys_len {
        let zs = if xs_len > ys_len {
            &xs[ys_len..]
        } else {
            &ys[xs_len..]
        };
        if boundary_seen {
            out.extend_from_slice(zs);
        } else {
            for &z in zs {
                out.push(limbs_xor_pos_neg_helper(!z, &mut boundary_seen));
            }
        }
    }
    if slice_test_zero(&out) {
        out.push(1);
    }
    out
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise xor of the `Integer`s to an output slice.
// `xs` and `ys` may not be empty or only contain zeros. The output slice must be at least as long
// as the longer of the two input slices. max(`xs.len()`, `ys.len()`) limbs will be written; if the
// number of significant limbs of the result is lower, some of the written limbs will be zero.
//
// Returns whether there is a carry.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than the longer of
// `xs` and `ys`.
//
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where the first input is positive
// and the second is negative.
pub_test! {limbs_xor_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    assert!(out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        slice_set_zero(&mut out[..x_i]);
        out[x_i] = xs[x_i].wrapping_neg();
        for (out, &x) in out[x_i + 1..xs_len].iter_mut().zip(xs[x_i + 1..].iter()) {
            *out = !x;
        }
        for out in &mut out[xs_len..y_i] {
            *out = Limb::MAX;
        }
        out[y_i] = ys[y_i] - 1;
        out[y_i + 1..ys_len].copy_from_slice(&ys[y_i + 1..]);
        return false;
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
        return false;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    slice_set_zero(&mut out[..min_i]);
    let mut boundary_seen = false;
    match x_i.cmp(&y_i) {
        Equal => {
            out[x_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen);
        }
        Less => {
            boundary_seen = true;
            out[x_i] = xs[x_i].wrapping_neg();
            for (out, &x) in out[x_i + 1..y_i].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
                *out = !x;
            }
            out[y_i] = xs[y_i] ^ (ys[y_i] - 1);
        }
        Greater => {
            boundary_seen = true;
            out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            out[x_i] = xs[x_i] ^ ys[x_i];
        }
    }
    let xys = out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()));
    if boundary_seen {
        for (out, (&x, &y)) in xys {
            *out = x ^ y;
        }
    } else {
        for (out, (&x, &y)) in xys {
            *out = limbs_xor_pos_neg_helper(x ^ !y, &mut boundary_seen);
        }
    }
    let max_len = max(xs_len, ys_len);
    if xs_len != ys_len {
        let (min_len, zs) = if max_len == xs_len {
            (ys_len, &xs[ys_len..])
        } else {
            (xs_len, &ys[xs_len..])
        };
        if boundary_seen {
            out[min_len..max_len].copy_from_slice(zs);
        } else {
            for (out, &z) in out[min_len..].iter_mut().zip(zs.iter()) {
                *out = limbs_xor_pos_neg_helper(!z, &mut boundary_seen);
            }
        }
    }
    slice_test_zero(&out[..max_len])
}}

fn limbs_xor_pos_neg_in_place_left_helper(
    xs: &mut [Limb],
    ys: &[Limb],
    x_i: usize,
    y_i: usize,
) -> bool {
    let max_i = max(x_i, y_i);
    let mut boundary_seen = false;
    match x_i.cmp(&y_i) {
        Equal => {
            xs[x_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen);
        }
        Less => {
            boundary_seen = true;
            xs[x_i].wrapping_neg_assign();
            limbs_not_in_place(&mut xs[x_i + 1..y_i]);
            xs[y_i] ^= ys[y_i] - 1;
        }
        Greater => {
            boundary_seen = true;
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            xs[x_i] ^= ys[x_i];
        }
    }
    let xys = xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter());
    if boundary_seen {
        for (x, &y) in xys {
            *x ^= y;
        }
    } else {
        for (x, &y) in xys {
            *x = limbs_xor_pos_neg_helper(*x ^ !y, &mut boundary_seen);
        }
    }
    boundary_seen
}

// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of one
// `Integer` and the negative of another, writes the limbs of the bitwise xor of the `Integer`s to
// the `Vec`. `xs` and `ys` may not be empty or only contain zeros.
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
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where `res == op1` and the first
// input is positive and the second is negative.
pub_test! {limbs_xor_pos_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
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
        return;
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        return;
    }
    let mut boundary_seen = limbs_xor_pos_neg_in_place_left_helper(xs, ys, x_i, y_i);
    match xs_len.cmp(&ys_len) {
        Less => {
            if boundary_seen {
                xs.extend_from_slice(&ys[xs_len..]);
            } else {
                for &y in &ys[xs_len..] {
                    xs.push(limbs_xor_pos_neg_helper(!y, &mut boundary_seen));
                }
            }
        }
        Greater => {
            if !boundary_seen {
                for x in &mut xs[ys_len..] {
                    *x = limbs_xor_pos_neg_helper(!*x, &mut boundary_seen);
                }
            }
        }
        _ => {}
    }
    if slice_test_zero(xs) {
        xs.push(1);
    }
}}

fn limbs_xor_pos_neg_in_place_right_helper(
    xs: &[Limb],
    ys: &mut [Limb],
    x_i: usize,
    y_i: usize,
) -> bool {
    let max_i = max(x_i, y_i);
    let mut boundary_seen = false;
    match x_i.cmp(&y_i) {
        Equal => {
            ys[y_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen);
        }
        Less => {
            boundary_seen = true;
            ys[x_i] = xs[x_i].wrapping_neg();
            for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
                *y = !x;
            }
            ys[y_i] -= 1;
            ys[y_i] ^= xs[y_i];
        }
        Greater => {
            boundary_seen = true;
            ys[x_i] ^= xs[x_i];
        }
    }
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter_mut());
    if boundary_seen {
        for (&x, y) in xys {
            *y ^= x;
        }
    } else {
        for (&x, y) in xys {
            *y = limbs_xor_pos_neg_helper(x ^ !*y, &mut boundary_seen);
        }
    }
    boundary_seen
}

// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of one
// `Integer` and the negative of another, writes the limbs of the bitwise xor of the `Integer`s to
// the second (right) slice. `xs` and `ys` may not be empty or only contain zeros.
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
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where `res == op2` and the first
// input is positive and the second is negative.
pub_test! {limbs_xor_pos_neg_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        ys[x_i] = xs[x_i].wrapping_neg();
        for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..].iter()) {
            *y = !x;
        }
        for y in ys.iter_mut().take(y_i).skip(xs_len) {
            *y = Limb::MAX;
        }
        ys[y_i] -= 1;
        return;
    } else if x_i >= ys_len {
        ys.extend_from_slice(&xs[ys_len..]);
        return;
    }
    let mut boundary_seen = limbs_xor_pos_neg_in_place_right_helper(xs, ys, x_i, y_i);
    if xs_len > ys_len {
        if boundary_seen {
            ys.extend_from_slice(&xs[ys_len..]);
        } else {
            for &x in &xs[ys_len..] {
                ys.push(limbs_xor_pos_neg_helper(!x, &mut boundary_seen));
            }
        }
    } else if xs_len < ys_len && !boundary_seen {
        for y in &mut ys[xs_len..] {
            *y = limbs_xor_pos_neg_helper(!*y, &mut boundary_seen);
        }
    }
    if slice_test_zero(ys) {
        ys.push(1);
    }
}}

// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise xor of the `Integer`s to the longer `Vec`
// (or the first one, if they are equally long). `xs` and `ys` may not be empty or only contain
// zeros. Returns a `bool` which is `false` when the output is to the first `Vec` and `true` when
// it's to the second `Vec`.
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
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where the first input is positive,
// the second is negative, and the result is written to the longer input slice.
pub_test! {limbs_xor_pos_neg_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        ys[x_i] = xs[x_i].wrapping_neg();
        for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..].iter()) {
            *y = !x;
        }
        for y in &mut ys[xs_len..y_i] {
            *y = Limb::MAX;
        }
        ys[y_i] -= 1;
        return true;
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        return false;
    }
    if xs_len >= ys_len {
        let mut boundary_seen = limbs_xor_pos_neg_in_place_left_helper(xs, ys, x_i, y_i);
        if xs_len != ys_len && !boundary_seen {
            for x in &mut xs[ys_len..] {
                *x = limbs_xor_pos_neg_helper(!*x, &mut boundary_seen);
            }
        }
        if slice_test_zero(xs) {
            xs.push(1);
        }
        false
    } else {
        let mut boundary_seen = limbs_xor_pos_neg_in_place_right_helper(xs, ys, x_i, y_i);
        if !boundary_seen {
            for y in &mut ys[xs_len..] {
                *y = limbs_xor_pos_neg_helper(!*y, &mut boundary_seen);
            }
        }
        if slice_test_zero(ys) {
            ys.push(1);
        }
        true
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, returns the limbs of the bitwise xor of the `Integer`s. `xs` and `ys` may not be
// empty or only contain zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros.
//
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where `res` is returned and both
// inputs are negative.
pub_test! {limbs_xor_neg_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        let (result, borrow) = limbs_sub(ys, xs);
        assert!(!borrow);
        return result;
    } else if x_i >= ys_len {
        let (result, borrow) = limbs_sub(xs, ys);
        assert!(!borrow);
        return result;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut out = vec![0; min_i];
    if x_i == y_i {
        out.push(xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg());
    } else {
        let (min_zs, max_zs) = if x_i <= y_i { (xs, ys) } else { (ys, xs) };
        out.push(min_zs[min_i].wrapping_neg());
        out.extend(min_zs[min_i + 1..max_i].iter().map(|z| !z));
        out.push((max_zs[max_i] - 1) ^ min_zs[max_i]);
    }
    out.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(x, y)| x ^ y),
    );
    match xs_len.cmp(&ys_len) {
        Less => out.extend_from_slice(&ys[xs_len..]),
        Greater => out.extend_from_slice(&xs[ys_len..]),
        _ => {}
    }
    out
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the max(`xs.len()`, `ys.len()`) limbs of the bitwise xor of the `Integer`s to
// an output slice. `xs` and `ys` may not be empty or only contain zeros. The output slice must be
// at least as long as the longer input slice.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than the longer of
// `xs` and `ys`.
//
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where both inputs are negative.
pub_test! {limbs_xor_neg_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    assert!(out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_sub_greater_to_out(out, ys, xs));
        return;
    } else if x_i >= ys_len {
        assert!(!limbs_sub_greater_to_out(out, xs, ys));
        return;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    slice_set_zero(&mut out[..min_i]);
    if x_i == y_i {
        out[x_i] = xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg();
    } else {
        let (min_zs, max_zs) = if x_i <= y_i { (xs, ys) } else { (ys, xs) };
        out[min_i] = min_zs[min_i].wrapping_neg();
        for (out, &z) in out[min_i + 1..max_i]
            .iter_mut()
            .zip(min_zs[min_i + 1..max_i].iter())
        {
            *out = !z;
        }
        out[max_i] = (max_zs[max_i] - 1) ^ min_zs[max_i];
    }
    for (out, (&x, &y)) in out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
    {
        *out = x ^ y;
    }
    match xs_len.cmp(&ys_len) {
        Less => out[xs_len..ys_len].copy_from_slice(&ys[xs_len..]),
        Greater => out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]),
        _ => {}
    }
}}

fn limbs_xor_neg_neg_in_place_helper(xs: &mut [Limb], ys: &[Limb], x_i: usize, y_i: usize) {
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    if x_i == y_i {
        xs[x_i] = xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg();
    } else if x_i <= y_i {
        xs[min_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[min_i + 1..max_i]);
        xs[max_i] ^= ys[max_i] - 1;
    } else {
        xs[min_i] = ys[min_i].wrapping_neg();
        for (x, &y) in xs[min_i + 1..max_i].iter_mut().zip(ys[min_i + 1..].iter()) {
            *x = !y;
        }
        xs[max_i] -= 1;
        xs[max_i] ^= ys[max_i];
    }
    for (x, &y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
        *x ^= y;
    }
}

// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of the
// negatives of two `Integer`s, writes the limbs of the bitwise xor of the `Integer`s to the `Vec`.
// `xs` and `ys` may not be empty or only contain zeros.
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
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where `res == op1` and both inputs
// are negative.
pub_test! {limbs_xor_neg_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_vec_sub_in_place_right(ys, xs));
    } else if x_i >= ys_len {
        assert!(!limbs_sub_greater_in_place_left(xs, ys));
    } else {
        limbs_xor_neg_neg_in_place_helper(xs, ys, x_i, y_i);
        if xs_len < ys_len {
            xs.extend_from_slice(&ys[xs_len..]);
        }
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the limbs of the bitwise xor of the `Integer`s to the longer slice (or the
// first one, if they are equally long). `xs` and `ys` may not be empty or only contain zeros.
// Returns `false` when the output is to the first slice and `true` when it's to the second slice.
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
// This is equivalent to `mpz_xor` from `mpz/xor.c`, GMP 6.2.1, where both inputs are negative and
// the result is written to the longer input slice.
pub_test! {limbs_xor_neg_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_sub_greater_in_place_left(ys, xs));
        true
    } else if x_i >= ys_len {
        assert!(!limbs_sub_greater_in_place_left(xs, ys));
        false
    } else if xs_len >= ys_len {
        limbs_xor_neg_neg_in_place_helper(xs, ys, x_i, y_i);
        false
    } else {
        limbs_xor_neg_neg_in_place_helper(ys, xs, y_i, x_i);
        true
    }
}}

impl Natural {
    fn xor_assign_neg_limb_pos(&mut self, other: Limb) {
        match self {
            &mut Natural::ZERO => {}
            Natural(Small(ref mut small)) => {
                let result = small.wrapping_neg() ^ other;
                if result == 0 {
                    *self = Natural(Large(vec![0, 1]));
                } else {
                    *small = result.wrapping_neg();
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_neg_xor_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }

    fn xor_neg_limb_pos(&self, other: Limb) -> Natural {
        match *self {
            Natural::ZERO => self.clone(),
            Natural(Small(ref small)) => {
                let result = small.wrapping_neg() ^ other;
                Natural(if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                })
            }
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_neg_xor_limb(limbs, other))
            }
        }
    }

    fn xor_assign_pos_limb_neg(&mut self, other: Limb) {
        match self {
            Natural(Small(ref mut small)) => {
                let result = *small ^ other;
                if result == 0 {
                    *self = Natural(Large(vec![0, 1]));
                } else {
                    *small = result.wrapping_neg();
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_pos_xor_limb_neg_in_place(limbs, other);
                self.trim();
            }
        }
    }

    fn xor_pos_limb_neg(&self, other: Limb) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => {
                let result = small ^ other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Natural(Large(ref limbs)) => Large(limbs_pos_xor_limb_neg(limbs, other)),
        })
    }

    fn xor_assign_neg_limb_neg(&mut self, other: Limb) {
        match *self {
            Natural(Small(ref mut small)) => *small = small.wrapping_neg() ^ other,
            Natural(Large(ref mut limbs)) => {
                limbs_neg_xor_limb_neg_in_place(limbs, other);
                self.trim();
            }
        }
    }

    fn xor_neg_limb_neg(&self, other: Limb) -> Natural {
        match *self {
            Natural(Small(small)) => Natural(Small(small.wrapping_neg() ^ other)),
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_neg_xor_limb_neg(limbs, other))
            }
        }
    }

    fn xor_assign_pos_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (Natural(Small(x)), _) => {
                other.xor_assign_neg_limb_pos(*x);
                *self = other;
            }
            (_, Natural(Small(y))) => self.xor_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ys))) => {
                if limbs_xor_pos_neg_in_place_either(xs, ys) {
                    *self = other;
                }
                self.trim();
            }
        }
    }

    fn xor_assign_pos_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.xor_neg_limb_pos(*x),
            (_, Natural(Small(y))) => self.xor_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_xor_pos_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn xor_assign_neg_pos(&mut self, mut other: Natural) {
        other.xor_assign_pos_neg_ref(&*self);
        *self = other;
    }

    fn xor_assign_neg_pos_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.xor_pos_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.xor_assign_neg_limb_pos(*y),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_xor_pos_neg_in_place_right(ys, xs);
                self.trim();
            }
        }
    }

    fn xor_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (&Natural(Small(x)), _) => other.xor_neg_limb_pos(x),
            (_, &Natural(Small(y))) => self.xor_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_xor_pos_neg(xs, ys))
            }
        }
    }

    fn xor_assign_neg_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (Natural(Small(x)), _) => *self = other.xor_neg_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.xor_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_xor_neg_neg_in_place_either(xs, ys) {
                    *self = other;
                }
                self.trim();
            }
        }
    }

    fn xor_assign_neg_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.xor_neg_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.xor_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_xor_neg_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn xor_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (&Natural(Small(x)), _) => other.xor_neg_limb_neg(x.wrapping_neg()),
            (_, &Natural(Small(y))) => self.xor_neg_limb_neg(y.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_xor_neg_neg(xs, ys))
            }
        }
    }
}

impl BitXor<Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise xor of two [`Integer`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = x \oplus y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
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
    /// assert_eq!(Integer::from(-123) ^ Integer::from(-456), 445);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) ^ -(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     8191
    /// );
    /// ```
    #[inline]
    fn bitxor(mut self, other: Integer) -> Integer {
        self ^= other;
        self
    }
}

impl BitXor<&Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise xor of two [`Integer`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = x \oplus y.
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
    /// assert_eq!(Integer::from(-123) ^ &Integer::from(-456), 445);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) ^ &-(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     8191
    /// );
    /// ```
    #[inline]
    fn bitxor(mut self, other: &Integer) -> Integer {
        self ^= other;
        self
    }
}

impl BitXor<Integer> for &Integer {
    type Output = Integer;

    /// Takes the bitwise xor of two [`Integer`]s, taking the first by reference and the second by
    /// value.
    ///
    /// $$
    /// f(x, y) = x \oplus y.
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
    /// assert_eq!(&Integer::from(-123) ^ Integer::from(-456), 445);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) ^ -(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     8191
    /// );
    /// ```
    #[inline]
    fn bitxor(self, mut other: Integer) -> Integer {
        other ^= self;
        other
    }
}

impl BitXor<&Integer> for &Integer {
    type Output = Integer;

    /// Takes the bitwise xor of two [`Integer`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = x \oplus y.
    /// $$
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
    /// assert_eq!(&Integer::from(-123) ^ &Integer::from(-456), 445);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) ^ &-(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     8191
    /// );
    /// ```
    fn bitxor(self, other: &Integer) -> Integer {
        match (self.sign, other.sign) {
            (true, true) => Integer {
                sign: true,
                abs: &self.abs ^ &other.abs,
            },
            (true, false) => Integer {
                sign: false,
                abs: self.abs.xor_pos_neg(&other.abs),
            },
            (false, true) => Integer {
                sign: false,
                abs: other.abs.xor_pos_neg(&self.abs),
            },
            (false, false) => Integer {
                sign: true,
                abs: self.abs.xor_neg_neg(&other.abs),
            },
        }
    }
}

impl BitXorAssign<Integer> for Integer {
    /// Bitwise-xors an [`Integer`] with another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by value.
    ///
    /// $$
    /// x \gets x \oplus y.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(u32::MAX);
    /// x ^= Integer::from(0x0000000f);
    /// x ^= Integer::from(0x00000f00);
    /// x ^= Integer::from(0x000f_0000);
    /// x ^= Integer::from(0x0f000000);
    /// assert_eq!(x, 0xf0f0_f0f0u32);
    /// ```
    fn bitxor_assign(&mut self, other: Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitxor_assign(other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.xor_assign_pos_neg(other.abs);
            }
            (false, true) => self.abs.xor_assign_neg_pos(other.abs),
            (false, false) => {
                self.sign = true;
                self.abs.xor_assign_neg_neg(other.abs);
            }
        }
    }
}

impl BitXorAssign<&Integer> for Integer {
    /// Bitwise-xors an [`Integer`] with another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets x \oplus y.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(u32::MAX);
    /// x ^= &Integer::from(0x0000000f);
    /// x ^= &Integer::from(0x00000f00);
    /// x ^= &Integer::from(0x000f_0000);
    /// x ^= &Integer::from(0x0f000000);
    /// assert_eq!(x, 0xf0f0_f0f0u32);
    /// ```
    fn bitxor_assign(&mut self, other: &Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitxor_assign(&other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.xor_assign_pos_neg_ref(&other.abs);
            }
            (false, true) => self.abs.xor_assign_neg_pos_ref(&other.abs),
            (false, false) => {
                self.sign = true;
                self.abs.xor_assign_neg_neg_ref(&other.abs);
            }
        }
    }
}
