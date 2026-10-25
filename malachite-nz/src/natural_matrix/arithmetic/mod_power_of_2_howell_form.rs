// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 Tommy Hofmann
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_howell_form::move_zero_rows_last;
use crate::natural_matrix::arithmetic::mod_power_of_2_strong_echelon_form::*;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::{pad, to_limb_matrix};
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2HowellForm, ModPowerOf2HowellFormAssign, ModPowerOf2IsReduced,
};
use malachite_base::num::basic::integers::PrimitiveInt;

fn mod_power_of_2_howell_form_assign_helper(a: &mut NaturalMatrix, pow: u64) {
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    if pow <= Limb::WIDTH {
        *a = NaturalMatrix::from(to_limb_matrix(a).mod_power_of_2_howell_form(pow));
    } else {
        pad(a);
        let (h, w) = (a.height, a.width);
        if h != 0 && w != 0 {
            strong_echelon_form_mod_power_of_2_in_place(&mut a.elements, h, w, pow);
            move_zero_rows_last(&mut a.elements, h, w);
        }
    }
}

impl ModPowerOf2HowellForm for NaturalMatrix {
    type Output = Self;
    /// Computes the Howell form of an [`NaturalMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Howell form of an $h
    /// \times w$ matrix $A$ modulo $2^k$ is the unique $\max(h, w) \times w$ matrix in row echelon
    /// form whose rows span the same module over $\mathbb{Z}/2^k\mathbb{Z}$ as those of $A$, whose
    /// pivots are powers of 2, whose elements above each pivot $2^v$ lie in $[0, 2^v)$, and in
    /// which every element of the span whose first $j$ elements are zero is in the span of the rows
    /// whose pivots are beyond column $j$. Its zero rows come last.
    ///
    /// It is the strong echelon form (see [`ModPowerOf2StrongEchelonForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonForm)) with its nonzero
    /// rows moved, in order, before its zero rows.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2HowellForm;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_howell_form(3).to_string(), "[2, 0; 0, 4]");
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_howell_form(3).to_string(),
    ///     "[2, 0, 4; 0, 2, 6; 0, 0, 0]"
    /// );
    ///
    /// // A matrix with more rows than columns
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_howell_form(3).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_howell_form(mut self, pow: u64) -> Self {
        mod_power_of_2_howell_form_assign_helper(&mut self, pow);
        self
    }
}

impl ModPowerOf2HowellForm for &NaturalMatrix {
    type Output = NaturalMatrix;
    /// Computes the Howell form of an [`NaturalMatrix`] modulo $2^k$, taking it by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Howell form of an $h
    /// \times w$ matrix $A$ modulo $2^k$ is the unique $\max(h, w) \times w$ matrix in row echelon
    /// form whose rows span the same module over $\mathbb{Z}/2^k\mathbb{Z}$ as those of $A$, whose
    /// pivots are powers of 2, whose elements above each pivot $2^v$ lie in $[0, 2^v)$, and in
    /// which every element of the span whose first $j$ elements are zero is in the span of the rows
    /// whose pivots are beyond column $j$. Its zero rows come last.
    ///
    /// It is the strong echelon form (see [`ModPowerOf2StrongEchelonForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonForm)) with its nonzero
    /// rows moved, in order, before its zero rows.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2HowellForm;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_howell_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_howell_form(3).to_string(),
    ///     "[2, 0, 4; 0, 2, 6; 0, 0, 0]"
    /// );
    ///
    /// // A matrix with more rows than columns
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_howell_form(3).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_howell_form(self, pow: u64) -> NaturalMatrix {
        self.clone().mod_power_of_2_howell_form(pow)
    }
}

impl ModPowerOf2HowellFormAssign for NaturalMatrix {
    /// Replaces an [`NaturalMatrix`] with its Howell form modulo $2^k$.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Howell form of an $h
    /// \times w$ matrix $A$ modulo $2^k$ is the unique $\max(h, w) \times w$ matrix in row echelon
    /// form whose rows span the same module over $\mathbb{Z}/2^k\mathbb{Z}$ as those of $A$, whose
    /// pivots are powers of 2, whose elements above each pivot $2^v$ lie in $[0, 2^v)$, and in
    /// which every element of the span whose first $j$ elements are zero is in the span of the rows
    /// whose pivots are beyond column $j$. Its zero rows come last.
    ///
    /// It is the strong echelon form (see [`ModPowerOf2StrongEchelonForm`](
    /// malachite_base::num::arithmetic::traits::ModPowerOf2StrongEchelonForm)) with its nonzero
    /// rows moved, in order, before its zero rows.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2HowellFormAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// a.mod_power_of_2_howell_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
    ///
    /// let mut a = NaturalMatrix::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// a.mod_power_of_2_howell_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0, 4; 0, 2, 6; 0, 0, 0]");
    ///
    /// // A matrix with more rows than columns
    /// let mut a = NaturalMatrix::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// a.mod_power_of_2_howell_form_assign(3);
    /// assert_eq!(a.to_string(), "[0, 2; 0, 0; 0, 0]");
    /// ```
    #[inline]
    fn mod_power_of_2_howell_form_assign(&mut self, pow: u64) {
        mod_power_of_2_howell_form_assign_helper(self, pow);
    }
}
