// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Mod, ModAdd, ModIsReduced, ModNeg, ModSub, ModSubAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_triple_gen_var_12;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_pair_gen_var_8, natural_triple_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_mod_sub() {
    let test = |r, s, t, out| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let m = Natural::from_str(t).unwrap();

        assert!(u.mod_is_reduced(&m));
        assert!(v.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_sub_assign(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_sub_assign(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_sub_assign(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_sub_assign(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_sub(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_sub(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_sub(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_sub(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_sub(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_sub(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_sub(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_sub(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "1", "0");
    test("0", "0", "32", "0");
    test("0", "27", "32", "5");
    test("10", "2", "16", "8");
    test("2", "10", "16", "8");
    test("0", "5", "128", "123");
    test("123", "0", "128", "123");
    test("123", "56", "512", "67");
    test("56", "123", "512", "445");
    test("7", "9", "10", "8");
}

#[test]
fn mod_sub_fail() {
    assert_panic!(Natural::ZERO.mod_sub(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_sub(Natural::from(3u8), Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_sub(Natural::from(30u8), Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_sub(Natural::ZERO, &Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_sub(Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_sub(Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_sub(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_sub(&Natural::from(3u8), Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_sub(&Natural::from(30u8), Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_sub(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_sub(&Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_sub(&Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_sub(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_sub(Natural::from(3u8), Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_sub(Natural::from(30u8), Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_sub(Natural::ZERO, &Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_sub(Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_sub(Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_sub(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_sub(&Natural::from(3u8), Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_sub(&Natural::from(30u8), Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_sub(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_sub(&Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_sub(&Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_sub_assign(Natural::ZERO, Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_sub_assign(Natural::from(3u8), Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_sub_assign(Natural::from(30u8), Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_sub_assign(Natural::ZERO, &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_sub_assign(Natural::from(3u8), &Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_sub_assign(Natural::from(30u8), &Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_sub_assign(Natural::ZERO, Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_sub_assign(&Natural::from(3u8), Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_sub_assign(&Natural::from(30u8), Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_sub_assign(Natural::ZERO, &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_sub_assign(&Natural::from(3u8), &Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_sub_assign(&Natural::from(30u8), &Natural::from(30u8));
    });
}

#[test]
fn mod_sub_properties() {
    natural_triple_gen_var_3().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let diff_val_val_val = x.clone().mod_sub(y.clone(), m.clone());
        let diff_val_ref_val = x.clone().mod_sub(&y, m.clone());
        let diff_ref_val_val = (&x).mod_sub(y.clone(), m.clone());
        let diff_ref_ref_val = (&x).mod_sub(&y, m.clone());
        let diff_val_val_ref = x.clone().mod_sub(y.clone(), &m);
        let diff_val_ref_ref = x.clone().mod_sub(&y, &m);
        let diff_ref_val_ref = (&x).mod_sub(y.clone(), &m);
        let diff = (&x).mod_sub(&y, &m);
        assert!(diff_val_val_val.is_valid());
        assert!(diff_val_ref_val.is_valid());
        assert!(diff_ref_val_val.is_valid());
        assert!(diff_val_val_ref.is_valid());
        assert!(diff_val_val_ref.is_valid());
        assert!(diff_val_ref_ref.is_valid());
        assert!(diff_ref_val_ref.is_valid());
        assert!(diff.is_valid());
        assert!(diff.mod_is_reduced(&m));
        assert_eq!(diff_val_val_val, diff);
        assert_eq!(diff_val_ref_val, diff);
        assert_eq!(diff_ref_val_val, diff);
        assert_eq!(diff_ref_ref_val, diff);
        assert_eq!(diff_val_val_ref, diff);
        assert_eq!(diff_val_ref_ref, diff);
        assert_eq!(diff_ref_val_ref, diff);

        assert_eq!(
            (Integer::from(&x) - Integer::from(&y)).mod_op(Integer::from(&m)),
            diff
        );

        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(y.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(&y, m.clone());
        assert_eq!(mut_x, diff);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(y.clone(), &m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);
        let mut mut_x = x.clone();
        mut_x.mod_sub_assign(&y, &m);
        assert_eq!(mut_x, diff);
        assert!(mut_x.is_valid());

        assert_eq!((&y).mod_sub(&x, &m), (&diff).mod_neg(&m));
        assert_eq!((&x).mod_add((&y).mod_neg(&m), &m), diff);
        assert_eq!((&diff).mod_add(&y, &m), x);
        assert_eq!(diff.mod_sub(x, &m), y.mod_neg(&m));
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert_eq!((&x).mod_sub(Natural::ZERO, &m), x);
        assert_eq!(Natural::ZERO.mod_sub(&x, &m), (&x).mod_neg(&m));
        assert_eq!((&x).mod_sub(&x, m), 0);
    });

    unsigned_triple_gen_var_12::<Limb>().test_properties(|(x, y, m)| {
        assert_eq!(
            x.mod_sub(y, m),
            Natural::from(x).mod_sub(Natural::from(y), Natural::from(m))
        );
    });
}
