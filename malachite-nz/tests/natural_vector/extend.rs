// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_pair_gen;

#[test]
fn test_extend() {
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.extend([3u32, 4].map(Natural::from));
    assert_eq!(v.to_string(), "(1, 2, 3, 4)");
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.extend(NaturalVector::zero(0));
    assert_eq!(v.to_string(), "(1, 2)");
}

#[test]
fn extend_properties() {
    natural_vector_pair_gen().test_properties(|(v, w)| {
        let mut x = v.clone();
        x.extend(w.elements.clone());
        assert_eq!(x.dimension(), v.dimension() + w.dimension());
        let mut elements = v.elements.clone();
        elements.extend(w.elements.clone());
        assert_eq!(x, NaturalVector::from_owned_elements(elements));
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
        e.extend(NaturalVector::zero(0));
        assert_eq!(e, v);
    });
}
