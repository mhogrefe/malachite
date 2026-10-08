// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_dimension() {
    let test = |xs: &[u32], out| {
        let v = NaturalVector {
            elements: xs.iter().copied().map(Natural::from).collect(),
        };
        assert_eq!(v.dimension(), out);
    };
    test(&[], 0);
    test(&[0], 1);
    test(&[1, 2, 3], 3);
}

#[test]
fn dimension_properties() {
    natural_vector_gen().test_properties(|v| {
        let d = v.dimension();
        assert_eq!(d, u64::exact_from(v.elements.len()));
        assert_eq!(v.clone().dimension(), d);
        // The string form has one element per comma-separated part.
        let s = v.to_string();
        assert_eq!(
            d,
            if d == 0 {
                0
            } else {
                u64::exact_from(s.matches(", ").count()) + 1
            }
        );
    });
}
