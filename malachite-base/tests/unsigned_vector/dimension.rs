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
fn test_dimension() {
    let test = |xs: &[u32], out| {
        assert_eq!(UnsignedVector::from_elements(xs).dimension(), out);
    };
    test(&[], 0);
    test(&[0], 1);
    test(&[1, 2, 3], 3);
}

#[test]
fn dimension_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let d = v.dimension();
        assert_eq!(d, u64::try_from(v.elements.len()).unwrap());
        let s = v.to_string();
        assert_eq!(
            d,
            if d == 0 {
                0
            } else {
                u64::try_from(s.matches(", ").count()).unwrap() + 1
            }
        );
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let n = xs.len();
        let v = UnsignedVector { elements: xs };
        assert_eq!(v.dimension(), u64::try_from(n).unwrap());
    });
}
