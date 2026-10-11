// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Sum,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_11, natural_vector_unsigned_pair_gen_var_3,
};

#[test]
fn test_mod_power_of_2_sum() {
    let test = |s, pow, out| {
        let xs = vec_from_str::<Natural>(s).unwrap();
        let sum = Natural::mod_power_of_2_sum(xs.iter(), pow);
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);
        let sum_alt = Natural::mod_power_of_2_sum(xs.into_iter(), pow);
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);
    };
    test("[]", 0, "0");
    test("[]", 100, "0");
    test("[0]", 0, "0");
    test("[1]", 1, "1");
    test("[2, 3, 5, 7]", 4, "1");
    test("[2, 3, 5, 7]", 100, "17");
    test("[18446744073709551615, 18446744073709551615, 2]", 65, "0");
    test(
        "[1267650600228229401496703205375, 1267650600228229401496703205375]",
        100,
        "1267650600228229401496703205374",
    );
    test(
        "[100000000000000000000, 1000000000000000000000, 10000000000000000000000]",
        80,
        "11100000000000000000000",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sum_fail_1() {
    Natural::mod_power_of_2_sum([Natural::ONE].into_iter(), 0);
}

#[test]
#[should_panic]
fn mod_power_of_2_sum_fail_2() {
    Natural::mod_power_of_2_sum(
        [Natural::ONE, Natural::from_str("1267650600228229401496703205376").unwrap()].iter(),
        100,
    );
}

#[test]
fn mod_power_of_2_sum_properties() {
    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        let xs = v.elements;
        let sum = Natural::mod_power_of_2_sum(xs.iter(), pow);
        assert!(sum.is_valid());
        assert!(sum.mod_power_of_2_is_reduced(pow));
        assert_eq!(Natural::mod_power_of_2_sum(xs.iter().cloned(), pow), sum);
        // It is the exact sum, reduced, and the modular sum taken one element at a time, in any
        // order.
        assert_eq!(Natural::sum(xs.iter()).mod_power_of_2(pow), sum);
        assert_eq!(
            xs.iter()
                .fold(Natural::ZERO, |s, x| s.mod_power_of_2_add(x, pow)),
            sum
        );
        assert_eq!(Natural::mod_power_of_2_sum(xs.iter().rev(), pow), sum);
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(x, pow)| {
        assert_eq!(Natural::mod_power_of_2_sum([&x].into_iter(), pow), x);
        assert_eq!(
            Natural::mod_power_of_2_sum([&x, &(&x).mod_power_of_2_neg(pow)].into_iter(), pow),
            0u32
        );
    });

    unsigned_vector_unsigned_pair_gen_var_4::<Limb>().test_properties(|(v, pow)| {
        // It agrees with the primitive implementation.
        assert_eq!(
            Natural::mod_power_of_2_sum(v.elements.iter().map(|&x| Natural::from(x)), pow),
            Limb::mod_power_of_2_sum(v.elements.iter(), pow)
        );
    });
}
