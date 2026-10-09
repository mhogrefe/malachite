// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::unsigned_pair_gen_var_51;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_standard_basis_index() {
    let test = |s, out| {
        assert_eq!(
            IntegerVector::from_str(s).unwrap().standard_basis_index(),
            out
        );
    };
    test("()", None);
    test("(0)", None);
    test("(1)", Some(0));
    test("(0, 0, 0)", None);
    test("(0, 1, 0)", Some(1));
    test("(0, 0, 1)", Some(2));
    // - an element other than 1, or a second nonzero element, disqualifies it
    test("(0, -1, 0)", None);
    test("(1, 0, 1)", None);
    test("(0, 1, 1)", None);
}

#[test]
fn standard_basis_index_properties() {
    integer_vector_gen().test_properties(|v| {
        match v.standard_basis_index() {
            Some(i) => {
                assert_eq!(v.pivot_index(), Some(i));
                assert_eq!(v, IntegerVector::standard_basis_vector(v.dimension(), i));
            }
            // A standard basis vector would equal the one at its pivot.
            None => {
                if let Some(i) = v.pivot_index() {
                    assert_ne!(v, IntegerVector::standard_basis_vector(v.dimension(), i));
                }
            }
        }
    });

    unsigned_pair_gen_var_51().test_properties(|(index, dimension)| {
        // Detection inverts construction.
        assert_eq!(
            IntegerVector::standard_basis_vector(dimension, index).standard_basis_index(),
            Some(index)
        );
    });
}
