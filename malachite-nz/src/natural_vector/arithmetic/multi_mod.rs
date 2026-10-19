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

use crate::natural::Natural;
use crate::natural::arithmetic::crt_comb::CrtComb;
use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

// With more moduli than this, reducing each element through a comb, which shares the work between
// the moduli, is faster than reducing it modulo each modulus separately. This is
// FMPZ_MAT_MOD_PRIMES_COMB_CUTOFF from fmpz_mat.h, FLINT 3.6.0.
const MULTI_MOD_COMB_CUTOFF: usize = 200;

// Reduces each of `xs` modulo each of `moduli`, returning one `Vec` of residues per modulus.
// `reduce` reduces an element modulo all the comb's moduli at once, and `rem` reduces it modulo a
// single modulus, given both as a `Natural` and as a `Limb`. A comb is used only when there are
// enough moduli and they are pairwise coprime and at least 2, as it requires.
//
// This is fmpz_mat_multi_mod_ui from fmpz_mat/multi_mod_ui.c, FLINT 3.6.0, for a slice of elements.
pub(crate) fn multi_mod_elements<T>(
    xs: &[T],
    moduli: &[Limb],
    reduce: impl Fn(&CrtComb, &T) -> Vec<Limb>,
    rem: impl Fn(&T, &Natural, Limb) -> Limb,
) -> Vec<Vec<Limb>> {
    assert!(!moduli.contains(&0), "division by zero");
    if moduli.len() > MULTI_MOD_COMB_CUTOFF
        && let Some(comb) = CrtComb::new(moduli)
    {
        let mut out = vec![Vec::with_capacity(xs.len()); moduli.len()];
        for x in xs {
            for (rs, r) in out.iter_mut().zip(reduce(&comb, x)) {
                rs.push(r);
            }
        }
        out
    } else {
        moduli
            .iter()
            .map(|&m| {
                let m_natural = Natural::from(m);
                xs.iter().map(|x| rem(x, &m_natural, m)).collect()
            })
            .collect()
    }
}

// Reduces each of `xs` modulo each of `moduli`, returning one `Vec` of residues per modulus.
pub(crate) fn naturals_multi_mod(xs: &[Natural], moduli: &[Limb]) -> Vec<Vec<Limb>> {
    multi_mod_elements(xs, moduli, CrtComb::reduce, |x, m, _| {
        Limb::exact_from(&(x % m))
    })
}

impl NaturalVector {
    /// Reduces a [`NaturalVector`] modulo each of several word-sized moduli, returning one vector
    /// of residues per modulus.
    ///
    /// Element $j$ of the result holds the residues of every element modulo `moduli[j]`, each in
    /// $[0, m_j)$. The moduli need not be prime or coprime. When there are more than 200 of them
    /// and they are pairwise coprime and at least 2, the work of reducing an element is shared
    /// between the moduli, as in FLINT. The result is the input that [`NaturalVector::multi_crt`]
    /// takes, so the two are inverses when the elements are less than the product of the moduli.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let residues = NaturalVector::from_str("(23, 6, 100)")
    ///     .unwrap()
    ///     .multi_mod(&[3, 5, 7]);
    /// assert_eq!(
    ///     residues.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["(2, 0, 1)", "(3, 1, 0)", "(2, 6, 2)"]
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_multi_mod_ui` from `fmpz_mat/multi_mod_ui.c`, FLINT 3.6.0,
    /// applied to a vector.
    #[inline]
    pub fn multi_mod(&self, moduli: &[Limb]) -> Vec<UnsignedVector<Limb>> {
        naturals_multi_mod(&self.elements, moduli)
            .into_iter()
            .map(UnsignedVector::from_owned_elements)
            .collect()
    }
}
