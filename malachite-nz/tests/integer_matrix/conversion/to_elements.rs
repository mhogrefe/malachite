// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_to_elements() {
    let test = |s, out| {
        let m = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(m.to_elements().to_debug_string(), out);
        assert_eq!(m.elements_ref().to_debug_string(), out);
        assert_eq!(m.into_elements().to_debug_string(), out);
    };
    test("[]_{0,0}", "[]");
    test("[]_{3,0}", "[]");
    test("[5]", "[5]");
    test("[1, 2, 3]", "[1, 2, 3]");
    test("[1, 2; 3, 4]", "[1, 2, 3, 4]");
    test("[1, -2; 3, 4]", "[1, -2, 3, 4]");
}

#[test]
fn to_elements_properties() {
    integer_matrix_gen().test_properties(|m| {
        let xs = m.to_elements();
        assert_eq!(xs, m.elements_ref());
        assert_eq!(m.elements_ref(), xs.as_slice());
        assert_eq!(u64::exact_from(xs.len()), m.height() * m.width());
        assert_eq!(m.clone().into_elements(), xs);
        assert_eq!(
            IntegerMatrix::from_owned_elements(m.height(), m.width(), xs),
            m
        );
    });
}
