// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_div_rem::{from_words, to_words};
use crate::natural_polynomial::modular::subproduct_tree::NaturalPowerOf2Modulus;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::modular::traits::ModPowerOf2FromRoots;
use malachite_base::unsigned_polynomial::modular::mod_from_roots::from_roots_to_out;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::PowerOf2Modulus;

// The monic polynomial with roots `roots` modulo $2^k$, where $k > 0$ is `pow`, computed in `T`,
// which $2^k$ must fit in.
fn mod_power_of_2_from_roots_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    roots: &[Natural],
    pow: u64,
) -> NaturalPolynomial
where
    Natural: From<T>,
{
    let mut out = vec![T::ZERO; roots.len() + 1];
    from_roots_to_out(&mut out, &to_words::<T>(roots), &PowerOf2Modulus(pow));
    from_words(out)
}

impl ModPowerOf2FromRoots<Natural> for NaturalPolynomial {
    /// Builds the monic [`NaturalPolynomial`] whose roots, with multiplicity, are the values in
    /// `roots`, modulo $2^k$. The roots must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}, k) = \prod_{i=0}^{n-1} (x - r_i) \bmod 2^k.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// Fewer than 20 factors are multiplied in one at a time; more are split into two halves, whose
    /// products are found the same way and then multiplied. When $2^k$ fits in one or two limbs,
    /// this is done in wrapping word arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log^2 (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(nk \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `roots.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any root is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2FromRoots;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // (x - 1)(x - 3) = x^2 - 4x + 3
    /// let roots = [1u32, 3].map(Natural::from);
    /// assert_eq!(
    ///     NaturalPolynomial::mod_power_of_2_from_roots(&roots, 3).to_string(),
    ///     "x^2+4*x+3"
    /// );
    /// assert_eq!(
    ///     NaturalPolynomial::mod_power_of_2_from_roots(&roots, 100).to_string(),
    ///     "x^2+1267650600228229401496703205372*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_product_roots_fmpz_vec` from
    /// `fmpz_mod_poly/product_roots_fmpz_vec.c`, FLINT 3.6.0, with the modulus $2^k$.
    fn mod_power_of_2_from_roots(roots: &[Natural], pow: u64) -> Self {
        for r in roots {
            assert!(
                r.significant_bits() <= pow,
                "every root must be reduced mod 2^pow, but {r} >= 2^{pow}"
            );
        }
        if pow == 0 {
            Self::ZERO
        } else if pow <= Limb::WIDTH {
            mod_power_of_2_from_roots_word::<Limb>(roots, pow)
        } else if pow <= DoubleLimb::WIDTH {
            mod_power_of_2_from_roots_word::<DoubleLimb>(roots, pow)
        } else {
            let mut out = vec![Natural::ZERO; roots.len() + 1];
            from_roots_to_out(&mut out, roots, &NaturalPowerOf2Modulus(pow));
            Self { coefficients: out }
        }
    }
}
