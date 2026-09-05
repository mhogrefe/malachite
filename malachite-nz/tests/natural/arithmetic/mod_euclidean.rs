// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    DivModEuclidean, Mod, ModEuclidean, ModEuclideanAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_2, natural_pair_gen_var_5};
use std::str::FromStr;

#[test]
fn test_mod_euclidean() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

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
    test("23", "10", "3");
    test("456", "123", "87");
    test("1000000000000", "3", "1");
}

#[test]
#[should_panic]
fn mod_euclidean_fail() {
    Natural::from(10u32).mod_euclidean(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_euclidean_val_ref_fail() {
    Natural::from(10u32).mod_euclidean(&Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_euclidean_assign_fail() {
    let mut x = Natural::from(10u32);
    x.mod_euclidean_assign(Natural::ZERO);
}

// It would be a little confusing to only pass y by value
#[allow(clippy::needless_pass_by_value)]
fn mod_euclidean_properties_helper(x: Natural, y: Natural) {
    let r = (&x).mod_euclidean(&y);
    assert!(r.is_valid());

    let r_alt = (&x).mod_euclidean(y.clone());
    assert_eq!(r_alt, r);
    let r_alt = x.clone().mod_euclidean(&y);
    assert_eq!(r_alt, r);
    let r_alt = x.clone().mod_euclidean(y.clone());
    assert_eq!(r_alt, r);

    let mut mut_x = x.clone();
    mut_x.mod_euclidean_assign(&y);
    assert!(mut_x.is_valid());
    assert_eq!(mut_x, r);

    let mut mut_x = x.clone();
    mut_x.mod_euclidean_assign(y.clone());
    assert_eq!(mut_x, r);

    // The remainder is the one Euclidean division produces...
    assert_eq!((&x).div_mod_euclidean(&y).1, r);
    // ...and, for `Natural`s, coincides with `mod_op`.
    assert_eq!((&x).mod_op(&y), r);
    assert!(r < y);
}

#[test]
fn mod_euclidean_properties() {
    natural_pair_gen_var_5().test_properties(|(x, y)| mod_euclidean_properties_helper(x, y));

    natural_gen().test_properties(|x| {
        assert_eq!((&x).mod_euclidean(Natural::ONE), 0);
    });

    natural_gen_var_2().test_properties(|ref x| {
        assert_eq!(x.mod_euclidean(x), 0);
        assert_eq!(Natural::ZERO.mod_euclidean(x), 0);
        if *x > 1u32 {
            assert_eq!(Natural::ONE.mod_euclidean(x), 1);
        }
    });
}
