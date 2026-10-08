// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_zero() {
    let test = |dimension, out| {
        let v = RationalVector::zero(dimension);
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
        let v = RationalVector::zero(dimension);
        assert_eq!(v.dimension(), dimension);
        assert!(v.elements.iter().all(|x| *x == 0u32));
        assert_eq!(v.pivot(), None);
        assert_eq!(v.pivot_index(), None);
        assert_eq!(
            v,
            RationalVector::from_owned_elements(vec![Rational::ZERO; usize::exact_from(dimension)])
        );
        assert_eq!(-&v, v);
        assert_eq!(RationalVector::from(IntegerVector::zero(dimension)), v);
        assert_eq!(
            v.to_numerators_and_denominator(),
            (IntegerVector::zero(dimension), Natural::ONE)
        );
    });

    rational_vector_gen().test_properties(|v| {
        // The zero vector of the same dimension is the additive identity.
        let zero = RationalVector::zero(v.dimension());
        assert_eq!(&v + &zero, v);
        assert_eq!(&zero + &v, v);
        assert_eq!(&v - &zero, v);
        assert_eq!(&zero - &v, -&v);
    });
}
