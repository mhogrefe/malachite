// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Dot, Mod, ModDot, ModIsReduced, ModPowerOf2Dot, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_natural_triple_gen_var_1,
    natural_vector_natural_vector_unsigned_triple_gen_var_1,
};

#[test]
fn test_mod_dot() {
    let test = |a, b, m, out| {
        let v = NaturalVector::from_str(a).unwrap();
        let w = NaturalVector::from_str(b).unwrap();
        let m = Natural::from_str(m).unwrap();
        let d = (&v).mod_dot(&w, &m);
        assert!(d.is_valid());
        assert_eq!(d.to_string(), out);
        assert_eq!((&v).mod_dot(&w, m.clone()), d);
        assert_eq!((&v).mod_dot(w.clone(), &m), d);
        assert_eq!((&v).mod_dot(w.clone(), m.clone()), d);
        assert_eq!(v.clone().mod_dot(&w, &m), d);
        assert_eq!(v.clone().mod_dot(&w, m.clone()), d);
        assert_eq!(v.clone().mod_dot(w.clone(), &m), d);
        assert_eq!(v.mod_dot(w, m), d);
    };
    test("()", "()", "1", "0");
    test("()", "()", "7", "0");
    // - m fits in a limb
    test("(1, 2, 3)", "(4, 5, 7)", "10", "5");
    test(
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        "18446744073709551615",
        "3",
    );
    // - m is larger than a limb
    test(
        "(1000000000000000000000000000056, 1000000000000000000000000000055)",
        "(1000000000000000000000000000056, 1000000000000000000000000000054)",
        "1000000000000000000000000000057",
        "7",
    );
}

#[test]
#[should_panic]
fn mod_dot_fail_1() {
    NaturalVector::from_str("()")
        .unwrap()
        .mod_dot(NaturalVector::from_str("()").unwrap(), Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_dot_fail_2() {
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .mod_dot(NaturalVector::from_str("(1)").unwrap(), Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_dot_fail_3() {
    NaturalVector::from_str("(1, 2)").unwrap().mod_dot(
        NaturalVector::from_str("(1, 7)").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_dot_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_1().test_properties(|(v, w, m)| {
        let d = (&v).mod_dot(&w, &m);
        assert!(d.is_valid());
        assert!(d.mod_is_reduced(&m));
        assert_eq!((&v).mod_dot(&w, m.clone()), d);
        assert_eq!((&v).mod_dot(w.clone(), &m), d);
        assert_eq!(v.clone().mod_dot(&w, &m), d);
        assert_eq!(v.clone().mod_dot(w.clone(), m.clone()), d);
        // It is symmetric, and it is the exact dot product, reduced.
        assert_eq!((&w).mod_dot(&v, &m), d);
        assert_eq!(v.dot(w).mod_op(&m), d);
    });

    natural_vector_natural_vector_unsigned_triple_gen_var_1().test_properties(|(v, w, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_dot`.
        assert_eq!(
            (&v).mod_dot(&w, Natural::power_of_2(pow)),
            v.mod_power_of_2_dot(w, pow)
        );
    });

    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<Limb>().test_properties(
        |(v, w, m)| {
            // It agrees with the UnsignedVector implementation.
            assert_eq!(
                NaturalVector::from(v.clone())
                    .mod_dot(NaturalVector::from(w.clone()), Natural::from(m)),
                Natural::from(v.mod_dot(w, m))
            );
        },
    );
}
