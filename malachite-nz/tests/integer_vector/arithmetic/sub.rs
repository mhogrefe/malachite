// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_gen, integer_vector_pair_gen_var_1, integer_vector_triple_gen_var_1,
    natural_vector_pair_gen_var_1,
};

#[test]
fn test_sub() {
    let test = |s, t, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let w = IntegerVector::from_str(t).unwrap();
        assert_eq!((v.clone() - w.clone()).to_string(), out);
        assert_eq!((v.clone() - &w).to_string(), out);
        assert_eq!((&v - w.clone()).to_string(), out);
        assert_eq!((&v - &w).to_string(), out);
        let mut x = v.clone();
        x -= w.clone();
        assert_eq!(x.to_string(), out);
        let mut x = v;
        x -= &w;
        assert_eq!(x.to_string(), out);
    };
    test("()", "()", "()");
    test("(0)", "(0)", "(0)");
    test("(1, -2, 3)", "(-1, 5, 0)", "(2, -7, 3)");
    test(
        "(1000000000000000000000, 1)",
        "(1000000000000000000000, -2)",
        "(0, 3)",
    );
}

fn pair() -> (IntegerVector, IntegerVector) {
    (
        IntegerVector::from_str("(1, 2)").unwrap(),
        IntegerVector::from_str("(1)").unwrap(),
    )
}

#[test]
#[should_panic]
fn sub_fail_1() {
    let (v, w) = pair();
    let _ = v - w;
}

#[test]
#[should_panic]
fn sub_fail_2() {
    let (v, w) = pair();
    let _ = v - &w;
}

#[test]
#[should_panic]
fn sub_fail_3() {
    let (v, w) = pair();
    let _ = &v - w;
}

#[test]
#[should_panic]
fn sub_fail_4() {
    let (v, w) = pair();
    let _ = &v - &w;
}

#[test]
#[should_panic]
fn sub_assign_fail_1() {
    let (mut v, w) = pair();
    v -= w;
}

#[test]
#[should_panic]
fn sub_assign_fail_2() {
    let (mut v, w) = pair();
    v -= &w;
}

#[test]
#[should_panic]
fn sub_empty_fail() {
    // Even the 0-dimensional vector cannot be subtracted from a vector of another dimension.
    let _ = &IntegerVector::from_str("()").unwrap() - &IntegerVector::from_str("(0)").unwrap();
}

#[test]
fn sub_properties() {
    integer_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let diff = &v - &w;
        assert_eq!(v.clone() - w.clone(), diff);
        assert_eq!(v.clone() - &w, diff);
        assert_eq!(&v - w.clone(), diff);
        let mut x = v.clone();
        x -= w.clone();
        assert_eq!(x, diff);
        let mut x = v.clone();
        x -= &w;
        assert_eq!(x, diff);

        // The difference is taken element by element.
        assert_eq!(diff.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&diff.elements) {
            assert_eq!(x - y, *z);
        }
        // Subtracting is adding the negation, swapping the operands negates the difference, and
        // adding back the subtrahend recovers the minuend.
        assert_eq!(&v + -&w, diff);
        assert_eq!(&w - &v, -&diff);
        assert_eq!(&diff + &w, v);
    });

    integer_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        assert_eq!((&u - &v) - &w, &u - (&v + &w));
        assert_eq!(&u - (&v - &w), (&u - &v) + &w);
    });

    integer_vector_gen().test_properties(|v| {
        let zero = IntegerVector {
            elements: vec![Integer::default(); v.elements.len()],
        };
        assert_eq!(&v - &zero, v);
        assert_eq!(&zero - &v, -&v);
        assert!((&v - &v).elements.iter().all(|x| *x == 0u32));
    });

    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // Subtracting a `NaturalVector` from its sum with another gives the other.
        assert_eq!(
            IntegerVector::from(&v + &w) - IntegerVector::from(w),
            IntegerVector::from(v)
        );
    });
}
