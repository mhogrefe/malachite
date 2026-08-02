// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivEuclidean, DivEuclideanAssign, DivModEuclidean};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_2, natural_pair_gen_var_5};
use std::str::FromStr;

#[test]
fn test_div_euclidean() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

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
    test("23", "10", "2");
    test("456", "123", "3");
    test("1000000000000", "3", "333333333333");
}

#[test]
#[should_panic]
fn div_euclidean_fail() {
    Natural::from(10u32).div_euclidean(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_euclidean_val_ref_fail() {
    Natural::from(10u32).div_euclidean(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_euclidean_assign_fail() {
    let mut x = Natural::from(10u32);
    x.div_euclidean_assign(Natural::ZERO);
}

// It would be a little confusing to only pass y by value
#[allow(clippy::needless_pass_by_value)]
fn div_euclidean_properties_helper(x: Natural, y: Natural) {
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

    // The quotient is the one Euclidean division produces...
    assert_eq!((&x).div_mod_euclidean(&y).0, q);
    // ...and, for `Natural`s, coincides with division.
    assert_eq!(&x / &y, q);
}

#[test]
fn div_euclidean_properties() {
    natural_pair_gen_var_5().test_properties(|(x, y)| div_euclidean_properties_helper(x, y));

    natural_gen().test_properties(|x| {
        assert_eq!((&x).div_euclidean(Natural::ONE), x);
    });

    natural_gen_var_2().test_properties(|ref x| {
        assert_eq!(x.div_euclidean(x), 1);
        assert_eq!(Natural::ZERO.div_euclidean(x), 0);
    });
}
