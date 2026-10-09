// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Height;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_sum_max_bits() {
    let test = |s, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        assert_eq!(v.sum_max_bits(), out);
        assert_eq!(v.height_significant_bits(), out.1);
    };
    test("()", (0, 0));
    test("(1, 5, 2)", (4, 3));
    test("(255, 255)", (9, 8));
    test("(128, 128, 128, 128)", (10, 8));
    test("(255, 1)", (9, 8));
}

#[test]
fn sum_max_bits_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let (sum_bits, max_bits) = v.sum_max_bits();
        assert_eq!(v.height_significant_bits(), max_bits);
        // The largest element is at most the sum, and the sum is at most the dimension times the
        // largest element.
        assert!(max_bits <= sum_bits);
        assert!(sum_bits <= max_bits + v.dimension().significant_bits());
        // The sum is computed exactly, without overflowing.
        let sum: u128 = v.elements.iter().map(|&x| u128::from(x)).sum();
        assert_eq!(sum_bits, sum.significant_bits());
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        // For a narrower type, the sum is still computed exactly.
        let sum: u64 = xs.iter().map(|&x| u64::from(x)).sum();
        let v = UnsignedVector::from_owned_elements(xs);
        assert_eq!(v.sum_max_bits().0, sum.significant_bits());
    });
}
