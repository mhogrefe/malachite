// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_vector_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_significant_bits() {
    let test = |s, out| {
        assert_eq!(
            UnsignedVector::<u8>::from_str(s)
                .unwrap()
                .significant_bits(),
            out
        );
    };
    test("()", 0);
    test("(1, 5, 2)", 6);
}

#[test]
fn significant_bits_properties() {
    unsigned_vector_gen().test_properties(|v| {
        let bits = v.significant_bits();
        // It is the sum of the numbers of significant bits of the elements.
        assert_eq!(
            v.elements
                .iter()
                .map(|&x| x.significant_bits())
                .sum::<u64>(),
            bits
        );
        // It is at least the number of bits of the largest element.
        assert!(bits >= v.max_bits().0);
        // The zero vector of the same dimension has no significant bits.
        let zero = UnsignedVector::<u64>::zero(v.dimension());
        assert_eq!(zero.significant_bits(), 0);
    });
}
