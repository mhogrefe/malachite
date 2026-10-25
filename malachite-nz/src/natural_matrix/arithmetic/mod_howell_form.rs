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

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::{
    pad, strong_echelon_form_in_place, swap_rows, to_limb_matrix,
};
use malachite_base::num::arithmetic::traits::{ModHowellForm, ModHowellFormAssign, ModIsReduced};

fn is_zero_row(xs: &[Natural], width: usize, i: usize) -> bool {
    xs[i * width..(i + 1) * width].iter().all(|x| *x == 0u32)
}

// Puts the `height` by `width` matrix with elements `xs`, which are reduced modulo `m`, into Howell
// form, and returns the number of nonzero rows. `height` must be at least `width`.
//
// This is equivalent to `fmpz_mat_howell_form_mod` from `fmpz_mat/howell_form_mod.c`, FLINT 3.6.0.
pub(crate) fn howell_form_in_place(
    xs: &mut [Natural],
    height: usize,
    width: usize,
    m: &Natural,
) -> usize {
    if height == 0 || width == 0 {
        return 0;
    }
    strong_echelon_form_in_place(xs, height, width, m);
    move_zero_rows_last(xs, height, width)
}

// Moves the zero rows of the `height` by `width` matrix with elements `xs`, which is in strong
// echelon form, after its nonzero rows, keeping the nonzero rows in order, and returns the number
// of nonzero rows.
//
// This is equivalent to the second half of `fmpz_mat_howell_form_mod` from
// `fmpz_mat/howell_form_mod.c`, FLINT 3.6.0.
pub(crate) fn move_zero_rows_last(xs: &mut [Natural], height: usize, width: usize) -> usize {
    let n = height;
    let mut k = n;
    for i in 0..n {
        if is_zero_row(xs, width, i) {
            k -= 1;
            for j in i + 1..n {
                if !is_zero_row(xs, width, j) {
                    swap_rows(xs, width, i, j);
                    k += 1;
                    break;
                }
            }
        }
    }
    k
}

fn mod_howell_form_assign_helper(a: &mut NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    if let Natural(Small(small_m)) = *m {
        *a = NaturalMatrix::from(to_limb_matrix(a).mod_howell_form(small_m));
    } else {
        pad(a);
        howell_form_in_place(&mut a.elements, a.height, a.width, m);
    }
}

impl ModHowellForm<Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the Howell form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix
    /// by value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The Howell form of an $h
    /// \times w$ matrix $A$ modulo $m$ is the unique $\max(h, w) \times w$ matrix $H$ whose rows
    /// span the same $\mathbb{Z}/m\mathbb{Z}$-module as the rows of $A$ and which has the following
    /// properties:
    /// - $H$ is in row echelon form, with its zero rows last;
    /// - the pivot (first nonzero element) of each nonzero row is a divisor of $m$;
    /// - every element above a pivot is less than the pivot;
    /// - for each $j$, every vector in the row span whose first $j$ elements are zero is a
    ///   combination of the rows of $H$ whose pivots are in column $j$ or later.
    ///
    /// A matrix with fewer rows than columns is first given zero rows until it is square, which
    /// leaves its row span unchanged; its Howell form may have more nonzero rows than the matrix
    /// itself. The Howell form is computed by moving the zero rows of the strong echelon form, as
    /// computed by
    /// [`ModStrongEchelonForm`](malachite_base::num::arithmetic::traits::ModStrongEchelonForm), to
    /// the end. The number of nonzero rows of the Howell form is the number of generators of the
    /// row span that it needs.
    ///
    /// When $m$ fits in a limb, the form is computed as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModHowellForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(Natural::from(8u32)).to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(Natural::from(4u32)).to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // The zero rows come last.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(Natural::from(4u32)).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(Natural::from(9u32)).to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!(a.mod_howell_form(m).to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_howell_form_mod` from `fmpz_mat/howell_form_mod.c`, FLINT
    /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which FLINT
    /// does not allow, and that it panics if the matrix is not reduced.
    #[inline]
    fn mod_howell_form(mut self, m: Natural) -> Self {
        mod_howell_form_assign_helper(&mut self, &m);
        self
    }
}

impl ModHowellForm<&Natural> for NaturalMatrix {
    type Output = Self;

    /// Computes the Howell form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix
    /// by value and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModHowellForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(&Natural::from(8u32)).to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(&Natural::from(4u32)).to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // The zero rows come last.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(&Natural::from(4u32)).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     a.mod_howell_form(&Natural::from(9u32)).to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!(a.mod_howell_form(&m).to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_howell_form_mod` from `fmpz_mat/howell_form_mod.c`, FLINT
    /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which FLINT
    /// does not allow, and that it panics if the matrix is not reduced.
    #[inline]
    fn mod_howell_form(mut self, m: &Natural) -> Self {
        mod_howell_form_assign_helper(&mut self, m);
        self
    }
}

impl ModHowellForm<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Howell form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix
    /// by reference and the modulus by value.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModHowellForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(Natural::from(8u32)).to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(Natural::from(4u32)).to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // The zero rows come last.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(Natural::from(4u32)).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(Natural::from(9u32)).to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!((&a).mod_howell_form(m).to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_howell_form_mod` from `fmpz_mat/howell_form_mod.c`, FLINT
    /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which FLINT
    /// does not allow, and that it panics if the matrix is not reduced.
    #[inline]
    fn mod_howell_form(self, m: Natural) -> NaturalMatrix {
        self.clone().mod_howell_form(m)
    }
}

impl ModHowellForm<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Computes the Howell form of a [`NaturalMatrix`] modulo a [`Natural`] $m$, taking the matrix
    /// by reference and the modulus by reference.
    ///
    /// See the documentation for the implementation that takes both arguments by value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModHowellForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(&Natural::from(8u32)).to_string(),
    ///     "[2, 0; 0, 2]"
    /// );
    ///
    /// // A matrix with fewer rows than columns may gain nonzero rows: here 2 * [2, 1] = [0, 2].
    /// let a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(&Natural::from(4u32)).to_string(),
    ///     "[2, 1; 0, 2]"
    /// );
    ///
    /// // The zero rows come last.
    /// let a = NaturalMatrix::from_str("[0, 2; 0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(&Natural::from(4u32)).to_string(),
    ///     "[0, 2; 0, 0; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[3, 6; 6, 3]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_howell_form(&Natural::from(9u32)).to_string(),
    ///     "[3, 6; 0, 0]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// assert_eq!((&a).mod_howell_form(&m).to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_howell_form_mod` from `fmpz_mat/howell_form_mod.c`, FLINT
    /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which FLINT
    /// does not allow, and that it panics if the matrix is not reduced.
    #[inline]
    fn mod_howell_form(self, m: &Natural) -> NaturalMatrix {
        self.clone().mod_howell_form(m)
    }
}

impl ModHowellFormAssign<Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Howell form modulo a [`Natural`] $m$, taking the
    /// modulus by value.
    ///
    /// See the documentation for the [`ModHowellForm`] implementation that takes both arguments by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModHowellFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// a.mod_howell_form_assign(Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[2, 0; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// a.mod_howell_form_assign(Natural::from(4u32));
    /// assert_eq!(a.to_string(), "[2, 1; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// a.mod_howell_form_assign(m);
    /// assert_eq!(a.to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_howell_form_mod` from `fmpz_mat/howell_form_mod.c`, FLINT
    /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which FLINT
    /// does not allow, and that it panics if the matrix is not reduced.
    #[inline]
    fn mod_howell_form_assign(&mut self, m: Natural) {
        mod_howell_form_assign_helper(self, &m);
    }
}

impl ModHowellFormAssign<&Natural> for NaturalMatrix {
    /// Replaces a [`NaturalMatrix`] with its Howell form modulo a [`Natural`] $m$, taking the
    /// modulus by reference.
    ///
    /// See the documentation for the [`ModHowellForm`] implementation that takes both arguments by
    /// value for details.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(hw^2 n \log n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's height and
    /// width, $w$ is its width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModHowellFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[4, 2; 2, 0]").unwrap();
    /// a.mod_howell_form_assign(&Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[2, 0; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 1]").unwrap();
    /// a.mod_howell_form_assign(&Natural::from(4u32));
    /// assert_eq!(a.to_string(), "[2, 1; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[100000000000000000005, 2; 10000000000000000000, 25]")
    ///     .unwrap();
    /// let m = Natural::from_str("1000000000000000000000").unwrap();
    /// a.mod_howell_form_assign(&m);
    /// assert_eq!(a.to_string(), "[5, 2; 0, 25]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_howell_form_mod` from `fmpz_mat/howell_form_mod.c`, FLINT
    /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which FLINT
    /// does not allow, and that it panics if the matrix is not reduced.
    #[inline]
    fn mod_howell_form_assign(&mut self, m: &Natural) {
        mod_howell_form_assign_helper(self, m);
    }
}
