// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{rational_vec_gen, rational_vector_gen};

#[test]
fn test_from_iterator() {
    let v: RationalVector = (1u32..=3).map(|d| Rational::from_unsigneds(1, d)).collect();
    assert_eq!(v.to_string(), "(1, 1/2, 1/3)");
    let v: RationalVector = core::iter::empty().collect();
    assert_eq!(v.to_string(), "()");
}

#[test]
fn from_iterator_properties() {
    rational_vec_gen().test_properties(|xs| {
        let v: RationalVector = xs.iter().cloned().collect();
        assert_eq!(v, RationalVector::from_owned_elements(xs.clone()));
        assert_eq!(v.elements, xs);
    });

    rational_vector_gen().test_properties(|v| {
        // Collecting a vector's elements gives the vector back.
        assert_eq!(v.elements.iter().cloned().collect::<RationalVector>(), v);
    });
}
