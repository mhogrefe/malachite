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
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_max_bits() {
    let test = |s, out| {
        let v = NaturalVector::from_str(s).unwrap();
        assert_eq!(v.max_bits(), out);
        assert_eq!(v.height_significant_bits(), out.0);
    };
    test("()", (0, false));
    test("(0, 0)", (0, false));
    test("(1, 5, 2)", (3, false));
    test("(18446744073709551616, 1)", (65, false));
}

#[test]
fn max_bits_properties() {
    natural_vector_gen().test_properties(|v| {
        let (bits, negative) = v.max_bits();
        // The count is the number of significant bits of the height, and the flag says whether any
        // element is negative.
        assert_eq!(v.height_significant_bits(), bits);
        assert_eq!(v.to_height().significant_bits(), bits);
        assert!(!negative);
        // The count is 0 exactly when every element is 0.
        assert_eq!(bits == 0, v.elements.iter().all(|x| *x == 0u32));
        // The `Natural` and `Integer` vectors agree.
        assert_eq!(IntegerVector::from(v.clone()).max_bits(), (bits, false));
    });
}
