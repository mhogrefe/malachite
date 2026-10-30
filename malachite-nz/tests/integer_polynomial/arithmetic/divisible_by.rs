// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::CheckedDivExact;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_pair_gen, integer_polynomial_pair_gen_var_4,
};

#[test]
fn test_divisible_by() {
    let test = |p, q, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let q = IntegerPolynomial::from_str(q).unwrap();
        assert_eq!((&p).divisible_by(&q), out);
        assert_eq!(p.divisible_by(q), out);
    };
    test("0", "0", true);
    test("x+1", "0", false);
    test("0", "x+1", true);
    test("x^3+3*x^2+3*x+1", "x+1", true);
    test("x^2+1", "x+1", false);
    test("x^2+3", "2*x+2", false);
    test("x^2+x+2", "x^2+1", false);
}

#[test]
fn divisible_by_properties() {
    integer_polynomial_pair_gen().test_properties(|(p, q)| {
        let divisible = (&p).divisible_by(&q);
        assert_eq!(p.clone().divisible_by(q.clone()), divisible);
        assert_eq!(
            divisible,
            p == IntegerPolynomial::ZERO || (&p).checked_div_exact(&q).is_some()
        );
    });

    integer_polynomial_pair_gen_var_4().test_properties(|(p, q)| {
        assert!(p.divisible_by(q));
    });
}
