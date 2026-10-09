// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::unsigned_vector_pair_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_extend() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    v.extend([3, 4]);
    assert_eq!(v.to_string(), "(1, 2, 3, 4)");
    let mut v = UnsignedVector::<u8>::from_str("(1, 2)").unwrap();
    v.extend(UnsignedVector::<u8>::zero(0));
    assert_eq!(v.to_string(), "(1, 2)");
}

#[test]
fn extend_properties() {
    unsigned_vector_pair_gen().test_properties(|(v, w)| {
        let mut x = v.clone();
        x.extend(w.elements.clone());
        assert_eq!(x.dimension(), v.dimension() + w.dimension());
        let mut elements = v.elements.clone();
        elements.extend(w.elements.clone());
        assert_eq!(x, UnsignedVector::<u64>::from_owned_elements(elements));
        // Extending by a vector is extending by its elements, and is pushing them one at a time.
        let mut y = v.clone();
        y.extend(w.clone());
        assert_eq!(y, x);
        let mut z = v.clone();
        for e in w.elements.clone() {
            z.push(e);
        }
        assert_eq!(z, x);
        // Extending by nothing changes nothing.
        let mut e = v.clone();
        e.extend(UnsignedVector::<u64>::zero(0));
        assert_eq!(e, v);
    });
}
