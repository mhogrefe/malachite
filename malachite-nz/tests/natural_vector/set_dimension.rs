// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::min;
use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_pair_gen;

#[test]
fn test_set_dimension() {
    let mut v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    v.set_dimension(5);
    assert_eq!(v.to_string(), "(1, 2, 3, 0, 0)");
    v.set_dimension(2);
    assert_eq!(v.to_string(), "(1, 2)");
    v.set_dimension(2);
    assert_eq!(v.to_string(), "(1, 2)");
    v.set_dimension(0);
    assert_eq!(v.to_string(), "()");
    v.set_dimension(1);
    assert_eq!(v.to_string(), "(0)");
}

#[test]
fn set_dimension_properties() {
    natural_vector_pair_gen().test_properties(|(v, w)| {
        let dimension = w.dimension();
        let mut x = v.clone();
        x.set_dimension(dimension);
        assert_eq!(x.dimension(), dimension);
        // The common coordinates are kept, and any new ones are zero.
        let k = usize::exact_from(min(v.dimension(), dimension));
        assert_eq!(x.elements[..k], v.elements[..k]);
        assert!(x.elements[k..].iter().all(|x| *x == 0u32));
        let mut elements = v.elements.clone();
        elements.resize(usize::exact_from(dimension), Natural::ZERO);
        assert_eq!(x.elements, elements);
        // Setting the same dimension again changes nothing.
        let mut y = x.clone();
        y.set_dimension(dimension);
        assert_eq!(y, x);
        // Growing and then shrinking back is the identity.
        let mut z = v.clone();
        z.set_dimension(v.dimension() + dimension);
        z.set_dimension(v.dimension());
        assert_eq!(z, v);
    });
}
