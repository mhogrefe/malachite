// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993, 1994, 1996, 1997, 2000, 2001, 2003, 2005, 2012, 2015-2018 Free
//      Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::add::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::{max, Ordering::*};
use core::ops::{BitAnd, BitAndAssign};
use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::NotAssign;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero};

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
// limbs of the bitwise and of the `Integer` and a negative number whose lowest limb is given by `y`
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
pub_test! {limbs_pos_and_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut out = xs.to_vec();
    out[0] &= y;
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, writes the
// limbs of the bitwise and of the `Integer` and a negative number whose lowest limb is given by `y`
// and whose other limbs are full of `true` bits, to an output slice. `xs` may not be empty. The
// output slice must be at least as long as the input slice.
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
pub_test! {limbs_pos_and_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    let len = xs.len();
    assert!(out.len() >= len);
    let (xs_head, xs_tail) = xs.split_first().unwrap();
    let (out_head, out_tail) = out[..len].split_first_mut().unwrap();
    *out_head = xs_head & y;
    out_tail.copy_from_slice(xs_tail);
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, writes the
// limbs of the bitwise and of the `Integer` and a negative number whose lowest limb is given by `y`
// and whose other limbs are full of `true` bits, to the input slice. `xs` may not be empty.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// # Panics
// Panics if `xs` is empty.
pub_test! {limbs_pos_and_limb_neg_in_place(xs: &mut [Limb], ys: Limb) {
    xs[0] &= ys;
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, returns the limbs of the bitwise and of the `Integer` and a negative number whose
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
// Panics if `xs` is empty.
pub_test! {limbs_neg_and_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut out = xs.to_vec();
    limbs_vec_neg_and_limb_neg_in_place(&mut out, y);
    out
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, writes the limbs of the bitwise and of the `Integer` and a negative number whose
// lowest limb is given by `y` and whose other limbs are full of `true` bits to an output slice.
// `xs` may not be empty or only contain zeros. Returns whether a carry occurs. The output slice
// must be at least as long as the input slice.
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
pub_test! {limbs_neg_and_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) -> bool {
    let out = &mut out[..xs.len()];
    if xs[0] == 0 {
        out.copy_from_slice(xs);
        false
    } else {
        let (xs_head, xs_tail) = xs.split_first().unwrap();
        let (out_head, out_tail) = out.split_first_mut().unwrap();
        let result_head = xs_head.wrapping_neg() & y;
        if result_head == 0 {
            *out_head = 0;
            limbs_add_limb_to_out(out_tail, xs_tail, 1)
        } else {
            *out_head = result_head.wrapping_neg();
            out_tail.copy_from_slice(xs_tail);
            false
        }
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, takes the bitwise and of the `Integer` and a negative number whose lowest limb is
// given by `y` and whose other limbs are full of `true` bits, in place. `xs` may not be empty or
// only contain zeros. Returns whether there is a carry.
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
pub_test! {limbs_slice_neg_and_limb_neg_in_place(xs: &mut [Limb], y: Limb) -> bool {
    let (xs_head, xs_tail) = xs.split_first_mut().unwrap();
    if *xs_head == 0 {
        false
    } else {
        *xs_head = xs_head.wrapping_neg() & y;
        if *xs_head == 0 {
            limbs_slice_add_limb_in_place(xs_tail, 1)
        } else {
            xs_head.wrapping_neg_assign();
            false
        }
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of the negative of an
// `Integer`, takes the bitwise and of the `Integer` and a negative number whose lowest limb is
// given by `y` and whose other limbs are full of `true` bits, in place. `xs` may not be empty or
// only contain zeros. If there is a carry, increases the length of the `Vec` by 1.
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
pub_test! {limbs_vec_neg_and_limb_neg_in_place(xs: &mut Vec<Limb>, y: Limb) {
    if limbs_slice_neg_and_limb_neg_in_place(xs, y) {
        xs.push(1);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, returns the limbs of the bitwise and of the `Integer`s. `xs` and `ys` may
// not be empty or only contain zeros.
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res` is returned, the first
// input is positive, and the second is negative.
pub_test! {limbs_and_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return Vec::new();
    } else if x_i >= ys_len {
        return xs.to_vec();
    }
    let max_i = max(x_i, y_i);
    let mut out = vec![0; max_i];
    out.push(
        xs[max_i]
            & if x_i <= y_i {
                ys[max_i].wrapping_neg()
            } else {
                !ys[max_i]
            },
    );
    out.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(&x, &y)| x & !y),
    );
    if xs_len > ys_len {
        out.extend_from_slice(&xs[ys_len..]);
    }
    out
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise and of the `Integer`s to an output slice.
// `xs` and `ys` may not be empty or only contain zeros. The output slice must be at least as long
// as the first input slice. `xs.len()` limbs will be written; if the number of significant limbs of
// the result is lower, some of the written limbs will be zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than `xs`.
//
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where the first input is positive
// and the second is negative.
pub_test! {limbs_and_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        slice_set_zero(&mut out[..xs_len]);
        return;
    } else if x_i >= ys_len {
        out[..xs_len].copy_from_slice(xs);
        return;
    }
    let max_i = max(x_i, y_i);
    slice_set_zero(&mut out[..max_i]);
    out[max_i] = xs[max_i]
        & if x_i <= y_i {
            ys[max_i].wrapping_neg()
        } else {
            !ys[max_i]
        };
    for (z, (x, y)) in out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
    {
        *z = x & !y;
    }
    if xs_len > ys_len {
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the limbs of the bitwise and of the `Integer`s to the first (left)
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res == op1`, the first input
// is positive, and the second is negative.
pub_test! {limbs_and_pos_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        slice_set_zero(xs);
        return;
    } else if x_i >= ys_len {
        return;
    }
    let max_i = max(x_i, y_i);
    slice_set_zero(&mut xs[..max_i]);
    xs[max_i] &= if x_i <= y_i {
        ys[max_i].wrapping_neg()
    } else {
        !ys[max_i]
    };
    for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
        *x &= !y;
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
// negative of another, writes the lowest min(`xs.len()`, `ys.len()`) limbs of the bitwise and of
// the `Integer`s to the second (right) slice. `xs` and `ys` may not be empty or only contain zeros.
// If `ys` is shorter than `xs`, the result may be too long to fit in `ys`. The extra limbs in this
// case are just `xs[ys.len()..]`.
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res == op2`, the first input
// is positive, the second is negative, and the length of `op2` is not changed; instead, a carry is
// returned.
pub_test! {limbs_slice_and_pos_neg_in_place_right(xs: &[Limb], ys: &mut [Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len || x_i >= ys_len {
        slice_set_zero(ys);
        return;
    }
    let max_i = max(x_i, y_i);
    slice_set_zero(&mut ys[..max_i]);
    {
        let ys_max_i = &mut ys[max_i];
        if x_i <= y_i {
            ys_max_i.wrapping_neg_assign();
        } else {
            ys_max_i.not_assign();
        }
        *ys_max_i &= xs[max_i];
    }
    for (x, y) in xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter_mut()) {
        *y = !*y & x;
    }
}}

// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of one
// `Integer` and the negative of another, writes the limbs of the bitwise and of the `Integer`s to
// the `Vec`. `xs` and `ys` may not be empty or only contain zeros.
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res == op2`, the first input
// is positive, and the second is negative.
pub_test! {limbs_vec_and_pos_neg_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>) {
    limbs_slice_and_pos_neg_in_place_right(xs, ys);
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Greater => {
            let ys_len = ys.len();
            ys.extend(xs[ys_len..].iter());
        }
        Less => {
            ys.truncate(xs_len);
        }
        _ => {}
    }
}}

fn limbs_and_neg_neg_helper(input: Limb, boundary_limb_seen: &mut bool) -> Limb {
    if *boundary_limb_seen {
        input
    } else {
        let result = input.wrapping_add(1);
        if result != 0 {
            *boundary_limb_seen = true;
        }
        result
    }
}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, returns the limbs of the bitwise and of the `Integer`s. `xs` and `ys` may not be
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res` is returned and both
// inputs are negative.
pub_test! {limbs_and_neg_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return ys.to_vec();
    } else if x_i >= ys_len {
        return xs.to_vec();
    }
    let max_i = max(x_i, y_i);
    let mut out = vec![0; max_i];
    let x = if x_i >= y_i {
        xs[max_i].wrapping_sub(1)
    } else {
        xs[max_i]
    };
    let y = if x_i <= y_i {
        ys[max_i].wrapping_sub(1)
    } else {
        ys[max_i]
    };
    let mut boundary_limb_seen = false;
    out.push(limbs_and_neg_neg_helper(x | y, &mut boundary_limb_seen));
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter());
    if boundary_limb_seen {
        out.extend(xys.map(|(&x, &y)| x | y));
    } else {
        for (&x, &y) in xys {
            out.push(limbs_and_neg_neg_helper(x | y, &mut boundary_limb_seen));
        }
    }
    if xs_len != ys_len {
        let zs = if xs_len > ys_len {
            &xs[ys_len..]
        } else {
            &ys[xs_len..]
        };
        if boundary_limb_seen {
            out.extend_from_slice(zs);
        } else {
            for &z in zs {
                out.push(limbs_and_neg_neg_helper(z, &mut boundary_limb_seen));
            }
        }
    }
    if !boundary_limb_seen {
        out.push(1);
    }
    out
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the max(`xs.len()`, `ys.len()`) limbs of the bitwise and of the `Integer`s to
// an output slice. `xs` and `ys` may not be empty or only contain zeros. Returns whether the
// least-significant max(`xs.len()`, `ys.len()`) limbs of the output are not all zero. The output
// slice must be at least as long as the longer input slice.
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where both inputs are negative.
pub_test! {limbs_and_neg_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        out[..ys_len].copy_from_slice(ys);
        if xs_len > ys_len {
            slice_set_zero(&mut out[ys_len..xs_len]);
        }
        return true;
    } else if x_i >= ys_len {
        out[..xs_len].copy_from_slice(xs);
        if ys_len > xs_len {
            slice_set_zero(&mut out[xs_len..ys_len]);
        }
        return true;
    }
    let max_i = max(x_i, y_i);
    slice_set_zero(&mut out[..max_i]);
    let x = if x_i >= y_i {
        xs[max_i].wrapping_sub(1)
    } else {
        xs[max_i]
    };
    let y = if x_i <= y_i {
        ys[max_i].wrapping_sub(1)
    } else {
        ys[max_i]
    };
    let mut boundary_limb_seen = false;
    out[max_i] = limbs_and_neg_neg_helper(x | y, &mut boundary_limb_seen);
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter());
    if boundary_limb_seen {
        for (z, (x, y)) in out[max_i + 1..].iter_mut().zip(xys) {
            *z = x | y;
        }
    } else {
        for (z, (x, y)) in out[max_i + 1..].iter_mut().zip(xys) {
            *z = limbs_and_neg_neg_helper(x | y, &mut boundary_limb_seen);
        }
    }
    let (xs, xs_len, ys_len) = if xs_len >= ys_len {
        (xs, xs_len, ys_len)
    } else {
        (ys, ys_len, xs_len)
    };
    if xs_len != ys_len {
        let zs = &xs[ys_len..];
        if boundary_limb_seen {
            out[ys_len..xs_len].copy_from_slice(zs);
        } else {
            for (z_out, &z_in) in out[ys_len..xs_len].iter_mut().zip(zs.iter()) {
                *z_out = limbs_and_neg_neg_helper(z_in, &mut boundary_limb_seen);
            }
        }
    }
    boundary_limb_seen
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the lower `xs.len()` limbs of the bitwise and of the `Integer`s to the first
// (left) slice. `xs` and `ys` may not be empty or only contain zeros, and `xs` must be at least as
// long as `ys`. Returns whether the least-significant `xs.len()` limbs of the output are not all
// zero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if `xs` or `ys` are empty or contain only zeros, or if `xs` is shorter than `ys`.
//
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res == op1`, both inputs are
// negative, and the length of `op1` is not changed; instead, a carry is returned.
pub_test! {limbs_slice_and_neg_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if x_i >= ys_len {
        return true;
    }
    let max_i = max(x_i, y_i);
    if y_i > x_i {
        slice_set_zero(&mut xs[x_i..y_i]);
    }
    let x = if x_i >= y_i {
        xs[max_i].wrapping_sub(1)
    } else {
        xs[max_i]
    };
    let y = if x_i <= y_i {
        ys[max_i].wrapping_sub(1)
    } else {
        ys[max_i]
    };
    let mut boundary_limb_seen = false;
    xs[max_i] = limbs_and_neg_neg_helper(x | y, &mut boundary_limb_seen);
    let xys = xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter());
    if boundary_limb_seen {
        for (x, &y) in xys {
            *x |= y;
        }
    } else {
        for (x, &y) in xys {
            *x = limbs_and_neg_neg_helper(*x | y, &mut boundary_limb_seen);
        }
    }
    if xs_len > ys_len && !boundary_limb_seen {
        for x in &mut xs[ys_len..] {
            *x = limbs_and_neg_neg_helper(*x, &mut boundary_limb_seen);
        }
    }
    boundary_limb_seen
}}

// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of the
// negatives of two `Integer`s, writes the limbs of the bitwise and of the `Integer`s to the `Vec`.
// `xs` and `ys` may not be empty or only contain zeros.
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where `res == op1` and both inputs
// are negative.
pub_test! {limbs_vec_and_neg_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let y_i = slice_leading_zeros(ys);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs.resize(ys_len, 0);
        xs.copy_from_slice(ys);
        return;
    }
    let boundary_limb_seen = if ys_len > xs_len {
        let mut boundary_limb_seen = limbs_slice_and_neg_neg_in_place_left(xs, &ys[..xs_len]);
        let zs = &ys[xs_len..];
        if boundary_limb_seen {
            xs.extend_from_slice(zs);
        } else {
            for &z in zs {
                xs.push(limbs_and_neg_neg_helper(z, &mut boundary_limb_seen));
            }
        }
        boundary_limb_seen
    } else {
        limbs_slice_and_neg_neg_in_place_left(xs, ys)
    };
    if !boundary_limb_seen {
        xs.push(1);
    }
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the lower max(`xs.len()`, `ys.len()`) limbs of the bitwise and of the
// `Integer`s to the longer slice (or the first one, if they are equally long). `xs` and `ys` may
// not be empty or only contain zeros. Returns a pair of `bool`s. The first is `false` when the
// output is to the first slice and `true` when it's to the second slice, and the second is whether
// the least-significant max(`xs.len()`, `ys.len()`) limbs of the output are not all zero.
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where both inputs are negative, the
// result is written to the longer input slice, and the length of `op1` is not changed; instead, a
// carry is returned.
pub_test! {limbs_slice_and_neg_neg_in_place_either(
    xs: &mut [Limb],
    ys: &mut [Limb]
) -> (bool, bool) {
    if xs.len() >= ys.len() {
        (false, limbs_slice_and_neg_neg_in_place_left(xs, ys))
    } else {
        (true, limbs_slice_and_neg_neg_in_place_left(ys, xs))
    }
}}

// Interpreting two `Vec`s of `Limb`s as the limbs (in ascending order) of the negatives of two
// `Integer`s, writes the limbs of the bitwise and of the `Integer`s to the longer `Vec` (or the
// first one, if they are equally long). `xs` and `ys` may not be empty or only contain zeros.
// Returns a `bool` which is `false` when the output is to the first slice and `true` when it's to
// the second slice.
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
// This is equivalent to `mpz_and` from `mpz/and.c`, GMP 6.2.1, where both inputs are negative and
// the result is written to the longer input slice.
pub_test! {limbs_vec_and_neg_neg_in_place_either(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>) -> bool {
    if xs.len() >= ys.len() {
        limbs_vec_and_neg_neg_in_place_left(xs, ys);
        false
    } else {
        limbs_vec_and_neg_neg_in_place_left(ys, xs);
        true
    }
}}

impl Natural {
    fn and_assign_pos_limb_neg(&mut self, other: Limb) {
        match *self {
            Natural(Small(ref mut small)) => *small &= other,
            Natural(Large(ref mut limbs)) => limbs_pos_and_limb_neg_in_place(limbs, other),
        }
    }

    fn and_pos_limb_neg(&self, other: Limb) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => Small(small & other),
            Natural(Large(ref limbs)) => Large(limbs_pos_and_limb_neg(limbs, other)),
        })
    }

    fn and_assign_neg_limb_neg(&mut self, other: Limb) {
        match *self {
            Natural::ZERO => {}
            Natural(Small(ref mut small)) => {
                let result = small.wrapping_neg() & other;
                if result == 0 {
                    *self = Natural(Large(vec![0, 1]));
                } else {
                    *small = result.wrapping_neg();
                }
            }
            Natural(Large(ref mut limbs)) => limbs_vec_neg_and_limb_neg_in_place(limbs, other),
        }
    }

    fn and_assign_pos_neg(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.and_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Small(ref mut x)), Natural(Large(ref ys))) => *x &= ys[0].wrapping_neg(),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_and_pos_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn and_assign_neg_pos(&mut self, mut other: Natural) {
        other.and_assign_pos_neg(self);
        *self = other;
    }

    fn and_assign_neg_pos_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), y) => *self = y.and_pos_limb_neg(x.wrapping_neg()),
            (Natural(Large(ref xs)), Natural(Small(y))) => {
                *self = Natural(Small(xs[0].wrapping_neg() & *y));
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_and_pos_neg_in_place_right(ys, xs);
                self.trim();
            }
        }
    }

    fn and_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.and_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), &Natural(Large(ref ys))) => {
                Natural(Small(x & ys[0].wrapping_neg()))
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_and_pos_neg(xs, ys))
            }
        }
    }

    fn and_neg_limb_neg(&self, other: Limb) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => {
                let result = small.wrapping_neg() & other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Natural(Large(ref limbs)) => Large(limbs_neg_and_limb_neg(limbs, other)),
        })
    }

    fn and_assign_neg_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (Natural(Small(x)), _) => *self = other.and_neg_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.and_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_vec_and_neg_neg_in_place_either(xs, ys) {
                    *self = other;
                }
                self.trim();
            }
        }
    }

    fn and_assign_neg_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.and_neg_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.and_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_and_neg_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn and_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.and_neg_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.and_neg_limb_neg(x.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_and_neg_neg(xs, ys))
            }
        }
    }
}

impl BitAnd<Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise and of two [`Integer`]s, taking both by value.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-123) & Integer::from(-456), -512);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) & -(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -1000000004096i64
    /// );
    /// ```
    #[inline]
    fn bitand(mut self, other: Integer) -> Integer {
        self &= other;
        self
    }
}

impl BitAnd<&Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise and of two [`Integer`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = x \wedge y.
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
    /// assert_eq!(Integer::from(-123) & &Integer::from(-456), -512);
    /// assert_eq!(
    ///     -Integer::from(10u32).pow(12) & &-(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -1000000004096i64
    /// );
    /// ```
    #[inline]
    fn bitand(mut self, other: &Integer) -> Integer {
        self &= other;
        self
    }
}

impl BitAnd<Integer> for &Integer {
    type Output = Integer;

    /// Takes the bitwise and of two [`Integer`]s, taking the first by reference and the seocnd by
    /// value.
    ///
    /// $$
    /// f(x, y) = x \wedge y.
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
    /// assert_eq!(&Integer::from(-123) & Integer::from(-456), -512);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) & -(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -1000000004096i64
    /// );
    /// ```
    #[inline]
    fn bitand(self, mut other: Integer) -> Integer {
        other &= self;
        other
    }
}

impl BitAnd<&Integer> for &Integer {
    type Output = Integer;

    /// Takes the bitwise and of two [`Integer`]s, taking both by reference.
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(&Integer::from(-123) & &Integer::from(-456), -512);
    /// assert_eq!(
    ///     &-Integer::from(10u32).pow(12) & &-(Integer::from(10u32).pow(12) + Integer::ONE),
    ///     -1000000004096i64
    /// );
    /// ```
    fn bitand(self, other: &Integer) -> Integer {
        match (self.sign, other.sign) {
            (true, true) => Integer {
                sign: true,
                abs: &self.abs & &other.abs,
            },
            (true, false) => Integer {
                sign: true,
                abs: self.abs.and_pos_neg(&other.abs),
            },
            (false, true) => Integer {
                sign: true,
                abs: other.abs.and_pos_neg(&self.abs),
            },
            (false, false) => Integer {
                sign: false,
                abs: self.abs.and_neg_neg(&other.abs),
            },
        }
    }
}

impl BitAndAssign<Integer> for Integer {
    /// Bitwise-ands an [`Integer`] with another [`Integer`] in place, taking the [`Integer`] on the
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::NEGATIVE_ONE;
    /// x &= Integer::from(0x70ffffff);
    /// x &= Integer::from(0x7ff0_ffff);
    /// x &= Integer::from(0x7ffff0ff);
    /// x &= Integer::from(0x7ffffff0);
    /// assert_eq!(x, 0x70f0f0f0);
    /// ```
    fn bitand_assign(&mut self, other: Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitand_assign(other.abs),
            (true, false) => self.abs.and_assign_pos_neg(&other.abs),
            (false, true) => {
                self.sign = true;
                self.abs.and_assign_neg_pos(other.abs);
            }
            (false, false) => self.abs.and_assign_neg_neg(other.abs),
        }
    }
}

impl BitAndAssign<&Integer> for Integer {
    /// Bitwise-ands an [`Integer`] with another [`Integer`] in place, taking the [`Integer`] on the
    /// right-hand side by reference.
    ///
    /// $$
    /// x \gets x \wedge y.
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
    /// use malachite_base::num::basic::traits::NegativeOne;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::NEGATIVE_ONE;
    /// x &= &Integer::from(0x70ffffff);
    /// x &= &Integer::from(0x7ff0_ffff);
    /// x &= &Integer::from(0x7ffff0ff);
    /// x &= &Integer::from(0x7ffffff0);
    /// assert_eq!(x, 0x70f0f0f0);
    /// ```
    fn bitand_assign(&mut self, other: &Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitand_assign(&other.abs),
            (true, false) => self.abs.and_assign_pos_neg(&other.abs),
            (false, true) => {
                self.sign = true;
                self.abs.and_assign_neg_pos_ref(&other.abs);
            }
            (false, false) => self.abs.and_assign_neg_neg_ref(&other.abs),
        }
    }
}
