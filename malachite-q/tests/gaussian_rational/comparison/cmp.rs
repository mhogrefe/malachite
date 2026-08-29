// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_q::gaussian_rational::{
    ComparableGaussianRational, ComparableGaussianRationalRef, GaussianRational,
};
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_pair_gen, gaussian_rational_triple_gen,
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
        "-22/7",
        "-2-2i",
        "-2",
        "-2+3i",
        "-1",
        "-1/2-i/2",
        "-1/2",
        "-123i",
        "-i",
        "-i/2",
        "0",
        "i/3",
        "i/2",
        "i",
        "123i",
        "1/3",
        "1/2",
        "1/2+i/2",
        "1-1000000000000i",
        "1",
        "1+i",
        "2-3i",
        "2",
        "2+3i",
        "22/7",
        "123",
        "1000000000000",
    ];
    let xs = strings
        .iter()
        .map(|s| GaussianRational::from_str(s).unwrap())
        .collect::<Vec<_>>();
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs.iter().enumerate() {
            let expected = i.cmp(&j);
            assert_eq!(
                ComparableGaussianRationalRef(x).cmp(&ComparableGaussianRationalRef(y)),
                expected
            );
            assert_eq!(
                ComparableGaussianRational(x.clone()).cmp(&ComparableGaussianRational(y.clone())),
                expected
            );
        }
    }

    // A `BTreeSet` sorts by the same order.
    let set = xs
        .iter()
        .map(|x| ComparableGaussianRational(x.clone()))
        .collect::<BTreeSet<_>>();
    let sorted = set.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(sorted, strings);
}

#[test]
// The antisymmetry assertion below is the property under test; swapping its operands would restate
// the line above it rather than assert anything.
#[cfg_attr(dylint_lib = "malachite_lints", expect(redundant_cmp_reverse))]
fn cmp_properties() {
    gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let ord = ComparableGaussianRationalRef(&x).cmp(&ComparableGaussianRationalRef(&y));
        assert_eq!(
            ComparableGaussianRational(x.clone()).cmp(&ComparableGaussianRational(y.clone())),
            ord
        );
        assert_eq!(x.real.cmp(&y.real).then(x.imaginary.cmp(&y.imaginary)), ord);
        assert_eq!(
            ComparableGaussianRationalRef(&y)
                .cmp(&ComparableGaussianRationalRef(&x))
                .reverse(),
            ord
        );
        assert_eq!(x == y, ord == Equal);
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(
            ComparableGaussianRationalRef(&x).cmp(&ComparableGaussianRationalRef(&x)),
            Equal
        );
    });

    gaussian_rational_triple_gen().test_properties(|(x, y, z)| {
        let x = ComparableGaussianRationalRef(&x);
        let y = ComparableGaussianRationalRef(&y);
        let z = ComparableGaussianRationalRef(&z);
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}
