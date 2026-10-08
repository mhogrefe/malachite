// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2012 Sebastian Pancratz
//
//      Copyright © 2026 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::bit_pack::limbs_pack_coefficients;
use crate::integer_polynomial::arithmetic::bit_unpack::{
    limbs_unpack_coefficients, limbs_unpack_coefficients_unsigned,
};
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul_middle::truncate_mul_middle_inputs;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::platform::Limb;
use alloc::vec;
use core::cmp::min;
use core::ptr;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::slices::slice_trailing_zeros;

// Multiplication by Kronecker substitution: each polynomial is evaluated at $2^b$, for a field
// width $b$ wide enough that no coefficient of the product overflows its field, by packing its
// coefficients into `bits`-bit fields of a single integer; the two integers are multiplied, and the
// wanted coefficients of the product are read back out of the fields of the result.

// Sets `out` to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of the product of the
// polynomials with coefficients `xs` and `ys`, both nonempty. `nlo < nhi <= xs.len() + ys.len() -
// 1` must hold, and `out` must have length `nhi - nlo`.
//
// # Worst-case complexity
// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
//
// $M(n, m) = O(n(m + \log n) \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is the
// largest number of significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mulmid_KS` from `fmpz_poly/mulmid_KS.c`, FLINT 3.6.0.
crate_test_fn! {mul_middle_to_out_kronecker<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    nlo: usize,
    nhi: usize,
) {
    assert_ne!(xs.len(), 0);
    assert_ne!(ys.len(), 0);
    assert!(nlo < nhi);
    assert!(nhi < xs.len() + ys.len());
    let (xs, ys, nlo, mut nhi) = truncate_mul_middle_inputs(xs, ys, nlo, nhi);
    // Strip zeros from the top of each input.
    let xs = &xs[..xs.len() - slice_trailing_zeros(xs)];
    let ys = &ys[..ys.len() - slice_trailing_zeros(ys)];
    if xs.is_empty() || ys.is_empty() {
        out.fill(C::ZERO);
        return;
    }
    let len1 = xs.len();
    let len2 = ys.len();
    let negate1 = xs[len1 - 1].is_negative();
    let negate2 = ys[len2 - 1].is_negative();
    let full_len = len1 + len2 - 1;
    if nlo >= min(nhi, full_len) {
        out.fill(C::ZERO);
        return;
    }
    let mut zero_high = 0;
    if nhi > full_len {
        zero_high = nhi - full_len;
        nhi = full_len;
    }
    let square = ptr::eq(xs, ys);
    let (bits1, negative1) = vec_max_bits(xs);
    let (bits2, negative2) = if square {
        (bits1, negative1)
    } else {
        vec_max_bits(ys)
    };
    let sign = negative1 || negative2;
    let loglen = u64::exact_from(min(len1, len2)).significant_bits();
    let bits = bits1 + bits2 + loglen + u64::from(sign);
    let limbs1 = usize::exact_from((bits * u64::exact_from(len1) - 1) >> Limb::LOG_WIDTH) + 1;
    let limbs2 = usize::exact_from((bits * u64::exact_from(len2) - 1) >> Limb::LOG_WIDTH) + 1;
    let product = if square {
        let mut packed = vec![0; limbs1];
        limbs_pack_coefficients(&mut packed, xs, bits, negate1);
        let product_len = limbs1 << 1;
        let mut product = vec![0; product_len + limbs_square_to_out_scratch_len(limbs1)];
        let (product_limbs, scratch) = product.split_at_mut(product_len);
        limbs_square_to_out(product_limbs, &packed, scratch);
        product.truncate(product_len);
        product
    } else {
        let mut packed = vec![0; limbs1 + limbs2];
        let (packed1, packed2) = packed.split_at_mut(limbs1);
        limbs_pack_coefficients(packed1, xs, bits, negate1);
        limbs_pack_coefficients(packed2, ys, bits, negate2);
        limbs_mul(packed1, packed2)
    };
    let (out, out_zero) = out.split_at_mut(nhi - nlo);
    if sign {
        limbs_unpack_coefficients(out, nlo, nhi, &product, bits, negate1 ^ negate2);
    } else {
        limbs_unpack_coefficients_unsigned(out, nlo, nhi, &product, bits);
    }
    if zero_high != 0 {
        out_zero.fill(C::ZERO);
    }
}}
