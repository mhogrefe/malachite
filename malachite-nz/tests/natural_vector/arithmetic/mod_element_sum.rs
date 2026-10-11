// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ElementSum, Mod, ModElementSum, ModIsReduced, ModPowerOf2ElementSum, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen_var_2, natural_vector_unsigned_pair_gen_var_3,
};

#[test]
fn test_mod_element_sum() {
    let test = |s, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let sum = v.to_mod_element_sum(&m);
        assert_eq!(sum.to_string(), out);
        assert_eq!(v.to_mod_element_sum(m.clone()), sum);
        assert_eq!(v.clone().into_mod_element_sum(&m), sum);
        assert_eq!(v.into_mod_element_sum(m), sum);
    };
    test("()", "1", "0");
    test("()", "7", "0");
    test("(5, 6, 7)", "10", "8");
    test(
        "(1000000000000000000000000000056, 1000000000000000000000000000056, 1)",
        "1000000000000000000000000000057",
        "1000000000000000000000000000056",
    );
}

#[test]
#[should_panic]
fn mod_element_sum_fail_1() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 7)")
        .unwrap()
        .to_mod_element_sum(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_element_sum_fail_2() {
    // The modulus is zero, even though there are no elements to check.
    NaturalVector::from_str("()")
        .unwrap()
        .to_mod_element_sum(Natural::ZERO);
}

#[test]
fn mod_element_sum_properties() {
    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        let sum = v.to_mod_element_sum(&m);
        assert_eq!(v.to_mod_element_sum(m.clone()), sum);
        assert_eq!(v.clone().into_mod_element_sum(&m), sum);
        assert_eq!(v.clone().into_mod_element_sum(m.clone()), sum);
        assert!(sum.mod_is_reduced(&m));
        // It is the sum of the elements, reduced.
        assert_eq!(v.into_element_sum().mod_op(&m), sum);
    });

    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_element_sum`.
        assert_eq!(
            v.to_mod_element_sum(Natural::power_of_2(pow)),
            v.to_mod_power_of_2_element_sum(pow)
        );
    });

    unsigned_vector_unsigned_pair_gen_var_5::<Limb>().test_properties(|(v, m)| {
        // It agrees with the UnsignedVector implementation.
        assert_eq!(
            NaturalVector::from(v.clone()).into_mod_element_sum(Natural::from(m)),
            Natural::from(v.into_mod_element_sum(m))
        );
    });
}
