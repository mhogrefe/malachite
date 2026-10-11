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
    ModInverse, ModIsReduced, ModMul, ModProduct, ModSquare,
};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_pair_gen_var_8, natural_vector_natural_pair_gen_var_2,
};

#[test]
fn test_mod_product() {
    let test = |s, m, out| {
        let xs = vec_from_str::<Natural>(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let product = Natural::mod_product(xs.iter(), &m);
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);
        assert_eq!(Natural::mod_product(xs.iter(), m.clone()), product);
        assert_eq!(Natural::mod_product(xs.iter().cloned(), &m), product);
        let product_alt = Natural::mod_product(xs.into_iter(), m);
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);
    };
    test("[]", "1", "0");
    test("[]", "1000000000000000000000000000000", "1");
    test("[0]", "1", "0");
    test("[2, 3, 5, 7]", "11", "1");
    test("[2, 3, 5, 7]", "210", "0");
    test(
        "[18446744073709551614, 18446744073709551614, 2]",
        "18446744073709551615",
        "2",
    );
    test(
        "[1000000000000000000000000000055, 1000000000000000000000000000055]",
        "1000000000000000000000000000057",
        "4",
    );
    test(
        "[1267650600228229401496703205374, 1267650600228229401496703205374]",
        "1267650600228229401496703205375",
        "1",
    );
}

#[test]
fn mod_product_early_return() {
    // Once the partial product is zero, the remaining factors are not examined, so the unreduced
    // final factor does not panic.
    let m = Natural::from_str("1000000000000000000000000000000").unwrap();
    let x = Natural::from_str("500000000000000000000000000000").unwrap();
    assert_eq!(
        Natural::mod_product([&Natural::ZERO, &m].into_iter(), &m),
        0u32
    );
    assert_eq!(
        Natural::mod_product([&x, &Natural::TWO, &m].into_iter(), &m),
        0u32
    );
}

#[test]
#[should_panic]
fn mod_product_fail_1() {
    Natural::mod_product(core::iter::empty::<Natural>(), Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_product_fail_2() {
    Natural::mod_product([Natural::ONE].iter(), &Natural::ONE);
}

#[test]
#[should_panic]
fn mod_product_fail_3() {
    let m = Natural::from_str("1000000000000000000000000000000").unwrap();
    Natural::mod_product([Natural::ONE, m.clone()].into_iter(), m);
}

#[test]
fn mod_product_properties() {
    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        let xs = v.elements;
        let product = Natural::mod_product(xs.iter(), &m);
        assert!(product.is_valid());
        assert!(product.mod_is_reduced(&m));
        assert_eq!(Natural::mod_product(xs.iter(), m.clone()), product);
        assert_eq!(Natural::mod_product(xs.iter().cloned(), &m), product);
        // It is the exact product, reduced, and the modular product taken one element at a time, in
        // any order.
        assert_eq!(Natural::product(xs.iter()) % &m, product);
        let one = if m == 1u32 {
            Natural::ZERO
        } else {
            Natural::ONE
        };
        assert_eq!(xs.iter().fold(one, |p, x| p.mod_mul(x, &m)), product);
        assert_eq!(Natural::mod_product(xs.iter().rev(), &m), product);
    });

    natural_pair_gen_var_8().test_properties(|(x, m)| {
        assert_eq!(Natural::mod_product([&x].into_iter(), &m), x);
        assert_eq!(
            Natural::mod_product([&x, &x].into_iter(), &m),
            (&x).mod_square(&m)
        );
        assert_eq!(
            Natural::mod_product([&x, &Natural::ZERO].into_iter(), &m),
            0u32
        );
        if x != 0u32
            && let Some(inverse) = (&x).mod_inverse(&m)
        {
            let one = if m == 1u32 {
                Natural::ZERO
            } else {
                Natural::ONE
            };
            assert_eq!(Natural::mod_product([&x, &inverse].into_iter(), &m), one);
        }
    });

    unsigned_vector_unsigned_pair_gen_var_5::<Limb>().test_properties(|(v, m)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::mod_product(
                v.elements.iter().map(|&x| Natural::from(x)),
                Natural::from(m)
            ),
            Limb::mod_product(v.elements.iter(), m)
        );
    });
}
