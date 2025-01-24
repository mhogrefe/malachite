// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_pair_gen_var_27};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen};
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_clone() {
    let test = |u| {
        let x = Natural::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);
        assert!(x.is_valid());

        let x = BigUint::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);

        let x = rug::Integer::from_str(u).unwrap();
        assert_eq!(x.to_string(), u);
    };
    test("123");
    test("1000000000000");
}

#[test]
fn test_clone_and_clone_from() {
    let test = |u, v| {
        // clone_from
        let mut x = Natural::from_str(u).unwrap();
        x.clone_from(&Natural::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
        assert!(x.is_valid());

        let mut x = BigUint::from_str(u).unwrap();
        x.clone_from(&BigUint::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);

        let mut x = rug::Integer::from_str(u).unwrap();
        x.clone_from(&rug::Integer::from_str(v).unwrap());
        assert_eq!(x.to_string(), v);
    };
    test("123", "456");
    test("123", "1000000000000");
    test("1000000000000", "123");
    test("1000000000000", "2000000000000");
}

#[allow(clippy::redundant_clone)]
#[test]
fn clone_and_clone_from_properties() {
    natural_gen().test_properties(|x| {
        let mut_x = x.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, x);

        assert_eq!(Natural::from(&BigUint::from(&x).clone()), x);
        assert_eq!(Natural::exact_from(&rug::Integer::from(&x).clone()), x);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        let n = Natural::from(u);
        let cloned_u = u;
        let cloned_n = n.clone();
        assert_eq!(cloned_u, cloned_n);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x.clone_from(&y);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, y);

        let mut num_x = BigUint::from(&x);
        num_x.clone_from(&BigUint::from(&y));
        assert_eq!(Natural::from(&num_x), y);

        let mut rug_x = rug::Integer::from(&x);
        rug_x.clone_from(&rug::Integer::from(&y));
        assert_eq!(Natural::exact_from(&rug_x), y);
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(u, v)| {
        let x = Natural::from(u);
        let y = Natural::from(v);

        let mut mut_u = u;
        let mut mut_x = x.clone();
        mut_u.clone_from(&v);
        mut_x.clone_from(&y);
        assert_eq!(mut_x, mut_u);
    });
}
