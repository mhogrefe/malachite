// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Mod, ModMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_2,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_div_exact::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_div_rem::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::*;

// Checks the exact quotient of `p` by `q`, which must divide it, in every way of calling it,
// against the naive version and each algorithm over `Natural`s; checks that its product with `q` is
// `p`; and returns it.
fn verify_exact(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    let r = p.mod_div_exact(q, m);
    assert!(r.is_valid());
    assert_eq!(p.clone().mod_div_exact(q.clone(), m), r);
    assert_eq!(p.clone().mod_div_exact(q, m), r);
    assert_eq!(p.mod_div_exact(q.clone(), m), r);
    assert_eq!(p.mod_div_exact(q, m.clone()), r);
    let mut s = p.clone();
    s.mod_div_exact_assign(q, m);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_div_exact_assign(q.clone(), m);
    assert_eq!(s, r);
    assert_eq!(mod_div_exact_top_wrapper(p, q, m).unwrap(), r);
    assert_eq!(
        mod_div_exact_bidirectional_basecase_wrapper(p, q, m).unwrap(),
        r
    );
    assert_eq!(
        mod_div_exact_bidirectional_newton_wrapper(p, q, m).unwrap(),
        r
    );
    assert_eq!(mod_div_rem_naive(p, q, m).unwrap().0, r);
    assert_eq!(verify_checked(p, q, m), Some(r.clone()));
    assert_eq!(q.mod_mul(&r, m), *p);
    r
}

// Checks the checked exact quotient of `p` by `q` and divisibility in every way of calling them,
// against the quotient and remainder, and returns the checked quotient.
fn verify_checked(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    let r = p.checked_mod_div_exact(q, m);
    assert_eq!(p.clone().checked_mod_div_exact(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_div_exact(q, m), r);
    assert_eq!(p.checked_mod_div_exact(q.clone(), m), r);
    let (s, t) = p.mod_div_rem(q, m);
    assert_eq!(
        r,
        if t == NaturalPolynomial::ZERO {
            Some(s)
        } else {
            None
        }
    );
    assert_eq!(p.mod_divisible_by(q, m), r.is_some());
    assert_eq!(p.clone().mod_divisible_by(q.clone(), m), r.is_some());
    r
}

#[test]
fn test_mod_div_exact() {
    let test = |p, q, m: &str, out: &str| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify_exact(&p, &q, &m).to_string(), out);
    };
    // - modulo 1
    test("0", "0", "1", "0");
    // - the dividend is 0
    test("0", "x+1", "7", "0");
    // - a non-monic divisor
    test("x^5+3*x^4+6*x+2", "3*x^2+x+1", "7", "5*x^3+4*x^2+4*x+2");
    // - a modulus of two limbs
    test(
        "3*x^4+x^3+16*x^2+5*x+5",
        "3*x^2+x+1",
        "1267650600228229401496703205653",
        "x^2+5",
    );
    // - a modulus of more than two limbs, a constant divisor
    test(
        "3*x^2+9*x+6",
        "3",
        "1361129467683753853853498429727072845827",
        "x^2+3*x+2",
    );
    // - a modulus of more than two limbs
    test(
        "15*x^5+17*x^4+21*x^3+14*x^2+6*x+2",
        "3*x^2+x+1",
        "1361129467683753853853498429727072845827",
        "5*x^3+4*x^2+4*x+2",
    );
    // - a modulus of more than two limbs, the divisor's low coefficients are 0
    test(
        "6*x^4+2*x^3",
        "2*x^3",
        "1361129467683753853853498429727072845827",
        "3*x+1",
    );
    // - a modulus of more than two limbs, the lowest coefficient is not invertible
    test(
        "x^5+5*x^4+x^3+3*x^2+15*x+3",
        "x^3+3",
        "4083388403051261561560495289181218537481",
        "x^2+5*x+1",
    );
}

#[test]
fn test_checked_mod_div_exact() {
    let test = |p, q, m: &str, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(
            verify_checked(&p, &q, &m).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - not divisible
    test(
        "x^3+2*x+1",
        "x^2+3",
        "1361129467683753853853498429727072845827",
        None,
    );
    // - divisible
    test(
        "x^2+3*x+2",
        "x+1",
        "1361129467683753853853498429727072845827",
        Some("x+2"),
    );
}

// Every algorithm, at lengths that reach the bidirectional method and the method of Karp and
// Markstein, against the naive version.
#[test]
fn test_mod_div_exact_algorithms() {
    for &(q_len, b_len) in &[(10, 5), (100, 50), (300, 150), (150, 300)] {
        for (m, low) in [
            ("7", 1u32),
            ("18446744073709551557", 0),
            ("1267650600228229401496703205653", 2),
            ("1361129467683753853853498429727072845827", 2),
            ("4083388403051261561560495289181218537481", 3),
        ] {
            let m = Natural::from_str(m).unwrap();
            let qs = natural_mod_generated_coefficients(q_len, &m);
            let mut bs = natural_mod_generated_coefficients(b_len, &m);
            *bs.last_mut().unwrap() = Natural::ONE;
            bs[0] = Natural::from(low);
            let q = NaturalPolynomial::from_coefficients_asc(qs);
            let b = NaturalPolynomial::from_coefficients_asc(bs);
            let p = (&b).mod_mul(&q, &m);
            verify_exact(&p, &b, &m);
        }
    }
}

#[test]
#[should_panic]
fn mod_div_exact_fail_1() {
    // The divisor is 0.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_div_exact(NaturalPolynomial::ZERO, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_div_exact_fail_2() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("2*x^2+x")
        .unwrap()
        .mod_div_exact(
            NaturalPolynomial::from_str("2*x+1").unwrap(),
            Natural::from(6u32),
        );
}

#[test]
#[should_panic]
fn mod_div_exact_fail_3() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1").unwrap().mod_div_exact(
        NaturalPolynomial::from_str("x+1").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_div_exact_assign_fail() {
    let mut p = NaturalPolynomial::from_str("2*x^2+x").unwrap();
    p.mod_div_exact_assign(
        NaturalPolynomial::from_str("2*x+1").unwrap(),
        Natural::from(6u32),
    );
}

#[test]
#[should_panic]
fn checked_mod_div_exact_fail() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("2*x^2+x")
        .unwrap()
        .checked_mod_div_exact(
            NaturalPolynomial::from_str("2*x+1").unwrap(),
            Natural::from(6u32),
        );
}

#[test]
fn mod_div_exact_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_3().test_properties(
        |(p, q, m)| {
            verify_exact(&p, &q, &m);
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_2().test_properties(
        |(p, q, m)| {
            verify_checked(&p, &q, &m);
            // The product of the polynomials is divisible by each.
            let pq = (&p).mod_mul(&q, &m);
            assert_eq!(verify_exact(&pq, &q, &m), p);
        },
    );

    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, _, m)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!(
            (&p).mod_div_exact(NaturalPolynomial::one().mod_op(&m), &m),
            p
        );
    });
}
