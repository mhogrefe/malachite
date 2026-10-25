// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_strong_echelon_form::{
    swap_rows, to_limb_matrix, two_rows,
};
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Determinant, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2MulAssign, ModPowerOf2NegAssign, ModPowerOf2SubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};

// The determinant modulo 2^pow of the n by n matrix with elements `xs`, which are reduced modulo
// 2^pow, by elimination over the chain ring Z/2^pow, as for `UnsignedMatrix`s.
fn determinant_mod_power_of_2(xs: &mut [Natural], n: usize, pow: u64) -> Natural {
    let mut det = Natural::ONE.mod_power_of_2(pow);
    let mut valuation = 0;
    for col in 0..n {
        let Some((v, best)) = (col..n)
            .filter_map(|i| xs[i * n + col].trailing_zeros().map(|v| (v, i)))
            .min()
        else {
            return Natural::ZERO;
        };
        if best != col {
            swap_rows(xs, n, col, best);
            det.mod_power_of_2_neg_assign(pow);
        }
        valuation += v;
        if valuation >= pow {
            return Natural::ZERO;
        }
        det.mod_power_of_2_mul_assign(&xs[col * n + col], pow);
        let u_inverse = (&xs[col * n + col] >> v)
            .mod_power_of_2_inverse(pow)
            .unwrap();
        for i in col + 1..n {
            let (pivot_row, other_row) = two_rows(xs, n, col, i);
            if other_row[col] != 0u32 {
                let s = (&other_row[col] >> v).mod_power_of_2_mul(&u_inverse, pow);
                for (y, x) in other_row[col..].iter_mut().zip(&pivot_row[col..]) {
                    y.mod_power_of_2_sub_assign((&s).mod_power_of_2_mul(x, pow), pow);
                }
            }
        }
    }
    det
}

fn determinant_helper(mut a: NaturalMatrix, pow: u64) -> Natural {
    assert_eq!(a.height, a.width, "the matrix must be square");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
    if pow <= Limb::WIDTH {
        Natural::from(to_limb_matrix(&a).mod_power_of_2_determinant(pow))
    } else {
        let n = a.height;
        determinant_mod_power_of_2(&mut a.elements, n, pow)
    }
}

impl ModPowerOf2Determinant for NaturalMatrix {
    type Output = Natural;
    /// Computes the determinant of a square [`NaturalMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The determinant is found by
    /// elimination over the chain ring $\mathbb{Z}/2^k\mathbb{Z}$: the pivot of each column is an
    /// element of smallest 2-adic valuation, which divides the others below it, so subtracting
    /// multiples of the pivot row clears the column without changing the determinant. The
    /// determinant is the product of the pivots, negated for each row swap; it is 0 once the
    /// valuations of the pivots add up to $k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$, or if the matrix is
    /// not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Determinant;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_determinant(3).to_string(), "6");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!(a.mod_power_of_2_determinant(3).to_string(), "7");
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!(a.mod_power_of_2_determinant(3).to_string(), "0");
    /// ```
    #[inline]
    fn mod_power_of_2_determinant(self, pow: u64) -> Natural {
        determinant_helper(self, pow)
    }
}

impl ModPowerOf2Determinant for &NaturalMatrix {
    type Output = Natural;
    /// Computes the determinant of a square [`NaturalMatrix`] modulo $2^k$, taking it by reference.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The determinant is found by
    /// elimination over the chain ring $\mathbb{Z}/2^k\mathbb{Z}$: the pivot of each column is an
    /// element of smallest 2-adic valuation, which divides the others below it, so subtracting
    /// multiples of the pivot row clears the column without changing the determinant. The
    /// determinant is the product of the pivots, negated for each row swap; it is 0 once the
    /// valuations of the pivots add up to $k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if any element of the matrix is greater than or equal to $2^k$, or if the matrix is
    /// not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Determinant;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_determinant(3).to_string(), "6");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 5]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_determinant(3).to_string(), "7");
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 6, 0]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_determinant(3).to_string(), "0");
    /// ```
    #[inline]
    fn mod_power_of_2_determinant(self, pow: u64) -> Natural {
        determinant_helper(self.clone(), pow)
    }
}
