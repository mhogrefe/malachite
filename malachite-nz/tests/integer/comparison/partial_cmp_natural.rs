// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_integer_natural_triple_gen, integer_natural_natural_triple_gen,
    integer_natural_pair_gen, natural_pair_gen,
};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_natural() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u).map(Ordering::reverse), out);
    };
    test("0", "0", Some(Equal));
    test("0", "5", Some(Less));
    test("123", "123", Some(Equal));
    test("123", "124", Some(Less));
    test("123", "122", Some(Greater));
    test("1000000000000", "123", Some(Greater));
    test("123", "1000000000000", Some(Less));
    test("1000000000000", "1000000000000", Some(Equal));
    test("-1000000000000", "1000000000000", Some(Less));
    test("-1000000000000", "0", Some(Less));
}

#[test]
fn partial_cmp_natural_properties() {
    integer_natural_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp(&y);
        assert_eq!(x.cmp(&Integer::from(&y)), cmp.unwrap());
        assert_eq!(
            rug::Integer::from(&x).partial_cmp(&rug::Integer::from(&y)),
            cmp
        );
        assert_eq!(y.partial_cmp(&x), cmp.map(Ordering::reverse));
    });

    integer_integer_natural_triple_gen().test_properties(|(x, z, y)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    integer_natural_natural_triple_gen().test_properties(|(y, x, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Integer::from(&y)), Some(x.cmp(&y)));
    });
}
