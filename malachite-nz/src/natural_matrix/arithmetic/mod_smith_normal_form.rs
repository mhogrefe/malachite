// Copyright © 2026 Mikhail Hogrefe
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
    to_limb_matrix, transform_pair, two_rows, xgcd_transform,
};
use malachite_base::num::arithmetic::traits::{
    DivExact, DivisibleBy, Gcd, GcdAssign, ModIsReduced, ModNeg, ModSmithNormalForm,
    ModSmithNormalFormAssign,
};
use malachite_base::num::basic::traits::{One, Zero};

// Replaces x and y, which are divisors of `m`, with 0 standing for m, by gcd(x, y) and lcm(x, y),
// again with 0 standing for m.
fn gcd_lcm(x: &mut Natural, y: &mut Natural, m: &Natural) {
    let a = if *x == 0u32 { m } else { &*x };
    let b = if *y == 0u32 { m } else { &*y };
    let g = a.gcd(b);
    let l = a.div_exact(&g) * b;
    *x = if g == *m { Natural::ZERO } else { g };
    *y = if l == *m { Natural::ZERO } else { l };
}

// The transformation of a pair of rows or columns with leading elements `a` and `b` that replaces
// `a` with gcd(a, b) and `b` with 0, as (s, t, u, v), with (x, y) becoming (sx + ty, ux + vy)
// modulo `m`; or `None` if `b` is 0. When `a` is nonzero and divides `b`, the transformation
// subtracts a multiple of the first from the second and leaves the first unchanged.
fn elimination(
    a: &Natural,
    b: &Natural,
    m: &Natural,
) -> Option<(Natural, Natural, Natural, Natural)> {
    if *b == 0u32 {
        None
    } else if *a != 0u32 && b.divisible_by(a) {
        Some((
            Natural::ONE,
            Natural::ZERO,
            b.div_exact(a).mod_neg(m),
            Natural::ONE,
        ))
    } else {
        let (_, s, t, u, v) = xgcd_transform(a, b, m);
        Some((s, t, u, v))
    }
}

// Replaces the `height` by `width` matrix with elements `xs`, which are reduced modulo `m`, with
// its Smith normal form modulo `m`, as for `UnsignedMatrix`s.
fn smith_normal_form_mod_in_place(xs: &mut [Natural], height: usize, width: usize, m: &Natural) {
    let (h, w) = (height, width);
    let d = h.min(w);
    for t in 0..d {
        loop {
            // Clear the column below the diagonal element.
            for i in t + 1..h {
                let Some((s, tt, u, v)) = elimination(&xs[t * w + t], &xs[i * w + t], m) else {
                    continue;
                };
                let (pivot_row, other_row) = two_rows(xs, w, t, i);
                for (x, y) in pivot_row[t..].iter_mut().zip(&mut other_row[t..]) {
                    (*x, *y) = transform_pair(x, y, (&s, &tt, &u, &v), m);
                }
            }
            // Clear the row to the right of the diagonal element.
            for j in t + 1..w {
                let Some((s, tt, u, v)) = elimination(&xs[t * w + t], &xs[t * w + j], m) else {
                    continue;
                };
                for row in xs.chunks_exact_mut(w).skip(t) {
                    let (left, right) = row.split_at_mut(j);
                    (left[t], right[0]) = transform_pair(&left[t], &right[0], (&s, &tt, &u, &v), m);
                }
            }
            if (t + 1..h).all(|i| xs[i * w + t] == 0u32) {
                break;
            }
        }
        let x = &mut xs[t * w + t];
        x.gcd_assign(m);
        if *x == *m {
            *x = Natural::ZERO;
        }
    }
    for j in (0..d).rev() {
        for i in 0..j {
            let (left, right) = xs.split_at_mut((i + 1) * (w + 1));
            gcd_lcm(&mut left[i * (w + 1)], &mut right[0], m);
        }
    }
}

fn mod_smith_normal_form_assign_helper(a: &mut NaturalMatrix, m: &Natural) {
    assert_ne!(*m, 0u32, "m cannot be zero");
    assert!(
        a.mod_is_reduced(m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
    if let Natural(Small(small_m)) = *m {
        *a = NaturalMatrix::from(to_limb_matrix(a).mod_smith_normal_form(small_m));
    } else {
        let (h, w) = (a.height, a.width);
        smith_normal_form_mod_in_place(&mut a.elements, h, w, m);
    }
}

impl ModSmithNormalForm<Natural> for NaturalMatrix {
    type Output = Self;
    /// Computes the Smith normal form of an [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// value and the modulus by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form of a
    /// matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible modulo
    /// $m$, whose diagonal elements are divisors of $m$ that each divide the next, with 0 standing
    /// for $m$. If $d_i$ are the diagonal elements of the Smith normal form of $A$ over the
    /// integers, those modulo $m$ are $\gcd(d_i, m)$.
    ///
    /// For each diagonal position, the elements below it are cleared by row operations that combine
    /// pairs of rows with extended GCDs, and then those to its right by column operations, until
    /// the column stays clear; the elements remain reduced modulo $m$ throughout. Each diagonal
    /// element $d$ is then replaced by $\gcd(d, m)$, an associate of $d$ modulo $m$, and pairs of
    /// diagonal elements that violate the divisibility condition are replaced by their GCDs and
    /// LCMs. Moduli that fit in a limb are handled as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(\min(h, w)(h + w)^2 n^2 \log^2 n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSmithNormalForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_smith_normal_form(Natural::from(6u32)).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 2, 8]").unwrap();
    /// assert_eq!(
    ///     a.mod_smith_normal_form(Natural::from(12u32)).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// // The rank modulo 12 is 1
    /// let a = NaturalMatrix::from_str("[2, 3; 4, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_smith_normal_form(Natural::from(12u32)).to_string(),
    ///     "[1, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_smith_normal_form(mut self, m: Natural) -> Self {
        mod_smith_normal_form_assign_helper(&mut self, &m);
        self
    }
}

impl ModSmithNormalForm<&Natural> for NaturalMatrix {
    type Output = Self;
    /// Computes the Smith normal form of an [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// value and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form of a
    /// matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible modulo
    /// $m$, whose diagonal elements are divisors of $m$ that each divide the next, with 0 standing
    /// for $m$. If $d_i$ are the diagonal elements of the Smith normal form of $A$ over the
    /// integers, those modulo $m$ are $\gcd(d_i, m)$.
    ///
    /// For each diagonal position, the elements below it are cleared by row operations that combine
    /// pairs of rows with extended GCDs, and then those to its right by column operations, until
    /// the column stays clear; the elements remain reduced modulo $m$ throughout. Each diagonal
    /// element $d$ is then replaced by $\gcd(d, m)$, an associate of $d$ modulo $m$, and pairs of
    /// diagonal elements that violate the divisibility condition are replaced by their GCDs and
    /// LCMs. Moduli that fit in a limb are handled as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(\min(h, w)(h + w)^2 n^2 \log^2 n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSmithNormalForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.mod_smith_normal_form(&Natural::from(6u32)).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 2, 8]").unwrap();
    /// assert_eq!(
    ///     a.mod_smith_normal_form(&Natural::from(12u32)).to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// // The rank modulo 12 is 1
    /// let a = NaturalMatrix::from_str("[2, 3; 4, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_smith_normal_form(&Natural::from(12u32)).to_string(),
    ///     "[1, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_smith_normal_form(mut self, m: &Natural) -> Self {
        mod_smith_normal_form_assign_helper(&mut self, m);
        self
    }
}

impl ModSmithNormalForm<Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;
    /// Computes the Smith normal form of an [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form of a
    /// matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible modulo
    /// $m$, whose diagonal elements are divisors of $m$ that each divide the next, with 0 standing
    /// for $m$. If $d_i$ are the diagonal elements of the Smith normal form of $A$ over the
    /// integers, those modulo $m$ are $\gcd(d_i, m)$.
    ///
    /// For each diagonal position, the elements below it are cleared by row operations that combine
    /// pairs of rows with extended GCDs, and then those to its right by column operations, until
    /// the column stays clear; the elements remain reduced modulo $m$ throughout. Each diagonal
    /// element $d$ is then replaced by $\gcd(d, m)$, an associate of $d$ modulo $m$, and pairs of
    /// diagonal elements that violate the divisibility condition are replaced by their GCDs and
    /// LCMs. Moduli that fit in a limb are handled as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(\min(h, w)(h + w)^2 n^2 \log^2 n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSmithNormalForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_smith_normal_form(&Natural::from(6u32)).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 2, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_smith_normal_form(&Natural::from(12u32))
    ///         .to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// // The rank modulo 12 is 1
    /// let a = NaturalMatrix::from_str("[2, 3; 4, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_smith_normal_form(&Natural::from(12u32))
    ///         .to_string(),
    ///     "[1, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_smith_normal_form(self, m: Natural) -> NaturalMatrix {
        self.clone().mod_smith_normal_form(&m)
    }
}

impl ModSmithNormalForm<&Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;
    /// Computes the Smith normal form of an [`NaturalMatrix`] modulo $m$, taking the matrix by
    /// reference and the modulus by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form of a
    /// matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible modulo
    /// $m$, whose diagonal elements are divisors of $m$ that each divide the next, with 0 standing
    /// for $m$. If $d_i$ are the diagonal elements of the Smith normal form of $A$ over the
    /// integers, those modulo $m$ are $\gcd(d_i, m)$.
    ///
    /// For each diagonal position, the elements below it are cleared by row operations that combine
    /// pairs of rows with extended GCDs, and then those to its right by column operations, until
    /// the column stays clear; the elements remain reduced modulo $m$ throughout. Each diagonal
    /// element $d$ is then replaced by $\gcd(d, m)$, an associate of $d$ modulo $m$, and pairs of
    /// diagonal elements that violate the divisibility condition are replaced by their GCDs and
    /// LCMs. Moduli that fit in a limb are handled as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(\min(h, w)(h + w)^2 n^2 \log^2 n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSmithNormalForm;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_smith_normal_form(&Natural::from(6u32)).to_string(),
    ///     "[1, 0; 0, 2]"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[6, 4; 2, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_smith_normal_form(&Natural::from(12u32))
    ///         .to_string(),
    ///     "[2, 0; 0, 4]"
    /// );
    ///
    /// // The rank modulo 12 is 1
    /// let a = NaturalMatrix::from_str("[2, 3; 4, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_smith_normal_form(&Natural::from(12u32))
    ///         .to_string(),
    ///     "[1, 0; 0, 0]"
    /// );
    /// ```
    #[inline]
    fn mod_smith_normal_form(self, m: &Natural) -> NaturalMatrix {
        self.clone().mod_smith_normal_form(m)
    }
}

impl ModSmithNormalFormAssign<Natural> for NaturalMatrix {
    /// Replaces an [`NaturalMatrix`] with its Smith normal form modulo $m$, taking the modulus by
    /// value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form of a
    /// matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible modulo
    /// $m$, whose diagonal elements are divisors of $m$ that each divide the next, with 0 standing
    /// for $m$. If $d_i$ are the diagonal elements of the Smith normal form of $A$ over the
    /// integers, those modulo $m$ are $\gcd(d_i, m)$.
    ///
    /// For each diagonal position, the elements below it are cleared by row operations that combine
    /// pairs of rows with extended GCDs, and then those to its right by column operations, until
    /// the column stays clear; the elements remain reduced modulo $m$ throughout. Each diagonal
    /// element $d$ is then replaced by $\gcd(d, m)$, an associate of $d$ modulo $m$, and pairs of
    /// diagonal elements that violate the divisibility condition are replaced by their GCDs and
    /// LCMs. Moduli that fit in a limb are handled as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(\min(h, w)(h + w)^2 n^2 \log^2 n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSmithNormalFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_smith_normal_form_assign(Natural::from(6u32));
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[6, 4; 2, 8]").unwrap();
    /// a.mod_smith_normal_form_assign(Natural::from(12u32));
    /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
    ///
    /// // The rank modulo 12 is 1
    /// let mut a = NaturalMatrix::from_str("[2, 3; 4, 6]").unwrap();
    /// a.mod_smith_normal_form_assign(Natural::from(12u32));
    /// assert_eq!(a.to_string(), "[1, 0; 0, 0]");
    /// ```
    #[inline]
    fn mod_smith_normal_form_assign(&mut self, m: Natural) {
        mod_smith_normal_form_assign_helper(self, &m);
    }
}

impl ModSmithNormalFormAssign<&Natural> for NaturalMatrix {
    /// Replaces an [`NaturalMatrix`] with its Smith normal form modulo $m$, taking the modulus by
    /// reference.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The Smith normal form of a
    /// matrix $A$ modulo $m$ is the unique diagonal matrix $UAV$, for $U$ and $V$ invertible modulo
    /// $m$, whose diagonal elements are divisors of $m$ that each divide the next, with 0 standing
    /// for $m$. If $d_i$ are the diagonal elements of the Smith normal form of $A$ over the
    /// integers, those modulo $m$ are $\gcd(d_i, m)$.
    ///
    /// For each diagonal position, the elements below it are cleared by row operations that combine
    /// pairs of rows with extended GCDs, and then those to its right by column operations, until
    /// the column stays clear; the elements remain reduced modulo $m$ throughout. Each diagonal
    /// element $d$ is then replaced by $\gcd(d, m)$, an associate of $d$ modulo $m$, and pairs of
    /// diagonal elements that violate the divisibility condition are replaced by their GCDs and
    /// LCMs. Moduli that fit in a limb are handled as for
    /// [`UnsignedMatrix`](malachite_base::unsigned_matrix::UnsignedMatrix)es of limbs.
    ///
    /// # Worst-case complexity
    /// $T(h, w, n) = O(\min(h, w)(h + w)^2 n^2 \log^2 n \log\log n)$
    ///
    /// $M(h, w, n) = O(hwn)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $n$ is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, or if any element of the matrix is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSmithNormalFormAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_smith_normal_form_assign(&Natural::from(6u32));
    /// assert_eq!(a.to_string(), "[1, 0; 0, 2]");
    ///
    /// let mut a = NaturalMatrix::from_str("[6, 4; 2, 8]").unwrap();
    /// a.mod_smith_normal_form_assign(&Natural::from(12u32));
    /// assert_eq!(a.to_string(), "[2, 0; 0, 4]");
    ///
    /// // The rank modulo 12 is 1
    /// let mut a = NaturalMatrix::from_str("[2, 3; 4, 6]").unwrap();
    /// a.mod_smith_normal_form_assign(&Natural::from(12u32));
    /// assert_eq!(a.to_string(), "[1, 0; 0, 0]");
    /// ```
    #[inline]
    fn mod_smith_normal_form_assign(&mut self, m: &Natural) {
        mod_smith_normal_form_assign_helper(self, m);
    }
}
