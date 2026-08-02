// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;

/// A candidate fused `add_mul`, kept for benchmarking against the composition `x + y * z`.
///
/// The composition cancels the denominator of `x` against the *product* of the two other
/// denominators, in one `gcd(b, d1 * f1)`. This variant instead cancels against each of them
/// separately, using
///
/// $$\gcd(b, d_1 f_1) = \gcd(b, d_1) \cdot \gcd(b / \gcd(b, d_1), f_1),$$
///
/// which holds for every prime valuation. Two things are traded. It pays one extra gcd, on
/// operands no larger than the single one it replaces. In exchange it cancels *before* the
/// product is formed, so the denominator of the sum is built as $(d_1/g_a)(f_1/g_b)$ from pieces
/// that are already reduced, and the double-width $d_1 f_1$ is never materialized nor divided by
/// the gcd afterwards.
///
/// **It is slower, and this function exists to record that.** The appeal of the split is that a
/// gcd costs more than linear time, so two gcds on $n$-bit operands should beat one against a
/// $2n$-bit product. That reasoning does not survive contact with
/// [`limbs_gcd_reduced`](malachite_nz::natural::arithmetic::gcd), which begins by reducing the
/// larger operand modulo the smaller: the single gcd therefore costs one division plus a
/// *balanced* gcd, and the split trades that cheap division for a second full gcd.
///
/// Measured against the composition, as `split / composed`, on denominators of equal width with
/// only the shared fraction varying:
///
/// | bits | 0% shared | 25% | 50% | 75% | 100% |
/// |------|-----------|------|------|------|------|
/// | 256  | 1.54      | 1.52 | 1.32 | 1.37 | 1.42 |
/// | 1024 | 1.72      | 1.66 | 1.56 | 1.48 | 1.36 |
/// | 4096 | 1.42      | 1.35 | 1.37 | 1.31 | 1.33 |
///
/// Sharing does move things the predicted way -- the gap narrows as the overlap grows -- but it
/// never closes. The `Algorithms` benchmark, whose triples are drawn from `rational_triple_gen`
/// and so are essentially always coprime, independently puts the split at 1.149.
///
/// To reproduce the shared-factor half: build `b = p * q * b0`, `d = p * d0`, `f = q * f0` with
/// the shared factors `p` and `q` carrying the chosen fraction of the bits and the private parts
/// padding every denominator back to the same width, then take all three numerators to be 1 so
/// the multiplication's cross-gcds stay trivial and only the cancellation under study is timed.
pub fn add_mul_split(x: &Rational, y: &Rational, z: &Rational) -> Rational {
    if x.numerator == 0 || y.numerator == 0 || z.numerator == 0 {
        if y.numerator == 0 || z.numerator == 0 {
            return x.clone();
        }
    }
    // The cross-cancellation of the product, exactly as `Mul` does it.
    let g_1 = (&y.numerator).gcd(&z.denominator);
    let g_2 = (&z.numerator).gcd(&y.denominator);
    let c_1 = (&y.numerator).div_exact(&g_1);
    let f_1 = (&z.denominator).div_exact(g_1);
    let e_1 = (&z.numerator).div_exact(&g_2);
    let d_1 = (&y.denominator).div_exact(g_2);
    let p = c_1 * e_1;
    let product_sign = y.sign == z.sign;

    // Cancel `b` against each of the product's denominator factors in turn, rather than against
    // their product.
    let g_a = (&x.denominator).gcd(&d_1);
    let b_rest = (&x.denominator).div_exact(&g_a);
    let d_2 = d_1.div_exact(&g_a);
    let g_b = (&b_rest).gcd(&f_1);
    let b_1 = b_rest.div_exact(&g_b);
    let f_2 = f_1.div_exact(&g_b);
    let g = g_a * g_b;
    // The denominator of the product, already divided by `g`.
    let q_1 = d_2 * f_2;

    let sum_n = Integer::from_sign_and_abs(x.sign, &x.numerator * &q_1)
        + Integer::from_sign_and_abs(product_sign, &b_1 * p);
    if sum_n == 0 {
        return Rational::ZERO;
    }
    let h = (&g).gcd(sum_n.unsigned_abs_ref());
    let sign = sum_n >= 0;
    let numerator = sum_n.unsigned_abs();
    if h == 1 {
        Rational {
            sign,
            numerator,
            denominator: g * b_1 * q_1,
        }
    } else {
        Rational {
            sign,
            numerator: numerator.div_exact(&h),
            denominator: g.div_exact(h) * b_1 * q_1,
        }
    }
}
