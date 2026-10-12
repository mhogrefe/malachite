// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    AbsSquared, Mod, ModAbsSquared, ModDot, ModIsReduced, ModPowerOf2AbsSquared, PowerOf2,
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
fn test_mod_abs_squared() {
    let test = |s, m, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let n = (&v).mod_abs_squared(&m);
        assert_eq!(n.to_string(), out);
        assert_eq!((&v).mod_abs_squared(m.clone()), n);
        assert_eq!(v.clone().mod_abs_squared(&m), n);
        assert_eq!(v.mod_abs_squared(m), n);
    };
    test("()", "1", "0");
    test("()", "7", "0");
    test("(1, 2, 3)", "10", "4");
    test(
        "(1000000000000000000000000000056, 1000000000000000000000000000056, 1)",
        "1000000000000000000000000000057",
        "3",
    );
}

#[test]
#[should_panic]
fn mod_abs_squared_fail_1() {
    // An element is not reduced.
    NaturalVector::from_str("(1, 7)")
        .unwrap()
        .mod_abs_squared(Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_abs_squared_fail_2() {
    // The modulus is zero, even though there are no elements to check.
    NaturalVector::from_str("()")
        .unwrap()
        .mod_abs_squared(Natural::ZERO);
}

#[test]
fn mod_abs_squared_properties() {
    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        let n = (&v).mod_abs_squared(&m);
        assert_eq!((&v).mod_abs_squared(m.clone()), n);
        assert_eq!(v.clone().mod_abs_squared(&m), n);
        assert_eq!(v.clone().mod_abs_squared(m.clone()), n);
        assert!(n.mod_is_reduced(&m));
        // It is the dot product of the vector with itself, and the squared norm, reduced.
        assert_eq!((&v).mod_dot(&v, &m), n);
        assert_eq!(v.abs_squared().mod_op(&m), n);
    });

    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_abs_squared`.
        assert_eq!(
            (&v).mod_abs_squared(Natural::power_of_2(pow)),
            v.mod_power_of_2_abs_squared(pow)
        );
    });

    unsigned_vector_unsigned_pair_gen_var_5::<Limb>().test_properties(|(v, m)| {
        // It agrees with the UnsignedVector implementation.
        assert_eq!(
            NaturalVector::from(v.clone()).mod_abs_squared(Natural::from(m)),
            Natural::from(v.mod_abs_squared(m))
        );
    });
}
