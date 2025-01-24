// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{integer_gen, integer_pair_gen};
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let test = |u| {
        let x = Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigInt::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);

        let x = rug::Integer::from_str(u).unwrap().clone();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
    test("-123");
    test("-1000000000000");
}

#[test]
fn test_clone_and_clone_from() {
    let test = |u, v| {
        let mut x = Integer::from_str(u).unwrap();
        x.clone_from(&Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigInt::from_str(u).unwrap();
        x.clone_from(&BigInt::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Integer::from_str(u).unwrap();
        x.clone_from(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("-123", "456");
    test("-123", "1000000000000");
    test("1000000000000", "-123");
    test("1000000000000", "2000000000000");
}

#[allow(clippy::redundant_clone)]
#[test]
fn clone_and_clone_from_properties() {
    integer_gen().test_properties(|x| {
        let mut_x = x.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, x);

        assert_eq!(Integer::from(&BigInt::from(&x).clone()), x);
        assert_eq!(Integer::from(&rug::Integer::from(&x).clone()), x);
    });

    integer_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x.clone_from(&y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, y);

        let mut num_x = BigInt::from(&x);
        num_x.clone_from(&BigInt::from(&y));
        assert_eq!(Integer::from(&num_x), y);

        let mut rug_x = rug::Integer::from(&x);
        rug_x.clone_from(&rug::Integer::from(&y));
        assert_eq!(Integer::from(&rug_x), y);
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(i, j)| {
        let x = Integer::from(i);
        let y = Integer::from(j);

        let mut mut_i = i;
        let mut mut_x = x.clone();
        mut_i.clone_from(&j);
        mut_x.clone_from(&y);
        assert_eq!(mut_x, mut_i);
    });
}
