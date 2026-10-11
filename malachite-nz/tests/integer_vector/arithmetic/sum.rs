// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use core::str::FromStr;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{integer_vector_gen, integer_vector_triple_gen_var_1};

#[test]
fn test_sum() {
    let test = |ss: &[&str], out| {
        let vs: Vec<IntegerVector> = ss
            .iter()
            .map(|s| IntegerVector::from_str(s).unwrap())
            .collect();
        let sum = IntegerVector::sum(vs.iter());
        assert_eq!(sum.to_string(), out);
        assert_eq!(IntegerVector::sum(vs.into_iter()), sum);
    };
    test(&["()"], "()");
    test(&["(1, -2, 3)"], "(1, -2, 3)");
    test(&["(1, -2, 3)", "(-4, 5, -6)"], "(-3, 3, -3)");
    test(&["(1, -2, 3)", "(-4, 5, -6)", "(7, 8, -9)"], "(4, 11, -12)");
    test(
        &["(18446744073709551615, -1)", "(-18446744073709551616, 1267650600228229401496703205376)"],
        "(-1, 1267650600228229401496703205375)",
    );
    test(&["()", "()", "()"], "()");
}

#[test]
#[should_panic]
fn sum_fail_1() {
    IntegerVector::sum(core::iter::empty::<IntegerVector>());
}

#[test]
#[should_panic]
fn sum_fail_2() {
    let v = IntegerVector::from_str("(1)").unwrap();
    let long = IntegerVector::from_str("(1, 1)").unwrap();
    IntegerVector::sum([v, long].into_iter());
}

#[test]
fn sum_properties() {
    integer_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        let sum = IntegerVector::sum([&u, &v, &w].into_iter());
        assert_eq!(
            IntegerVector::sum([u.clone(), v.clone(), w.clone()].into_iter()),
            sum
        );
        // It is repeated addition, in any order, and entry by entry the scalar sum.
        assert_eq!(&u + &v + &w, sum);
        assert_eq!(IntegerVector::sum([&w, &u, &v].into_iter()), sum);
        for (i, s) in sum.elements.iter().enumerate() {
            assert_eq!(
                Integer::sum([&u.elements[i], &v.elements[i], &w.elements[i]].into_iter()),
                *s
            );
        }
        // Negating every vector negates the sum.
        assert_eq!(IntegerVector::sum([-&u, -&v, -&w].iter()), -&sum);
    });

    integer_vector_gen().test_properties(|v| {
        // The sum of one vector is that vector.
        assert_eq!(IntegerVector::sum([&v].into_iter()), v);
    });
}
