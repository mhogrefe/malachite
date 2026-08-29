// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::gaussian_integer::{
    ComparableGaussianInteger, ComparableGaussianIntegerRef, GaussianInteger,
};
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_pair_gen, gaussian_integer_triple_gen,
};
use std::cmp::Ordering::*;
use std::collections::BTreeSet;
use std::str::FromStr;

#[test]
fn test_ord() {
    // In increasing lexicographic order.
    let strings = &[
        "-1000000000000-i",
        "-1000000000000",
        "-1000000000000+i",
        "-2-2i",
        "-2",
        "-2+3i",
        "-1",
        "-123i",
        "-i",
        "0",
        "i",
        "123i",
        "1-1000000000000i",
        "1",
        "1+i",
        "2-3i",
        "2",
        "2+3i",
        "123",
        "1000000000000",
    ];
    let xs = strings
        .iter()
        .map(|s| GaussianInteger::from_str(s).unwrap())
        .collect::<Vec<_>>();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs.iter().enumerate() {
            let expected = i.cmp(&j);
            assert_eq!(
                ComparableGaussianIntegerRef(x).cmp(&ComparableGaussianIntegerRef(y)),
                expected
            );
            assert_eq!(
                ComparableGaussianInteger(x.clone()).cmp(&ComparableGaussianInteger(y.clone())),
                expected
            );
        }
    }

    // A `BTreeSet` sorts by the same order.
    let set = xs
        .iter()
        .map(|x| ComparableGaussianInteger(x.clone()))
        .collect::<BTreeSet<_>>();
    let sorted = set.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(sorted, strings);
}

#[test]
// The antisymmetry assertion below is the property under test; swapping its operands would restate
// the line above it rather than assert anything.
#[cfg_attr(dylint_lib = "malachite_lints", expect(redundant_cmp_reverse))]
fn cmp_properties() {
    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let ord = ComparableGaussianIntegerRef(&x).cmp(&ComparableGaussianIntegerRef(&y));
        assert_eq!(
            ComparableGaussianInteger(x.clone()).cmp(&ComparableGaussianInteger(y.clone())),
            ord
        );
        assert_eq!(x.real.cmp(&y.real).then(x.imaginary.cmp(&y.imaginary)), ord);
        assert_eq!(
            ComparableGaussianIntegerRef(&y)
                .cmp(&ComparableGaussianIntegerRef(&x))
                .reverse(),
            ord
        );
        assert_eq!(x == y, ord == Equal);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(
            ComparableGaussianIntegerRef(&x).cmp(&ComparableGaussianIntegerRef(&x)),
            Equal
        );
    });

    gaussian_integer_triple_gen().test_properties(|(x, y, z)| {
        let x = ComparableGaussianIntegerRef(&x);
        let y = ComparableGaussianIntegerRef(&y);
        let z = ComparableGaussianIntegerRef(&z);
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}
