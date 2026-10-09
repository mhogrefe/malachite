// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{natural_vec_gen, natural_vector_gen};

#[test]
fn test_from_iterator() {
    let v: NaturalVector = (1u32..=3).map(Natural::from).collect();
    assert_eq!(v.to_string(), "(1, 2, 3)");
    let v: NaturalVector = core::iter::empty().collect();
    assert_eq!(v.to_string(), "()");
}

#[test]
fn from_iterator_properties() {
    natural_vec_gen().test_properties(|xs| {
        let v: NaturalVector = xs.iter().cloned().collect();
        assert_eq!(v, NaturalVector::from_owned_elements(xs.clone()));
        assert_eq!(v.elements, xs);
    });

    natural_vector_gen().test_properties(|v| {
        // Collecting a vector's elements gives the vector back.
        assert_eq!(v.elements.iter().cloned().collect::<NaturalVector>(), v);
    });
}
