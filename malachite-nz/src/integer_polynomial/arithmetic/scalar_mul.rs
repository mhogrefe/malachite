// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::{One, Zero};

// Multiplies every element of `xs` by `c`, in place.
//
// Nothing is trimmed: when `c` is zero, every element becomes zero, and a caller holding the
// coefficients of a polynomial must trim them.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements
// of `xs` and of `c`.
//
// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar_mul_fmpz.c`, FLINT 3.6.0,
// where `poly1 == poly2`.
#[doc(hidden)]
pub fn integers_mul_scalar_assign(xs: &mut [Integer], c: &Integer) {
    match *c {
        integer_zero!() => xs.fill(Integer::ZERO),
        integer_one!() => {}
        integer_negative_one!() => {
            for x in xs {
                x.neg_assign();
            }
        }
        _ => {
            for x in xs {
                *x *= c;
            }
        }
    }
}

// Returns a `Vec` holding every element of `xs` multiplied by `c`.
//
// Nothing is trimmed: when `c` is zero, every element is zero, and a caller holding the
// coefficients of a polynomial must trim them.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements
// of `xs` and of `c`.
//
// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar_mul_fmpz.c`, FLINT 3.6.0.
crate_test_fn! {
integers_mul_scalar(xs: &[Integer], c: &Integer) -> Vec<Integer> {
    match *c {
        integer_zero!() => alloc::vec![Integer::ZERO; xs.len()],
        integer_one!() => xs.to_vec(),
        integer_negative_one!() => xs.iter().map(|x| -x).collect(),
        _ => xs.iter().map(|x| x * c).collect(),
    }
}
}

// Sets each element of `out` to the element of `xs` at the same index multiplied by `c`. `xs` must
// be at least as long as `out`.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the first
// `out.len()` elements of `xs` and of `c`.
//
// This is equivalent to `_fmpz_vec_scalar_mul_fmpz` from `fmpz_vec/scalar_mul_fmpz.c`, FLINT 3.6.0,
// where the output is separate from the input.
crate_test_fn! {integers_mul_scalar_to_out(out: &mut [Integer], xs: &[Integer], c: &Integer) {
    let xs = &xs[..out.len()];
    match *c {
        integer_zero!() => out.fill(Integer::ZERO),
        integer_one!() => out.clone_from_slice(xs),
        integer_negative_one!() => {
            for (o, x) in out.iter_mut().zip(xs) {
                *o = -x;
            }
        }
        _ => {
            for (o, x) in out.iter_mut().zip(xs) {
                *o = x * c;
            }
        }
    }
}}
