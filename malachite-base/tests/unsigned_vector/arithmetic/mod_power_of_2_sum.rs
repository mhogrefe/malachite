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
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_4,
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1,
};
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_sum() {
    let test = |ss: &[&str], pow, out| {
        let vs: Vec<UnsignedVector<u8>> = ss
            .iter()
            .map(|s| UnsignedVector::from_str(s).unwrap())
            .collect();
        let sum = UnsignedVector::mod_power_of_2_sum(vs.iter(), pow);
        assert_eq!(sum.to_string(), out);
        assert_eq!(UnsignedVector::mod_power_of_2_sum(vs.into_iter(), pow), sum);
    };
    test(&["()"], 3, "()");
    test(&["(0, 0)"], 0, "(0, 0)");
    test(&["(5, 1, 3)"], 3, "(5, 1, 3)");
    test(&["(5, 1, 3)", "(2, 7, 4)"], 3, "(7, 0, 7)");
    test(&["(5, 1, 3)", "(2, 7, 4)", "(6, 6, 6)"], 3, "(5, 6, 5)");
    test(&["(255, 1)", "(255, 255)", "(2, 0)"], 8, "(0, 0)");
    test(&["()", "()", "()"], 5, "()");
}

fn mod_power_of_2_sum_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T> {
        elements: vec![T::ONE],
    };
    let big = UnsignedVector::<T> {
        elements: vec![T::exact_from(8u8)],
    };
    let long = UnsignedVector::<T> {
        elements: vec![T::ONE, T::ONE],
    };
    // The iterator is empty.
    assert_panic!(UnsignedVector::<T>::mod_power_of_2_sum(
        core::iter::empty::<UnsignedVector<T>>(),
        3
    ));
    // An element is not reduced.
    assert_panic!(UnsignedVector::mod_power_of_2_sum(
        [&v, &big].into_iter(),
        3
    ));
    assert_panic!(UnsignedVector::mod_power_of_2_sum([&big].into_iter(), 3));
    // The dimensions differ.
    assert_panic!(UnsignedVector::mod_power_of_2_sum(
        [&v, &long].into_iter(),
        3
    ));
    // The power is too large.
    assert_panic!(UnsignedVector::mod_power_of_2_sum(
        [&v].into_iter(),
        T::WIDTH + 1
    ));
}

#[test]
fn mod_power_of_2_sum_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sum_fail_helper);
}

fn mod_power_of_2_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1::<T>()
        .test_properties(|(u, v, w, pow)| {
            let sum = UnsignedVector::mod_power_of_2_sum([&u, &v, &w].into_iter(), pow);
            assert!(sum.mod_power_of_2_is_reduced(pow));
            assert_eq!(
                UnsignedVector::mod_power_of_2_sum(
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
                UnsignedVector::mod_power_of_2_sum([&w, &u, &v].into_iter(), pow),
                sum
            );
            for (i, &s) in sum.elements.iter().enumerate() {
                assert_eq!(
                    T::mod_power_of_2_sum(
                        [u.elements[i], v.elements[i], w.elements[i]].into_iter(),
                        pow
                    ),
                    s
                );
            }
        });

    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        // The sum of one vector is that vector.
        assert_eq!(UnsignedVector::mod_power_of_2_sum([&v].into_iter(), pow), v);
    });
}

#[test]
fn mod_power_of_2_sum_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sum_properties_helper);
}
