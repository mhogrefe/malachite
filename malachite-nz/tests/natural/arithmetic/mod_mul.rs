// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModAdd, ModIsReduced, ModMul, ModMulAssign, ModMulPrecomputed, ModMulPrecomputedAssign, ModNeg,
    ModSquare,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::JoinHalves;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_pair_gen_var_36, unsigned_triple_gen_var_12};
use malachite_nz::natural::arithmetic::mod_mul::{
    limbs_mod_mul_two_limbs, limbs_precompute_mod_mul_two_limbs,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz::test_util::generators::{
    large_type_gen_var_21, natural_pair_gen_var_8, natural_quadruple_gen_var_1,
    natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural::arithmetic::mod_mul::{
    limbs_mod_mul_two_limbs_naive, limbs_precompute_mod_mul_two_limbs_alt,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_precompute_mod_mul_two_limbs() {
    let test = |m_1, m_0, inv_2, inv_1, inv_0| {
        assert_eq!(
            limbs_precompute_mod_mul_two_limbs(m_1, m_0),
            (inv_2, inv_1, inv_0)
        );
        assert_eq!(
            limbs_precompute_mod_mul_two_limbs_alt(m_1, m_0),
            (inv_2, inv_1, inv_0)
        );
    };
    test(1, 1, u32::MAX, 0, u32::MAX);
    test(1, 2, u32::MAX - 1, 3, 0xfffffff8);
    test(123, 456, 34918433, 1162528328, 1277088208);
    test(u32::MAX, u32::MAX - 1, 1, 0, 2);
    test(u32::MAX, u32::MAX, 1, 0, 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_mul_two_limbs() {
    let test = |x_1, x_0, y_1, y_0, m_1, m_0, r_1, r_0| {
        let (inv_2, inv_1, inv_0) = limbs_precompute_mod_mul_two_limbs(m_1, m_0);
        assert_eq!(
            limbs_mod_mul_two_limbs(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0),
            (r_1, r_0)
        );
        assert_eq!(
            limbs_mod_mul_two_limbs_naive(x_1, x_0, y_1, y_0, m_1, m_0),
            (r_1, r_0)
        );
    };
    test(0, 0, 0, 0, 1, 1, 0, 0);
    test(1, 0, 0, 1, 1, 1, 1, 0);
    test(123, 456, 654, 321, 789, 876, 213, 4164192732);
    test(123, 456, 789, 876, u32::MAX, u32::MAX, 467532, 496503);
}

#[test]
fn test_mod_mul() {
    let test = |r, s, t, out| {
        let u = Natural::from_str(r).unwrap();
        let v = Natural::from_str(s).unwrap();
        let m = Natural::from_str(t).unwrap();

        assert!(u.mod_is_reduced(&m));
        assert!(v.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_mul_assign(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_mul_assign(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_mul_assign(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_mul_assign(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_mul(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_mul(v.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_mul(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_mul(&v, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_mul(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_mul(v.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_mul(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_mul(&v, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!((u * v % m).to_string(), out);
    };
    test("0", "0", "1", "0");
    test("1", "0", "32", "0");
    test("1", "2", "32", "2");
    test("3", "4", "15", "12");
    test("7", "6", "10", "2");
    test("10", "14", "16", "12");
    test("1", "123", "128", "123");
    test("123", "1", "128", "123");
    test("123", "456", "512", "280");
    test("1000000000", "2000000000", "4294967296", "1321730048");
    test("1000000000", "2000000000", "4294967297", "856068761");
    test(
        "1000000000000000",
        "2000000000000000",
        "1000000000000000000000001",
        "999999999999999998000001",
    );
}

#[test]
fn limbs_precompute_mod_mul_two_limbs_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_pair_gen_var_36().test_properties_with_config(&config, |(m_1, m_0)| {
        let (inv_2, inv_1, inv_0) = limbs_precompute_mod_mul_two_limbs(m_1, m_0);
        assert_eq!(
            limbs_precompute_mod_mul_two_limbs_alt(m_1, m_0),
            (inv_2, inv_1, inv_0)
        );
        assert!(inv_2 != 0 || inv_1 != 0 || inv_0 != 0);
    });
}

#[test]
fn limbs_mod_mul_two_limbs_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    large_type_gen_var_21().test_properties_with_config(
        &config,
        |(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0)| {
            let (r_1, r_0) =
                limbs_mod_mul_two_limbs(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0);
            assert_eq!(
                limbs_mod_mul_two_limbs_naive(x_1, x_0, y_1, y_0, m_1, m_0),
                (r_1, r_0)
            );
            let x = Natural::from(DoubleLimb::join_halves(x_1, x_0));
            let y = Natural::from(DoubleLimb::join_halves(y_1, y_0));
            let m = Natural::from(DoubleLimb::join_halves(m_1, m_0));
            let q = &x * &y / &m;
            let r = Natural::from(DoubleLimb::join_halves(r_1, r_0));
            assert_eq!(q * m + r, x * y);
        },
    );
}

#[test]
fn mod_mul_fail() {
    assert_panic!(Natural::ZERO.mod_mul(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_mul(Natural::from(3u8), Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_mul(Natural::from(30u8), Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_mul(Natural::ZERO, &Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_mul(Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_mul(Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_mul(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_mul(&Natural::from(3u8), Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_mul(&Natural::from(30u8), Natural::from(30u8)));

    assert_panic!(Natural::ZERO.mod_mul(Natural::ZERO, Natural::ZERO));
    assert_panic!(Natural::from(30u8).mod_mul(&Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!(Natural::from(3u8).mod_mul(&Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_mul(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_mul(Natural::from(3u8), Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_mul(Natural::from(30u8), Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_mul(Natural::ZERO, &Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_mul(Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_mul(Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_mul(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_mul(&Natural::from(3u8), Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_mul(&Natural::from(30u8), Natural::from(30u8)));

    assert_panic!((&Natural::ZERO).mod_mul(Natural::ZERO, Natural::ZERO));
    assert_panic!((&Natural::from(30u8)).mod_mul(&Natural::from(3u8), &Natural::from(30u8)));
    assert_panic!((&Natural::from(3u8)).mod_mul(&Natural::from(30u8), &Natural::from(30u8)));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_mul_assign(Natural::ZERO, Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_mul_assign(Natural::from(3u8), Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_mul_assign(Natural::from(30u8), Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_mul_assign(Natural::ZERO, &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_mul_assign(Natural::from(3u8), &Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_mul_assign(Natural::from(30u8), &Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_mul_assign(Natural::ZERO, Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_mul_assign(&Natural::from(3u8), Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_mul_assign(&Natural::from(30u8), Natural::from(30u8));
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_mul_assign(Natural::ZERO, &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u8);
        x.mod_mul_assign(&Natural::from(3u8), &Natural::from(30u8));
    });
    assert_panic!({
        let mut x = Natural::from(3u8);
        x.mod_mul_assign(&Natural::from(30u8), &Natural::from(30u8));
    });
}

#[test]
fn mod_mul_properties() {
    natural_triple_gen_var_3().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let product_val_val_val = x.clone().mod_mul(y.clone(), m.clone());
        let product_val_ref_val = x.clone().mod_mul(&y, m.clone());
        let product_ref_val_val = (&x).mod_mul(y.clone(), m.clone());
        let product_ref_ref_val = (&x).mod_mul(&y, m.clone());
        let product_val_val_ref = x.clone().mod_mul(y.clone(), &m);
        let product_val_ref_ref = x.clone().mod_mul(&y, &m);
        let product_ref_val_ref = (&x).mod_mul(y.clone(), &m);
        let product = (&x).mod_mul(&y, &m);
        assert!(product_val_val_val.is_valid());
        assert!(product_val_ref_val.is_valid());
        assert!(product_ref_val_val.is_valid());
        assert!(product_val_val_ref.is_valid());
        assert!(product_val_val_ref.is_valid());
        assert!(product_val_ref_ref.is_valid());
        assert!(product_ref_val_ref.is_valid());
        assert!(product.is_valid());
        assert!(product.mod_is_reduced(&m));
        assert_eq!(product_val_val_val, product);
        assert_eq!(product_val_ref_val, product);
        assert_eq!(product_ref_val_val, product);
        assert_eq!(product_ref_ref_val, product);
        assert_eq!(product_val_val_ref, product);
        assert_eq!(product_val_ref_ref, product);
        assert_eq!(product_ref_val_ref, product);

        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(y.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(&y, m.clone());
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(y.clone(), &m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_assign(&y, &m);
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);

        let product_pre_val_val_val = x.clone().mod_mul_precomputed(y.clone(), m.clone(), &data);
        let product_pre_val_ref_val = x.clone().mod_mul_precomputed(&y, m.clone(), &data);
        let product_pre_ref_val_val = (&x).mod_mul_precomputed(y.clone(), m.clone(), &data);
        let product_pre_ref_ref_val = (&x).mod_mul_precomputed(&y, m.clone(), &data);
        let product_pre_val_val_ref = x.clone().mod_mul_precomputed(y.clone(), &m, &data);
        let product_pre_val_ref_ref = x.clone().mod_mul_precomputed(&y, &m, &data);
        let product_pre_ref_val_ref = (&x).mod_mul_precomputed(y.clone(), &m, &data);
        let product_pre_ref_ref_ref = (&x).mod_mul_precomputed(&y, &m, &data);
        assert!(product_pre_val_val_val.is_valid());
        assert!(product_pre_val_ref_val.is_valid());
        assert!(product_pre_ref_val_val.is_valid());
        assert!(product_pre_val_val_ref.is_valid());
        assert!(product_pre_val_val_ref.is_valid());
        assert!(product_pre_val_ref_ref.is_valid());
        assert!(product_pre_ref_val_ref.is_valid());
        assert!(product_pre_ref_ref_ref.is_valid());
        assert_eq!(product_pre_val_val_val, product);
        assert_eq!(product_pre_val_ref_val, product);
        assert_eq!(product_pre_ref_val_val, product);
        assert_eq!(product_pre_ref_ref_val, product);
        assert_eq!(product_pre_val_val_ref, product);
        assert_eq!(product_pre_val_ref_ref, product);
        assert_eq!(product_pre_ref_val_ref, product);

        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(y.clone(), m.clone(), &data);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(&y, m.clone(), &data);
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(y.clone(), &m, &data);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_mul_precomputed_assign(&y, &m, &data);
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        assert_eq!((&x * &y) % &m, product);

        assert_eq!((&y).mod_mul(&x, &m), product);
        assert_eq!((&x).mod_mul((&y).mod_neg(&m), &m), (&product).mod_neg(&m));
        assert_eq!(x.mod_neg(&m).mod_mul(y, &m), product.mod_neg(m));
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert_eq!((&x).mod_mul(Natural::ZERO, &m), 0);
        assert_eq!(Natural::ZERO.mod_mul(&x, &m), 0);
        if m > 1 {
            assert_eq!((&x).mod_mul(Natural::ONE, &m), x);
            assert_eq!(Natural::ONE.mod_mul(&x, &m), x);
        }
        assert_eq!((&x).mod_mul(&x, &m), x.mod_square(m));
    });

    natural_quadruple_gen_var_1().test_properties(|(ref x, ref y, ref z, ref m)| {
        assert_eq!(x.mod_mul(y, m).mod_mul(z, m), x.mod_mul(y.mod_mul(z, m), m));
        assert_eq!(
            x.mod_mul(y.mod_add(z, m), m),
            x.mod_mul(y, m).mod_add(x.mod_mul(z, m), m)
        );
        assert_eq!(
            x.mod_add(y, m).mod_mul(z, m),
            x.mod_mul(z, m).mod_add(y.mod_mul(z, m), m)
        );
    });

    unsigned_triple_gen_var_12::<Limb>().test_properties(|(x, y, m)| {
        assert_eq!(
            x.mod_mul(y, m),
            Natural::from(x).mod_mul(Natural::from(y), Natural::from(m))
        );
    });
}
