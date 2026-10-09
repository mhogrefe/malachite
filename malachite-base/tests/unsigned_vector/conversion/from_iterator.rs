// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_from_iterator() {
    let v: UnsignedVector<u8> = (1..=3).collect();
    assert_eq!(v.to_string(), "(1, 2, 3)");
    let v: UnsignedVector<u8> = core::iter::empty().collect();
    assert_eq!(v.to_string(), "()");
}

fn from_iterator_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vec_gen::<T>().test_properties(|xs| {
        let v: UnsignedVector<T> = xs.iter().copied().collect();
        assert_eq!(v, UnsignedVector::from_owned_elements(xs.clone()));
        assert_eq!(v.elements, xs);
    });
}

#[test]
fn from_iterator_properties() {
    apply_fn_to_unsigneds!(from_iterator_properties_helper);

    unsigned_vector_gen().test_properties(|v| {
        // Collecting a vector's elements gives the vector back.
        assert_eq!(
            v.elements.iter().copied().collect::<UnsignedVector<u64>>(),
            v
        );
    });
}
