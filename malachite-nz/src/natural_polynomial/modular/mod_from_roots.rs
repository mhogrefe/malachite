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
use crate::natural_polynomial::modular::subproduct_tree::NaturalModulus;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::modular::traits::ModFromRoots;
use malachite_base::unsigned_polynomial::modular::mod_from_roots::from_roots_to_out;
use malachite_base::unsigned_polynomial::modular::subproduct_tree::Modulus;

// The monic polynomial with roots `roots` modulo `m`, computed in `T`, which `m` must fit in.
fn mod_from_roots_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    roots: &[Natural],
    m: &Natural,
) -> NaturalPolynomial
where
    Natural: From<T>,
{
    let mut out = vec![T::ZERO; roots.len() + 1];
    from_roots_to_out(&mut out, &to_words::<T>(roots), &Modulus(T::exact_from(m)));
    from_words(out)
}

fn mod_from_roots(roots: &[Natural], m: &Natural) -> NaturalPolynomial {
    assert_ne!(*m, 0u32);
    for r in roots {
        assert!(r < m, "every root must be reduced mod m, but {r} >= {m}");
    }
    if *m == 1u32 {
        return NaturalPolynomial::ZERO;
    }
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_from_roots_word::<Limb>(roots, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_from_roots_word::<DoubleLimb>(roots, m)
    } else {
        let mut out = vec![Natural::ZERO; roots.len() + 1];
        from_roots_to_out(&mut out, roots, &NaturalModulus::new(m));
        NaturalPolynomial { coefficients: out }
    }
}

impl ModFromRoots<Natural, Natural> for NaturalPolynomial {
    /// Builds the monic [`NaturalPolynomial`] whose roots, with multiplicity, are the values in
    /// `roots`, modulo $m$, taking $m$ by value. The roots must already be reduced modulo $m$.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}, m) = \prod_{i=0}^{n-1} (x - r_i) \bmod m.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Fewer than 20 factors are multiplied in one at a time; more are split into two halves, whose
    /// products are found the same way and then multiplied. When $m$ fits in one or two limbs, this
    /// is done in word arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(nb \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(nb \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `roots.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any root is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::modular::traits::ModFromRoots;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// let roots = [1u32, 2, 3].map(Natural::from);
    /// // (x - 1)(x - 2)(x - 3) = x^3 - 6x^2 + 11x - 6
    /// assert_eq!(
    ///     NaturalPolynomial::mod_from_roots(&roots, m).to_string(),
    ///     "x^3+x^2+4*x+1"
    /// );
    ///
    /// // 2^127 - 1
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let roots = [Natural::ONE, Natural::from_str("1267650600228229401496703205376").unwrap()];
    /// assert_eq!(
    ///     NaturalPolynomial::mod_from_roots(&roots, m).to_string(),
    ///     "x^2+170141182192818631503457902219180900350*x+1267650600228229401496703205376"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_product_roots_fmpz_vec` from
    /// `fmpz_mod_poly/product_roots_fmpz_vec.c`, FLINT 3.6.0.
    #[inline]
    fn mod_from_roots(roots: &[Natural], m: Natural) -> Self {
        mod_from_roots(roots, &m)
    }
}

impl ModFromRoots<Natural, &Natural> for NaturalPolynomial {
    /// Builds the monic [`NaturalPolynomial`] whose roots, with multiplicity, are the values in
    /// `roots`, modulo $m$, taking $m$ by reference. The roots must already be reduced modulo $m$.
    ///
    /// $$
    /// f((r_i)_{i=0}^{n-1}, m) = \prod_{i=0}^{n-1} (x - r_i) \bmod m.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// Fewer than 20 factors are multiplied in one at a time; more are split into two halves, whose
    /// products are found the same way and then multiplied. When $m$ fits in one or two limbs, this
    /// is done in word arithmetic.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(nb \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(nb \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `roots.len()`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any root is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::polynomial::modular::traits::ModFromRoots;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// let roots = [1u32, 2, 3].map(Natural::from);
    /// // (x - 1)(x - 2)(x - 3) = x^3 - 6x^2 + 11x - 6
    /// assert_eq!(
    ///     NaturalPolynomial::mod_from_roots(&roots, &m).to_string(),
    ///     "x^3+x^2+4*x+1"
    /// );
    ///
    /// // 2^127 - 1
    /// let m = Natural::from_str("170141183460469231731687303715884105727").unwrap();
    /// let roots = [Natural::ONE, Natural::from_str("1267650600228229401496703205376").unwrap()];
    /// assert_eq!(
    ///     NaturalPolynomial::mod_from_roots(&roots, &m).to_string(),
    ///     "x^2+170141182192818631503457902219180900350*x+1267650600228229401496703205376"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_product_roots_fmpz_vec` from
    /// `fmpz_mod_poly/product_roots_fmpz_vec.c`, FLINT 3.6.0.
    #[inline]
    fn mod_from_roots(roots: &[Natural], m: &Natural) -> Self {
        mod_from_roots(roots, m)
    }
}
