// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_pair_gen_var_53;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_zero() {
    let test = |height, width, out| {
        let m = UnsignedMatrix::<u64>::zero(height, width);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        assert_eq!((m.height(), m.width()), (height, width));
    };
    test(0, 0, "[]_{0,0}");
    test(0, 3, "[]_{0,3}");
    test(2, 0, "[]_{2,0}");
    test(1, 1, "[0]");
    test(2, 3, "[0, 0, 0; 0, 0, 0]");
}

#[test]
#[should_panic]
fn zero_fail() {
    // The number of elements does not fit in a `usize`.
    UnsignedMatrix::<u64>::zero(u64::MAX, 2);
}

#[test]
fn zero_properties() {
    unsigned_pair_gen_var_53().test_properties(|(height, width)| {
        let m = UnsignedMatrix::<u64>::zero(height, width);
        assert!(m.is_valid());
        assert_eq!((m.height(), m.width()), (height, width));
        assert!(m.is_zero());
        assert_eq!(m.transpose(), UnsignedMatrix::<u64>::zero(width, height));
        assert!((0..height).all(|i| m.row(i) == UnsignedVector::<u64>::zero(width)));
        assert!((0..width).all(|j| m.column(j) == UnsignedVector::<u64>::zero(height)));
    });
}
