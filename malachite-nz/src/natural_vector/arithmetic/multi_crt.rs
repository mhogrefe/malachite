// Copyright © 2026 Mikhail Hogrefe
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
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::unsigned_vector::UnsignedVector;

// How the vector CRT functions combine the residues of one element.
pub(crate) enum ElementCombiner {
    // There is one modulus, so an element's only residue is already its canonical value.
    Single(Limb),
    Comb(Box<CrtComb>),
}

// Checks the arguments of the vector CRT functions and prepares to combine them, returning `None`
// if the moduli are unusable: with one modulus, if it is 0; with more, if any is less than 2 or two
// are not coprime. These are the conditions of `Natural::multi_crt`.
pub(crate) fn multi_crt_combiner(
    moduli: &[Limb],
    residues: &[UnsignedVector<Limb>],
) -> Option<ElementCombiner> {
    assert!(!moduli.is_empty(), "moduli must be nonempty");
    assert_eq!(
        residues.len(),
        moduli.len(),
        "one residue vector per modulus is required"
    );
    let dimension = residues[0].elements.len();
    assert!(
        residues.iter().all(|rs| rs.elements.len() == dimension),
        "the residue vectors must all have the same dimension"
    );
    let combiner = if let [m] = *moduli {
        if m == 0 {
            return None;
        }
        ElementCombiner::Single(m)
    } else {
        ElementCombiner::Comb(Box::new(CrtComb::new(moduli)?))
    };
    for (rs, &m) in residues.iter().zip(moduli) {
        for &r in &rs.elements {
            assert!(
                r < m,
                "residues must be reduced modulo the moduli, but {r} >= {m}"
            );
        }
    }
    Some(combiner)
}

// Applies `f` to the residues of each element in turn, one residue per modulus, collecting the
// results.
pub(crate) fn map_columns<T>(
    residues: &[UnsignedVector<Limb>],
    mut f: impl FnMut(&[Limb]) -> T,
) -> Vec<T> {
    let mut column = vec![0; residues.len()];
    (0..residues[0].elements.len())
        .map(|i| {
            for (c, rs) in column.iter_mut().zip(residues) {
                *c = rs.elements[i];
            }
            f(&column)
        })
        .collect()
}

impl NaturalVector {
    /// Combines vectors of residues modulo pairwise-coprime word-sized moduli into the vector whose
    /// every element is the unique number below the moduli product that is congruent to each of its
    /// residues, returning `None` if the moduli are unusable.
    ///
    /// `residues[j]` holds the residues of every element modulo `moduli[j]`, so element $i$ of the
    /// result is the combination of `residues[0][i]`, `residues[1][i]`, and so on, as
    /// [`Natural::multi_crt`] would compute it. The work that depends only on the moduli is done
    /// once, not once per element. The residues must be already reduced, and the moduli are
    /// unusable under the conditions of [`Natural::multi_crt`]: with one modulus, if it is 0; with
    /// more, if any is 0 or 1 or two are not coprime.
    ///
    /// For the representatives of smallest absolute value instead, use
    /// [`IntegerVector::multi_balanced_crt`](
    /// crate::integer_vector::IntegerVector::multi_balanced_crt).
    ///
    /// # Worst-case complexity
    /// $T(n, d) = O(n (\log n)^3 \log\log n + dn (\log n)^2 \log\log n)$
    ///
    /// $M(n, d) = O(n \log n + dn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of significant bits of the
    /// product of the moduli, and $d$ is the dimension of the residue vectors.
    ///
    /// # Panics
    /// Panics if `moduli` is empty, if the number of residue vectors differs from the number of
    /// moduli, if the residue vectors do not all have the same dimension, or if, when the moduli
    /// are usable, any residue is greater than or equal to its modulus.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// // 23 is 2 mod 3, 3 mod 5, and 2 mod 7; 6 is 0, 1, and 6; and 100 is 1, 0, and 2.
    /// let residues = [
    ///     UnsignedVector::from_str("(2, 0, 1)").unwrap(),
    ///     UnsignedVector::from_str("(3, 1, 0)").unwrap(),
    ///     UnsignedVector::from_str("(2, 6, 2)").unwrap(),
    /// ];
    /// assert_eq!(
    ///     NaturalVector::multi_crt(&[3, 5, 7], &residues)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(23, 6, 100)"
    /// );
    ///
    /// // 4 and 6 are not coprime.
    /// assert!(NaturalVector::multi_crt(&[4, 6], &residues[..2]).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_multi_CRT_ui` from `fmpz_vec/multi_CRT_ui.c`, FLINT 3.6.0,
    /// with sign = 0 and the residues required to be reduced.
    pub fn multi_crt(moduli: &[Limb], residues: &[UnsignedVector<Limb>]) -> Option<Self> {
        Some(Self {
            elements: match multi_crt_combiner(moduli, residues)? {
                ElementCombiner::Single(_) => residues[0]
                    .elements
                    .iter()
                    .copied()
                    .map(Natural::from)
                    .collect(),
                ElementCombiner::Comb(comb) => map_columns(residues, |rs| comb.combine(rs)),
            },
        })
    }
}
