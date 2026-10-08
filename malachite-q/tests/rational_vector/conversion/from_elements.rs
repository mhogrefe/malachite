// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{rational_vec_gen, rational_vector_gen};

#[test]
fn test_from_elements() {
    let test = |xs: &[i32], out| {
        let xs: Vec<Rational> = xs.iter().copied().map(Rational::from).collect();
        let v = RationalVector::from_elements(&xs);
        assert_eq!(v.to_string(), out);
        let v_alt = RationalVector::from_owned_elements(xs);
        assert_eq!(v_alt, v);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, 2, 3], "(1, 2, 3)");
    test(&[5, 0, 0], "(5, 0, 0)");
    test(&[-1, 0, 1], "(-1, 0, 1)");
}

#[test]
fn from_elements_properties() {
    rational_vec_gen().test_properties(|xs| {
        let v = RationalVector::from_elements(&xs);
        assert_eq!(v.elements, xs);
        assert_eq!(v.dimension(), u64::try_from(xs.len()).unwrap());
        let v_alt = RationalVector::from_owned_elements(xs.clone());
        assert_eq!(v_alt, v);
        // Building a vector and taking its elements back out are inverses.
        assert_eq!(v.into_elements(), xs);
    });

    rational_vector_gen().test_properties(|v| {
        assert_eq!(RationalVector::from_elements(v.elements_ref()), v);
        assert_eq!(RationalVector::from_owned_elements(v.to_elements()), v);
    });
}
