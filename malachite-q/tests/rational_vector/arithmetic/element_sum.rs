// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Abs, ElementSum, L1Norm};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{rational_vector_gen, rational_vector_pair_gen_var_1};

#[test]
fn test_element_sum() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        let sum = v.to_element_sum();
        assert_eq!(sum.to_string(), out);
        assert_eq!(v.into_element_sum(), sum);
    };
    test("()", "0");
    test("(-5/2)", "-5/2");
    test("(1/2, -1/3)", "1/6");
    test("(1/2, 1/3, 1/6)", "1");
    test("(1/3, -1/3)", "0");
}

#[test]
fn element_sum_properties() {
    rational_vector_gen().test_properties(|v| {
        let sum = v.to_element_sum();
        assert_eq!(v.clone().into_element_sum(), sum);
        assert_eq!(
            v.elements.iter().fold(Rational::ZERO, |acc, x| acc + x),
            sum
        );
        // The absolute value of the sum is at most the $\ell^1$ norm, and negating the vector
        // negates the sum.
        assert!((&sum).abs() <= v.to_l1_norm());
        assert_eq!((-&v).into_element_sum(), -sum);
    });

    rational_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // The sum of the elements is additive.
        assert_eq!(
            (&v + &w).into_element_sum(),
            v.to_element_sum() + w.to_element_sum()
        );
    });

    integer_vector_gen().test_properties(|v| {
        // It agrees with the IntegerVector implementation.
        assert_eq!(
            RationalVector::from(v.clone()).into_element_sum(),
            Rational::from(v.into_element_sum())
        );
    });
}
