// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    DivModEuclidean, Mod, ModEuclidean, ModEuclideanAssign, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_8, integer_pair_gen_var_1, integer_pair_gen_var_2,
};
use std::str::FromStr;

#[test]
fn test_mod_euclidean() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u.clone().mod_euclidean(v.clone()).to_string(), out);
        assert_eq!(u.clone().mod_euclidean(&v).to_string(), out);
        assert_eq!((&u).mod_euclidean(v.clone()).to_string(), out);
        assert_eq!((&u).mod_euclidean(&v).to_string(), out);

        let mut mut_u = u.clone();
        mut_u.mod_euclidean_assign(v.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u;
        mut_u.mod_euclidean_assign(&v);
        assert_eq!(mut_u.to_string(), out);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("23", "1", "0");
    // The remainder is always nonnegative, regardless of the signs of the operands.
    test("23", "10", "3");
    test("23", "-10", "3");
    test("-23", "10", "7");
    test("-23", "-10", "7");
    test("-50", "-23", "19");
    test("50", "-23", "4");
    test("123", "-1", "0");
    test("1000000000000", "3", "1");
    test("-1000000000000", "3", "2");
}

#[test]
#[should_panic]
fn mod_euclidean_fail() {
    Integer::from(10).mod_euclidean(Integer::ZERO);
}

#[test]
#[should_panic]
fn mod_euclidean_val_ref_fail() {
    Integer::from(10).mod_euclidean(&Integer::ZERO);
}

#[test]
#[should_panic]
fn mod_euclidean_assign_fail() {
    let mut x = Integer::from(10);
    x.mod_euclidean_assign(Integer::ZERO);
}

// It would be a little confusing to only pass y by value
#[allow(clippy::needless_pass_by_value)]
fn mod_euclidean_properties_helper(x: Integer, y: Integer) {
    let r = (&x).mod_euclidean(&y);
    assert!(r.is_valid());

    let r_alt = (&x).mod_euclidean(y.clone());
    assert_eq!(r_alt, r);
    let r_alt = x.clone().mod_euclidean(&y);
    assert_eq!(r_alt, r);
    let r_alt = x.clone().mod_euclidean(y.clone());
    assert_eq!(r_alt, r);

    // The assign form leaves the (nonnegative) remainder in place as an `Integer`.
    let mut mut_x = x.clone();
    mut_x.mod_euclidean_assign(&y);
    assert!(mut_x.is_valid());
    assert_eq!(mut_x, r);

    let mut mut_x = x.clone();
    mut_x.mod_euclidean_assign(y.clone());
    assert_eq!(mut_x, r);

    // The remainder is the one Euclidean division produces...
    assert_eq!((&x).div_mod_euclidean(&y).1, r);
    // ...is smaller than the absolute value of the divisor...
    assert!(r < (&y).unsigned_abs());
    // ...and makes the difference from x a multiple of y.
    assert_eq!((&x - Integer::from(&r)).mod_op(&y), 0);
    // For a positive divisor, the Euclidean remainder coincides with `mod_op`.
    if y > 0 {
        assert_eq!((&x).mod_op(&y), r);
    }
}

#[test]
fn mod_euclidean_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    integer_pair_gen_var_1()
        .test_properties_with_config(&config, |(x, y)| mod_euclidean_properties_helper(x, y));

    integer_pair_gen_var_2()
        .test_properties_with_config(&config, |(x, y)| mod_euclidean_properties_helper(x, y));

    integer_gen().test_properties(|x| {
        assert_eq!((&x).mod_euclidean(Integer::ONE), 0);
        assert_eq!((&x).mod_euclidean(Integer::NEGATIVE_ONE), 0);
    });

    integer_gen_var_8().test_properties(|ref x| {
        assert_eq!(x.mod_euclidean(x), 0);
        assert_eq!(Integer::ZERO.mod_euclidean(x), Natural::ZERO);
    });
}
