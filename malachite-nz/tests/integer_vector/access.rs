// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_gen, integer_vector_unsigned_pair_gen_var_1,
};

#[test]
fn test_index() {
    let test = |s, i: usize, out: u32| {
        let v = IntegerVector::from_str(s).unwrap();
        assert_eq!(v[i], out);
    };
    test("(5)", 0, 5);
    test("(1, 2, 3)", 0, 1);
    test("(1, 2, 3)", 1, 2);
    test("(1, 2, 3)", 2, 3);
    test("(0, 0)", 1, 0);
}

#[test]
#[should_panic]
fn index_fail_1() {
    let v = IntegerVector::from_str("()").unwrap();
    let _ = &v[0];
}

#[test]
#[should_panic]
fn index_fail_2() {
    let v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    let _ = &v[3];
}

#[test]
fn test_index_mut() {
    let test = |s, i: usize, x: u32, out| {
        let mut v = IntegerVector::from_str(s).unwrap();
        v[i] = Integer::from(x);
        assert_eq!(v.to_string(), out);
    };
    test("(5)", 0, 7, "(7)");
    test("(1, 2, 3)", 0, 0, "(0, 2, 3)");
    test("(1, 2, 3)", 2, 100, "(1, 2, 100)");
}

#[test]
#[should_panic]
fn index_mut_fail_1() {
    let mut v = IntegerVector::from_str("()").unwrap();
    v[0] = Integer::ZERO;
}

#[test]
#[should_panic]
fn index_mut_fail_2() {
    let mut v = IntegerVector::from_str("(1, 2, 3)").unwrap();
    v[3] = Integer::ZERO;
}

#[test]
fn index_properties() {
    integer_vector_unsigned_pair_gen_var_1().test_properties(|(v, i)| {
        let x = &v[i];
        assert_eq!(x, &v.elements[i]);
        assert_eq!(x, &v.elements_ref()[i]);

        // Changing one element through `IndexMut` changes only that element, and not the dimension.
        let mut w = v.clone();
        w[i] += Integer::ONE;
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!(&w[i], &(x + Integer::ONE));
        for j in 0..v.elements.len() {
            if j != i {
                assert_eq!(w[j], v[j]);
            }
        }
        w[i] -= Integer::ONE;
        assert_eq!(w, v);
    });

    integer_vector_gen().test_properties(|v| {
        // Indexing every position in turn reads back the elements.
        let xs: Vec<Integer> = (0..v.elements.len()).map(|i| v[i].clone()).collect();
        assert_eq!(xs, v.elements);
    });
}
