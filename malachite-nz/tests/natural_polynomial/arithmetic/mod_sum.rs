// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModSum};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;

#[test]
fn test_mod_sum() {
    let test = |ss: &[&str], m, out| {
        let ps: Vec<NaturalPolynomial> = ss
            .iter()
            .map(|s| NaturalPolynomial::from_str(s).unwrap())
            .collect();
        let m = Natural::from_str(m).unwrap();
        let sum = NaturalPolynomial::mod_sum(ps.iter(), &m);
        assert_eq!(sum.to_string(), out);
        assert_eq!(NaturalPolynomial::mod_sum(ps.iter(), m.clone()), sum);
        assert_eq!(NaturalPolynomial::mod_sum(ps.into_iter(), &m), sum);
    };
    test(&[], "7", "0");
    test(
        &["x^2+2*x+3", "7*x+5", "x^3+7*x^2+6*x"],
        "9",
        "x^3+8*x^2+6*x+8",
    );
    test(
        &["x+1000000000000000000000000000056", "1000000000000000000000000000056*x+1"],
        "1000000000000000000000000000057",
        "0",
    );
    test(
        &["1267650600228229401496703205374", "633825300114114700748351602688*x+1"],
        "1267650600228229401496703205375",
        "633825300114114700748351602688*x",
    );
}

#[test]
#[should_panic]
fn mod_sum_fail_1() {
    let p = NaturalPolynomial::from_str("7*x+1").unwrap();
    NaturalPolynomial::mod_sum([&p].into_iter(), &Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_sum_fail_2() {
    NaturalPolynomial::mod_sum(core::iter::empty::<NaturalPolynomial>(), Natural::ZERO);
}

#[test]
fn mod_sum_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            let sum = NaturalPolynomial::mod_sum([&p, &q].into_iter(), &m);
            assert!(sum.mod_is_reduced(&m));
            assert_eq!(
                NaturalPolynomial::mod_sum([p.clone(), q.clone()].into_iter(), &m),
                sum
            );
            // It is `mod_add`, in either order, and summing more terms keeps adding.
            assert_eq!((&p).mod_add(&q, &m), sum);
            assert_eq!(NaturalPolynomial::mod_sum([&q, &p].into_iter(), &m), sum);
            assert_eq!(
                NaturalPolynomial::mod_sum([&p, &q, &p].into_iter(), &m),
                (&sum).mod_add(&p, &m)
            );
            // The sum of one polynomial is that polynomial, and the sum of none is zero.
            assert_eq!(NaturalPolynomial::mod_sum([&p].into_iter(), &m), p);
            assert_eq!(
                NaturalPolynomial::mod_sum(core::iter::empty::<NaturalPolynomial>(), &m),
                NaturalPolynomial::ZERO
            );
        },
    );

    unsigned_polynomial_unsigned_polynomial_unsigned_triple_gen_var_2::<Limb>().test_properties(
        |(p, q, m)| {
            // It agrees with the UnsignedPolynomial implementation.
            let sum = UnsignedPolynomial::mod_sum([&p, &q].into_iter(), m);
            assert_eq!(
                NaturalPolynomial::mod_sum(
                    [NaturalPolynomial::from(p), NaturalPolynomial::from(q)].into_iter(),
                    Natural::from(m)
                ),
                NaturalPolynomial::from(sum)
            );
        },
    );
}
