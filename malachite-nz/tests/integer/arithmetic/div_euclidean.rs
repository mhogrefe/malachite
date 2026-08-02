// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    DivEuclidean, DivEuclideanAssign, DivModEuclidean, DivRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::rounding_modes::RoundingMode::Floor;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_8, integer_pair_gen_var_1, integer_pair_gen_var_2,
};
use std::str::FromStr;

#[test]
fn test_div_euclidean() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u.clone().div_euclidean(v.clone()).to_string(), out);
        assert_eq!(u.clone().div_euclidean(&v).to_string(), out);
        assert_eq!((&u).div_euclidean(v.clone()).to_string(), out);
        assert_eq!((&u).div_euclidean(&v).to_string(), out);

        let mut mut_u = u.clone();
        mut_u.div_euclidean_assign(v.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u;
        mut_u.div_euclidean_assign(&v);
        assert_eq!(mut_u.to_string(), out);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("23", "1", "23");
    // The quotient is rounded so that the remainder would be nonnegative, regardless of the signs
    // of the operands.
    test("23", "10", "2");
    test("23", "-10", "-2");
    test("-23", "10", "-3");
    test("-23", "-10", "3");
    test("-50", "-23", "3");
    test("50", "-23", "-2");
    test("123", "-1", "-123");
    test("1000000000000", "3", "333333333333");
    test("-1000000000000", "3", "-333333333334");
}

#[test]
#[should_panic]
fn div_euclidean_fail() {
    Integer::from(10).div_euclidean(Integer::ZERO);
}

#[test]
#[should_panic]
fn div_euclidean_val_ref_fail() {
    Integer::from(10).div_euclidean(&Integer::ZERO);
}

#[test]
#[should_panic]
fn div_euclidean_assign_fail() {
    let mut x = Integer::from(10);
    x.div_euclidean_assign(Integer::ZERO);
}

// It would be a little confusing to only pass y by value
#[allow(clippy::needless_pass_by_value)]
fn div_euclidean_properties_helper(x: Integer, y: Integer) {
    let q = (&x).div_euclidean(&y);
    assert!(q.is_valid());

    let q_alt = (&x).div_euclidean(y.clone());
    assert_eq!(q_alt, q);
    let q_alt = x.clone().div_euclidean(&y);
    assert_eq!(q_alt, q);
    let q_alt = x.clone().div_euclidean(y.clone());
    assert_eq!(q_alt, q);

    let mut mut_x = x.clone();
    mut_x.div_euclidean_assign(&y);
    assert!(mut_x.is_valid());
    assert_eq!(mut_x, q);

    let mut mut_x = x.clone();
    mut_x.div_euclidean_assign(y.clone());
    assert_eq!(mut_x, q);

    // The quotient is the one Euclidean division produces.
    assert_eq!((&x).div_mod_euclidean(&y).0, q);
    // For a positive divisor, the Euclidean quotient coincides with floor division.
    if y > 0 {
        assert_eq!((&x).div_round(&y, Floor).0, q);
    }
}

#[test]
fn div_euclidean_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    integer_pair_gen_var_1()
        .test_properties_with_config(&config, |(x, y)| div_euclidean_properties_helper(x, y));

    integer_pair_gen_var_2()
        .test_properties_with_config(&config, |(x, y)| div_euclidean_properties_helper(x, y));

    integer_gen().test_properties(|x| {
        assert_eq!((&x).div_euclidean(Integer::ONE), x);
        assert_eq!((&x).div_euclidean(Integer::NEGATIVE_ONE), -&x);
    });

    integer_gen_var_8().test_properties(|ref x| {
        assert_eq!(x.div_euclidean(x), 1);
        assert_eq!(Integer::ZERO.div_euclidean(x), 0);
    });
}
