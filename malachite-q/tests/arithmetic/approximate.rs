// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Reciprocal, RoundToMultiple};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::arithmetic::traits::{Approximate, ApproximateAssign};
use malachite_q::test_util::arithmetic::approximate::approximate_naive;
use malachite_q::test_util::generators::{
    rational_gen, rational_natural_natural_triple_gen_var_1, rational_natural_pair_gen_var_3,
    rational_natural_pair_gen_var_4,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_approximate() {
    let test = |x, d, out| {
        let mut x = Rational::from_str(x).unwrap();
        let d = Natural::from_str(d).unwrap();
        let a = x.clone().approximate(&d);
        assert!(a.is_valid());
        assert_eq!(a.to_string(), out);
        let a = (&x).approximate(&d);
        assert!(a.is_valid());
        assert_eq!(a.to_string(), out);

        x.approximate_assign(&d);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test("3", "1", "3");
    test("3", "10", "3");
    test("27/32", "1", "1");
    test("27/32", "3", "1");
    test("27/32", "4", "3/4");
    test("27/32", "5", "4/5");
    test("27/32", "6", "5/6");
    test("27/32", "12", "5/6");
    test("27/32", "13", "11/13");
    test("27/32", "18", "11/13");
    test("27/32", "19", "16/19");
    test("27/32", "31", "16/19");
    test("27/32", "32", "27/32");
    test("27/32", "100", "27/32");

    test("6369051672525773/4503599627370496", "10", "7/5");
    test("6369051672525773/4503599627370496", "100", "140/99");
    test("6369051672525773/4503599627370496", "1000", "1393/985");
    test(
        "6369051672525773/4503599627370496",
        "1000000",
        "665857/470832",
    );

    test("884279719003555/281474976710656", "10", "22/7");
    test("884279719003555/281474976710656", "100", "311/99");
    test("884279719003555/281474976710656", "1000", "355/113");
    test(
        "884279719003555/281474976710656",
        "1000000",
        "3126535/995207",
    );

    test("6121026514868073/2251799813685248", "10", "19/7");
    test("6121026514868073/2251799813685248", "100", "193/71");
    test("6121026514868073/2251799813685248", "1000", "1457/536");
    test(
        "6121026514868073/2251799813685248",
        "1000000",
        "1084483/398959",
    );

    test("7/9", "2", "1");
    test("-7/9", "2", "-1");
    test("1/2", "1", "0");
    test("-1/2", "1", "0");
    test("3/2", "1", "2");
    test("-3/2", "1", "-2");
    test("1/4", "2", "0");
    test("-1/4", "2", "0");
}

#[test]
#[should_panic]
fn approximate_assign_fail() {
    let mut x = Rational::ONE;
    x.approximate_assign(&Natural::ZERO);
}

#[test]
#[should_panic]
fn approximate_fail() {
    Rational::ONE.approximate(&Natural::ZERO);
}

#[test]
#[should_panic]
fn approximate_ref_fail() {
    (&Rational::ONE).approximate(&Natural::ZERO);
}

#[test]
fn approximate_properties() {
    rational_natural_pair_gen_var_3().test_properties(|(x, max_denominator)| {
        let a = x.clone().approximate(&max_denominator);
        assert!(a.is_valid());

        let a_alt = (&x).approximate(&max_denominator);
        assert!(a_alt.is_valid());
        assert_eq!(a, a_alt);

        let mut a_alt = x.clone();
        a_alt.approximate_assign(&max_denominator);
        assert!(a_alt.is_valid());
        assert_eq!(a_alt, a);

        assert_eq!(
            (&x).round_to_multiple(Rational::from(a.denominator_ref()).reciprocal(), Nearest)
                .0,
            a
        );
        assert_eq!((-x).approximate(&max_denominator), -a);
    });

    rational_natural_pair_gen_var_4().test_properties(|(x, max_denominator)| {
        assert_eq!(
            (&x).approximate(&max_denominator),
            approximate_naive(&x, &max_denominator)
        );
    });

    rational_natural_natural_triple_gen_var_1().test_properties(|(x, d, max_d)| {
        let a = (&x).approximate(&max_d);
        let a_alt = (&x)
            .round_to_multiple(Rational::from(&d).reciprocal(), Nearest)
            .0;
        assert!((&x - a_alt).ge_abs(&(&x - &a)));

        let a_worse = x.approximate(&d);
        assert!(a_worse.denominator_ref() <= a.denominator_ref());
    });

    rational_gen().test_properties(|x| {
        assert_eq!((&x).approximate(x.denominator_ref()), x);
        assert_eq!(
            (&x).approximate(&Natural::ONE),
            Integer::rounding_from(x, Nearest).0
        );
    });
}
