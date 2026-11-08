// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::factorization::traits::Factor;
use crate::num::linear_algebra::traits::ModRank;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::UnsignedVector;
use crate::vector::Vector;
use alloc::vec;
use alloc::vec::Vec;

// Whether the vectors are linearly independent modulo m, by trying every nonzero vector of
// coefficients in [0, m) and checking that its combination of the vectors is nonzero. This takes
// m^r - 1 combinations for r vectors, so it is only for small m and r.
pub fn mod_are_linearly_independent_naive<T: PrimitiveUnsigned>(
    xs: &[UnsignedVector<T>],
    m: T,
) -> bool {
    let r = xs.len();
    let d = xs.first().map_or(0, |x| x.elements.len());
    let mut c = vec![T::ZERO; r];
    loop {
        // advance the odometer c; stop once it wraps around to zero
        let mut i = 0;
        while i < r {
            c[i] += T::ONE;
            if c[i] != m {
                break;
            }
            c[i] = T::ZERO;
            i += 1;
        }
        if i == r {
            return true;
        }
        let vanishes = (0..d).all(|j| {
            xs.iter().zip(&c).fold(T::ZERO, |s, (x, &ci)| {
                s.mod_add(x.elements[j].mod_mul(ci, m), m)
            }) == T::ZERO
        });
        if vanishes {
            return false;
        }
    }
}

// Whether the vectors are linearly independent modulo m, as the matrix whose rows they are having
// full rank modulo every prime factor of m.
pub fn mod_are_linearly_independent_by_prime_factors(xs: &[UnsignedVector<u64>], m: u64) -> bool {
    let r = u64::exact_from(xs.len());
    let d = u64::exact_from(xs.first().map_or(0, |x| x.elements.len()));
    m.factor().into_iter().all(|(p, _)| {
        let elements: Vec<u64> = xs
            .iter()
            .flat_map(|x| x.elements.iter().map(|&e| e % p))
            .collect();
        UnsignedMatrix::from_elements(r, d, &elements).mod_rank(p) == r
    })
}

// The rows of a matrix, as vectors.
pub fn rows_as_vectors<T: PrimitiveUnsigned>(m: &UnsignedMatrix<T>) -> Vec<UnsignedVector<T>> {
    m.rows()
        .map(|row| UnsignedVector::from_elements(row.as_slice()))
        .collect()
}
