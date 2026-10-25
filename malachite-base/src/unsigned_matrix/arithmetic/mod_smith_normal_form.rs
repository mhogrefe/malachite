// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ExtendedGcd, ModIsReduced, ModSmithNormalForm, ModSmithNormalFormAssign,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_strong_echelon_form::{two_rows, xgcd_transform};

// The transformation of a pair of rows or columns with leading elements `a` and `b` that replaces
// `a` with gcd(a, b) and `b` with 0, as (s, t, u, v), with (x, y) becoming (sx + ty, ux + vy)
// modulo `m`; or `None` if `b` is 0. When `a` is nonzero and divides `b`, the transformation
// subtracts a multiple of the first from the second and leaves the first unchanged, so that a pivot
// that divides the rest of its row and column stays in place.
fn elimination<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    a: U,
    b: U,
    m: U,
) -> Option<(U, U, U, U)> {
    if b == U::ZERO {
        None
    } else if a != U::ZERO && b.divisible_by(a) {
        Some((U::ONE, U::ZERO, (b / a).mod_neg(m), U::ONE))
    } else {
        let (_, s, t, u, v) = xgcd_transform::<U, S>(a, b, m);
        Some((s, t, u, v))
    }
}

// Replaces the pair (x, y) with (sx + ty, ux + vy) modulo `m`.
fn transform<U: PrimitiveUnsigned>(x: &mut U, y: &mut U, (s, t, u, v): (U, U, U, U), m: U) {
    (*x, *y) = (
        s.mod_mul(*x, m).mod_add(t.mod_mul(*y, m), m),
        u.mod_mul(*x, m).mod_add(v.mod_mul(*y, m), m),
    );
}

// Replaces x and y, which are divisors of `m`, with 0 standing for m, by gcd(x, y) and lcm(x, y),
// again with 0 standing for m.
fn gcd_lcm<U: PrimitiveUnsigned>(x: &mut U, y: &mut U, m: U) {
    let a = if *x == U::ZERO { m } else { *x };
    let b = if *y == U::ZERO { m } else { *y };
    let g = a.gcd(b);
    let l = a / g * b;
    *x = if g == m { U::ZERO } else { g };
    *y = if l == m { U::ZERO } else { l };
}

// Replaces the `height` by `width` matrix with elements `xs`, which are reduced modulo `m`, with
// its Smith normal form modulo `m`. For each diagonal position, the elements below it are cleared
// by row operations combining pairs of rows with extended GCDs, and then those to its right by
// column operations, until the column stays clear; the entries remain reduced modulo `m`
// throughout. This terminates because, as integers, the diagonal element either divides the element
// being cleared, and stays in place, or is replaced by a proper divisor of itself. Each diagonal
// element d is then replaced by gcd(d, m), an associate of d in Z/m, and pairs of diagonal elements
// that violate the divisibility condition are replaced by their GCDs and LCMs, with 0 standing for
// m.
pub(crate) fn smith_normal_form_mod_in_place<
    U: PrimitiveUnsigned + ExtendedGcd<U, Gcd = U, Cofactor = S> + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    xs: &mut [U],
    height: usize,
    width: usize,
    m: U,
) {
    let (h, w) = (height, width);
    let d = h.min(w);
    for t in 0..d {
        loop {
            // Clear the column below the diagonal element.
            for i in t + 1..h {
                let Some(transformation) = elimination::<U, S>(xs[t * w + t], xs[i * w + t], m)
                else {
                    continue;
                };
                let (pivot_row, other_row) = two_rows(xs, w, t, i);
                for (x, y) in pivot_row[t..].iter_mut().zip(&mut other_row[t..]) {
                    transform(x, y, transformation, m);
                }
            }
            // Clear the row to the right of the diagonal element.
            for j in t + 1..w {
                let Some(transformation) = elimination::<U, S>(xs[t * w + t], xs[t * w + j], m)
                else {
                    continue;
                };
                for row in xs.chunks_exact_mut(w).skip(t) {
                    let (left, right) = row.split_at_mut(j);
                    transform(&mut left[t], &mut right[0], transformation, m);
                }
            }
            if (t + 1..h).all(|i| xs[i * w + t] == U::ZERO) {
                break;
            }
        }
        let x = &mut xs[t * w + t];
        x.gcd_assign(m);
        if *x == m {
            *x = U::ZERO;
        }
    }
    for j in (0..d).rev() {
        for i in 0..j {
            let (left, right) = xs.split_at_mut((i + 1) * (w + 1));
            gcd_lcm(&mut left[i * (w + 1)], &mut right[0], m);
        }
    }
}

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

macro_rules! impl_mod_smith_normal_form {
    ($u:ident, $s:ident) => {
        impl ModSmithNormalForm<$u> for UnsignedMatrix<$u> {
            type Output = Self;
            /// Computes the Smith normal form of an [`UnsignedMatrix`] modulo $m$, taking the
            /// matrix by value.
            ///
            /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form
            /// of a matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$
            /// invertible modulo $m$, whose diagonal elements are divisors of $m$ that each divide
            /// the next, with 0 standing for $m$. If $d_i$ are the diagonal elements of the Smith
            /// normal form of $A$ over the integers, those modulo $m$ are $\gcd(d_i, m)$.
            ///
            /// For each diagonal position, the elements below it are cleared by row operations that
            /// combine pairs of rows with extended GCDs, and then those to its right by column
            /// operations, until the column stays clear; the elements remain reduced modulo $m$
            /// throughout. Each diagonal element $d$ is then replaced by $\gcd(d, m)$, an associate
            /// of $d$ modulo $m$, and pairs of diagonal elements that violate the divisibility
            /// condition are replaced by their GCDs and LCMs.
            ///
            /// # Worst-case complexity
            /// $T(h, w, n) = O(\min(h, w)(h + w)n(h + w + n))$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is
            /// its width, and $n$ is `m.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// ```
            /// use core::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModSmithNormalForm;
            /// use malachite_base::unsigned_matrix::UnsignedMatrix;
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
            /// assert_eq!(a.mod_smith_normal_form(6).to_string(), "[1, 0; 0, 2]");
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 2, 8]").unwrap();
            /// assert_eq!(a.mod_smith_normal_form(12).to_string(), "[2, 0; 0, 4]");
            ///
            /// // The rank modulo 12 is 1
            /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 4, 6]").unwrap();
            /// assert_eq!(a.mod_smith_normal_form(12).to_string(), "[1, 0; 0, 0]");
            /// ```
            #[inline]
            fn mod_smith_normal_form(mut self, m: $u) -> Self {
                self.mod_smith_normal_form_assign(m);
                self
            }
        }

        impl ModSmithNormalForm<$u> for &UnsignedMatrix<$u> {
            type Output = UnsignedMatrix<$u>;
            /// Computes the Smith normal form of an [`UnsignedMatrix`] modulo $m$, taking the
            /// matrix by reference.
            ///
            /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form
            /// of a matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$
            /// invertible modulo $m$, whose diagonal elements are divisors of $m$ that each divide
            /// the next, with 0 standing for $m$. If $d_i$ are the diagonal elements of the Smith
            /// normal form of $A$ over the integers, those modulo $m$ are $\gcd(d_i, m)$.
            ///
            /// For each diagonal position, the elements below it are cleared by row operations that
            /// combine pairs of rows with extended GCDs, and then those to its right by column
            /// operations, until the column stays clear; the elements remain reduced modulo $m$
            /// throughout. Each diagonal element $d$ is then replaced by $\gcd(d, m)$, an associate
            /// of $d$ modulo $m$, and pairs of diagonal elements that violate the divisibility
            /// condition are replaced by their GCDs and LCMs.
            ///
            /// # Worst-case complexity
            /// $T(h, w, n) = O(\min(h, w)(h + w)n(h + w + n))$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is
            /// its width, and $n$ is `m.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// ```
            /// use core::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModSmithNormalForm;
            /// use malachite_base::unsigned_matrix::UnsignedMatrix;
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
            /// assert_eq!((&a).mod_smith_normal_form(6).to_string(), "[1, 0; 0, 2]");
            ///
            /// let a = UnsignedMatrix::<u8>::from_str("[6, 4; 2, 8]").unwrap();
            /// assert_eq!((&a).mod_smith_normal_form(12).to_string(), "[2, 0; 0, 4]");
            ///
            /// // The rank modulo 12 is 1
            /// let a = UnsignedMatrix::<u8>::from_str("[2, 3; 4, 6]").unwrap();
            /// assert_eq!((&a).mod_smith_normal_form(12).to_string(), "[1, 0; 0, 0]");
            /// ```
            #[inline]
            fn mod_smith_normal_form(self, m: $u) -> UnsignedMatrix<$u> {
                self.clone().mod_smith_normal_form(m)
            }
        }

        impl ModSmithNormalFormAssign<$u> for UnsignedMatrix<$u> {
            /// Replaces an [`UnsignedMatrix`] with its Smith normal form modulo $m$.
            ///
            /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form
            /// of a matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$
            /// invertible modulo $m$, whose diagonal elements are divisors of $m$ that each divide
            /// the next, with 0 standing for $m$. If $d_i$ are the diagonal elements of the Smith
            /// normal form of $A$ over the integers, those modulo $m$ are $\gcd(d_i, m)$.
            ///
            /// For each diagonal position, the elements below it are cleared by row operations that
            /// combine pairs of rows with extended GCDs, and then those to its right by column
            /// operations, until the column stays clear; the elements remain reduced modulo $m$
            /// throughout. Each diagonal element $d$ is then replaced by $\gcd(d, m)$, an associate
            /// of $d$ modulo $m$, and pairs of diagonal elements that violate the divisibility
            /// condition are replaced by their GCDs and LCMs.
            ///
            /// # Worst-case complexity
            /// $T(h, w, n) = O(\min(h, w)(h + w)n(h + w + n))$
            ///
            /// $M(h, w) = O(hw)$
            ///
            /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is
            /// its width, and $n$ is `m.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to
            /// `m`.
            ///
            /// # Examples
            /// ```
            /// use core::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModSmithNormalFormAssign;
            /// use malachite_base::unsigned_matrix::UnsignedMatrix;
            ///
            /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
            /// a.mod_smith_normal_form_assign(6);
            /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
            ///
            /// let mut a = UnsignedMatrix::<u8>::from_str("[6, 4; 2, 8]").unwrap();
            /// a.mod_smith_normal_form_assign(12);
            /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
            ///
            /// // The rank modulo 12 is 1
            /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 3; 4, 6]").unwrap();
            /// a.mod_smith_normal_form_assign(12);
            /// assert_eq!(a.to_string(), "[1, 0; 0, 0]");
            /// ```
            fn mod_smith_normal_form_assign(&mut self, m: $u) {
                assert_arguments(self, m);
                smith_normal_form_mod_in_place::<$u, $s>(
                    &mut self.elements,
                    self.height,
                    self.width,
                    m,
                );
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_smith_normal_form);
