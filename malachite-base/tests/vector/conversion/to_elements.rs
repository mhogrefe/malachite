// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::vector::Vector;

#[test]
fn test_to_elements() {
    let test = |s, out: &[u32]| {
        let v = Vector::<u32>::from_str(s).unwrap();
        assert_eq!(v.to_elements(), out);
        assert_eq!(v.elements_ref(), out);
        assert_eq!(v.into_elements(), out);
    };
    test("()", &[]);
    test("(0)", &[0]);
    test("(1, 2, 3)", &[1, 2, 3]);
}

#[test]
fn to_elements_properties() {
    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = Vector {
            elements: xs.clone(),
        };
        assert_eq!(v.to_elements(), xs);
        assert_eq!(v.elements_ref(), xs.as_slice());
        assert_eq!(v.into_elements(), xs);
    });
}
