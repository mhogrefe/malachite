// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Average, AverageAssign};
use malachite_base::num::basic::traits::Two;
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_pair_gen, rational_triple_gen};
use std::str::FromStr;

#[test]
fn test_average() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        assert_eq!(u.clone().average(v.clone()).to_string(), out);
        assert_eq!(u.clone().average(&v).to_string(), out);
        assert_eq!((&u).average(v.clone()).to_string(), out);
        assert_eq!((&u).average(&v).to_string(), out);

        let mut mut_u = u.clone();
        mut_u.average_assign(v.clone());
        assert_eq!(mut_u.to_string(), out);

        let mut mut_u = u;
        mut_u.average_assign(&v);
        assert_eq!(mut_u.to_string(), out);
    };
    test("0", "0", "0");
    test("1/2", "1/3", "5/12");
    test("3", "4", "7/2");
    test("-3", "-4", "-7/2");
    test("22/7", "-22/7", "0");
    // the average of two integers is not generally an integer, and the average of two non-integers
    // can be
    test("1", "2", "3/2");
    test("1/2", "3/2", "1");
    test("-1/3", "1", "1/3");
    test("1000000000000", "1000000000002", "1000000000001");
}

#[test]
fn average_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let avg = (&x).average(&y);
        assert!(avg.is_valid());
        assert_eq!((&y).average(&x), avg);
        assert_eq!(x.clone().average(y.clone()), avg);
        assert_eq!(x.clone().average(&y), avg);
        assert_eq!((&x).average(y.clone()), avg);

        let mut mut_x = x.clone();
        mut_x.average_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, avg);
        let mut mut_x = x.clone();
        mut_x.average_assign(&y);
        assert_eq!(mut_x, avg);

        // the average is exact, so it is equidistant from both values and lies between them
        assert_eq!(&avg - &x, &y - &avg);
        assert_eq!((&x + &y) / Rational::TWO, avg);
        assert!(avg >= core::cmp::min(&x, &y).clone());
        assert!(avg <= core::cmp::max(&x, &y).clone());
        assert_eq!((-&x).average(-&y), -&avg);
        assert_eq!((&x).average(&x), x);
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        // averaging is affine: shifting both inputs shifts the average
        assert_eq!((&x + &z).average(&y + &z), (&x).average(&y) + &z);
        assert_eq!((&x * &z).average(&y * &z), (&x).average(&y) * &z);
    });

    integer_pair_gen().test_properties(|(x, y)| {
        // the average of two Integers, computed exactly, agrees with the Rational average
        assert_eq!(
            Rational::from(&x).average(Rational::from(&y)),
            Rational::from(&x + &y) / Rational::TWO
        );
    });
}
