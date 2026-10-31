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
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_pair_gen, rational_polynomial_pair_gen_var_4,
};

#[test]
fn test_divisible_by() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from_str(t).unwrap();
        assert_eq!((&p).divisible_by(&q), out);
        assert_eq!((&p).divisible_by(q.clone()), out);
        assert_eq!(p.clone().divisible_by(&q), out);
        assert_eq!(p.divisible_by(q), out);
    };
    test("0", "0", true);
    test("x", "0", false);
    test("0", "x", true);
    test("x", "3/5", true);
    test("x^2-1/4", "x-1/2", true);
    test("x^2+1", "x+1", false);
}

#[test]
fn divisible_by_properties() {
    rational_polynomial_pair_gen().test_properties(|(p, q)| {
        let divisible = (&p).divisible_by(&q);
        assert_eq!((&p).divisible_by(q.clone()), divisible);
        assert_eq!(p.clone().divisible_by(&q), divisible);
        assert_eq!(
            divisible,
            p == RationalPolynomial::ZERO || (&p).checked_div_exact(&q).is_some()
        );
    });

    rational_polynomial_pair_gen_var_4().test_properties(|(p, q)| {
        assert!(p.divisible_by(q));
    });
}
