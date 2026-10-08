// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_gen, natural_vector_pair_gen_var_1, natural_vector_triple_gen_var_1,
};

#[test]
fn test_add() {
    let test = |s, t, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
        assert_eq!((v.clone() + w.clone()).to_string(), out);
        assert_eq!((v.clone() + &w).to_string(), out);
        assert_eq!((&v + w.clone()).to_string(), out);
        assert_eq!((&v + &w).to_string(), out);
        let mut x = v.clone();
        x += w.clone();
        assert_eq!(x.to_string(), out);
        let mut x = v;
        x += &w;
        assert_eq!(x.to_string(), out);
    };
    test("()", "()", "()");
    test("(0)", "(0)", "(0)");
    test("(1, 2, 3)", "(10, 0, 5)", "(11, 2, 8)");
    test(
        "(1000000000000000000000, 1)",
        "(1000000000000000000000, 2)",
        "(2000000000000000000000, 3)",
    );
}

fn pair() -> (NaturalVector, NaturalVector) {
    (
        NaturalVector::from_str("(1, 2)").unwrap(),
        NaturalVector::from_str("(1)").unwrap(),
    )
}

#[test]
#[should_panic]
fn add_fail_1() {
    let (v, w) = pair();
    let _ = v + w;
}

#[test]
#[should_panic]
fn add_fail_2() {
    let (v, w) = pair();
    let _ = v + &w;
}

#[test]
#[should_panic]
fn add_fail_3() {
    let (v, w) = pair();
    let _ = &v + w;
}

#[test]
#[should_panic]
fn add_fail_4() {
    let (v, w) = pair();
    let _ = &v + &w;
}

#[test]
#[should_panic]
fn add_assign_fail_1() {
    let (mut v, w) = pair();
    v += w;
}

#[test]
#[should_panic]
fn add_assign_fail_2() {
    let (mut v, w) = pair();
    v += &w;
}

#[test]
#[should_panic]
fn add_empty_fail() {
    // Even the 0-dimensional vector cannot be added to a vector of another dimension.
    let _ = &NaturalVector::from_str("()").unwrap() + &NaturalVector::from_str("(0)").unwrap();
}

#[test]
fn add_properties() {
    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let sum = &v + &w;
        assert_eq!(v.clone() + w.clone(), sum);
        assert_eq!(v.clone() + &w, sum);
        assert_eq!(&v + w.clone(), sum);
        let mut x = v.clone();
        x += w.clone();
        assert_eq!(x, sum);
        let mut x = v.clone();
        x += &w;
        assert_eq!(x, sum);

        // The sum is taken element by element, and addition is commutative.
        assert_eq!(sum.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&sum.elements) {
            assert_eq!(x + y, *z);
        }
        assert_eq!(&w + &v, sum);
    });

    natural_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // Addition is associative.
        assert_eq!((&u + &v) + &w, &u + (&v + &w));
    });

    natural_vector_gen().test_properties(|v| {
        // The zero vector of the same dimension is the identity.
        let zero = NaturalVector {
            elements: vec![Natural::default(); v.elements.len()],
        };
        assert_eq!(&v + &zero, v);
        assert_eq!(&zero + &v, v);
    });

    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // The sum agrees with the sum of the vectors as `IntegerVector`s.
        assert_eq!(
            IntegerVector::from(&v + &w),
            IntegerVector::from(v) + IntegerVector::from(w)
        );
    });
}
