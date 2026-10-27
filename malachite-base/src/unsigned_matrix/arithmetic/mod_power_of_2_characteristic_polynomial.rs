// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2CharacteristicPolynomial, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::permanent_over_ring::PermanentElement;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;
use alloc::vec::Vec;

// Replaces the n by n matrix with elements `xs`, which are reduced modulo 2^pow, with a similar
// upper Hessenberg matrix, whose elements below the subdiagonal are 0. For each column `j`, the
// element below the diagonal with the smallest 2-adic valuation is moved to row `j + 1` by swapping
// two rows and the corresponding two columns. Since Z/2^pow is a chain ring, it divides the other
// elements below it, so multiples of row `j + 1` clear them; each row operation is followed by the
// inverse column operation, so the matrix stays similar to the original.
crate_test_fn! {
hessenberg_mod_power_of_2<E: PermanentElement>(xs: &mut [E], n: usize, pow: u64) {
    for j in 0..n.saturating_sub(2) {
        let Some((v, best)) = (j + 1..n)
            .filter(|&i| !xs[i * n + j].is_zero())
            .map(|i| (xs[i * n + j].trailing_zeros(), i))
            .min()
        else {
            continue;
        };
        let p = j + 1;
        if best != p {
            for c in 0..n {
                xs.swap(best * n + c, p * n + c);
            }
            for r in 0..n {
                xs.swap(r * n + best, r * n + p);
            }
        }
        let u_inverse = xs[p * n + j].shr(v).mod_power_of_2_inverse(pow).unwrap();
        for i in j + 2..n {
            if xs[i * n + j].is_zero() {
                continue;
            }
            let mut s = xs[i * n + j].shr(v);
            s.mod_power_of_2_mul_assign(&u_inverse, pow);
            // Row `p` is 0 before column `j`.
            for c in j..n {
                let mut t = xs[p * n + c].clone();
                t.mod_power_of_2_mul_assign(&s, pow);
                xs[i * n + c].mod_power_of_2_sub_assign(&t, pow);
            }
            for r in 0..n {
                let mut t = xs[r * n + i].clone();
                t.mod_power_of_2_mul_assign(&s, pow);
                xs[r * n + p].mod_power_of_2_add_assign(&t, pow);
            }
        }
    }
}}

// The coefficients of the characteristic polynomial of the n by n upper Hessenberg matrix H with
// elements `xs`, which are reduced modulo 2^pow, from the constant coefficient up. Expanding det(xI
// - H_m), for the leading m by m submatrix H_m, along its last column gives p_m(x) = (x -
// h_{m-1,m-1}) p_{m-1}(x) minus the sum, for 1 <= i < m, of h_{m-1-i,m-1} p_{m-1-i}(x) times the
// product of the subdiagonal elements h_{j,j-1} for m - i <= j < m, so no division is needed.
crate_test_fn! {
charpoly_hessenberg_mod_power_of_2<E: PermanentElement>(
    xs: &[E],
    n: usize,
    pow: u64,
) -> Vec<E> {
    let one = E::one().mod_power_of_2(pow);
    let mut ps: Vec<Vec<E>> = Vec::with_capacity(n + 1);
    ps.push(vec![one.clone()]);
    for m in 1..=n {
        let c = m - 1;
        let previous = &ps[c];
        // x p_{m-1} - h_{c,c} p_{m-1}
        let mut p = vec![E::zero(); m + 1];
        for (k, y) in previous.iter().enumerate() {
            p[k + 1].mod_power_of_2_add_assign(y, pow);
            let mut t = y.clone();
            t.mod_power_of_2_mul_assign(&xs[c * n + c], pow);
            p[k].mod_power_of_2_sub_assign(&t, pow);
        }
        let mut product = one.clone();
        for i in 1..m {
            product.mod_power_of_2_mul_assign(&xs[(c + 1 - i) * n + c - i], pow);
            if product.is_zero() {
                break;
            }
            let h = &xs[(c - i) * n + c];
            if h.is_zero() {
                continue;
            }
            let mut factor = product.clone();
            factor.mod_power_of_2_mul_assign(h, pow);
            for (k, y) in ps[c - i].iter().enumerate() {
                let mut t = y.clone();
                t.mod_power_of_2_mul_assign(&factor, pow);
                p[k].mod_power_of_2_sub_assign(&t, pow);
            }
        }
        ps.push(p);
    }
    ps.pop().unwrap()
}}

/// The coefficients of the characteristic polynomial of the n by n matrix with elements `xs`, which
/// are reduced modulo 2^pow, from the constant coefficient up: the matrix is reduced to a similar
/// upper Hessenberg matrix, whose characteristic polynomial follows from a division-free
/// recurrence. Modulo 1, every coefficient is 0.
#[doc(hidden)]
pub fn charpoly_mod_power_of_2<E: PermanentElement>(mut xs: Vec<E>, n: usize, pow: u64) -> Vec<E> {
    if pow == 0 {
        return vec![E::zero(); n + 1];
    }
    hessenberg_mod_power_of_2(&mut xs, n, pow);
    charpoly_hessenberg_mod_power_of_2(&xs, n, pow)
}

fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most the width of the element type, {}, but it is {pow}",
        T::WIDTH
    );
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2CharacteristicPolynomial for UnsignedMatrix<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the characteristic polynomial of a square [`UnsignedMatrix`] modulo $2^k$, taking
    /// it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The characteristic
    /// polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$
    /// whose coefficients are reduced modulo $2^k$, and that of the $0 \times 0$ matrix is 1;
    /// modulo 1, every polynomial is 0. Since reduction modulo $2^k$ is a ring homomorphism, the
    /// result is the integer characteristic polynomial with its coefficients reduced modulo $2^k$.
    ///
    /// The matrix is reduced to a similar upper Hessenberg matrix, whose elements more than one
    /// below the diagonal are 0. Although $\mathbb{Z}/2^k\mathbb{Z}$ is not a field, its ideals
    /// form a chain, so the element below the diagonal of smallest 2-adic valuation in each column
    /// divides the others: it is moved just below the diagonal by swapping two rows and the same
    /// two columns, and then multiples of its row clear the elements below it, each followed by the
    /// inverse column operation. The characteristic polynomial of the Hessenberg matrix follows
    /// from a recurrence over its leading principal submatrices that needs no division.
    ///
    /// $$
    /// f(A, k) = \det(xI - A) \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `pow` is greater than `T::WIDTH`, or if any element
    /// is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CharacteristicPolynomial;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_characteristic_polynomial(3)
    ///         .to_string(),
    ///     "x^2+3*x+6"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_characteristic_polynomial(4)
    ///         .to_string(),
    ///     "x^3+9*x^2+14*x+8"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[3, 1; 4, 1]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_characteristic_polynomial(8)
    ///         .to_string(),
    ///     "x^2+252*x+255"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_characteristic_polynomial(0)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    fn mod_power_of_2_characteristic_polynomial(self, pow: u64) -> UnsignedPolynomial<T> {
        assert_arguments(&self, pow);
        let n = self.height;
        UnsignedPolynomial::from_coefficients_asc(charpoly_mod_power_of_2(self.elements, n, pow))
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2CharacteristicPolynomial for &UnsignedMatrix<T> {
    type Output = UnsignedPolynomial<T>;

    /// Computes the characteristic polynomial of a square [`UnsignedMatrix`] modulo $2^k$, taking
    /// it by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The characteristic
    /// polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic polynomial of degree $n$
    /// whose coefficients are reduced modulo $2^k$, and that of the $0 \times 0$ matrix is 1;
    /// modulo 1, every polynomial is 0. Since reduction modulo $2^k$ is a ring homomorphism, the
    /// result is the integer characteristic polynomial with its coefficients reduced modulo $2^k$.
    ///
    /// The matrix is reduced to a similar upper Hessenberg matrix, whose elements more than one
    /// below the diagonal are 0. Although $\mathbb{Z}/2^k\mathbb{Z}$ is not a field, its ideals
    /// form a chain, so the element below the diagonal of smallest 2-adic valuation in each column
    /// divides the others: it is moved just below the diagonal by swapping two rows and the same
    /// two columns, and then multiples of its row clear the elements below it, each followed by the
    /// inverse column operation. The characteristic polynomial of the Hessenberg matrix follows
    /// from a recurrence over its leading principal submatrices that needs no division.
    ///
    /// $$
    /// f(A, k) = \det(xI - A) \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `pow` is greater than `T::WIDTH`, or if any element
    /// is not reduced modulo $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2CharacteristicPolynomial;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_characteristic_polynomial(3).to_string(),
    ///     "x^2+3*x+6"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_characteristic_polynomial(4).to_string(),
    ///     "x^3+9*x^2+14*x+8"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[3, 1; 4, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_characteristic_polynomial(8).to_string(),
    ///     "x^2+252*x+255"
    /// );
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[0, 0; 0, 0]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_characteristic_polynomial(0).to_string(),
    ///     "0"
    /// );
    /// ```
    fn mod_power_of_2_characteristic_polynomial(self, pow: u64) -> UnsignedPolynomial<T> {
        assert_arguments(self, pow);
        UnsignedPolynomial::from_coefficients_asc(charpoly_mod_power_of_2(
            self.elements.clone(),
            self.height,
            pow,
        ))
    }
}
