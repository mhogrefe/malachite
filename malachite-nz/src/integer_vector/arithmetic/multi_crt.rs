// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural_vector::arithmetic::multi_crt::{
    ElementCombiner, map_columns, multi_crt_combiner,
};
use crate::platform::Limb;
use malachite_base::unsigned_vector::UnsignedVector;

impl IntegerVector {
    /// Combines vectors of residues modulo pairwise-coprime word-sized moduli into the vector whose
    /// every element is the representative of smallest absolute value that is congruent to each of
    /// its residues, returning `None` if the moduli are unusable.
    ///
    /// `residues[j]` holds the residues of every element modulo `moduli[j]`, so element $i$ of the
    /// result is the combination of `residues[0][i]`, `residues[1][i]`, and so on, as
    /// [`Integer::multi_balanced_crt`] would compute it. Each element $x$ of the result satisfies
    /// $-M/2 < x \leq M/2$, where $M$ is the product of the moduli, so a tie at $M/2$ is positive.
    /// The work that depends only on the moduli is done once, not once per element. The residues
    /// must be already reduced, and the moduli are unusable under the conditions of
    /// [`Integer::multi_balanced_crt`]: with one modulus, if it is 0; with more, if any is 0 or 1
    /// or two are not coprime.
    ///
    /// For the representatives in $[0, M)$ instead, use
    /// [`NaturalVector::multi_crt`](crate::natural_vector::NaturalVector::multi_crt).
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// // 23 is 2 mod 3, 3 mod 5, and 2 mod 7; 6 is 0, 1, and 6; and -5 is 1, 0, and 2.
    /// let residues = [
    ///     UnsignedVector::from_str("(2, 0, 1)").unwrap(),
    ///     UnsignedVector::from_str("(3, 1, 0)").unwrap(),
    ///     UnsignedVector::from_str("(2, 6, 2)").unwrap(),
    /// ];
    /// assert_eq!(
    ///     IntegerVector::multi_balanced_crt(&[3, 5, 7], &residues)
    ///         .unwrap()
    ///         .to_string(),
    ///     "(23, 6, -5)"
    /// );
    ///
    /// // 4 and 6 are not coprime.
    /// assert!(IntegerVector::multi_balanced_crt(&[4, 6], &residues[..2]).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_multi_CRT_ui` from `fmpz_vec/multi_CRT_ui.c`, FLINT 3.6.0,
    /// with sign = 1 and the residues required to be reduced.
    pub fn multi_balanced_crt(moduli: &[Limb], residues: &[UnsignedVector<Limb>]) -> Option<Self> {
        Some(Self {
            elements: match multi_crt_combiner(moduli, residues)? {
                // A residue above half the modulus is closer to zero once the modulus is
                // subtracted; at exactly half, it stays positive.
                ElementCombiner::Single(m) => residues[0]
                    .elements
                    .iter()
                    .map(|&r| {
                        if r > m >> 1 {
                            -Integer::from(m - r)
                        } else {
                            Integer::from(r)
                        }
                    })
                    .collect(),
                ElementCombiner::Comb(comb) => {
                    map_columns(residues, |rs| comb.combine_balanced(rs))
                }
            },
        })
    }
}
