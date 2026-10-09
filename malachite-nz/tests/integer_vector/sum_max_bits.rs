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
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_nz::test_util::integer_vector::arithmetic::sum_max_bits::vec_sum_max_bits_naive;

#[test]
fn test_sum_max_bits() {
    let test = |s, out| {
        let v = IntegerVector::from_str(s).unwrap();
        assert_eq!(v.sum_max_bits(), out);
        assert_eq!(v.height_significant_bits(), out.1);
    };
    test("()", (0, 0));
    test("(0, 0)", (0, 0));
    test("(1, -5, 2)", (4, 3));
    // - every element fits in one limb, and the sum carries into the high limb
    test("(18446744073709551615, -18446744073709551615)", (65, 64));
    // - an element does not fit in one limb
    test("(18446744073709551616, -1)", (65, 65));
    test("(1, -36893488147419103232)", (66, 66));
}

#[test]
fn sum_max_bits_properties() {
    integer_vector_gen().test_properties(|v| {
        let (sum_bits, max_bits) = v.sum_max_bits();
        assert_eq!(v.height_significant_bits(), max_bits);
        assert_eq!(v.l1_norm_significant_bits(), sum_bits);
        // The largest element is at most the sum, and the sum is at most the dimension times the
        // largest element.
        assert!(max_bits <= sum_bits);
        assert!(sum_bits <= max_bits + v.dimension().significant_bits());
        assert_eq!(vec_sum_max_bits_naive(&v.elements), (sum_bits, max_bits));
        assert_eq!((-&v).sum_max_bits(), (sum_bits, max_bits));
        assert_eq!(
            (sum_bits, max_bits) == (0, 0),
            v.elements.iter().all(|x| *x == 0u32)
        );
    });
}
