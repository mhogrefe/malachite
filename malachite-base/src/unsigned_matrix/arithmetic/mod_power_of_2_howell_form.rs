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

use crate::num::arithmetic::traits::{
    ModPowerOf2HowellForm, ModPowerOf2HowellFormAssign, ModPowerOf2IsReduced,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_howell_form::move_zero_rows_last;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_strong_echelon_form::*;
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::pad;

// Puts the `height` by `width` matrix with elements `xs`, which are reduced modulo 2^pow, into
// Howell form modulo 2^pow, and returns the number of nonzero rows. `height` must be at least
// `width`.
//
// This is equivalent to `nmod_mat_howell_form` from `nmod_mat/howell_form.c`, FLINT 3.6.0, with
// modulus 2^pow.
pub(crate) fn howell_form_mod_power_of_2_in_place<T: PrimitiveUnsigned>(
    xs: &mut [T],
    height: usize,
    width: usize,
    pow: u64,
) -> usize {
    if height == 0 || width == 0 {
        return 0;
    }
    strong_echelon_form_mod_power_of_2_in_place(xs, height, width, pow);
    move_zero_rows_last(xs, height, width)
}

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most the width of the element type, {}, but it is {pow}",
        T::WIDTH
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2HowellForm for UnsignedMatrix<T> {
    type Output = Self;
    /// Computes the Howell form of an [`UnsignedMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Howell form of an $h
    /// \times w$ matrix $A$ modulo $2^k$ is the unique $\max(h, w) \times w$ matrix in row echelon
    /// form whose rows span the same module over $\mathbb{Z}/2^k\mathbb{Z}$ as those of $A$, whose
    /// pivots are powers of 2, whose elements above each pivot $2^v$ lie in $[0, 2^v)$, and in
    /// which every element of the span whose first $j$ elements are zero is in the span of the rows
    /// whose pivots are beyond column $j$. Its zero rows come last.
    ///
    /// It is the strong echelon form (see [`ModPowerOf2StrongEchelonForm`](
    /// crate::num::arithmetic::traits::ModPowerOf2StrongEchelonForm)) with its nonzero rows moved,
    /// in order, before its zero rows.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, and $w$ is its width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2HowellForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_howell_form(3).to_string(), "[2, 0; 0, 4]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_howell_form(3).to_string(),
    ///     "[2, 0, 4; 0, 2, 6; 0, 0, 0]"
    /// );
    ///
    /// // A matrix with more rows than columns
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_howell_form(3).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_howell_form(mut self, pow: u64) -> Self {
        self.mod_power_of_2_howell_form_assign(pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2HowellForm for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;
    /// Computes the Howell form of an [`UnsignedMatrix`] modulo $2^k$, taking it by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Howell form of an $h
    /// \times w$ matrix $A$ modulo $2^k$ is the unique $\max(h, w) \times w$ matrix in row echelon
    /// form whose rows span the same module over $\mathbb{Z}/2^k\mathbb{Z}$ as those of $A$, whose
    /// pivots are powers of 2, whose elements above each pivot $2^v$ lie in $[0, 2^v)$, and in
    /// which every element of the span whose first $j$ elements are zero is in the span of the rows
    /// whose pivots are beyond column $j$. Its zero rows come last.
    ///
    /// It is the strong echelon form (see [`ModPowerOf2StrongEchelonForm`](
    /// crate::num::arithmetic::traits::ModPowerOf2StrongEchelonForm)) with its nonzero rows moved,
    /// in order, before its zero rows.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, and $w$ is its width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2HowellForm;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_howell_form(3).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_howell_form(3).to_string(),
    ///     "[2, 0, 4; 0, 2, 6; 0, 0, 0]"
    /// );
    ///
    /// // A matrix with more rows than columns
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_howell_form(3).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_howell_form(self, pow: u64) -> UnsignedMatrix<T> {
        self.clone().mod_power_of_2_howell_form(pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2HowellFormAssign for UnsignedMatrix<T> {
    /// Replaces an [`UnsignedMatrix`] with its Howell form modulo $2^k$.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The Howell form of an $h
    /// \times w$ matrix $A$ modulo $2^k$ is the unique $\max(h, w) \times w$ matrix in row echelon
    /// form whose rows span the same module over $\mathbb{Z}/2^k\mathbb{Z}$ as those of $A$, whose
    /// pivots are powers of 2, whose elements above each pivot $2^v$ lie in $[0, 2^v)$, and in
    /// which every element of the span whose first $j$ elements are zero is in the span of the rows
    /// whose pivots are beyond column $j$. Its zero rows come last.
    ///
    /// It is the strong echelon form (see [`ModPowerOf2StrongEchelonForm`](
    /// crate::num::arithmetic::traits::ModPowerOf2StrongEchelonForm)) with its nonzero rows moved,
    /// in order, before its zero rows.
    ///
    /// # Worst-case complexity
    /// $T(h, w) = O(hw^2)$
    ///
    /// $M(h, w) = O(hw)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, and $w$ is its width.
    ///
    /// # Panics
    /// Panics if `pow` is greater than the width of `T`, or if any element of the matrix is greater
    /// than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2HowellFormAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 4; 6, 0]").unwrap();
    /// a.mod_power_of_2_howell_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[4, 2, 6; 2, 0, 4]").unwrap();
    /// a.mod_power_of_2_howell_form_assign(3);
    /// assert_eq!(a.to_string(), "[2, 0, 4; 0, 2, 6; 0, 0, 0]");
    ///
    /// // A matrix with more rows than columns
    /// let mut a = UnsignedMatrix::<u8>::from_str("[0, 2; 0, 4; 0, 6]").unwrap();
    /// a.mod_power_of_2_howell_form_assign(3);
    /// assert_eq!(a.to_string(), "[0, 2; 0, 0; 0, 0]");
    /// ```
    fn mod_power_of_2_howell_form_assign(&mut self, pow: u64) {
        assert_arguments(self, pow);
        pad(self);
        howell_form_mod_power_of_2_in_place(&mut self.elements, self.height, self.width, pow);
    }
}
