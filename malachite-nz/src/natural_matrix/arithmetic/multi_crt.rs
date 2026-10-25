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

use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::arithmetic::multi_crt::multi_crt_elements;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the arguments of the matrix CRT functions, returning the shape of the residue matrices and
// each one's elements.
pub(crate) fn matrix_residue_slices<'a>(
    moduli: &[Limb],
    residues: &'a [UnsignedMatrix<Limb>],
) -> (u64, u64, Vec<&'a [Limb]>) {
    assert!(!moduli.is_empty(), "moduli must be nonempty");
    assert_eq!(
        residues.len(),
        moduli.len(),
        "one residue matrix per modulus is required"
    );
    let (height, width) = (residues[0].height(), residues[0].width());
    assert!(
        residues
            .iter()
            .all(|rs| rs.height() == height && rs.width() == width),
        "the residue matrices must all have the same shape"
    );
    (
        height,
        width,
        residues.iter().map(|rs| rs.elements_ref()).collect(),
    )
}

impl NaturalMatrix {
    /// Combines matrices of residues modulo pairwise-coprime word-sized moduli into the matrix
    /// whose every element is the unique number below the moduli product that is congruent to each
    /// of its residues, returning `None` if the moduli are unusable.
    ///
    /// `residues[j]` holds the residues of every element modulo `moduli[j]`, so the element in row
    /// $i$ and column $k$ of the result is the combination of the elements in that position of
    /// `residues[0]`, `residues[1]`, and so on, as
    /// [`Natural::multi_crt`](crate::natural::Natural::multi_crt) would compute it. The work that
    /// depends only on the moduli is done once, not once per element. The residues must be already
    /// reduced, and the moduli are unusable under the conditions of
    /// [`Natural::multi_crt`](crate::natural::Natural::multi_crt): with one modulus, if it is 0;
    /// with more, if any is 0 or 1 or two are not coprime.
    ///
    /// For the representatives of smallest absolute value instead, use
    /// [`IntegerMatrix::multi_balanced_crt`](
    /// crate::integer_matrix::IntegerMatrix::multi_balanced_crt).
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let residues = [
    ///     UnsignedMatrix::from_str("[2, 0; 1, 0]").unwrap(),
    ///     UnsignedMatrix::from_str("[3, 1; 0, 0]").unwrap(),
    ///     UnsignedMatrix::from_str("[2, 6; 2, 0]").unwrap(),
    /// ];
    /// assert_eq!(
    ///     NaturalMatrix::multi_crt(&[3, 5, 7], &residues)
    ///         .unwrap()
    ///         .to_string(),
    ///     "[23, 6; 100, 0]"
    /// );
    ///
    /// // 4 and 6 are not coprime.
    /// assert!(NaturalMatrix::multi_crt(&[4, 6], &residues[..2]).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_multi_CRT_ui` from `fmpz_mat/multi_CRT_ui.c`, FLINT 3.6.0,
    /// with sign = 0 and the residues required to be reduced.
    pub fn multi_crt(moduli: &[Limb], residues: &[UnsignedMatrix<Limb>]) -> Option<Self> {
        let (height, width, residues) = matrix_residue_slices(moduli, residues);
        Some(Self::from_owned_elements(
            height,
            width,
            multi_crt_elements(moduli, &residues)?,
        ))
    }
}
