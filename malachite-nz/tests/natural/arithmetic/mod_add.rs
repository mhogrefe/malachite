// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModAdd, ModAddAssign, ModIsReduced, ModNeg, ModShl, ModSub,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_triple_gen_var_12;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_pair_gen_var_8, natural_quadruple_gen_var_1, natural_triple_gen_var_3,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_add() {
    let test = |r, s, t, out| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let m = Natural::from_str(t).unwrap();

        assert!(u.mod_is_reduced(&m));
        assert!(v.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_add_assign(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_add_assign(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_add_assign(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_add_assign(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_add(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_add(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(((u + v) % m).to_string(), out);
    };
    test("0", "0", "1", "0");
    test("0", "0", "32", "0");
    test("0", "2", "32", "2");
    test("10", "14", "16", "8");
    test("0", "123", "128", "123");
    test("123", "0", "128", "123");
    test("123", "456", "512", "67");
    test("0", "3", "5", "3");
    test("7", "5", "10", "2");
}

#[test]
fn mod_add_fail() {
    assert_panic!(Natural::ZERO.mod_add(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_add(Natural::from(3u8), Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_add(Natural::from(30u8), Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_add(Natural::ZERO, &Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_add(Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_add(Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_add(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_add(&Natural::from(3u8), Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_add(&Natural::from(30u8), Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_add(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_add(&Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_add(&Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_add(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_add(Natural::from(3u8), Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_add(Natural::from(30u8), Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_add(Natural::ZERO, &Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_add(Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_add(Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_add(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_add(&Natural::from(3u8), Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_add(&Natural::from(30u8), Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_add(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_add(&Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_add(&Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_add_assign(Natural::ZERO, Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_add_assign(Natural::from(3u8), Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_add_assign(Natural::from(30u8), Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_add_assign(Natural::ZERO, &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_add_assign(Natural::from(3u8), &Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_add_assign(Natural::from(30u8), &Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_add_assign(Natural::ZERO, Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_add_assign(&Natural::from(3u8), Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_add_assign(&Natural::from(30u8), Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_add_assign(Natural::ZERO, &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_add_assign(&Natural::from(3u8), &Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_add_assign(&Natural::from(30u8), &Natural::from(30u8));
    });
}

#[test]
fn mod_add_properties() {
    natural_triple_gen_var_3().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let sum_val_val_val = x.clone().mod_add(y.clone(), m.clone());
        let sum_val_ref_val = x.clone().mod_add(&y, m.clone());
        let sum_ref_val_val = (&x).mod_add(y.clone(), m.clone());
        let sum_ref_ref_val = (&x).mod_add(&y, m.clone());
        let sum_val_val_ref = x.clone().mod_add(y.clone(), &m);
        let sum_val_ref_ref = x.clone().mod_add(&y, &m);
        let sum_ref_val_ref = (&x).mod_add(y.clone(), &m);
        let sum = (&x).mod_add(&y, &m);
        assert!(sum_val_val_val.is_valid());
        assert!(sum_val_ref_val.is_valid());
        assert!(sum_ref_val_val.is_valid());
        assert!(sum_val_val_ref.is_valid());
        assert!(sum_val_val_ref.is_valid());
        assert!(sum_val_ref_ref.is_valid());
        assert!(sum_ref_val_ref.is_valid());
        assert!(sum.is_valid());
        assert!(sum.mod_is_reduced(&m));
        assert_eq!(sum_val_val_val, sum);
        assert_eq!(sum_val_ref_val, sum);
        assert_eq!(sum_ref_val_val, sum);
        assert_eq!(sum_ref_ref_val, sum);
        assert_eq!(sum_val_val_ref, sum);
        assert_eq!(sum_val_ref_ref, sum);
        assert_eq!(sum_ref_val_ref, sum);

        assert_eq!((&x + &y) % &m, sum);

        let mut mut_x = x.clone();
        mut_x.mod_add_assign(y.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.mod_add_assign(&y, m.clone());
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_add_assign(y.clone(), &m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x.mod_add_assign(&y, &m);
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        assert_eq!((&y).mod_add(&x, &m), sum);
        assert_eq!((&x).mod_sub((&y).mod_neg(&m), &m), sum);
        assert_eq!((&sum).mod_sub(&x, &m), y);
        assert_eq!(sum.mod_sub(y, m), x);
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert_eq!((&x).mod_add(Natural::ZERO, &m), x);
        assert_eq!(Natural::ZERO.mod_add(&x, &m), x);
        assert_eq!((&x).mod_add(&x, &m), x.mod_shl(1, &m));
    });

    natural_quadruple_gen_var_1().test_properties(|(x, y, z, m)| {
        assert_eq!(
            (&x).mod_add(&y, &m).mod_add(&z, &m),
            x.mod_add(y.mod_add(z, &m), m)
        );
    });

    unsigned_triple_gen_var_12::<Limb>().test_properties(|(x, y, m)| {
        assert_eq!(
            x.mod_add(y, m),
            Natural::from(x).mod_add(Natural::from(y), Natural::from(m))
        );
    });
}
