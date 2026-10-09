// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_height_index() {
    let test = |s, out| {
        assert_eq!(NaturalVector::from_str(s).unwrap().height_index(), out);
    };
    test("()", None);
    test("(1, 3, 2)", Some(1));
    test("(5, 5)", Some(0));
    test("(0)", Some(0));
}

#[test]
fn height_index_properties() {
    natural_vector_gen().test_properties(|v| {
        let height = |x: &Natural| x.clone();
        let index = v.height_index();
        assert_eq!(index.is_none(), v.dimension() == 0);
        if let Some(i) = index {
            // The element at the index has the largest height, and every element before it has a
            // smaller one.
            let i = usize::exact_from(i);
            let h = height(&v.elements[i]);
            assert!(v.elements.iter().all(|x| height(x) <= h));
            assert!(v.elements[..i].iter().all(|x| height(x) < h));
        }
    });
}
