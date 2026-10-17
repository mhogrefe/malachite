// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;

fn ints(xs: &[u64]) -> Vec<u64> {
    xs.to_vec()
}

#[test]
fn test_from_elements() {
    let test = |height, width, xs: &[u64], out| {
        let xs = ints(xs);
        let m = UnsignedMatrix::from_elements(height, width, &xs);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        assert_eq!(UnsignedMatrix::from_owned_elements(height, width, xs), m);
    };
    test(0, 0, &[], "[]_{0,0}");
    test(0, 4, &[], "[]_{0,4}");
    test(4, 0, &[], "[]_{4,0}");
    test(1, 1, &[5], "[5]");
    test(2, 3, &[1, 2, 3, 4, 5, 6], "[1, 2, 3; 4, 5, 6]");
    test(3, 2, &[1, 20, 3, 40, 5, 60], "[1, 20; 3, 40; 5, 60]");
}

#[test]
#[should_panic]
fn from_elements_fail_1() {
    UnsignedMatrix::from_elements(2, 2, &ints(&[1, 2, 3]));
}

#[test]
#[should_panic]
fn from_elements_fail_2() {
    UnsignedMatrix::from_elements(0, 2, &ints(&[1]));
}

#[test]
#[should_panic]
fn from_elements_fail_3() {
    UnsignedMatrix::<u64>::from_elements(u64::MAX, 2, &[]);
}

#[test]
#[should_panic]
fn from_owned_elements_fail() {
    UnsignedMatrix::from_owned_elements(2, 2, ints(&[1, 2, 3, 4, 5]));
}

#[test]
fn from_elements_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let m_alt = UnsignedMatrix::from_elements(m.height(), m.width(), &m.elements);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
        let m_alt = UnsignedMatrix::from_owned_elements(m.height(), m.width(), m.elements.clone());
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
    });
}
