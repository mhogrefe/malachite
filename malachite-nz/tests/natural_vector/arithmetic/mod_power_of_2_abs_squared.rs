// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    AbsSquared, ModPowerOf2, ModPowerOf2AbsSquared, ModPowerOf2Dot, ModPowerOf2IsReduced,
};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_3;

#[test]
fn test_mod_power_of_2_abs_squared() {
    let test = |s, pow, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let n = (&v).mod_power_of_2_abs_squared(pow);
        assert_eq!(n.to_string(), out);
        assert_eq!(v.mod_power_of_2_abs_squared(pow), n);
    };
    test("()", 0, "0");
    test("(1, 2, 3)", 3, "6");
    test("(1267650600228229401496703205375, 2)", 100, "5");
}

#[test]
#[should_panic]
fn mod_power_of_2_abs_squared_fail() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 8)")
        .unwrap()
        .mod_power_of_2_abs_squared(3);
}

#[test]
fn mod_power_of_2_abs_squared_properties() {
    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        let n = (&v).mod_power_of_2_abs_squared(pow);
        assert_eq!(v.clone().mod_power_of_2_abs_squared(pow), n);
        assert!(n.mod_power_of_2_is_reduced(pow));
        // It is the dot product of the vector with itself, and the squared norm, reduced.
        assert_eq!((&v).mod_power_of_2_dot(&v, pow), n);
        assert_eq!(v.abs_squared().mod_power_of_2(pow), n);
    });

    unsigned_vector_unsigned_pair_gen_var_4::<Limb>().test_properties(|(v, pow)| {
        // It agrees with the UnsignedVector implementation.
        assert_eq!(
            NaturalVector::from(v.clone()).mod_power_of_2_abs_squared(pow),
            Natural::from(v.mod_power_of_2_abs_squared(pow))
        );
    });
}
