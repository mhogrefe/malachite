// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModSum};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_5,
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2,
};
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_sum() {
    let test = |ss: &[&str], m: u8, out| {
        let vs: Vec<UnsignedVector<u8>> = ss
            .iter()
            .map(|s| UnsignedVector::from_str(s).unwrap())
            .collect();
        let sum = UnsignedVector::mod_sum(vs.iter(), m);
        assert_eq!(sum.to_string(), out);
        assert_eq!(UnsignedVector::mod_sum(vs.into_iter(), m), sum);
    };
    test(&["()"], 3, "()");
    test(&["(0, 0)"], 1, "(0, 0)");
    test(&["(5, 1, 3)"], 7, "(5, 1, 3)");
    test(&["(5, 1, 3)", "(2, 6, 4)"], 7, "(0, 0, 0)");
    test(&["(5, 1, 3)", "(2, 6, 4)", "(6, 2, 5)"], 7, "(6, 2, 5)");
    test(&["(254, 1)", "(254, 254)", "(2, 0)"], 255, "(0, 0)");
    test(&["()", "()", "()"], 5, "()");
}

fn mod_sum_fail_helper<T: PrimitiveUnsigned>() {
    let seven = T::exact_from(7u8);
    let v = UnsignedVector::<T> {
        elements: vec![T::ONE],
    };
    let big = UnsignedVector::<T> {
        elements: vec![seven],
    };
    let long = UnsignedVector::<T> {
        elements: vec![T::ONE, T::ONE],
    };
    // The iterator is empty.
    assert_panic!(UnsignedVector::<T>::mod_sum(
        core::iter::empty::<UnsignedVector<T>>(),
        seven
    ));
    // An element is not reduced.
    assert_panic!(UnsignedVector::mod_sum([&v, &big].into_iter(), seven));
    assert_panic!(UnsignedVector::mod_sum([&big].into_iter(), seven));
    // The modulus is zero.
    assert_panic!(UnsignedVector::mod_sum([&v].into_iter(), T::ZERO));
    // The dimensions differ.
    assert_panic!(UnsignedVector::mod_sum([&v, &long].into_iter(), seven));
}

#[test]
fn mod_sum_fail() {
    apply_fn_to_unsigneds!(mod_sum_fail_helper);
}

fn mod_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2::<T>()
        .test_properties(|(u, v, w, m)| {
            let sum = UnsignedVector::mod_sum([&u, &v, &w].into_iter(), m);
            assert!(sum.mod_is_reduced(&m));
            assert_eq!(
                UnsignedVector::mod_sum([u.clone(), v.clone(), w.clone()].into_iter(), m),
                sum
            );
            // It is repeated `mod_add`, in any order, and entry by entry the scalar sum.
            assert_eq!((&u).mod_add(&v, m).mod_add(&w, m), sum);
            assert_eq!(UnsignedVector::mod_sum([&w, &u, &v].into_iter(), m), sum);
            for (i, &s) in sum.elements.iter().enumerate() {
                assert_eq!(
                    T::mod_sum([u.elements[i], v.elements[i], w.elements[i]].into_iter(), m),
                    s
                );
            }
        });

    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        // The sum of one vector is that vector.
        assert_eq!(UnsignedVector::mod_sum([&v].into_iter(), m), v);
    });
}

#[test]
fn mod_sum_properties() {
    apply_fn_to_unsigneds!(mod_sum_properties_helper);
}
