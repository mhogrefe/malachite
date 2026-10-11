// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ElementSum, ModPowerOf2, ModPowerOf2ElementSum, ModPowerOf2IsReduced,
};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_3;

#[test]
fn test_mod_power_of_2_element_sum() {
    let test = |s, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let sum = v.to_mod_power_of_2_element_sum(pow);
        assert_eq!(sum.to_string(), out);
        assert_eq!(v.into_mod_power_of_2_element_sum(pow), sum);
    };
    test("()", 0, "0");
    test("()", 3, "0");
    test("(5, 6, 7)", 3, "2");
    test("(18446744073709551615, 1)", 64, "0");
    test(
        "(1267650600228229401496703205375, 1267650600228229401496703205375)",
        100,
        "1267650600228229401496703205374",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_element_sum_fail() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 8)")
        .unwrap()
        .to_mod_power_of_2_element_sum(3);
}

#[test]
fn mod_power_of_2_element_sum_properties() {
    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        let sum = v.to_mod_power_of_2_element_sum(pow);
        assert_eq!(v.clone().into_mod_power_of_2_element_sum(pow), sum);
        assert!(sum.mod_power_of_2_is_reduced(pow));
        // It is the sum of the elements, reduced.
        assert_eq!(v.into_element_sum().mod_power_of_2(pow), sum);
    });

    unsigned_vector_unsigned_pair_gen_var_4::<Limb>().test_properties(|(v, pow)| {
        // It agrees with the UnsignedVector implementation.
        assert_eq!(
            NaturalVector::from(v.clone()).into_mod_power_of_2_element_sum(pow),
            Natural::from(v.into_mod_power_of_2_element_sum(pow))
        );
    });
}
