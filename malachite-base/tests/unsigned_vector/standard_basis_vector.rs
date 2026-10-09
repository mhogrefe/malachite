// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2IsReduced};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_pair_gen_var_51;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_standard_basis_vector() {
    let test = |dimension, index, out| {
        let v = UnsignedVector::<u8>::standard_basis_vector(dimension, index);
        assert_eq!(v.to_string(), out);
        assert_eq!(v.dimension(), dimension);
    };
    test(1, 0, "(1)");
    test(3, 0, "(1, 0, 0)");
    test(3, 1, "(0, 1, 0)");
    test(3, 2, "(0, 0, 1)");
}

#[test]
#[should_panic]
fn standard_basis_vector_fail_1() {
    // The index equals the dimension.
    UnsignedVector::<u8>::standard_basis_vector(3, 3);
}

#[test]
#[should_panic]
fn standard_basis_vector_fail_2() {
    // The index exceeds the dimension.
    UnsignedVector::<u8>::standard_basis_vector(3, 5);
}

#[test]
#[should_panic]
fn standard_basis_vector_fail_3() {
    // The 0-dimensional vector has no elements.
    UnsignedVector::<u8>::standard_basis_vector(0, 0);
}

fn standard_basis_vector_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_51().test_properties(|(index, dimension)| {
        let v = UnsignedVector::<T>::standard_basis_vector(dimension, index);
        assert_eq!(v.dimension(), dimension);
        let i = usize::exact_from(index);
        assert_eq!(v.pivot_index(), Some(index));
        assert_eq!(v.pivot(), Some(T::ONE));
        for (j, &x) in v.elements.iter().enumerate() {
            assert_eq!(x == T::ONE, j == i);
            assert_eq!(x == T::ZERO, j != i);
        }
        // It is reduced modulo every number greater than 1 and every positive power of 2.
        assert!(v.mod_is_reduced(&T::TWO));
        assert!(!v.mod_is_reduced(&T::ONE));
        assert!(v.mod_power_of_2_is_reduced(1));
        assert!(!v.mod_power_of_2_is_reduced(0));
    });
}

#[test]
fn standard_basis_vector_properties() {
    apply_fn_to_unsigneds!(standard_basis_vector_properties_helper);
}
