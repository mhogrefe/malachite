// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_zero() {
    let test = |dimension, out| {
        let v = NaturalVector::zero(dimension);
        assert_eq!(v.to_string(), out);
        assert_eq!(v.dimension(), dimension);
    };
    test(0, "()");
    test(1, "(0)");
    test(3, "(0, 0, 0)");
}

#[test]
fn zero_properties() {
    unsigned_gen_var_5::<u64>().test_properties(|dimension| {
        let v = NaturalVector::zero(dimension);
        assert_eq!(v.dimension(), dimension);
        assert!(v.elements.iter().all(|x| *x == 0u32));
        assert_eq!(v.pivot(), None);
        assert_eq!(v.pivot_index(), None);
        assert_eq!(
            v,
            NaturalVector::from_owned_elements(vec![Natural::ZERO; usize::exact_from(dimension)])
        );
        assert_eq!(
            NaturalVector::from(UnsignedVector::<u64>::zero(dimension)),
            v
        );
    });

    natural_vector_gen().test_properties(|v| {
        let zero = NaturalVector::zero(v.dimension());
        // The zero vector of the same dimension is the additive identity, and is any vector reduced
        // modulo 1.
        assert_eq!(&v + &zero, v);
        assert_eq!(&zero + &v, v);
        assert_eq!((&v).mod_power_of_2(0), zero);
    });
}
