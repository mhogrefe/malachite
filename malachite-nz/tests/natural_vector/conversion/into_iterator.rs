// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_into_iterator() {
    let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    assert_eq!(
        v.clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>(),
        ["1", "2", "3"]
    );
    let mut strings = Vec::new();
    for x in &v {
        strings.push(x.to_string());
    }
    assert_eq!(strings, ["1", "2", "3"]);
    let mut w = v;
    for x in &mut w {
        *x += Natural::ONE;
    }
    assert_eq!(w.to_string(), "(2, 3, 4)");
    assert_eq!(NaturalVector::zero(0).into_iter().count(), 0);
}

#[test]
fn into_iterator_properties() {
    natural_vector_gen().test_properties(|v| {
        // Iterating by value, by reference, and over the elements give the same elements in the
        // same order, as many as the dimension.
        let xs: Vec<Natural> = v.clone().into_iter().collect();
        assert_eq!(xs, v.elements);
        assert!(IntoIterator::into_iter(&v).eq(v.elements.iter()));
        assert_eq!(
            IntoIterator::into_iter(&v).count(),
            usize::exact_from(v.dimension())
        );
        // Collecting the elements back gives the same vector.
        assert_eq!(v.clone().into_iter().collect::<NaturalVector>(), v);
        // Changing the elements through `&mut` is changing them in `elements`.
        let mut w = v.clone();
        for x in &mut w {
            *x += Natural::ONE;
        }
        let mut w_alt = v.clone();
        for x in &mut w_alt.elements {
            *x += Natural::ONE;
        }
        assert_eq!(w, w_alt);
    });
}
