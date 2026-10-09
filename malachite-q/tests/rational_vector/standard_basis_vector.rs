// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_pair_gen_var_51;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;

#[test]
fn test_standard_basis_vector() {
    let test = |dimension, index, out| {
        let v = RationalVector::standard_basis_vector(dimension, index);
        assert_eq!(v.to_string(), out);
        assert_eq!(v.dimension(), dimension);
    };
    test(1, 0, "(1)");
    test(3, 0, "(1, 0, 0)");
    test(3, 1, "(0, 1, 0)");
    test(3, 2, "(0, 0, 1)");
}

#[test]
#[should_panic]
fn standard_basis_vector_fail_1() {
    // The index equals the dimension.
    RationalVector::standard_basis_vector(3, 3);
}

#[test]
#[should_panic]
fn standard_basis_vector_fail_2() {
    // The index exceeds the dimension.
    RationalVector::standard_basis_vector(3, 5);
}

#[test]
#[should_panic]
fn standard_basis_vector_fail_3() {
    // The 0-dimensional vector has no elements.
    RationalVector::standard_basis_vector(0, 0);
}

#[test]
fn standard_basis_vector_properties() {
    unsigned_pair_gen_var_51().test_properties(|(index, dimension)| {
        let v = RationalVector::standard_basis_vector(dimension, index);
        assert_eq!(v.dimension(), dimension);
        let i = usize::exact_from(index);
        assert_eq!(v.pivot_index(), Some(index));
        for (j, x) in v.elements.iter().enumerate() {
            assert_eq!(*x == 1u32, j == i);
            assert_eq!(*x == 0u32, j != i);
        }
        assert_eq!(v.pivot(), Some(&Rational::ONE));
        assert_eq!(
            RationalVector::from(IntegerVector::standard_basis_vector(dimension, index)),
            v
        );
    });
}
