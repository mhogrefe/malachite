// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ElementSum, Height, L1Norm};
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{natural_vector_gen, natural_vector_pair_gen_var_1};

#[test]
fn test_element_sum() {
    let test = |s, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let sum = v.to_element_sum();
        assert_eq!(sum.to_string(), out);
        assert_eq!(v.into_element_sum(), sum);
    };
    test("()", "0");
    test("(5)", "5");
    test("(1, 3, 2)", "6");
    test("(18446744073709551615, 1)", "18446744073709551616");
}

#[test]
fn element_sum_properties() {
    natural_vector_gen().test_properties(|v| {
        let sum = v.to_element_sum();
        assert_eq!(v.clone().into_element_sum(), sum);
        assert_eq!(v.elements.iter().fold(Natural::ZERO, |acc, x| acc + x), sum);
        // For nonnegative elements the sum is the $\ell^1$ norm, so it is at least the height and
        // at most the dimension times the height.
        assert_eq!(v.to_l1_norm(), sum);
        let height = v.to_height();
        assert!(height <= sum);
        assert!(sum <= &height * Natural::from(v.dimension()));
        // It agrees with the IntegerVector implementation.
        assert_eq!(
            IntegerVector::from(v).into_element_sum(),
            Integer::from(&sum)
        );
    });

    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // The sum of the elements is additive.
        assert_eq!(
            (&v + &w).into_element_sum(),
            v.to_element_sum() + w.to_element_sum()
        );
    });
}
