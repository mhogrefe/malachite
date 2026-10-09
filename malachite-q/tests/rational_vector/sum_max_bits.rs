// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::max;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, L1Norm};
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_sum_max_bits() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        assert_eq!(v.sum_max_bits(), out);
        assert_eq!(v.height_significant_bits(), out.1);
    };
    test("()", (1, 0));
    test("(0, 0)", (1, 1));
    test("(1/2, -1/3)", (6, 2));
    test("(1/2, 1/2)", (2, 2));
}

#[test]
fn sum_max_bits_properties() {
    rational_vector_gen().test_properties(|v| {
        let (sum_bits, max_bits) = v.sum_max_bits();
        assert_eq!(v.height_significant_bits(), max_bits);
        assert_eq!(v.l1_norm_significant_bits(), sum_bits);
        // Negating the vector changes neither count. The norm has at least 1 bit, since 0 is 0/1,
        // and the height is 0 exactly for the 0-dimensional vector, since every rational number has
        // height at least 1.
        assert_eq!((-&v).sum_max_bits(), (sum_bits, max_bits));
        assert_ne!(sum_bits, 0);
        assert_eq!(max_bits == 0, v.dimension() == 0);
    });

    integer_vector_gen().test_properties(|v| {
        // As a rational number, the norm of an integer vector gains a 1-bit denominator, and as
        // rationals, the elements keep their bit counts, except that zeros become 0/1, which has 1
        // bit.
        let (sum_bits, max_bits) = v.sum_max_bits();
        let expected = if v.dimension() == 0 {
            (1, 0)
        } else {
            (sum_bits + 1, max(max_bits, 1))
        };
        assert_eq!(RationalVector::from(v).sum_max_bits(), expected);
    });
}
