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

use crate::integer_matrix::IntegerMatrix;
use crate::integer_vector::arithmetic::multi_crt::multi_balanced_crt_elements;
use crate::natural_matrix::arithmetic::multi_crt::matrix_residue_slices;
use crate::platform::Limb;
use malachite_base::matrix::Matrix;
use malachite_base::unsigned_matrix::UnsignedMatrix;

impl IntegerMatrix {
    /// Combines matrices of residues modulo pairwise-coprime word-sized moduli into the matrix
    /// whose every element is the unique number $x$ with $-P/2 < x \leq P/2$, where $P$ is the
    /// moduli product, that is congruent to each of its residues, returning `None` if the moduli
    /// are unusable.
    ///
    /// `residues[j]` holds the residues of every element modulo `moduli[j]`, so the element in row
    /// $i$ and column $k$ of the result is the combination of the elements in that position of
    /// `residues[0]`, `residues[1]`, and so on, as
    /// [`Integer::multi_balanced_crt`](crate::integer::Integer::multi_balanced_crt) would compute
    /// it. The work that depends only on the moduli is done once, not once per element. The
    /// residues must be already reduced, and the moduli are unusable under the conditions of
    /// [`Integer::multi_balanced_crt`](crate::integer::Integer::multi_balanced_crt): with one
    /// modulus, if it is 0; with more, if any is 0 or 1 or two are not coprime.
    ///
    /// For the representatives in $[0, P)$ instead, use
    /// [`NaturalMatrix::multi_crt`](crate::natural_matrix::NaturalMatrix::multi_crt).
    ///
    /// # Worst-case complexity
    /// $T(n, d) = O(n (\log n)^3 \log\log n + dn (\log n)^2 \log\log n)$
    ///
    /// $M(n, d) = O(n \log n + dn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of significant bits of the
    /// product of the moduli, and $d$ is the number of elements of each residue matrix.
    ///
    /// # Panics
    /// Panics if `moduli` is empty, if the number of residue matrices differs from the number of
    /// moduli, if the residue matrices do not all have the same shape, or if, when the moduli are
    /// usable, any residue is greater than or equal to its modulus.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let residues = [
    ///     UnsignedMatrix::from_str("[2, 0; 1, 0]").unwrap(),
    ///     UnsignedMatrix::from_str("[3, 1; 0, 0]").unwrap(),
    ///     UnsignedMatrix::from_str("[2, 6; 2, 0]").unwrap(),
    /// ];
    /// assert_eq!(
    ///     IntegerMatrix::multi_balanced_crt(&[3, 5, 7], &residues)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[23, 6; -5, 0]"
    /// );
    ///
    /// // 4 and 6 are not coprime.
    /// assert!(IntegerMatrix::multi_balanced_crt(&[4, 6], &residues[..2]).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_multi_CRT_ui` from `fmpz_mat/multi_CRT_ui.c`, FLINT 3.6.0,
    /// with sign = 1 and the residues required to be reduced.
    pub fn multi_balanced_crt(moduli: &[Limb], residues: &[UnsignedMatrix<Limb>]) -> Option<Self> {
        let (height, width, residues) = matrix_residue_slices(moduli, residues);
        Some(Self::from_owned_elements(
            height,
            width,
            multi_balanced_crt_elements(moduli, &residues)?,
        ))
    }
}
