// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::max;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, HeightRef};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::vector::Vector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_height() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        let height = v.to_height();
        assert_eq!(height.to_string(), out);
        assert_eq!(*v.height_ref(), height);
        assert_eq!(v.height_significant_bits(), height.significant_bits());
        assert_eq!(v.into_height(), height);
    };
    test("()", "0");
    test("(1/2, -3, 1/4)", "4");
    test("(3, -1/3)", "3");
}

#[test]
fn height_properties() {
    rational_vector_gen().test_properties(|v| {
        let height = v.to_height();
        assert_eq!(*v.height_ref(), height);
        assert_eq!(v.clone().into_height(), height);
        assert_eq!(v.height_significant_bits(), height.significant_bits());
        // The height is the largest height of any element, and is the height of the element at
        // `height_index`.
        assert_eq!(
            v.elements
                .iter()
                .map(Height::to_height)
                .max()
                .unwrap_or_default(),
            height
        );
        match v.height_index() {
            None => assert_eq!(height, 0u32),
            Some(i) => assert_eq!(v.elements[usize::exact_from(i)].to_height(), height),
        }
        // Negating a vector does not change its height.
        assert_eq!((-&v).to_height(), height);
    });

    integer_vector_gen().test_properties(|v| {
        // As rationals, the elements of an integer vector keep their heights, except that each zero
        // becomes 0/1, of height 1.
        let height = RationalVector::from(v.clone()).to_height();
        if v.dimension() == 0 {
            assert_eq!(height, 0u32);
        } else {
            assert_eq!(height, max(v.to_height(), Natural::ONE));
        }
    });
}
