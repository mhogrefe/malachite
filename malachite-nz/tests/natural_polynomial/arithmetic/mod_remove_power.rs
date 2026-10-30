// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModMul, ModPow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_remove_power::*;

// Checks the removal of the largest power of `q` from `p` in every way of calling it, against the
// naive version; checks that `q` to the power times the result is `p`, and that `q` does not divide
// the result unless it is 0; and returns it.
fn verify(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> (NaturalPolynomial, u64) {
    let (s, e) = p.mod_remove_power(q, m);
    assert!(s.is_valid());
    assert_eq!(p.clone().mod_remove_power(q.clone(), m), (s.clone(), e));
    assert_eq!(p.mod_remove_power(q, m.clone()), (s.clone(), e));
    let mut t = p.clone();
    assert_eq!(t.mod_remove_power_assign(q, m), e);
    assert_eq!(t, s);
    let mut t = p.clone();
    assert_eq!(t.mod_remove_power_assign(q.clone(), m), e);
    assert_eq!(t, s);
    assert_eq!(mod_remove_power_naive(p, q, m), (s.clone(), e));
    assert_eq!(q.mod_pow(e, m).mod_mul(&s, m), *p);
    if s != NaturalPolynomial::ZERO {
        assert!(!(&s).mod_divisible_by(q, m));
    }
    (s, e)
}

#[test]
fn test_mod_remove_power() {
    let test = |p, q, m: &str, out: &str, out_e| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        let (s, e) = verify(&p, &q, &m);
        assert_eq!(s.to_string(), out);
        assert_eq!(e, out_e);
    };
    // - modulo 1
    test("0", "0", "1", "0", 0);
    // - the dividend is 0
    test("0", "x+1", "7", "0", 0);
    // - not divisible
    test(
        "x^2+2",
        "x+1",
        "1361129467683753853853498429727072845827",
        "x^2+2",
        0,
    );
    // - divisible several times
    test(
        "x^4+5*x^3+9*x^2+7*x+2",
        "x+1",
        "1361129467683753853853498429727072845827",
        "x+2",
        3,
    );
    // - a non-monic factor
    test(
        "45*x^2+30*x+5",
        "3*x+1",
        "1361129467683753853853498429727072845827",
        "5",
        2,
    );
    // - a modulus of one limb
    test("x^4+4*x^2+3", "x+3", "6", "x^2+1", 2);
}

#[test]
#[should_panic]
fn mod_remove_power_fail_1() {
    // The factor is 0.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_remove_power(NaturalPolynomial::ZERO, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_remove_power_fail_2() {
    // The factor is a constant.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_remove_power(
            NaturalPolynomial::from_str("3").unwrap(),
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_remove_power_fail_3() {
    // The factor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("x^2+1")
        .unwrap()
        .mod_remove_power(
            NaturalPolynomial::from_str("2*x+1").unwrap(),
            Natural::from(6u32),
        );
}

#[test]
#[should_panic]
fn mod_remove_power_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mod_remove_power_assign(
        NaturalPolynomial::from_str("3").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
fn mod_remove_power_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_4().test_properties(
        |(p, q, m)| {
            let e = verify(&p, &q, &m).1;
            assert!(p == NaturalPolynomial::ZERO || e != 0);
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_2().test_properties(
        |(p, q, m)| {
            if q.len() < 2 {
                return;
            }
            let (s, e) = verify(&p, &q, &m);
            // Multiplying by a power of the factor adds to the exponent.
            if p != NaturalPolynomial::ZERO {
                let pq2 = (&q).mod_mul(&q, &m).mod_mul(&p, &m);
                assert_eq!(verify(&pq2, &q, &m), (s, e + 2));
            }
        },
    );
}
