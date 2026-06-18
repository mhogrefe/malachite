// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivAssignEuclidean, DivEuclidean, DivMod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_gen_var_2, natural_pair_gen_var_5};
use std::str::FromStr;

#[test]
fn test_div_euclidean() {
    let test = |s, t, qr| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

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
    test("1", "1", "(1, 0)");
    test("123", "1", "(123, 0)");
    test("123", "123", "(1, 0)");
    test("123", "456", "(0, 123)");
    test("456", "123", "(3, 87)");
    test("1000000000000", "3", "(333333333333, 1)");
    test("1000000000000", "123", "(8130081300, 100)");
}

#[test]
#[should_panic]
fn div_euclidean_fail() {
    Natural::from(10u32).div_euclidean(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_assign_euclidean_fail() {
    let mut x = Natural::from(10u32);
    x.div_assign_euclidean(Natural::ZERO);
}

fn div_euclidean_properties_helper(x: Natural, y: Natural) {
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

    // For `Natural`s, Euclidean division coincides with `div_mod`.
    assert_eq!((&x).div_mod(&y), (q.clone(), r.clone()));
    assert!(r < y);
    assert_eq!(&q * &y + &r, x);
    // The by-value variant agrees and consumes both operands.
    assert_eq!(x.div_euclidean(y), (q, r));
}

#[test]
fn div_euclidean_properties() {
    natural_pair_gen_var_5().test_properties(|(x, y)| div_euclidean_properties_helper(x, y));

    natural_gen().test_properties(|x| {
        assert_eq!((&x).div_euclidean(Natural::ONE), (x, Natural::ZERO));
    });

    natural_gen_var_2().test_properties(|x| {
        assert_eq!((&x).div_euclidean(&x), (Natural::ONE, Natural::ZERO));
        assert_eq!(
            Natural::ZERO.div_euclidean(&x),
            (Natural::ZERO, Natural::ZERO)
        );
        if x > 1 {
            assert_eq!(
                Natural::ONE.div_euclidean(&x),
                (Natural::ZERO, Natural::ONE)
            );
        }
    });
}
