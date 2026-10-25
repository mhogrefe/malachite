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

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_nz::integer_matrix::arithmetic::fraction_matrix::{Fraction, reconstruct_fractions};
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;

// The rational matrix with the given fractions.
fn from_fractions(height: u64, width: u64, fractions: Vec<Fraction>) -> RationalMatrix {
    RationalMatrix::from_owned_elements(
        height,
        width,
        fractions
            .into_iter()
            .map(|f| Rational {
                sign: f.sign,
                numerator: f.numerator,
                denominator: f.denominator,
            })
            .collect(),
    )
}

fn assert_reconstruct_arguments(a: &NaturalMatrix, m: &Natural) {
    assert!(*m > 2u32, "m must be greater than 2");
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

impl RationalMatrix {
    /// Recovers a [`RationalMatrix`] from a matrix of residues modulo $m$ by rational
    /// reconstruction, taking the residues by value and the modulus by reference.
    ///
    /// The residues must already be reduced modulo $m$, which must be greater than 2. Each element
    /// $x = n/d$ of the result satisfies $n \equiv rd \pmod m$, where $r$ is the corresponding
    /// residue. The elements are reconstructed in row-major order, and each residue is first
    /// multiplied by the product $D$ of the denominators found so far: the fraction $p/q$ recovered
    /// from $rD \bmod m$, with the balanced bounds $|p|, q \leq \lfloor\sqrt{(m-1)/2}\rfloor$ of
    /// [`Rational::reconstruct`], gives the element $p/(qD)$, and $D$ becomes $qD$. Elements that
    /// share a large denominator therefore need only small reconstructions after the first, and the
    /// matrix can be recovered where reconstructing each element on its own would fail or give a
    /// different fraction. `None` is returned if some step finds no fraction.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^2 b \log(nb) \log\log(nb) + nb (\log b)^2 \log\log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is less than 3, or if any residue is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[51, 68; 0, 100]").unwrap();
    /// assert_eq!(
    ///     RationalMatrix::reconstruct(a, &Natural::from(101u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[1/2, 2/3; 0, -1]"
    /// );
    ///
    /// // The residues of 1/6 and 1/36 modulo 73. The second is reconstructed as 6 * 1/36 = 1/6,
    /// // although 71 on its own reconstructs to -2.
    /// let a = NaturalMatrix::from_str("[61, 71]").unwrap();
    /// assert_eq!(
    ///     RationalMatrix::reconstruct(a, &Natural::from(73u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[1/6, 1/36]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3]").unwrap();
    /// assert_eq!(RationalMatrix::reconstruct(a, &Natural::from(11u32)), None);
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_set_fmpz_mat_mod_fmpz` from
    /// `fmpq_mat/set_fmpz_mat_mod_fmpz.c`, FLINT 3.6.0, except that it panics if a residue is not
    /// reduced or $m < 3$, where FLINT reduces the residues and reconstructs only zeros.
    pub fn reconstruct(a: NaturalMatrix, m: &Natural) -> Option<Self> {
        assert_reconstruct_arguments(&a, m);
        let (height, width) = (a.height(), a.width());
        reconstruct_fractions(a.into_elements().into_iter(), m)
            .map(|x| from_fractions(height, width, x))
    }

    /// Recovers a [`RationalMatrix`] from a matrix of residues modulo $m$ by rational
    /// reconstruction, taking the residues and the modulus by reference.
    ///
    /// The residues must already be reduced modulo $m$, which must be greater than 2. Each element
    /// $x = n/d$ of the result satisfies $n \equiv rd \pmod m$, where $r$ is the corresponding
    /// residue. The elements are reconstructed in row-major order, and each residue is first
    /// multiplied by the product $D$ of the denominators found so far: the fraction $p/q$ recovered
    /// from $rD \bmod m$, with the balanced bounds $|p|, q \leq \lfloor\sqrt{(m-1)/2}\rfloor$ of
    /// [`Rational::reconstruct`], gives the element $p/(qD)$, and $D$ becomes $qD$. Elements that
    /// share a large denominator therefore need only small reconstructions after the first, and the
    /// matrix can be recovered where reconstructing each element on its own would fail or give a
    /// different fraction. `None` is returned if some step finds no fraction.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^2 b \log(nb) \log\log(nb) + nb (\log b)^2 \log\log b)$
    ///
    /// $M(n, b) = O(n^2 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is less than 3, or if any residue is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[51, 68; 0, 100]").unwrap();
    /// assert_eq!(
    ///     RationalMatrix::reconstruct_ref(&a, &Natural::from(101u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[1/2, 2/3; 0, -1]"
    /// );
    ///
    /// // The residues of 1/6 and 1/36 modulo 73. The second is reconstructed as 6 * 1/36 = 1/6,
    /// // although 71 on its own reconstructs to -2.
    /// let a = NaturalMatrix::from_str("[61, 71]").unwrap();
    /// assert_eq!(
    ///     RationalMatrix::reconstruct_ref(&a, &Natural::from(73u32))
    ///         .unwrap()
    ///         .to_string(),
    ///     "[1/6, 1/36]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3]").unwrap();
    /// assert_eq!(
    ///     RationalMatrix::reconstruct_ref(&a, &Natural::from(11u32)),
    ///     None
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_set_fmpz_mat_mod_fmpz` from
    /// `fmpq_mat/set_fmpz_mat_mod_fmpz.c`, FLINT 3.6.0, except that it panics if a residue is not
    /// reduced or $m < 3$, where FLINT reduces the residues and reconstructs only zeros.
    pub fn reconstruct_ref(a: &NaturalMatrix, m: &Natural) -> Option<Self> {
        assert_reconstruct_arguments(a, m);
        reconstruct_fractions(a.elements_ref().iter(), m)
            .map(|x| from_fractions(a.height(), a.width(), x))
    }
}
