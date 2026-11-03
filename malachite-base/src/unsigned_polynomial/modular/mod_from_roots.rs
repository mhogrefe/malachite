// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::polynomial::modular::traits::ModFromRoots;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::subproduct_tree::{Modulus, TreeModulus};
use alloc::vec;

// The number of roots below which the product is built one factor at a time.
const MOD_FROM_ROOTS_BASECASE_CUTOFF: usize = 20;

/// Writes into `out`, which has length `xs.len() + 1`, the coefficients of $(x - x_0)(x - x_1)
/// \cdots (x - x_{n-1})$ modulo $m$ or $2^k$, where the roots `xs` are reduced.
///
/// This is not part of the public API; it is public so that `malachite-nz` can build
/// `NaturalPolynomial`s from their roots.
///
/// This is equivalent to `_nmod_poly_product_roots_nmod_vec` from
/// `nmod_poly/product_roots_nmod_vec.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn from_roots_to_out<C: Clone, M: TreeModulus<C>>(out: &mut [C], xs: &[C], m: &M) {
    let n = xs.len();
    assert_eq!(out.len(), n + 1);
    if n == 0 {
        out[0] = m.one();
    } else if n < MOD_FROM_ROOTS_BASECASE_CUTOFF {
        out[n] = m.one();
        out[n - 1] = m.neg(&xs[0]);
        for i in 1..n {
            out[n - i - 1] = m.neg(&m.mul(&out[n - i], &xs[i]));
            for j in 0..i - 1 {
                let product = m.mul(&out[n - i + j + 1], &xs[i]);
                out[n - i + j] = m.sub(&out[n - i + j], &product);
            }
            out[n - 1] = m.sub(&out[n - 1], &xs[i]);
        }
    } else {
        let half = n.div_ceil(2);
        let mut tmp = vec![m.zero(); n + 2];
        let (left, right) = tmp.split_at_mut(half + 1);
        from_roots_to_out(left, &xs[..half], m);
        from_roots_to_out(right, &xs[half..], m);
        m.mul_to_out(out, left, right);
    }
}

pub(crate) fn mod_from_roots_helper<T: PrimitiveUnsigned>(
    roots: &[T],
    m: T,
) -> UnsignedPolynomial<T> {
    assert_ne!(m, T::ZERO);
    for &r in roots {
        assert!(r < m, "every root must be reduced mod m, but {r} >= {m}");
    }
    if m == T::ONE {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; roots.len() + 1];
    from_roots_to_out(&mut out, roots, &Modulus(m));
    UnsignedPolynomial::from_coefficients_asc(out)
}

impl<T: PrimitiveUnsigned> ModFromRoots<T> for UnsignedPolynomial<T> {
    /// Builds the monic [`UnsignedPolynomial`] whose roots, with multiplicity, are the values in
    /// `roots`, modulo $m$. The roots must already be reduced modulo $m$.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}, m) = \prod_{i=0}^{n-1} (x - r_i) \bmod m.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Fewer than 20 factors are multiplied in one at a time; more are split into two halves, whose
    /// products are found the same way and then multiplied.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log^2 n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `roots.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any root is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::modular::traits::ModFromRoots;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::mod_from_roots(&[], 7).to_string(),
    ///     "1"
    /// );
    /// // (x - 1)(x - 2)(x - 3) = x^3 - 6x^2 + 11x - 6
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::mod_from_roots(&[1, 2, 3], 7).to_string(),
    ///     "x^3+x^2+4*x+1"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::mod_from_roots(&[5, 5], 7).to_string(),
    ///     "x^2+4*x+4"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::mod_from_roots(&[0], 1).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_product_roots_nmod_vec` from
    /// `nmod_poly/product_roots_nmod_vec.c`, FLINT 3.6.0.
    #[inline]
    fn mod_from_roots(roots: &[T], m: T) -> Self {
        mod_from_roots_helper(roots, m)
    }
}
