// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Product;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2Product,
    ModPowerOf2Square, Parity,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_11, natural_vector_unsigned_pair_gen_var_3,
};

#[test]
fn test_mod_power_of_2_product() {
    let test = |s, pow, out| {
        let xs = vec_from_str::<Natural>(s).unwrap();
        let product = Natural::mod_power_of_2_product(xs.iter(), pow);
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        let product_alt = Natural::mod_power_of_2_product(xs.into_iter(), pow);
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);
    };
    test("[]", 0, "0");
    test("[]", 100, "1");
    test("[0]", 0, "0");
    test("[1]", 1, "1");
    test("[2, 3, 5, 7]", 4, "2");
    test("[2, 3, 5, 7]", 100, "210");
    test(
        "[18446744073709551615, 18446744073709551617, 3]",
        65,
        "36893488147419103229",
    );
    test(
        "[1267650600228229401496703205375, 1267650600228229401496703205375]",
        100,
        "1",
    );
    test(
        "[100000000000000000000, 1000000000000000000000, 0]",
        80,
        "0",
    );
}

#[test]
fn mod_power_of_2_product_early_return() {
    // Once the partial product is zero, the remaining factors are not examined, so the unreduced
    // final factor does not panic.
    let big = Natural::from_str("1267650600228229401496703205376").unwrap();
    let x = Natural::from_str("633825300114114700748351602688").unwrap();
    assert_eq!(
        Natural::mod_power_of_2_product([&Natural::ZERO, &big].into_iter(), 100),
        0u32
    );
    assert_eq!(
        Natural::mod_power_of_2_product([&x, &Natural::TWO, &big].into_iter(), 100),
        0u32
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_product_fail_1() {
    Natural::mod_power_of_2_product([Natural::ONE].into_iter(), 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_product_fail_2() {
    Natural::mod_power_of_2_product(
        [Natural::ONE, Natural::from_str("1267650600228229401496703205376").unwrap()].iter(),
        100,
    );
}

#[test]
fn mod_power_of_2_product_properties() {
    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        let xs = v.elements;
        let product = Natural::mod_power_of_2_product(xs.iter(), pow);
        assert!(product.is_valid());
        assert!(product.mod_power_of_2_is_reduced(pow));
        assert_eq!(
            Natural::mod_power_of_2_product(xs.iter().cloned(), pow),
            product
        );
        // It is the exact product, reduced, and the modular product taken one element at a time, in
        // any order.
        assert_eq!(Natural::product(xs.iter()).mod_power_of_2(pow), product);
        assert_eq!(
            xs.iter().fold(Natural::ONE.mod_power_of_2(pow), |p, x| p
                .mod_power_of_2_mul(x, pow)),
            product
        );
        assert_eq!(
            Natural::mod_power_of_2_product(xs.iter().rev(), pow),
            product
        );
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(x, pow)| {
        assert_eq!(Natural::mod_power_of_2_product([&x].into_iter(), pow), x);
        assert_eq!(
            Natural::mod_power_of_2_product([&x, &x].into_iter(), pow),
            (&x).mod_power_of_2_square(pow)
        );
        assert_eq!(
            Natural::mod_power_of_2_product([&x, &Natural::ZERO].into_iter(), pow),
            0u32
        );
        if x.odd() {
            let inverse = (&x).mod_power_of_2_inverse(pow).unwrap();
            assert_eq!(
                Natural::mod_power_of_2_product([&x, &inverse].into_iter(), pow),
                Natural::ONE.mod_power_of_2(pow)
            );
        }
    });

    unsigned_vector_unsigned_pair_gen_var_4::<Limb>().test_properties(|(v, pow)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::mod_power_of_2_product(v.elements.iter().map(|&x| Natural::from(x)), pow),
            Limb::mod_power_of_2_product(v.elements.iter(), pow)
        );
    });
}
