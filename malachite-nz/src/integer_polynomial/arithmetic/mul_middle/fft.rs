// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Daniel Schultz
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::natural::arithmetic::mul::fft::fmpz_poly_mul_mid_default_mpn_ctx;

// Sets `out` to the coefficients of $x^i$ for `nlo` $\leq i <$ `nhi` of the product of the
// polynomials with coefficients `xs` and `ys`, both nonempty, using the small-prime FFT, and
// returns `true`; or, if the coefficients of the product are too large for it, returns `false`,
// leaving unchanged the entries of `out` for coefficients of the product (entries past the
// product's length may have been zeroed). `nlo <= nhi` must hold, and `out` must have length `nhi -
// nlo`. The implementation, which shares its transforms with `Natural` multiplication, is in
// `natural/arithmetic/mul/fft.rs`.
//
// # Worst-case complexity
// $T(n, m) = O(n(m + \log n))$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len() + ys.len()`, and $m$ is the largest
// number of significant bits of any element of `xs` or `ys`.
//
// This is equivalent to `_fmpz_poly_mul_mid_default_mpn_ctx` from `fft_small/default_ctx.c`, FLINT
// 3.6.0.
crate_test_fn! {
#[inline]
mul_middle_to_out_fft<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    nlo: usize,
    nhi: usize,
) -> bool {
    fmpz_poly_mul_mid_default_mpn_ctx(out, nlo, nhi, xs, ys)
}}
