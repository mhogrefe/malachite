// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::polynomial::modular::traits::ModPowerOf2FromRoots;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_from_roots::from_roots_to_out;
use crate::unsigned_polynomial::modular::subproduct_tree::PowerOf2Modulus;
use alloc::vec;

pub(crate) fn mod_power_of_2_from_roots_helper<T: PrimitiveUnsigned>(
    roots: &[T],
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert!(pow <= T::WIDTH);
    for &r in roots {
        assert!(
            r.significant_bits() <= pow,
            "every root must be reduced mod 2^pow, but {r} >= 2^{pow}"
        );
    }
    if pow == 0 {
        return UnsignedPolynomial::ZERO;
    }
    let mut out = vec![T::ZERO; roots.len() + 1];
    from_roots_to_out(&mut out, roots, &PowerOf2Modulus(pow));
    UnsignedPolynomial::from_coefficients_asc(out)
}

impl<T: PrimitiveUnsigned> ModPowerOf2FromRoots<T> for UnsignedPolynomial<T> {
    /// Builds the monic [`UnsignedPolynomial`] whose roots, with multiplicity, are the values in
    /// `roots`, modulo $2^k$. The roots must already be reduced modulo $2^k$, and $k$ may be at
    /// most the width of the type.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}, k) = \prod_{i=0}^{n-1} (x - r_i) \bmod 2^k.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any root is greater than or equal to
    /// $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2FromRoots;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::mod_power_of_2_from_roots(&[], 3).to_string(),
    ///     "1"
    /// );
    /// // (x - 1)(x - 3) = x^2 - 4x + 3
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::mod_power_of_2_from_roots(&[1, 3], 3).to_string(),
    ///     "x^2+4*x+3"
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::mod_power_of_2_from_roots(&[0], 0).to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_product_roots_nmod_vec` from
    /// `nmod_poly/product_roots_nmod_vec.c`, FLINT 3.6.0, with the modulus $2^k$.
    #[inline]
    fn mod_power_of_2_from_roots(roots: &[T], pow: u64) -> Self {
        mod_power_of_2_from_roots_helper(roots, pow)
    }
}
