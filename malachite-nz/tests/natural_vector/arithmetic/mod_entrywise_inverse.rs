// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    CoprimeWith, ModEntrywiseInverse, ModEntrywiseMul, ModIsReduced,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_5;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen_var_2;
use malachite_nz::test_util::natural_vector::arithmetic::mod_entrywise_inverse::*;

#[test]
fn test_mod_entrywise_inverse() {
    let test = |s, m: Natural, out: Option<&str>| {
        let v = NaturalVector::from_str(s).unwrap();
        let r = (&v).mod_entrywise_inverse(&m);
        assert_eq!(r.as_ref().map(ToString::to_string).as_deref(), out);
        assert_eq!((&v).mod_entrywise_inverse(m.clone()), r);
        assert_eq!(v.clone().mod_entrywise_inverse(&m), r);
        assert_eq!(v.clone().mod_entrywise_inverse(m.clone()), r);
        assert_eq!(natural_vector_mod_entrywise_inverse_naive(&v, &m), r);
    };
    test("()", Natural::ONE, Some("()"));
    test("(0)", Natural::ONE, Some("(0)"));
    test("(1, 2, 3, 4)", Natural::from(7u32), Some("(1, 4, 5, 2)"));
    test("(1, 2, 3)", Natural::from(6u32), None);
    test("(0, 1)", Natural::from(7u32), None);
    test(
        "(18446744073709551616, 3, 170141183460469231731687303715884105726)",
        Natural::from_str("170141183460469231731687303715884105727").unwrap(),
        Some(
            "(9223372036854775808, 113427455640312821154458202477256070485, \
            170141183460469231731687303715884105726)",
        ),
    );
    test(
        "(18446744073709551616, 6)",
        Natural::from_str("36893488147419103232").unwrap(),
        None,
    );
}

#[test]
#[should_panic]
fn mod_entrywise_inverse_fail_1() {
    // An element is not reduced.
    (&NaturalVector::from_str("(1, 7)").unwrap()).mod_entrywise_inverse(&Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_entrywise_inverse_fail_2() {
    // The modulus is zero.
    (&NaturalVector::from_str("()").unwrap()).mod_entrywise_inverse(&Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_entrywise_inverse_fail_3() {
    NaturalVector::from_str("(1, 7)")
        .unwrap()
        .mod_entrywise_inverse(Natural::from(7u32));
}

fn check_inverses(v: &NaturalVector, r: &NaturalVector, m: &Natural) {
    // The inverses are reduced, there is one for each element, and each times its element is 1.
    assert!(r.mod_is_reduced(m));
    assert_eq!(r.dimension(), v.dimension());
    let ones = NaturalVector {
        elements: vec![Natural::ONE % m; v.elements.len()],
    };
    assert_eq!(v.mod_entrywise_mul(r, m), ones);
    // Inverting again gives back the original vector.
    assert_eq!(r.mod_entrywise_inverse(m).as_ref(), Some(v));
}

#[test]
fn mod_entrywise_inverse_properties() {
    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        let r = (&v).mod_entrywise_inverse(&m);
        assert_eq!((&v).mod_entrywise_inverse(m.clone()), r);
        assert_eq!(v.clone().mod_entrywise_inverse(&m), r);
        assert_eq!(v.clone().mod_entrywise_inverse(m.clone()), r);
        assert_eq!(natural_vector_mod_entrywise_inverse_naive(&v, &m), r);
        // There are inverses exactly when every element is coprime to `m`.
        assert_eq!(r.is_some(), v.elements.iter().all(|x| x.coprime_with(&m)));
        if let Some(r) = r {
            check_inverses(&v, &r, &m);
        }

        // Keeping only the invertible elements always gives inverses.
        let u = natural_vector_retain_units(&v, &m);
        let r = (&u).mod_entrywise_inverse(&m).unwrap();
        check_inverses(&u, &r, &m);
    });

    unsigned_vector_unsigned_pair_gen_var_5::<u64>().test_properties(|(v, m)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            (&v).mod_entrywise_inverse(m).map(NaturalVector::from),
            NaturalVector::from(v).mod_entrywise_inverse(Natural::from(m))
        );
    });
}
