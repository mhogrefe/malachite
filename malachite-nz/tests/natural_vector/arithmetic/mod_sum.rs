// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModSum};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen_var_2,
    natural_vector_natural_vector_natural_vector_natural_quadruple_gen_var_1,
};

#[test]
fn test_mod_sum() {
    let test = |ss: &[&str], m, out| {
        let vs: Vec<NaturalVector> = ss
            .iter()
            .map(|s| NaturalVector::from_str(s).unwrap())
            .collect();
        let m = Natural::from_str(m).unwrap();
        let sum = NaturalVector::mod_sum(vs.iter(), &m);
        assert_eq!(sum.to_string(), out);
        assert_eq!(NaturalVector::mod_sum(vs.iter(), m.clone()), sum);
        assert_eq!(NaturalVector::mod_sum(vs.iter().cloned(), &m), sum);
        assert_eq!(NaturalVector::mod_sum(vs.into_iter(), m), sum);
    };
    test(&["()"], "3", "()");
    test(&["(5, 1, 3)"], "7", "(5, 1, 3)");
    test(&["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"], "7", "(6, 2, 5)");
    test(
        &[
            "(1000000000000000000000000000056, 1)",
            "(1000000000000000000000000000056, 1000000000000000000000000000055)",
        ],
        "1000000000000000000000000000057",
        "(1000000000000000000000000000055, 1000000000000000000000000000056)",
    );
    test(
        &["(1267650600228229401496703205374)", "(1267650600228229401496703205374)"],
        "1267650600228229401496703205375",
        "(1267650600228229401496703205373)",
    );
    test(&["()", "()"], "5", "()");
}

#[test]
#[should_panic]
fn mod_sum_fail_1() {
    NaturalVector::mod_sum(core::iter::empty::<NaturalVector>(), Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_sum_fail_2() {
    let v = NaturalVector::from_str("(1)").unwrap();
    let big = NaturalVector::from_str("(7)").unwrap();
    NaturalVector::mod_sum([&v, &big].into_iter(), &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_sum_fail_3() {
    let v = NaturalVector::from_str("(1)").unwrap();
    let long = NaturalVector::from_str("(1, 1)").unwrap();
    NaturalVector::mod_sum([v, long].into_iter(), Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_sum_fail_4() {
    let v = NaturalVector::from_str("(0)").unwrap();
    NaturalVector::mod_sum([v].into_iter(), Natural::ZERO);
}

#[test]
fn mod_sum_properties() {
    natural_vector_natural_vector_natural_vector_natural_quadruple_gen_var_1().test_properties(
        |(u, v, w, m)| {
            let sum = NaturalVector::mod_sum([&u, &v, &w].into_iter(), &m);
            assert!(sum.mod_is_reduced(&m));
            assert_eq!(
                NaturalVector::mod_sum([u.clone(), v.clone(), w.clone()].into_iter(), m.clone()),
                sum
            );
            // It is repeated `mod_add`, in any order, and entry by entry the scalar sum.
            assert_eq!((&u).mod_add(&v, &m).mod_add(&w, &m), sum);
            assert_eq!(NaturalVector::mod_sum([&w, &u, &v].into_iter(), &m), sum);
            for (i, s) in sum.elements.iter().enumerate() {
                assert_eq!(
                    Natural::mod_sum(
                        [&u.elements[i], &v.elements[i], &w.elements[i]].into_iter(),
                        &m
                    ),
                    *s
                );
            }
        },
    );

    natural_vector_natural_pair_gen_var_2().test_properties(|(v, m)| {
        // The sum of one vector is that vector.
        assert_eq!(NaturalVector::mod_sum([&v].into_iter(), &m), v);
    });

    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2::<Limb>()
        .test_properties(|(u, v, w, m)| {
            // It agrees with the UnsignedVector implementation.
            let sum = UnsignedVector::mod_sum([&u, &v, &w].into_iter(), m);
            assert_eq!(
                NaturalVector::mod_sum(
                    [NaturalVector::from(u), NaturalVector::from(v), NaturalVector::from(w)]
                        .into_iter(),
                    Natural::from(m)
                ),
                NaturalVector::from(sum)
            );
        });
}
