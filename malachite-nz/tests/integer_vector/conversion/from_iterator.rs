// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{integer_vec_gen, integer_vector_gen};

#[test]
fn test_from_iterator() {
    let v: IntegerVector = (-1..=1).map(Integer::from).collect();
    assert_eq!(v.to_string(), "(-1, 0, 1)");
    let v: IntegerVector = core::iter::empty().collect();
    assert_eq!(v.to_string(), "()");
}

#[test]
fn from_iterator_properties() {
    integer_vec_gen().test_properties(|xs| {
        let v: IntegerVector = xs.iter().cloned().collect();
        assert_eq!(v, IntegerVector::from_owned_elements(xs.clone()));
        assert_eq!(v.elements, xs);
    });

    integer_vector_gen().test_properties(|v| {
        // Collecting a vector's elements gives the vector back.
        assert_eq!(v.elements.iter().cloned().collect::<IntegerVector>(), v);
    });
}
