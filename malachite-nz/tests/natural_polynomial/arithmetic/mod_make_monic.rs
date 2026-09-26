// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivisibleBy, Gcd, ModInverse, ModMul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{ModMakeMonic, ModMakeMonicAssign, Polynomial};
use malachite_base::test_util::generators::unsigned_polynomial_unsigned_unsigned_triple_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_natural_natural_triple_gen_var_1;

#[test]
fn test_mod_make_monic() {
    let test = |s, m, out: Result<&str, &str>| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        let q = (&p).mod_make_monic(&m);
        assert_eq!(
            q.as_ref()
                .map(ToString::to_string)
                .map_err(ToString::to_string),
            out.map(ToString::to_string).map_err(ToString::to_string)
        );
        assert_eq!((&p).mod_make_monic(m.clone()), q);
        assert_eq!(p.clone().mod_make_monic(&m), q);
        assert_eq!(p.clone().mod_make_monic(m.clone()), q);
        for by_value in [false, true] {
            let mut r = p.clone();
            let result = if by_value {
                r.mod_make_monic_assign(m.clone())
            } else {
                r.mod_make_monic_assign(&m)
            };
            match &q {
                Ok(q) => {
                    assert_eq!(result, Ok(()));
                    assert_eq!(r, *q);
                }
                Err(g) => {
                    assert_eq!(result, Err(g.clone()));
                    assert_eq!(r, p);
                }
            }
        }
    };
    test("0", "7", Ok("0"));
    test("3*x^2+x+2", "7", Ok("x^2+5*x+3"));
    test("x+1", "7", Ok("x+1"));
    test("5", "7", Ok("1"));
    test("2*x+1", "4", Err("2"));
    test("6*x^2+1", "9", Err("3"));
    test("0", "1", Ok("0"));
    test(
        "3*x^3+7",
        "1000000000000000000000000000057",
        Ok("x^3+666666666666666666666666666707"),
    );
    test("10*x^2+3", "1000000000000000000000000000000", Err("10"));
}

#[test]
#[should_panic]
fn mod_make_monic_fail_1() {
    // m is 0.
    let _ = (&NaturalPolynomial::ZERO).mod_make_monic(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_make_monic_fail_2() {
    // A coefficient is not reduced.
    let _ = (&NaturalPolynomial::from_str("7*x+1").unwrap()).mod_make_monic(Natural::from(7u32));
}

#[test]
fn mod_make_monic_properties() {
    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, _, m)| {
        let q = (&p).mod_make_monic(&m);
        assert_eq!((&p).mod_make_monic(m.clone()), q);
        assert_eq!(p.clone().mod_make_monic(&m), q);
        assert_eq!(p.clone().mod_make_monic(m.clone()), q);
        let mut r = p.clone();
        let result = r.mod_make_monic_assign(&m);
        let leading = p.leading_coefficient();
        match q {
            Ok(q) => {
                assert!(q.is_valid());
                assert_eq!(result, Ok(()));
                assert_eq!(r, q);
                assert_eq!(q == NaturalPolynomial::ZERO, p == NaturalPolynomial::ZERO);
                if p != NaturalPolynomial::ZERO {
                    assert!(q.is_monic());
                    let inverse = leading.mod_inverse(&m).unwrap();
                    for (c, d) in p.coefficients_asc().iter().zip(q.coefficients_asc()) {
                        assert_eq!(c.mod_mul(&inverse, &m), *d);
                    }
                    assert_eq!((&q).mod_make_monic(&m), Ok(q));
                }
            }
            Err(g) => {
                assert_eq!(result, Err(g.clone()));
                assert_eq!(r, p);
                assert!(leading.mod_inverse(&m).is_none());
                assert_eq!(g, leading.gcd(&m));
                assert!(g > 1u32 && g < m);
                assert!((&m).divisible_by(&g));
            }
        }
    });

    unsigned_polynomial_unsigned_unsigned_triple_gen_var_2::<u64>().test_properties(|(p, _, m)| {
        // The u64 and Natural versions agree.
        let q = NaturalPolynomial::from(p.clone());
        assert_eq!(
            (&q).mod_make_monic(Natural::from(m)),
            (&p).mod_make_monic(m)
                .map(NaturalPolynomial::from)
                .map_err(Natural::from)
        );
    });
}
