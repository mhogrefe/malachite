// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_q::test_util::generators::{
    rational_rational_natural_natural_quadruple_gen_var_1, rational_rational_natural_triple_gen,
    rational_rational_natural_triple_gen_var_1,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_mutate_numerator() {
    let mut q = Rational::from_str("22/7").unwrap();
    let ret = q.mutate_numerator(|x| {
        *x -= Natural::ONE;
        true
    });
    assert_eq!(q, 3);
    assert!(ret);
}

#[test]
fn test_mutate_denominator() {
    let mut q = Rational::from_str("22/7").unwrap();
    let ret = q.mutate_denominator(|x| {
        *x -= Natural::ONE;
        true
    });
    assert_eq!(q.to_string(), "11/3");
    assert!(ret);
}

#[test]
#[should_panic]
fn mutate_denominator_fail() {
    let mut q = Rational::from_str("22/7").unwrap();
    q.mutate_denominator(|x| {
        *x = Natural::ZERO;
        true
    });
}

#[test]
fn test_mutate_numerator_and_denominator() {
    let mut q = Rational::from_str("22/7").unwrap();
    let ret = q.mutate_numerator_and_denominator(|x, y| {
        *x -= Natural::ONE;
        *y -= Natural::ONE;
        true
    });
    assert_eq!(q.to_string(), "7/2");
    assert!(ret);
}

#[test]
#[should_panic]
fn mutate_numerator_and_denominator_fail() {
    let mut q = Rational::from_str("22/7").unwrap();
    q.mutate_numerator_and_denominator(|x, y| {
        *x = Natural::ONE;
        *y = Natural::ZERO;
        true
    });
}

#[test]
fn mutate_numerator_properties() {
    rational_rational_natural_triple_gen().test_properties(|(mut q, out, new_numerator)| {
        let out_2 = out.clone();
        let new_numerator_2 = new_numerator.clone();
        let old_sign = q >= 0;
        let old_denominator = q.to_denominator();
        assert_eq!(
            q.mutate_numerator(|x| {
                *x = new_numerator;
                out
            }),
            out_2
        );
        assert!(q.is_valid());
        assert_eq!(
            q,
            Rational::from_sign_and_naturals(old_sign, new_numerator_2, old_denominator)
        );
    });
}

#[test]
fn mutate_denominator_properties() {
    rational_rational_natural_triple_gen_var_1().test_properties(
        |(mut q, out, new_denominator)| {
            let out_2 = out.clone();
            let new_denominator_2 = new_denominator.clone();
            let old_sign = q >= 0;
            let old_numerator = q.to_numerator();
            assert_eq!(
                q.mutate_denominator(|x| {
                    *x = new_denominator;
                    out
                }),
                out_2
            );
            assert!(q.is_valid());
            assert_eq!(
                q,
                Rational::from_sign_and_naturals(old_sign, old_numerator, new_denominator_2)
            );
        },
    );
}

#[test]
fn mutate_numerator_and_denominator_properties() {
    rational_rational_natural_natural_quadruple_gen_var_1().test_properties(
        |(mut q, out, new_numerator, new_denominator)| {
            let out_2 = out.clone();
            let new_numerator_2 = new_numerator.clone();
            let new_denominator_2 = new_denominator.clone();
            let old_sign = q >= 0;
            assert_eq!(
                q.mutate_numerator_and_denominator(|x, y| {
                    *x = new_numerator;
                    *y = new_denominator;
                    out
                }),
                out_2
            );
            assert!(q.is_valid());
            assert_eq!(
                q,
                Rational::from_sign_and_naturals(old_sign, new_numerator_2, new_denominator_2)
            );
        },
    );
}
