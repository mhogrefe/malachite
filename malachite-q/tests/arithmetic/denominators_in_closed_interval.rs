// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::comparison::is_strictly_ascending;
use malachite_base::iterators::prefix_to_string;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::exhaustive::exhaustive_positive_naturals;
use malachite_nz::natural::Natural;
use malachite_q::arithmetic::traits::DenominatorsInClosedInterval;
use malachite_q::exhaustive::exhaustive_rationals_with_denominator_inclusive_range;
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen_var_3};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_denominators_in_closed_interval() {
    let test = |a, b, out| {
        let a = Rational::from_str(a).unwrap();
        let b = Rational::from_str(b).unwrap();
        assert_eq!(
            prefix_to_string(Rational::denominators_in_closed_interval(a, b), 20),
            out
        );
    };
    test(
        "0",
        "2",
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, ...]",
    );
    test(
        "1/3",
        "1/2",
        "[2, 3, 5, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, ...]",
    );
    test(
        "99/100",
        "101/100",
        "[1, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, \
        117, 118, ...]",
    );
    test(
        "1/1000000000001",
        "1/1000000000000",
        "[1000000000000, 1000000000001, 3000000000001, 3000000000002, 4000000000001, \
        4000000000003, 5000000000001, 5000000000002, 5000000000003, 5000000000004, 6000000000001, \
        6000000000005, 7000000000001, 7000000000002, 7000000000003, 7000000000004, 7000000000005, \
        7000000000006, 8000000000001, 8000000000003, ...]",
    );
    // about e to about π
    test(
        "268876667/98914198",
        "245850922/78256779",
        "[1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, ...]",
    );
}

#[test]
#[should_panic]
fn denominators_in_closed_interval_fail_1() {
    Rational::denominators_in_closed_interval(Rational::ONE, Rational::ONE);
}

#[test]
#[should_panic]
fn denominators_in_closed_interval_fail_2() {
    Rational::denominators_in_closed_interval(Rational::ONE, Rational::ZERO);
}

#[test]
fn simplest_denominators_in_closed_interval_properties() {
    rational_pair_gen_var_3().test_properties(|(a, b)| {
        let ds = Rational::denominators_in_closed_interval(a.clone(), b.clone())
            .take(20)
            .collect_vec();
        assert!(is_strictly_ascending(ds.iter()));
        for d in &ds {
            assert!(exhaustive_rationals_with_denominator_inclusive_range(
                d.clone(),
                a.clone(),
                b.clone()
            )
            .next()
            .is_some());
        }
        for d in 1u32..=20 {
            let d = Natural::from(d);
            if !ds.contains(&d) {
                assert!(exhaustive_rationals_with_denominator_inclusive_range(
                    d.clone(),
                    a.clone(),
                    b.clone(),
                )
                .next()
                .is_none());
            }
        }
    });

    rational_gen().test_properties(|a| {
        assert!(
            Rational::denominators_in_closed_interval(a.clone(), a + Rational::ONE)
                .take(20)
                .eq(exhaustive_positive_naturals().take(20))
        );
    });
}
