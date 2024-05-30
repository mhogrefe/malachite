// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModMul, ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2MulAssign, ModPowerOf2Neg, ModPowerOf2Square, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_triple_gen_var_11;
use malachite_nz::natural::arithmetic::mod_power_of_2_mul::{
    limbs_mod_power_of_2_mul, limbs_mod_power_of_2_mul_ref_ref, limbs_mod_power_of_2_mul_val_ref,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_natural_natural_unsigned_quadruple_gen_var_2,
    natural_natural_unsigned_triple_gen_var_4, natural_unsigned_pair_gen_var_11,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_mul() {
    let test = |xs, ys, pow, out: &[Limb]| {
        assert_eq!(limbs_mod_power_of_2_mul_ref_ref(xs, ys, pow), out);

        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_mod_power_of_2_mul_val_ref(&mut mut_xs, ys, pow), out);

        let mut mut_xs = xs.to_vec();
        let mut mut_ys = ys.to_vec();
        assert_eq!(limbs_mod_power_of_2_mul(&mut mut_xs, &mut mut_ys, pow), out);

        let product = Natural::from_limbs_asc(out);
        assert_eq!(
            Natural::from_limbs_asc(xs).mod_power_of_2_mul(Natural::from_limbs_asc(ys), pow),
            product
        );
        assert_eq!(
            (Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys)).mod_power_of_2(pow),
            product
        );
    };
    // - max_len <= xs_len + ys_len + 1
    // - xs_len >= limit && ys_len >= limit
    // - xs_len == max_len
    // - ys_len == max_len
    test(&[1], &[1], 1, &[1]);
    test(&[1], &[1], 5, &[1]);
    // - xs_len < max_len
    // - ys_len < max_len
    test(&[1], &[1], 33, &[1, 0]);
    test(&[2], &[1], 3, &[2]);
    test(&[1], &[2], 3, &[2]);
    test(&[2], &[3], 2, &[2]);
    // - xs_len < limit || ys_len < limit
    test(&[1, 2, 3], &[6, 7], 100, &[6, 19, 32, 5]);
    test(&[6, 7], &[1, 2, 3], 100, &[6, 19, 32, 5]);
    // - max_len > xs_len + ys_len + 1
    test(&[3255925883], &[3653042335], 131, &[2997571685, 2769295845]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_mul_fail_1() {
    limbs_mod_power_of_2_mul(&mut vec![1], &mut vec![], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_mul_fail_2() {
    limbs_mod_power_of_2_mul(&mut vec![], &mut vec![1], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_mul_val_ref_fail_1() {
    limbs_mod_power_of_2_mul_val_ref(&mut vec![1], &[], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_mul_val_ref_fail_2() {
    limbs_mod_power_of_2_mul_val_ref(&mut vec![], &[1], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_mul_ref_ref_fail_1() {
    limbs_mod_power_of_2_mul_ref_ref(&[1], &[], 2);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_mul_ref_ref_fail_2() {
    limbs_mod_power_of_2_mul_ref_ref(&[], &[1], 2);
}

#[test]
fn test_mod_power_of_2_mul() {
    let test = |s, t, pow, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert!(u.mod_power_of_2_is_reduced(pow));
        assert!(v.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_mul_assign(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_mul_assign(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_mul(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_mul(v.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_mul(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_mul(&v, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("1", "1", 5, "1");
    test("1", "1", 33, "1");
    test("1", "2", 5, "2");
    test("3", "2", 5, "6");
    test("10", "14", 4, "12");
    test("123", "456", 9, "280");
    test("123456789", "987654321", 60, "121932631112635269");
}

#[test]
fn mod_power_of_2_mul_fail() {
    assert_panic!(Natural::ZERO.mod_power_of_2_mul(Natural::ONE, 0));
    assert_panic!(Natural::ONE.mod_power_of_2_mul(Natural::ZERO, 0));

    assert_panic!(Natural::ZERO.mod_power_of_2_mul(&Natural::ONE, 0));
    assert_panic!(Natural::ONE.mod_power_of_2_mul(&Natural::ZERO, 0));

    assert_panic!((&Natural::ZERO).mod_power_of_2_mul(Natural::ONE, 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_mul(Natural::ZERO, 0));

    assert_panic!((&Natural::ZERO).mod_power_of_2_mul(Natural::ONE, 0));
    assert_panic!((&Natural::ONE).mod_power_of_2_mul(Natural::ZERO, 0));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_power_of_2_mul_assign(Natural::ONE, 0);
    });
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_mul_assign(Natural::ZERO, 0);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_power_of_2_mul_assign(&Natural::ONE, 0);
    });
    assert_panic!({
        let mut x = Natural::ONE;
        x.mod_power_of_2_mul_assign(&Natural::ZERO, 0);
    });
}

#[test]
fn limbs_mod_power_of_2_mul_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_20().test_properties_with_config(
        &config,
        |(mut xs, mut ys, pow)| {
            let old_xs = xs.clone();
            let old_ys = ys.clone();
            let product =
                Natural::from_limbs_asc(&xs).mod_power_of_2_mul(Natural::from_limbs_asc(&ys), pow);
            assert_eq!(
                (Natural::from_limbs_asc(&xs) * Natural::from_limbs_asc(&ys)).mod_power_of_2(pow),
                product
            );
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_mul(&mut xs, &mut ys, pow)),
                product,
            );
            let mut xs = old_xs.clone();
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_mul_val_ref(&mut xs, &ys, pow)),
                product,
            );
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_mul_ref_ref(
                    &old_xs, &old_ys, pow
                )),
                product,
            );
        },
    );
}

#[test]
fn mod_power_of_2_mul_properties() {
    natural_natural_unsigned_triple_gen_var_4().test_properties(|(x, y, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        assert!(y.mod_power_of_2_is_reduced(pow));
        let product_val_val = x.clone().mod_power_of_2_mul(y.clone(), pow);
        let product_val_ref = x.clone().mod_power_of_2_mul(&y, pow);
        let product_ref_val = (&x).mod_power_of_2_mul(y.clone(), pow);
        let product = (&x).mod_power_of_2_mul(&y, pow);
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert!(product.mod_power_of_2_is_reduced(pow));
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        assert_eq!((&x * &y).mod_power_of_2(pow), product);
        assert_eq!((&x).mod_mul(&y, Natural::power_of_2(pow)), product);

        let mut mut_x = x.clone();
        mut_x.mod_power_of_2_mul_assign(y.clone(), pow);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x.mod_power_of_2_mul_assign(&y, pow);
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        assert_eq!((&y).mod_power_of_2_mul(&x, pow), product);
        assert_eq!(
            (&x).mod_power_of_2_mul((&y).mod_power_of_2_neg(pow), pow),
            (&product).mod_power_of_2_neg(pow)
        );
        assert_eq!(
            x.mod_power_of_2_neg(pow).mod_power_of_2_mul(y, pow),
            product.mod_power_of_2_neg(pow)
        );
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(ref x, pow)| {
        assert_eq!(x.mod_power_of_2_mul(Natural::ZERO, pow), 0);
        assert_eq!(Natural::ZERO.mod_power_of_2_mul(x, pow), 0);
        if pow != 0 {
            assert_eq!(x.mod_power_of_2_mul(Natural::ONE, pow), *x);
            assert_eq!(Natural::ONE.mod_power_of_2_mul(x, pow), *x);
        }
        assert_eq!(x.mod_power_of_2_mul(x, pow), x.mod_power_of_2_square(pow));
    });

    natural_natural_natural_unsigned_quadruple_gen_var_2().test_properties(
        |(ref x, ref y, ref z, pow)| {
            assert_eq!(
                x.mod_power_of_2_mul(y, pow).mod_power_of_2_mul(z, pow),
                x.mod_power_of_2_mul(y.mod_power_of_2_mul(z, pow), pow)
            );
            assert_eq!(
                x.mod_power_of_2_mul(y.mod_power_of_2_add(z, pow), pow),
                x.mod_power_of_2_mul(y, pow)
                    .mod_power_of_2_add(x.mod_power_of_2_mul(z, pow), pow)
            );
            assert_eq!(
                x.mod_power_of_2_add(y, pow).mod_power_of_2_mul(z, pow),
                x.mod_power_of_2_mul(z, pow)
                    .mod_power_of_2_add(y.mod_power_of_2_mul(z, pow), pow)
            );
        },
    );

    unsigned_triple_gen_var_11::<Limb>().test_properties(|(x, y, pow)| {
        assert_eq!(
            x.mod_power_of_2_mul(y, pow),
            Natural::from(x).mod_power_of_2_mul(Natural::from(y), pow)
        );
    });
}
