// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018, 2026 Fredrik Johansson
//
//      Copyright © 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural_vector::arithmetic::multi_mod::multi_mod_elements;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

// Reduces each of `xs` modulo each of `moduli`, returning one `Vec` of residues per modulus. A
// negative element's residues are those of its absolute value, negated modulo each modulus.
pub(crate) fn integers_multi_mod(xs: &[Integer], moduli: &[Limb]) -> Vec<Vec<Limb>> {
    multi_mod_elements(
        xs,
        moduli,
        |comb, x| {
            let mut rs = comb.reduce(x.unsigned_abs_ref());
            if *x < 0u32 {
                for (r, &m) in rs.iter_mut().zip(moduli) {
                    if *r != 0 {
                        *r = m - *r;
                    }
                }
            }
            rs
        },
        |x, m_natural, m| {
            let r = Limb::exact_from(&(x.unsigned_abs_ref() % m_natural));
            if *x < 0u32 && r != 0 { m - r } else { r }
        },
    )
}

impl IntegerVector {
    /// Reduces an [`IntegerVector`] modulo each of several word-sized moduli, returning one vector
    /// of residues per modulus.
    ///
    /// Element $j$ of the result holds the residues of every element modulo `moduli[j]`, each in
    /// $[0, m_j)$. Negative elements have nonnegative residues. The moduli need not be prime or
    /// coprime. When there are more than 200 of them and they are pairwise coprime and at least 2,
    /// the work of reducing an element is shared between the moduli, as in FLINT. The result is the
    /// input that [`NaturalVector::multi_crt`](crate::natural_vector::NaturalVector::multi_crt)
    /// takes, so the two are inverses when the elements are nonnegative and less than the product
    /// of the moduli; for an element whose absolute value is at most half the product,
    /// [`IntegerVector::multi_balanced_crt`] recovers it.
    ///
    /// # Worst-case complexity
    /// $T(n, d) = O(n (\log n)^3 \log\log n + dn (\log n)^2 \log\log n)$
    ///
    /// $M(n, d) = O(n \log n + dn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the larger of the number of significant
    /// bits of the product of the moduli and the largest number of significant bits of an element,
    /// and $d$ is the number of elements.
    ///
    /// # Panics
    /// Panics if any modulus is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let residues = IntegerVector::from_str("(-23, 6, -100)")
    ///     .unwrap()
    ///     .multi_mod(&[3, 5, 7]);
    /// assert_eq!(
    ///     residues.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["(1, 0, 2)", "(2, 1, 0)", "(5, 6, 5)"]
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_multi_mod_ui` from `fmpz_mat/multi_mod_ui.c`, FLINT 3.6.0,
    /// applied to a vector.
    #[inline]
    pub fn multi_mod(&self, moduli: &[Limb]) -> Vec<UnsignedVector<Limb>> {
        integers_multi_mod(&self.elements, moduli)
            .into_iter()
            .map(UnsignedVector::from_owned_elements)
            .collect()
    }
}
