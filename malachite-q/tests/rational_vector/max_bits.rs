// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::max;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Height;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_max_bits() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        assert_eq!(v.max_bits(), out);
        assert_eq!(v.height_significant_bits(), out.0);
    };
    test("()", (0, false));
    test("(0, 0)", (1, false));
    test("(1/4, -2)", (3, true));
    test("(1/18446744073709551616)", (65, false));
}

#[test]
fn max_bits_properties() {
    rational_vector_gen().test_properties(|v| {
        let (bits, negative) = v.max_bits();
        // The count is the number of significant bits of the height, and the flag says whether any
        // element is negative.
        assert_eq!(v.height_significant_bits(), bits);
        assert_eq!(v.to_height().significant_bits(), bits);
        assert_eq!(negative, v.elements.iter().any(|x| *x < 0u32));
        // Every element has height at least 1, so the count is 0 exactly for the 0-dimensional
        // vector.
        assert_eq!(bits == 0, v.dimension() == 0);
    });

    integer_vector_gen().test_properties(|v| {
        // As rationals, the elements of an integer vector keep their bit counts, except that each
        // zero becomes 0/1, which has 1 bit.
        let (bits, negative) = v.max_bits();
        let expected = if v.dimension() == 0 { 0 } else { max(bits, 1) };
        assert_eq!(RationalVector::from(v).max_bits(), (expected, negative));
    });
}
