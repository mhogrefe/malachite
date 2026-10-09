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
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_significant_bits() {
    let test = |s, out| {
        assert_eq!(NaturalVector::from_str(s).unwrap().significant_bits(), out);
    };
    test("()", 0);
    test("(1, 5, 2)", 6);
}

#[test]
fn significant_bits_properties() {
    natural_vector_gen().test_properties(|v| {
        let bits = v.significant_bits();
        // It is the sum of the numbers of significant bits of the elements.
        assert_eq!(
            v.elements
                .iter()
                .map(SignificantBits::significant_bits)
                .sum::<u64>(),
            bits
        );
        // It is at least the number of bits of the largest element.
        assert!(bits >= v.max_bits().0);
        // The zero vector of the same dimension has no significant bits.
        let zero = NaturalVector::zero(v.dimension());
        assert_eq!(zero.significant_bits(), 0);
    });
}
