// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Content, DivisibleBy};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::content_chained::integers_content_chained;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_vec_natural_pair_gen;
use malachite_nz::test_util::integer_polynomial::arithmetic::content_chained::*;

#[test]
fn test_integers_content_chained() {
    let test = |xs: &[&str], x, out| {
        let xs: Vec<Integer> = xs.iter().map(|x| Integer::from_str(x).unwrap()).collect();
        let x = Natural::from_str(x).unwrap();
        let g = integers_content_chained(&xs, &x);
        assert_eq!(g.to_string(), out);
        assert_eq!(integers_content_chained_naive(&xs, &x), g);
    };
    // - xs.iter().position(|c| *c != 0u32) is None
    test(&[], "6", "6");
    test(&[], "0", "0");
    // Zeros alone.
    test(&["0", "0"], "6", "6");
    // - one element, so no loop iteration
    // - gcd != 1u32 && let [middle] = xs
    test(&["-4"], "6", "2");
    test(&["-4"], "0", "4");
    // Zeros at both ends are skipped.
    test(&["0", "-4", "0"], "6", "2");
    // - gcd == 1 from the start, so no loop iteration
    test(&["4", "6"], "1", "1");
    // - *first.unsigned_abs_ref() == 1u32
    test(&["-1", "6"], "4", "1");
    // - *last.unsigned_abs_ref() == 1u32
    test(&["6", "1"], "4", "1");
    // - one loop iteration, and nothing left
    test(&["4", "6"], "8", "2");
    // - one loop iteration, then one element left
    test(&["4", "6", "10"], "0", "2");
    // - gcd == 1 ends the loop while at least 2 elements remain
    test(&["2", "6", "6", "3"], "6", "1");
    // - two loop iterations, then one element left
    test(&["12", "-24", "36", "48", "-60"], "12", "12");
    // - three loop iterations, and nothing left
    test(&["12", "24", "36", "48", "60", "72"], "0", "12");
    // - gcd == 1 ends the loop with one element left, which is skipped
    test(&["6", "10", "15", "6", "6", "12", "6"], "0", "1");
    // Elements of many limbs.
    test(
        &["1000000000000000000000", "-3000000000000000000000", "0"],
        "5000000000000000000000",
        "1000000000000000000000",
    );
}

#[test]
fn integers_content_chained_properties() {
    integer_vec_natural_pair_gen().test_properties(|(xs, x)| {
        let g = integers_content_chained(&xs, &x);
        assert_eq!(integers_content_chained_naive(&xs, &x), g);
        // The result divides `x` and every element, and is zero only if they all are.
        assert!((&x).divisible_by(&g));
        assert!(xs.iter().all(|c| c.unsigned_abs_ref().divisible_by(&g)));
        assert_eq!(g == 0u32, x == 0u32 && xs.iter().all(|c| *c == 0u32));
        // The order of the elements and their signs make no difference.
        let ys: Vec<Integer> = xs.iter().rev().map(|c| -c).collect();
        assert_eq!(integers_content_chained(&ys, &x), g);
        // With `x` zero, this is the content of the polynomial with coefficients `xs`.
        let content = IntegerPolynomial::from_coefficients_asc(xs.clone()).content();
        assert_eq!(integers_content_chained(&xs, &Natural::ZERO), content);
    });
}
