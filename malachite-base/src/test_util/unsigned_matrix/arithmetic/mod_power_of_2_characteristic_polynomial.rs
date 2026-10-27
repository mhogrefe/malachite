// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::ModPowerOf2Mul;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::Polynomial;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::berkowitz_over_ring::{
    BerkowitzRing, charpoly_berkowitz_over_ring,
};
use crate::unsigned_matrix::arithmetic::permanent_over_ring::{
    ModPowerOf2Ring, PermanentElement, PermanentModRing, PermanentRing,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// The integers modulo 2^pow, for Berkowitz's algorithm, which needs no division and so works modulo
// any power of 2. The dot products are reduced after every step.
impl<E: PermanentElement> BerkowitzRing for ModPowerOf2Ring<E> {
    type Element = E;

    fn zero(&self) -> E {
        E::zero()
    }

    fn one(&self) -> E {
        PermanentRing::one(self)
    }

    fn neg(&self, x: &E) -> E {
        let mut y = E::zero();
        PermanentModRing::sub_assign(self, &mut y, x);
        y
    }

    fn add(&self, x: &E, y: &E) -> E {
        let mut z = x.clone();
        PermanentRing::add_assign(self, &mut z, y);
        z
    }

    fn sub_assign(&self, x: &mut E, y: &E) {
        PermanentModRing::sub_assign(self, x, y);
    }

    fn mul(&self, x: &E, y: &E) -> E {
        let mut z = x.clone();
        PermanentRing::mul_assign(self, &mut z, y);
        z
    }

    fn dot(&self, xs: &[E], ys: &[E]) -> E {
        let mut s = E::zero();
        for (x, y) in xs.iter().zip(ys) {
            PermanentRing::add_assign(self, &mut s, &BerkowitzRing::mul(self, x, y));
        }
        s
    }

    fn dot_rev(&self, xs: &[E], ys: &[E]) -> E {
        let mut s = E::zero();
        for (x, y) in xs.iter().zip(ys.iter().rev()) {
            PermanentRing::add_assign(self, &mut s, &BerkowitzRing::mul(self, x, y));
        }
        s
    }
}

// The coefficients of the characteristic polynomial modulo 2^pow of the n by n matrix with elements
// `xs`, which are reduced modulo 2^pow, from the constant coefficient up, by Berkowitz's
// division-free algorithm, in time $O(n^4)$. It serves `UnsignedMatrix`s and malachite-nz's
// `NaturalMatrix`s.
pub fn mod_power_of_2_characteristic_polynomial_berkowitz<E: PermanentElement>(
    xs: &[E],
    n: usize,
    pow: u64,
) -> Vec<E> {
    charpoly_berkowitz_over_ring(&ModPowerOf2Ring::new(pow), xs, n)
}

// The characteristic polynomial modulo 2^pow of a square matrix, by Berkowitz's division-free
// algorithm, in time $O(n^4)$.
pub fn unsigned_matrix_mod_power_of_2_characteristic_polynomial_berkowitz<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    assert!(a.is_square(), "the matrix must be square");
    UnsignedPolynomial::from_coefficients_asc(mod_power_of_2_characteristic_polynomial_berkowitz(
        a.elements_ref(),
        usize::exact_from(a.height()),
        pow,
    ))
}

// The value modulo 2^pow of the polynomial `p`, whose coefficients are reduced modulo 2^pow, at the
// square matrix `a`, whose elements are reduced modulo 2^pow, by Horner's rule.
pub fn unsigned_polynomial_mod_power_of_2_evaluate_at_matrix<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    assert!(a.is_square(), "the matrix must be square");
    let n = a.height();
    let mut result = UnsignedMatrix::zero(n, n);
    for &c in p.coefficients_asc().iter().rev() {
        result = result.mod_power_of_2_mul(a, pow);
        let mut elements = result.into_elements();
        let k = usize::exact_from(n);
        for i in 0..k {
            elements[i * k + i].mod_power_of_2_add_assign(c, pow);
        }
        result = UnsignedMatrix::from_owned_elements(n, n, elements);
    }
    result
}
