// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::repeat;
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    DivisibleBy, ElementaryDivisors, Rank, SmithNormalForm,
};
use malachite_base::num::basic::traits::One;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::elementary_divisors::*;
use malachite_nz::integer_matrix::random::random_integer_matrices_from_elementary_operations;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};

// Checks the elementary divisors of `a`: both ways of computing them agree, there are as many as
// the rank, each divides the next, and they are the nonzero diagonal elements of the Smith normal
// form.
fn verify_elementary_divisors(a: &IntegerMatrix) {
    let ed = a.elementary_divisors();
    assert_eq!(a.clone().elementary_divisors(), ed);
    assert_eq!(u64::try_from(ed.len()).unwrap(), a.rank());
    assert_eq!(elementary_divisors_via_snf(a, ed.len()), ed);
    assert_eq!(elementary_divisors_luebeck(a), ed);
    assert!(ed.windows(2).all(|w| (&w[1]).divisible_by(&w[0])));
    let s = a.smith_normal_form();
    for (i, d) in ed.iter().enumerate() {
        assert_eq!(Natural::try_from(&s[(i, i)]).unwrap(), *d);
    }
}

#[test]
fn test_elementary_divisors() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let actual = (&a).elementary_divisors().to_debug_string();
        assert_eq!(a.elementary_divisors().to_debug_string(), actual);
        assert_eq!(actual, out);
    };
    test("[]_{0,0}", "[]");
    test("[]_{2,0}", "[]");
    test("[0, 0; 0, 0]", "[]");
    test("[-3]", "[3]");
    test("[1, 2; 3, 4]", "[1, 2]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1, 3]");
    test("[2, 4, 4; -6, 6, 12; 10, -4, -16]", "[2, 6, 12]");
    test("[6, 4; 4, 6; 9, 3]", "[1, 2]");
    test(
        "[1267650600228229401496703205376, 1; 2535301200456458802993406410752, 3]",
        "[1, 1267650600228229401496703205376]",
    );
}

// A 36 by 36 matrix, mixed from diag(1, ..., 1, 2, 2, 4, 8, 0, 0) by elementary row and column
// operations, whose rank of 34 is large enough for the algorithm of Luebeck to be used.
#[test]
fn test_elementary_divisors_luebeck() {
    let n = 36;
    let tail = [2u32, 2, 4, 8, 0, 0];
    let mut d = IntegerMatrix::zero(n, n);
    for i in 0..usize::try_from(n).unwrap() {
        let j = i + tail.len();
        d[(i, i)] = Integer::from(if j >= 36 { tail[j - 36] } else { 1 });
    }
    let a = random_integer_matrices_from_elementary_operations(EXAMPLE_SEED, repeat(d), 4 * n)
        .next()
        .unwrap();
    let mut expected = vec![Natural::ONE; 30];
    expected.extend([2u32, 2, 4, 8].map(Natural::from));
    assert_eq!((&a).elementary_divisors(), expected);
    verify_elementary_divisors(&a);
}

#[test]
fn elementary_divisors_properties() {
    integer_matrix_gen().test_properties(|a| {
        verify_elementary_divisors(&a);
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        verify_elementary_divisors(&(a * b));
    });
}
