// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::arithmetic::mul::kronecker::mul_to_out_kronecker;
use crate::integer_polynomial::arithmetic::mul::schonhage_strassen::*;
use crate::integer_polynomial::arithmetic::mul_high::classical::mul_high_to_out_classical;
use crate::integer_polynomial::arithmetic::mul_high::karatsuba::mul_high_to_out_karatsuba_n;
use crate::integer_vector::arithmetic::max_limbs::vec_max_limbs;
use crate::platform::Limb;
use core::cmp::max;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

pub mod classical;
pub mod karatsuba;

// Sets `out` to the coefficients of the product of the polynomials with coefficients `xs` and `ys`,
// both nonempty, except that the first `start` coefficients are unspecified: the coefficients of
// $x^i$ for $i < `start`$ need not be computed. `out` must have length `xs.len() + ys.len() - 1`.
//
// # Worst-case complexity
// $T(n, m) = O(n^2 m \log m \log\log m)$
//
// $M(n, m) = O(n(m + \log n) \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is the
// largest number of significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mulhigh` from `fmpz_poly/mulhigh.c`, FLINT 3.6.0.
crate_test_fn! {mul_high_to_out(out: &mut [Integer], xs: &[Integer], ys: &[Integer], start: usize) {
    let limbs1 = vec_max_limbs(xs);
    let limbs2 = vec_max_limbs(ys);
    let limbsx = max(limbs1, limbs2);
    if start < 5 {
        mul_high_to_out_classical(out, xs, ys, start);
        return;
    }
    if limbsx > 4 && start < 17 && xs.len() == start + 1 && ys.len() == start + 1 {
        mul_high_to_out_karatsuba_n(out, xs, ys);
    } else {
        // Kronecker substitution for small or very unbalanced coefficient sizes, and
        // Schönhage–Strassen otherwise. Either computes the whole product.
        let limbs = limbs1 + limbs2;
        let len = u64::exact_from(xs.len() + ys.len());
        if limbs <= 8 || limbs >> 11 > len || (limbs << Limb::LOG_WIDTH << 2) < len {
            mul_to_out_kronecker(out, xs, ys);
        } else {
            mul_to_out_schonhage_strassen(out, xs, ys);
        }
    }
}}
