// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, HeightRef, UnsignedAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_height() {
    let test = |s, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let height = v.to_height();
        assert_eq!(height.to_string(), out);
        assert_eq!(*v.height_ref(), height);
        assert_eq!(v.height_significant_bits(), height.significant_bits());
        assert_eq!(v.into_height(), height);
    };
    test("()", "0");
    test("(1, -3, 2)", "3");
    test("(-5, 5)", "5");
}

#[test]
fn height_properties() {
    integer_vector_gen().test_properties(|v| {
        let height = v.to_height();
        assert_eq!(*v.height_ref(), height);
        assert_eq!(v.clone().into_height(), height);
        assert_eq!(v.height_significant_bits(), height.significant_bits());
        // The height is the largest height of any element, and is the height of the element at
        // `height_index`.
        assert_eq!(
            v.elements
                .iter()
                .map(UnsignedAbs::unsigned_abs)
                .max()
                .unwrap_or_default(),
            height
        );
        match v.height_index() {
            None => assert_eq!(height, 0u32),
            Some(i) => assert_eq!(*v.elements[usize::exact_from(i)].unsigned_abs_ref(), height),
        }
        // Negating a vector does not change its height.
        assert_eq!((-&v).to_height(), height);
    });
}
