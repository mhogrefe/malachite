// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModIsReduced, ModPowerOf2, ModPowerOf2IsReduced};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_zero() {
    let test = |dimension, out| {
        let v = UnsignedVector::<u8>::zero(dimension);
        assert_eq!(v.to_string(), out);
        assert_eq!(v.dimension(), dimension);
    };
    test(0, "()");
    test(1, "(0)");
    test(3, "(0, 0, 0)");
}

fn zero_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_gen_var_5::<u64>().test_properties(|dimension| {
        let v = UnsignedVector::<T>::zero(dimension);
        assert_eq!(v.dimension(), dimension);
        assert!(v.elements.iter().all(|&x| x == T::ZERO));
        assert_eq!(v.pivot(), None);
        assert_eq!(v.pivot_index(), None);
        assert_eq!(
            v,
            UnsignedVector::from_owned_elements(vec![T::ZERO; usize::exact_from(dimension)])
        );
        // The zero vector is reduced modulo every positive number and every power of 2.
        assert!(v.mod_is_reduced(&T::ONE));
        assert!(v.mod_power_of_2_is_reduced(0));
    });
}

#[test]
fn zero_properties() {
    apply_fn_to_unsigneds!(zero_properties_helper);

    unsigned_vector_gen().test_properties(|v| {
        // Any vector reduced modulo 1 is the zero vector of the same dimension.
        assert_eq!((&v).mod_power_of_2(0), UnsignedVector::zero(v.dimension()));
    });
}
