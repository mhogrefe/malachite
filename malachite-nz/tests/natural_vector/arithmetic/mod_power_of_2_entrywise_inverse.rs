// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModEntrywiseInverse, ModPowerOf2, ModPowerOf2EntrywiseInverse, ModPowerOf2EntrywiseMul,
    ModPowerOf2IsReduced, Parity, PowerOf2,
};
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_unsigned_pair_gen_var_3;
use malachite_nz::test_util::natural_vector::arithmetic::mod_power_of_2_entrywise_inverse::*;

#[test]
fn test_mod_power_of_2_entrywise_inverse() {
    let test = |s, pow, out: Option<&str>| {
        let v = NaturalVector::from_str(s).unwrap();
        let r = (&v).mod_power_of_2_entrywise_inverse(pow);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!(v.clone().mod_power_of_2_entrywise_inverse(pow), r);
        assert_eq!(
            natural_vector_mod_power_of_2_entrywise_inverse_naive(&v, pow),
            r
        );
    };
    test("()", 0, Some("()"));
    test("(0)", 0, Some("(0)"));
    test("(1, 3, 5, 7)", 4, Some("(1, 11, 13, 7)"));
    test("(1, 2, 3)", 4, None);
    test(
        "(1267650600228229401496703205375, 3, 18446744073709551617)",
        100,
        Some(
            "(1267650600228229401496703205375, 845100400152152934331135470251, \
            1267650600209782657422993653761)",
        ),
    );
    test("(18446744073709551616, 3)", 100, None);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_inverse_fail_1() {
    // An element is not reduced.
    (&NaturalVector::from_str("(1, 8)").unwrap()).mod_power_of_2_entrywise_inverse(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_inverse_fail_2() {
    NaturalVector::from_str("(1, 8)")
        .unwrap()
        .mod_power_of_2_entrywise_inverse(3);
}

fn check_inverses(v: &NaturalVector, r: &NaturalVector, pow: u64) {
    // The inverses are reduced, there is one for each element, and each times its element is 1.
    assert!(r.mod_power_of_2_is_reduced(pow));
    assert_eq!(r.dimension(), v.dimension());
    let ones = NaturalVector {
        elements: vec![Natural::ONE.mod_power_of_2(pow); v.elements.len()],
    };
    assert_eq!(v.mod_power_of_2_entrywise_mul(r, pow), ones);
    // Inverting again gives back the original vector.
    assert_eq!(r.mod_power_of_2_entrywise_inverse(pow).as_ref(), Some(v));
    // It is `mod_entrywise_inverse` with modulus $2^k$.
    assert_eq!(
        v.mod_entrywise_inverse(Natural::power_of_2(pow)).as_ref(),
        Some(r)
    );
}

#[test]
fn mod_power_of_2_entrywise_inverse_properties() {
    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        let r = (&v).mod_power_of_2_entrywise_inverse(pow);
        assert_eq!(v.clone().mod_power_of_2_entrywise_inverse(pow), r);
        assert_eq!(
            natural_vector_mod_power_of_2_entrywise_inverse_naive(&v, pow),
            r
        );
        // There are inverses exactly when the modulus is 1 or every element is odd.
        assert_eq!(r.is_some(), pow == 0 || v.elements.iter().all(Parity::odd));
        if let Some(r) = r {
            check_inverses(&v, &r, pow);
        } else {
            assert_eq!((&v).mod_entrywise_inverse(Natural::power_of_2(pow)), None);
        }

        // Keeping only the odd elements always gives inverses.
        let u = natural_vector_retain_odd(&v);
        let r = (&u).mod_power_of_2_entrywise_inverse(pow).unwrap();
        check_inverses(&u, &r, pow);
    });

    unsigned_vector_unsigned_pair_gen_var_4::<u64>().test_properties(|(v, pow)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            (&v).mod_power_of_2_entrywise_inverse(pow)
                .map(NaturalVector::from),
            NaturalVector::from(v).mod_power_of_2_entrywise_inverse(pow)
        );
    });
}
