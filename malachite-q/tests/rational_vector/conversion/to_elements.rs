// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_base::vector::Vector;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{rational_vec_gen, rational_vector_gen};

#[test]
fn test_to_elements() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        assert_eq!(v.to_elements().to_debug_string(), out);
        assert_eq!(v.elements_ref().to_debug_string(), out);
        assert_eq!(v.into_elements().to_debug_string(), out);
    };
    test("()", "[]");
    test("(0)", "[0]");
    test("(1, 2, 3)", "[1, 2, 3]");
    test("(5, 0, 0)", "[5, 0, 0]");
}

#[test]
fn to_elements_properties() {
    rational_vector_gen().test_properties(|v| {
        let xs = v.to_elements();
        assert_eq!(xs, v.elements);
        assert_eq!(v.elements_ref(), xs.as_slice());
        assert_eq!(u64::try_from(xs.len()).unwrap(), v.dimension());
        // The `Display` output is the elements' own, joined and parenthesized.
        assert_eq!(v.to_string(), format!("({})", xs.iter().join(", ")));
        assert_eq!(v.into_elements(), xs);
    });

    rational_vec_gen().test_properties(|xs| {
        assert_eq!(RationalVector::from_elements(&xs).to_elements(), xs);
    });
}
