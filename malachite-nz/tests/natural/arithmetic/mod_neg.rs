// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Mod, ModAdd, ModIsReduced, ModNeg, ModNegAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_pair_gen_var_16;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_pair_gen_var_8;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_neg() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert!(u.mod_is_reduced(&v));
        let n = u.clone().mod_neg(v.clone());
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&v));
        assert_eq!(n.to_string(), out);

        let n = u.clone().mod_neg(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_neg(v.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_neg(&v);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_neg_assign(v.clone());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = u;
        n.mod_neg_assign(&v);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "5", "0");
    test("7", "10", "3");
    test("100", "101", "1");
    test("4294967294", "4294967295", "1");
    test("1", "4294967295", "4294967294");
    test("7", "1000000000000", "999999999993");
    test("999999999993", "1000000000000", "7");
}

#[test]
fn mod_neg_fail() {
    assert_panic!(Natural::ZERO.mod_neg(Natural::ZERO));
    assert_panic!(Natural::from(30u32).mod_neg(Natural::ONE));

    assert_panic!(Natural::ZERO.mod_neg(&Natural::ZERO));
    assert_panic!(Natural::from(30u32).mod_neg(&Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_neg(Natural::ZERO));
    assert_panic!((&Natural::from(30u32)).mod_neg(Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_neg(&Natural::ZERO));
    assert_panic!((&Natural::from(30u32)).mod_neg(&Natural::ONE));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_neg_assign(Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u32);
        x.mod_neg_assign(Natural::ONE);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_neg_assign(&Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u32);
        x.mod_neg_assign(&Natural::ONE);
    });
}

#[test]
fn mod_neg_properties() {
    natural_pair_gen_var_8().test_properties(|(n, m)| {
        assert!(n.mod_is_reduced(&m));
        let neg = (&n).mod_neg(&m);
        assert!(neg.is_valid());
        assert!(neg.mod_is_reduced(&m));

        let neg_alt = (&n).mod_neg(m.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let neg_alt = n.clone().mod_neg(&m);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let neg_alt = n.clone().mod_neg(m.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_neg_assign(&m);
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        let mut n_alt = n.clone();
        n_alt.mod_neg_assign(m.clone());
        assert!(neg_alt.is_valid());
        assert_eq!(neg_alt, neg);

        assert_eq!(neg, (-&n).mod_op(Integer::from(&m)));
        assert_eq!((&neg).mod_neg(&m), n);
        assert_eq!((&n).mod_add(&neg, &m), 0);
        assert_eq!(n == neg, n == Natural::ZERO || n << 1 == m);
    });

    unsigned_pair_gen_var_16::<Limb>().test_properties(|(n, m)| {
        assert_eq!(n.mod_neg(m), Natural::from(n).mod_neg(Natural::from(m)));
    });
}
