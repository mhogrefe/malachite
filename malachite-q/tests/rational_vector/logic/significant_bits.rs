// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::vector::Vector;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_significant_bits() {
    let test = |s, out| {
        assert_eq!(RationalVector::from_str(s).unwrap().significant_bits(), out);
    };
    test("()", 0);
    test("(1/2, -1/3)", 6);
}

#[test]
fn significant_bits_properties() {
    rational_vector_gen().test_properties(|v| {
        let bits = v.significant_bits();
        // It is the sum of the numbers of significant bits of the elements.
        assert_eq!(
            v.elements
                .iter()
                .map(SignificantBits::significant_bits)
                .sum::<u64>(),
            bits
        );
        // The zero vector of the same dimension has one bit per element, for the denominators.
        let zero = RationalVector::zero(v.dimension());
        assert_eq!(zero.significant_bits(), v.dimension());
    });
}
