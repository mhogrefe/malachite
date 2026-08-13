// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModAssign, DivisibleBy, ModEuclidean,
};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_pair_gen_var_1, natural_pair_gen_var_5};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_balanced_mod_natural() {
    let test = |s, t, out| {
        let x = Natural::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        assert_eq!(x.clone().balanced_mod(y.clone()).to_string(), out);
        assert_eq!(x.clone().balanced_mod(&y).to_string(), out);
        assert_eq!((&x).balanced_mod(y.clone()).to_string(), out);
        assert_eq!((&x).balanced_mod(&y).to_string(), out);
    };
    test("0", "10", "0");
    test("3", "10", "3");
    // - exactly half the modulus is the top of the range, so it stays positive
    test("5", "10", "5");
    // - above half, the closest representative is negative
    test("6", "10", "-4");
    test("7", "10", "-3");
    test("23", "10", "3");
    test("27", "10", "-3");
    // - an odd modulus has no tie
    test("4", "9", "4");
    test("5", "9", "-4");
    test("1", "1", "0");
    test("1000000000000", "3", "1");
    test("1000000000001", "3", "-1");
}

#[test]
fn test_balanced_mod_integer() {
    let test = |s, t, out| {
        let x = Integer::from_str(s).unwrap();
        let y = Integer::from_str(t).unwrap();
        assert_eq!(x.clone().balanced_mod(y.clone()).to_string(), out);
        assert_eq!((&x).balanced_mod(&y).to_string(), out);
        let mut mut_x = x;
        mut_x.balanced_mod_assign(&y);
        assert_eq!(mut_x.to_string(), out);
    };
    test("23", "10", "3");
    test("27", "10", "-3");
    test("25", "10", "5");
    // - a negative value is reduced into the same range
    test("-23", "10", "-3");
    test("-27", "10", "3");
    test("-25", "10", "5");
    // - only the magnitude of the modulus matters
    test("23", "-10", "3");
    test("27", "-10", "-3");
    test("-27", "-10", "3");
    test("0", "10", "0");
}

#[test]
fn balanced_mod_fail() {
    assert_panic!(Natural::from(10u32).balanced_mod(Natural::ZERO));
    assert_panic!(Integer::from(10).balanced_mod(Integer::ZERO));
    assert_panic!(Integer::from(10).balanced_mod_assign(Integer::ZERO));
}

#[test]
fn balanced_mod_properties() {
    natural_pair_gen_var_5().test_properties(|(x, y)| {
        let r = (&x).balanced_mod(&y);
        assert!(r.is_valid());
        assert_eq!(x.clone().balanced_mod(y.clone()), r);
        assert_eq!(x.clone().balanced_mod(&y), r);
        assert_eq!((&x).balanced_mod(y.clone()), r);

        // The congruence and the range determine the result uniquely, so together they are a
        // complete specification.
        assert!((Integer::from(&x) - &r).divisible_by(Integer::from(&y)));
        let double = &r << 1u64;
        assert!(double > -Integer::from(&y));
        assert!(double <= y);
        // it agrees with the Euclidean remainder when that is already small enough
        let e = (&x).mod_euclidean(&y);
        assert_eq!(r == e, e <= &y >> 1u64);
    });

    integer_pair_gen_var_1().test_properties(|(x, y)| {
        let r = (&x).balanced_mod(&y);
        assert!(r.is_valid());
        assert_eq!(x.clone().balanced_mod(y.clone()), r);
        let mut mut_x = x.clone();
        mut_x.balanced_mod_assign(&y);
        assert_eq!(mut_x, r);

        assert!((&x - &r).divisible_by(y.clone()));
        let double = &r << 1u64;
        let abs_y = Integer::from(y.unsigned_abs_ref());
        assert!(double > -&abs_y);
        assert!(double <= abs_y);
        // negating the modulus changes nothing, and negating the value negates the result unless it
        // sits exactly at the positive endpoint
        assert_eq!((&x).balanced_mod(-&y), r);
        let neg = (-&x).balanced_mod(&y);
        if double == abs_y {
            assert_eq!(neg, r);
        } else {
            assert_eq!(neg, -&r);
        }
    });
}
