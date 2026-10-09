// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::min;
use core::str::FromStr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::vector::{SelectCoordinates, SelectCoordinatesAssign, Vector};
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_unsigned_vec_pair_gen_var_1;

#[test]
fn test_select_coordinates() {
    let test = |s, indices: &[u64], out| {
        let v = RationalVector::from_str(s).unwrap();
        let x = (&v).select_coordinates(indices);
        assert_eq!(x.to_string(), out);
        assert_eq!(v.clone().select_coordinates(indices), x);
        let mut y = v;
        y.select_coordinates_assign(indices);
        assert_eq!(y, x);
    };
    test("(1/2, -2, 3)", &[2, 0], "(3, 1/2)");
    test("(1/2, -2, 3)", &[1, 1, 1], "(-2, -2, -2)");
    test("(1/2, -2, 3)", &[0, 2], "(1/2, 3)");
    test("(1/2, -2, 3)", &[0, 1, 2], "(1/2, -2, 3)");
    test("(1/2, -2, 3)", &[0, 1, 2, 0, 1], "(1/2, -2, 3, 1/2, -2)");
    test("(1/2, -2, 3)", &[], "()");
    test("()", &[], "()");
}

#[test]
#[should_panic]
fn select_coordinates_fail_1() {
    // An increasing selection with an index that is too large.
    let _ = (&RationalVector::from_str("(1/2, -2, 3)").unwrap()).select_coordinates(&[0, 3]);
}

#[test]
#[should_panic]
fn select_coordinates_fail_2() {
    // A non-increasing selection with an index that is too large.
    let _ = (&RationalVector::from_str("(1/2, -2, 3)").unwrap()).select_coordinates(&[3, 0]);
}

#[test]
#[should_panic]
fn select_coordinates_fail_3() {
    let _ = RationalVector::from_str("(1/2, -2, 3)")
        .unwrap()
        .select_coordinates(&[0, 3]);
}

#[test]
#[should_panic]
fn select_coordinates_fail_4() {
    let _ = RationalVector::from_str("(1/2, -2, 3)")
        .unwrap()
        .select_coordinates(&[3, 0]);
}

#[test]
#[should_panic]
fn select_coordinates_fail_5() {
    // The 0-dimensional vector has no coordinates to select.
    let _ = (&RationalVector::from_str("()").unwrap()).select_coordinates(&[0]);
}

#[test]
#[should_panic]
fn select_coordinates_assign_fail_1() {
    let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    v.select_coordinates_assign(&[0, 3]);
}

#[test]
#[should_panic]
fn select_coordinates_assign_fail_2() {
    let mut v = RationalVector::from_str("(1/2, -2, 3)").unwrap();
    v.select_coordinates_assign(&[3, 0]);
}

#[test]
fn select_coordinates_properties() {
    rational_vector_unsigned_vec_pair_gen_var_1().test_properties(|(v, indices)| {
        let x = (&v).select_coordinates(&indices);
        assert_eq!(v.clone().select_coordinates(&indices), x);
        let mut y = v.clone();
        y.select_coordinates_assign(&indices);
        assert_eq!(y, x);

        // Element j of the result is element indices[j] of the original.
        assert_eq!(x.dimension(), u64::exact_from(indices.len()));
        for (j, &i) in indices.iter().enumerate() {
            assert_eq!(x.elements[j], v.elements[usize::exact_from(i)]);
        }
        // Selecting from a selection composes the selections.
        let k = u64::exact_from(indices.len());
        let reversed: Vec<u64> = (0..k).rev().collect();
        let mut reversed_indices = indices.clone();
        reversed_indices.reverse();
        assert_eq!(
            (&x).select_coordinates(&reversed),
            (&v).select_coordinates(&reversed_indices)
        );
        // Selecting coordinates commutes with negation.
        assert_eq!((-&v).select_coordinates(&indices), -&x);

        // Selecting the first coordinates in order is reducing the dimension, selecting all of them
        // is the identity, and selecting none gives the 0-dimensional vector.
        let k = min(v.dimension(), k);
        let prefix: Vec<u64> = (0..k).collect();
        let mut t = v.clone();
        t.set_dimension(k);
        assert_eq!((&v).select_coordinates(&prefix), t);
        let all: Vec<u64> = (0..v.dimension()).collect();
        assert_eq!((&v).select_coordinates(&all), v);
        assert_eq!((&v).select_coordinates(&[]), RationalVector::zero(0));
    });
}
