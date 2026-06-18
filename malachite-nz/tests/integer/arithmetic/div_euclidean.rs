// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    DivAssignEuclidean, DivEuclidean, DivMod, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_8, integer_pair_gen_var_1, integer_pair_gen_var_2,
};
use std::str::FromStr;

#[test]
fn test_div_euclidean() {
    let test = |s, t, qr| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        assert_eq!(u.clone().div_euclidean(v.clone()).to_debug_string(), qr);
        assert_eq!(u.clone().div_euclidean(&v).to_debug_string(), qr);
        assert_eq!((&u).div_euclidean(v.clone()).to_debug_string(), qr);
        assert_eq!((&u).div_euclidean(&v).to_debug_string(), qr);

        let mut mut_u = u.clone();
        let r = mut_u.div_assign_euclidean(v.clone());
        assert_eq!((mut_u, r).to_debug_string(), qr);

        let mut mut_u = u;
        let r = mut_u.div_assign_euclidean(&v);
        assert_eq!((mut_u, r).to_debug_string(), qr);
    };
    test("0", "1", "(0, 0)");
    test("0", "123", "(0, 0)");
    test("23", "1", "(23, 0)");
    test("23", "10", "(2, 3)");
    test("23", "-10", "(-2, 3)");
    test("-23", "10", "(-3, 7)");
    test("-23", "-10", "(3, 7)");
    test("-50", "-23", "(3, 19)");
    test("50", "-23", "(-2, 4)");
    test("123", "-1", "(-123, 0)");
    test("1000000000000", "3", "(333333333333, 1)");
    test("-1000000000000", "3", "(-333333333334, 2)");
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
fn div_assign_euclidean_fail() {
    let mut x = Integer::from(10);
    x.div_assign_euclidean(Integer::ZERO);
}

fn div_euclidean_properties_helper(x: Integer, y: Integer) {
    let mut mut_x = x.clone();
    let r = mut_x.div_assign_euclidean(&y);
    assert!(mut_x.is_valid());
    assert!(r.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    let r_alt = mut_x.div_assign_euclidean(y.clone());
    assert_eq!(mut_x, q);
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = (&x).div_euclidean(&y);
    assert!(q_alt.is_valid());
    assert!(r_alt.is_valid());
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = (&x).div_euclidean(y.clone());
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().div_euclidean(&y);
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    let (q_alt, r_alt) = x.clone().div_euclidean(y.clone());
    assert_eq!(q_alt, q);
    assert_eq!(r_alt, r);

    // The remainder (a `Natural`) is smaller than the absolute value of the divisor.
    assert!(r < (&y).unsigned_abs());
    // The defining relation x = q * y + r holds.
    assert_eq!(&q * &y + Integer::from(&r), x);
    // For a positive divisor, Euclidean division coincides with `div_mod`.
    if y > 0 {
        let (q_alt, r_alt) = (&x).div_mod(&y);
        assert_eq!(q_alt, q);
        assert_eq!(r, r_alt);
    }
    // The by-value variant agrees and consumes both operands.
    assert_eq!(x.div_euclidean(y), (q, r));
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
        let (q, r) = (&x).div_euclidean(Integer::ONE);
        assert_eq!(q, x);
        assert_eq!(r, 0);

        let (q, r) = (&x).div_euclidean(Integer::NEGATIVE_ONE);
        assert_eq!(q, -&x);
        assert_eq!(r, 0);
    });

    integer_gen_var_8().test_properties(|ref x| {
        assert_eq!(x.div_euclidean(Integer::ONE), (x.clone(), Natural::ZERO));
        assert_eq!(x.div_euclidean(Integer::NEGATIVE_ONE), (-x, Natural::ZERO));
        assert_eq!(x.div_euclidean(x), (Integer::ONE, Natural::ZERO));
        assert_eq!(
            Integer::ZERO.div_euclidean(x),
            (Integer::ZERO, Natural::ZERO)
        );
    });
}
