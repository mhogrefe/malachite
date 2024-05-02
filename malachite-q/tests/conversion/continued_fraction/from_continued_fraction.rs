// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::vecs::vec_from_str;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_gen, natural_vec_integer_pair_gen_var_1};
use malachite_q::conversion::traits::ContinuedFraction;
use malachite_q::test_util::conversion::continued_fraction::from_continued_fraction::*;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_from_continued_fraction() {
    let test = |floor: &str, xs: &str, out: &str| {
        let floor = Integer::from_str(floor).unwrap();
        let xs: Vec<Natural> = vec_from_str(xs).unwrap();
        let x = Rational::from_continued_fraction(floor.clone(), xs.iter().cloned());
        assert!(x.is_valid());
        assert_eq!(Rational::from_continued_fraction_ref(&floor, xs.iter()), x);
        assert_eq!(from_continued_fraction_alt(floor, xs), x);
        assert_eq!(x.to_string(), out);
    };
    test("0", "[]", "0");
    test("3", "[7]", "22/7");
    test("3", "[6, 1]", "22/7");
    test("-4", "[1, 6]", "-22/7");
    test(
        "3",
        "[7, 15, 1, 292, 1, 1, 1, 2, 1, 3, 1, 14, 2, 1, 1, 2, 2, 2, 2]",
        "14885392687/4738167652",
    );
}

#[test]
#[should_panic]
fn from_continued_fraction_fail_1() {
    Rational::from_continued_fraction(Integer::ONE, std::iter::once(Natural::ZERO));
}

#[test]
#[should_panic]
fn from_continued_fraction_fail_2() {
    Rational::from_continued_fraction(
        Integer::ONE,
        vec![Natural::from(3u32), Natural::ZERO, Natural::ONE].into_iter(),
    );
}

#[test]
#[should_panic]
fn from_continued_fraction_ref_fail_1() {
    Rational::from_continued_fraction_ref(&Integer::ONE, [Natural::ZERO].iter());
}

#[test]
#[should_panic]
fn from_continued_fraction_ref_fail_2() {
    Rational::from_continued_fraction_ref(
        &Integer::ONE,
        [Natural::from(3u32), Natural::ZERO, Natural::ONE].iter(),
    );
}

#[test]
fn from_continued_fraction_properties() {
    natural_vec_integer_pair_gen_var_1().test_properties(|(xs, floor)| {
        let x = Rational::from_continued_fraction(floor.clone(), xs.iter().cloned());
        assert!(x.is_valid());
        assert_eq!(Rational::from_continued_fraction_ref(&floor, xs.iter()), x);
        assert_eq!(from_continued_fraction_alt(floor.clone(), xs.clone()), x);
        if xs.last() != Some(&Natural::ONE) {
            let (floor_alt, cf) = (&x).continued_fraction();
            let xs_alt = cf.collect_vec();
            assert_eq!(floor_alt, floor);
            assert_eq!(xs_alt, xs);
        }
        if !xs.is_empty() {
            let mut alt_xs = xs;
            let last = alt_xs.last_mut().unwrap();
            if *last > 1u32 {
                *last -= Natural::ONE;
                alt_xs.push(Natural::ONE);
                assert_eq!(
                    Rational::from_continued_fraction(floor, alt_xs.into_iter()),
                    x
                );
            }
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(
            Rational::from_continued_fraction_ref(&x, std::iter::empty()),
            x
        );
    });
}
