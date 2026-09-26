// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;

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
    if *c == 0u32 {
        xs.fill(Integer::ZERO);
    } else if *c == -1i32 {
        for x in xs {
            x.neg_assign();
        }
    } else if *c != 1u32 {
        for x in xs {
            *x *= c;
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
    if *c == 0u32 {
        alloc::vec![Integer::ZERO; xs.len()]
    } else if *c == 1u32 {
        xs.to_vec()
    } else if *c == -1i32 {
        xs.iter().map(|x| -x).collect()
    } else {
        xs.iter().map(|x| x * c).collect()
    }
}
}
