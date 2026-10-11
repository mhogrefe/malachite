// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Sum;
use core::str::FromStr;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_triple_gen_var_1;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{rational_vector_gen, rational_vector_triple_gen_var_1};

#[test]
fn test_sum() {
    let test = |ss: &[&str], out| {
        let vs: Vec<RationalVector> = ss
            .iter()
            .map(|s| RationalVector::from_str(s).unwrap())
            .collect();
        let sum = RationalVector::sum(vs.iter());
        assert_eq!(sum.to_string(), out);
        assert_eq!(RationalVector::sum(vs.into_iter()), sum);
    };
    test(&["()"], "()");
    test(&["(1/2, -2, 3)"], "(1/2, -2, 3)");
    test(&["(1/2, -2, 3)", "(1/3, 5, -1/6)"], "(5/6, 3, 17/6)");
    test(
        &["(1/2, -2, 3)", "(1/3, 5, -1/6)", "(1/6, 0, 1/2)"],
        "(1, 3, 10/3)",
    );
    test(
        &["(18446744073709551615/2, -1/3)", "(1/2, 1/3)"],
        "(9223372036854775808, 0)",
    );
    test(&["()", "()", "()"], "()");
}

#[test]
#[should_panic]
fn sum_fail_1() {
    RationalVector::sum(core::iter::empty::<RationalVector>());
}

#[test]
#[should_panic]
fn sum_fail_2() {
    let v = RationalVector::from_str("(1)").unwrap();
    let long = RationalVector::from_str("(1, 1)").unwrap();
    RationalVector::sum([v, long].into_iter());
}

#[test]
fn sum_properties() {
    rational_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        let sum = RationalVector::sum([&u, &v, &w].into_iter());
        assert_eq!(
            RationalVector::sum([u.clone(), v.clone(), w.clone()].into_iter()),
            sum
        );
        // It is repeated addition, in any order, and entry by entry the scalar sum.
        assert_eq!(&u + &v + &w, sum);
        assert_eq!(RationalVector::sum([&w, &u, &v].into_iter()), sum);
        for (i, s) in sum.elements.iter().enumerate() {
            assert_eq!(
                Rational::sum([&u.elements[i], &v.elements[i], &w.elements[i]].into_iter()),
                *s
            );
        }
        // Negating every vector negates the sum.
        assert_eq!(RationalVector::sum([-&u, -&v, -&w].iter()), -&sum);
    });

    rational_vector_gen().test_properties(|v| {
        // The sum of one vector is that vector.
        assert_eq!(RationalVector::sum([&v].into_iter()), v);
    });

    integer_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // As RationalVectors, the sum of IntegerVectors is the same.
        let sum = IntegerVector::sum([&u, &v, &w].into_iter());
        assert_eq!(
            RationalVector::sum([u, v, w].map(RationalVector::from).into_iter()),
            RationalVector::from(sum)
        );
    });
}
