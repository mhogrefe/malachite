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
    ExtendedGcd, ModHowellForm, ModHowellFormAssign, ModIsReduced,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::{
    pad, strong_echelon_form_in_place, swap_rows,
};

fn is_zero_row<T: PrimitiveUnsigned>(xs: &[T], width: usize, i: usize) -> bool {
    xs[i * width..(i + 1) * width].iter().all(|&x| x == T::ZERO)
}

// Puts the `height` by `width` matrix with elements `xs`, which are reduced modulo `m`, into Howell
// form, and returns the number of nonzero rows. `height` must be at least `width`.
//
// This is equivalent to `nmod_mat_howell_form` from `nmod_mat/howell_form.c`, FLINT 3.6.0.
pub(crate) fn howell_form_in_place<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    xs: &mut [U],
    height: usize,
    width: usize,
    m: U,
) -> usize {
    let n = height;
    let mut k = n;
    if height == 0 || width == 0 {
        return 0;
    }
    strong_echelon_form_in_place::<U, S>(xs, height, width, m);
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

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

macro_rules! impl_mod_howell_form {
    ($u:ident, $s:ident) => {
        impl ModHowellForm<$u> for UnsignedMatrix<$u> {
            type Output = Self;

            /// Computes the Howell form of an [`UnsignedMatrix`] modulo $m$, taking it by value.
            ///
            /// The elements of the matrix must already be reduced modulo $m$. The Howell form of an
            /// $h \times w$ matrix $A$ modulo $m$ is the unique $\max(h, w) \times w$ matrix $H$
            /// whose rows span the same $\mathbb{Z}/m\mathbb{Z}$-module as the rows of $A$ and
            /// which has the following properties:
            /// - $H$ is in row echelon form, with its zero rows last;
            /// - the pivot (first nonzero element) of each nonzero row is a divisor of $m$;
            /// - every element above a pivot is less than the pivot;
            /// - for each $j$, every vector in the row span whose first $j$ elements are zero is a
            ///   combination of the rows of $H$ whose pivots are in column $j$ or later.
            ///
            /// A matrix with fewer rows than columns is first given zero rows until it is square,
            /// which leaves its row span unchanged; its Howell form may have more nonzero rows than
            /// the matrix itself. The Howell form is computed by moving the zero rows of the strong
            /// echelon form, as computed by
            /// [`ModStrongEchelonForm`](crate::num::arithmetic::traits::ModStrongEchelonForm), to
            /// the end. The number of nonzero rows of the Howell form is the number of generators
            /// of the row span that it needs.
            ///
            /// # Worst-case complexity
            /// $T(h, w) = O(hw^2)$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's
            /// height and width, and $w$ is its width.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// See [here](super::mod_howell_form#mod_howell_form).
            ///
            /// This is equivalent to `nmod_mat_howell_form` from `nmod_mat/howell_form.c`, FLINT
            /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which
            /// FLINT does not allow, and that it panics if the matrix is not reduced.
            #[inline]
            fn mod_howell_form(mut self, m: $u) -> Self {
                self.mod_howell_form_assign(m);
                self
            }
        }

        impl ModHowellForm<$u> for &UnsignedMatrix<$u> {
            type Output = UnsignedMatrix<$u>;

            /// Computes the Howell form of an [`UnsignedMatrix`] modulo $m$, taking it by
            /// reference.
            ///
            /// See the documentation for the implementation that takes the matrix by value for
            /// details.
            ///
            /// # Worst-case complexity
            /// $T(h, w) = O(hw^2)$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's
            /// height and width, and $w$ is its width.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// See [here](super::mod_howell_form#mod_howell_form).
            ///
            /// This is equivalent to `nmod_mat_howell_form` from `nmod_mat/howell_form.c`, FLINT
            /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which
            /// FLINT does not allow, and that it panics if the matrix is not reduced.
            #[inline]
            fn mod_howell_form(self, m: $u) -> UnsignedMatrix<$u> {
                self.clone().mod_howell_form(m)
            }
        }

        impl ModHowellFormAssign<$u> for UnsignedMatrix<$u> {
            /// Replaces an [`UnsignedMatrix`] with its Howell form modulo $m$.
            ///
            /// See the documentation for the [`ModHowellForm`] implementation that takes the matrix
            /// by value for details.
            ///
            /// # Worst-case complexity
            /// $T(h, w) = O(hw^2)$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the larger of the matrix's
            /// height and width, and $w$ is its width.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// See [here](super::mod_howell_form#mod_howell_form_assign).
            ///
            /// This is equivalent to `nmod_mat_howell_form` from `nmod_mat/howell_form.c`, FLINT
            /// 3.6.0, except that a matrix with fewer rows than columns is given zero rows, which
            /// FLINT does not allow, and that it panics if the matrix is not reduced.
            fn mod_howell_form_assign(&mut self, m: $u) {
                assert_arguments(self, m);
                pad(self);
                howell_form_in_place::<$u, $s>(&mut self.elements, self.height, self.width, m);
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_howell_form);
