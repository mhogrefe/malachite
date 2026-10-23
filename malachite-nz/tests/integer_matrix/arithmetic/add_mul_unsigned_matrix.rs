// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::add_mul_unsigned_matrix::*;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_1;
use std::panic::catch_unwind;

#[test]
fn test_add_mul_unsigned_matrix_assign() {
    let test = |b, a, c, out| {
        let mut b = IntegerMatrix::from_str(b).unwrap();
        let a = UnsignedMatrix::<u64>::from_str(a).unwrap();
        let c = Integer::from_str(c).unwrap();
        add_mul_unsigned_matrix_assign(&mut b, &a, &c);
        assert_eq!(b.to_string(), out);
    };
    test("[]_{0,2}", "[]_{0,2}", "5", "[]_{0,2}");
    test("[1, 2; 3, 4]", "[0, 1; 2, 3]", "0", "[1, 2; 3, 4]");
    test("[1, 2; 3, 4]", "[0, 1; 2, 3]", "-2", "[1, 0; -1, -2]");
    test(
        "[0]",
        "[18446744073709551615]",
        "18446744073709551616",
        "[340282366920938463444927863358058659840]",
    );
}

#[test]
fn add_mul_unsigned_matrix_assign_fail() {
    assert_panic!({
        let mut b = IntegerMatrix::from_str("[1, 2]").unwrap();
        let a = UnsignedMatrix::<u64>::from_str("[1; 2]").unwrap();
        add_mul_unsigned_matrix_assign(&mut b, &a, &Integer::ONE);
    });
}

#[test]
fn add_mul_unsigned_matrix_assign_properties() {
    integer_matrix_pair_gen_var_1().test_properties(|(b, x)| {
        // A matrix of words from the absolute values of the low bits of x, and c from its first
        // element.
        let a = UnsignedMatrix::<u64>::from_owned_elements(
            x.height(),
            x.width(),
            x.elements
                .iter()
                .map(|y| u64::wrapping_from(y.unsigned_abs_ref()))
                .collect(),
        );
        let c = x.elements.first().cloned().unwrap_or_default();
        let mut result = b.clone();
        add_mul_unsigned_matrix_assign(&mut result, &a, &c);
        for ((r, y), z) in result.elements.iter().zip(&b.elements).zip(&a.elements) {
            assert_eq!(*r, y + &c * Integer::from(*z));
        }
    });
}
