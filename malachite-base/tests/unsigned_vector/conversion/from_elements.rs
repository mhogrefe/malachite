// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_from_elements() {
    let test = |xs: &[u32], out| {
        let v = UnsignedVector::from_elements(xs);
        assert_eq!(v.to_string(), out);
        assert_eq!(UnsignedVector::from_owned_elements(xs.to_vec()), v);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, 2, 3], "(1, 2, 3)");
}

#[test]
fn from_elements_properties() {
    unsigned_vector_gen().test_properties(|v| {
        assert_eq!(UnsignedVector::from_elements(v.elements_ref()), v);
        assert_eq!(UnsignedVector::from_owned_elements(v.to_elements()), v);
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = UnsignedVector::from_elements(&xs);
        assert_eq!(v.elements, xs);
        assert_eq!(UnsignedVector::from_owned_elements(xs.clone()), v);
        assert_eq!(v.into_elements(), xs);
    });
}
