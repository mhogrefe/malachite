// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Abs, L1Norm};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_l1_norm() {
    let test = |s, out, bits| {
        let v = RationalVector::from_str(s).unwrap();
        let norm = v.to_l1_norm();
        assert_eq!(norm.to_string(), out);
        assert_eq!(v.l1_norm_significant_bits(), bits);
        assert_eq!(v.into_l1_norm(), norm);
    };
    test("()", "0", 0);
    test("(1/2, -1/3)", "5/6", 6);
}

#[test]
fn l1_norm_properties() {
    rational_vector_gen().test_properties(|v| {
        let norm = v.to_l1_norm();
        assert_eq!(v.clone().into_l1_norm(), norm);
        let bits = v.l1_norm_significant_bits();
        assert_eq!(norm.significant_bits(), bits);
        assert_eq!(v.elements.iter().map(Abs::abs).sum::<Rational>(), norm);
        // Negating the vector does not change its norm, and the norm is 0 only when every element
        // is.
        assert_eq!((-&v).to_l1_norm(), norm);
        assert_eq!(norm == 0u32, v.elements.iter().all(|x| *x == 0u32));
    });

    integer_vector_gen().test_properties(|v| {
        // The norm of an integer vector is the same as a rational vector.
        assert_eq!(
            RationalVector::from(v.clone()).to_l1_norm(),
            Rational::from(v.to_l1_norm())
        );
    });
}
