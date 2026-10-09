// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, L1Norm};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_sum_max_bits() {
    let test = |s, out| {
        let v = NaturalVector::from_str(s).unwrap();
        assert_eq!(v.sum_max_bits(), out);
        assert_eq!(v.height_significant_bits(), out.1);
    };
    test("()", (0, 0));
    test("(1, 5, 2)", (4, 3));
    test("(18446744073709551615, 18446744073709551615)", (65, 64));
    test("(18446744073709551616, 1)", (65, 65));
}

#[test]
fn sum_max_bits_properties() {
    natural_vector_gen().test_properties(|v| {
        let (sum_bits, max_bits) = v.sum_max_bits();
        assert_eq!(v.height_significant_bits(), max_bits);
        assert_eq!(v.l1_norm_significant_bits(), sum_bits);
        // The largest element is at most the sum, and the sum is at most the dimension times the
        // largest element.
        assert!(max_bits <= sum_bits);
        assert!(sum_bits <= max_bits + v.dimension().significant_bits());
        // The `Natural` and `Integer` vectors agree.
        assert_eq!(
            IntegerVector::from(v.clone()).sum_max_bits(),
            (sum_bits, max_bits)
        );
    });
}
