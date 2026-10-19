// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Trace;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_gen_var_1, natural_matrix_pair_gen_var_1,
};

#[test]
fn test_trace() {
    let test = |s, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let trace = a.to_trace();
        assert!(trace.is_valid());
        assert_eq!(trace.to_string(), out);
        assert_eq!(a.into_trace(), trace);
    };
    test("[]_{0,0}", "0");
    test("[5]", "5");
    test("[1, 2; 3, 4]", "5");
    test("[1, 0, 0; 0, 1, 0; 0, 0, 1]", "3");
    test("[18446744073709551615, 7; 8, 1]", "18446744073709551616");
    test("[0, 9; 9, 0]", "0");
    // The trace of the n by n identity matrix is n.
    for n in 0..10 {
        assert_eq!(NaturalMatrix::identity(n).into_trace(), n);
    }
}

#[test]
#[should_panic]
fn to_trace_fail_1() {
    let _ = NaturalMatrix::from_str("[1, 2]").unwrap().to_trace();
}

#[test]
#[should_panic]
fn to_trace_fail_2() {
    let _ = NaturalMatrix::from_str("[]_{0,3}").unwrap().to_trace();
}

#[test]
#[should_panic]
fn into_trace_fail_1() {
    let _ = NaturalMatrix::from_str("[1, 2]").unwrap().into_trace();
}

#[test]
#[should_panic]
fn into_trace_fail_2() {
    let _ = NaturalMatrix::from_str("[]_{3,0}").unwrap().into_trace();
}

#[test]
fn trace_properties() {
    natural_matrix_gen_var_1().test_properties(|a| {
        let trace = a.to_trace();
        assert!(trace.is_valid());
        assert_eq!(a.clone().into_trace(), trace);
        assert_eq!(
            (0..a.height())
                .map(|i| a[(usize::exact_from(i), usize::exact_from(i))].clone())
                .sum::<Natural>(),
            trace
        );
        // The trace is unchanged by transposition.
        assert_eq!(a.transpose().into_trace(), trace);
    });

    natural_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        if !a.is_square() {
            return;
        }
        // The trace is additive.
        assert_eq!((&a + &b).into_trace(), a.to_trace() + b.to_trace());
    });
}
