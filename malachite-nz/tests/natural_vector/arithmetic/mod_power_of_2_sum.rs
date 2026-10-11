// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Sum,
};
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_natural_vector_unsigned_quadruple_gen_var_1,
    natural_vector_unsigned_pair_gen_var_3,
};

#[test]
fn test_mod_power_of_2_sum() {
    let test = |ss: &[&str], pow, out| {
        let vs: Vec<NaturalVector> = ss
            .iter()
            .map(|s| NaturalVector::from_str(s).unwrap())
            .collect();
        let sum = NaturalVector::mod_power_of_2_sum(vs.iter(), pow);
        assert_eq!(sum.to_string(), out);
        assert_eq!(NaturalVector::mod_power_of_2_sum(vs.into_iter(), pow), sum);
    };
    test(&["()"], 3, "()");
    test(&["(5, 1, 3)"], 3, "(5, 1, 3)");
    test(&["(5, 1, 3)", "(2, 7, 4)", "(6, 6, 6)"], 3, "(5, 6, 5)");
    test(
        &[
            "(18446744073709551615, 1)",
            "(18446744073709551615, 18446744073709551616)",
            "(3, 36893488147419103231)",
        ],
        65,
        "(1, 18446744073709551616)",
    );
    test(
        &["(1267650600228229401496703205375)", "(1267650600228229401496703205375)"],
        100,
        "(1267650600228229401496703205374)",
    );
    test(&["()", "()"], 5, "()");
}

#[test]
#[should_panic]
fn mod_power_of_2_sum_fail_1() {
    NaturalVector::mod_power_of_2_sum(core::iter::empty::<NaturalVector>(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sum_fail_2() {
    let v = NaturalVector::from_str("(1)").unwrap();
    let big = NaturalVector::from_str("(8)").unwrap();
    NaturalVector::mod_power_of_2_sum([&v, &big].into_iter(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sum_fail_3() {
    let v = NaturalVector::from_str("(1)").unwrap();
    let long = NaturalVector::from_str("(1, 1)").unwrap();
    NaturalVector::mod_power_of_2_sum([v, long].into_iter(), 3);
}

#[test]
fn mod_power_of_2_sum_properties() {
    natural_vector_natural_vector_natural_vector_unsigned_quadruple_gen_var_1().test_properties(
        |(u, v, w, pow)| {
            let sum = NaturalVector::mod_power_of_2_sum([&u, &v, &w].into_iter(), pow);
            assert!(sum.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                NaturalVector::mod_power_of_2_sum(
                    [u.clone(), v.clone(), w.clone()].into_iter(),
                    pow
                ),
                sum
            );
            // It is repeated `mod_power_of_2_add`, in any order, and entry by entry the scalar sum.
            assert_eq!(
                (&u).mod_power_of_2_add(&v, pow).mod_power_of_2_add(&w, pow),
                sum
            );
            assert_eq!(
                NaturalVector::mod_power_of_2_sum([&w, &u, &v].into_iter(), pow),
                sum
            );
            for (i, s) in sum.elements.iter().enumerate() {
                assert_eq!(
                    Natural::mod_power_of_2_sum(
                        [&u.elements[i], &v.elements[i], &w.elements[i]].into_iter(),
                        pow
                    ),
                    *s
                );
            }
        },
    );

    natural_vector_unsigned_pair_gen_var_3().test_properties(|(v, pow)| {
        // The sum of one vector is that vector.
        assert_eq!(NaturalVector::mod_power_of_2_sum([&v].into_iter(), pow), v);
    });

    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1::<Limb>()
        .test_properties(|(u, v, w, pow)| {
            // It agrees with the UnsignedVector implementation.
            let sum = UnsignedVector::mod_power_of_2_sum([&u, &v, &w].into_iter(), pow);
            assert_eq!(
                NaturalVector::mod_power_of_2_sum(
                    [NaturalVector::from(u), NaturalVector::from(v), NaturalVector::from(w)]
                        .into_iter(),
                    pow
                ),
                NaturalVector::from(sum)
            );
        });
}
