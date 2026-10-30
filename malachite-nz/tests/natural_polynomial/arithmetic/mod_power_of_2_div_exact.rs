// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Mul};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5,
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6,
    natural_polynomial_natural_unsigned_triple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_exact::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul::*;

// Checks the exact quotient of `p` by `q`, which must divide it, in every way of calling it,
// against the naive version and each algorithm over `Natural`s; checks that its product with `q` is
// `p`; and returns it.
fn verify_exact(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    let r = p.mod_power_of_2_div_exact(q, pow);
    assert!(r.is_valid());
    assert_eq!(p.clone().mod_power_of_2_div_exact(q.clone(), pow), r);
    assert_eq!(p.clone().mod_power_of_2_div_exact(q, pow), r);
    assert_eq!(p.mod_power_of_2_div_exact(q.clone(), pow), r);

    let mut s = p.clone();
    s.mod_power_of_2_div_exact_assign(q, pow);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_power_of_2_div_exact_assign(q.clone(), pow);
    assert_eq!(s, r);
    assert_eq!(mod_power_of_2_div_exact_top_wrapper(p, q, pow).unwrap(), r);
    assert_eq!(
        mod_power_of_2_div_exact_bidirectional_basecase_wrapper(p, q, pow).unwrap(),
        r
    );
    assert_eq!(
        mod_power_of_2_div_exact_bidirectional_newton_wrapper(p, q, pow).unwrap(),
        r
    );
    assert_eq!(mod_power_of_2_div_rem_naive(p, q, pow).unwrap().0, r);
    assert_eq!(verify_checked(p, q, pow), Some(r.clone()));
    assert_eq!(q.mod_power_of_2_mul(&r, pow), *p);
    r
}

// Checks the checked exact quotient of `p` by `q` and divisibility in every way of calling them,
// against the quotient and remainder, and returns the checked quotient.
fn verify_checked(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<NaturalPolynomial> {
    let r = p.checked_mod_power_of_2_div_exact(q, pow);
    assert_eq!(
        p.clone().checked_mod_power_of_2_div_exact(q.clone(), pow),
        r
    );
    assert_eq!(p.clone().checked_mod_power_of_2_div_exact(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_div_exact(q.clone(), pow), r);
    let (s, t) = p.mod_power_of_2_div_rem(q, pow);
    assert_eq!(
        r,
        if t == NaturalPolynomial::ZERO {
            Some(s)
        } else {
            None
        }
    );
    assert_eq!(p.mod_power_of_2_divisible_by(q, pow), r.is_some());
    assert_eq!(
        p.clone().mod_power_of_2_divisible_by(q.clone(), pow),
        r.is_some()
    );
    r
}

#[test]
fn test_mod_power_of_2_div_exact() {
    let test = |p, q, pow: u64, out: &str| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();

        assert_eq!(verify_exact(&p, &q, pow).to_string(), out);
    };
    // - modulo 1
    test("0", "0", 0, "0");
    // - the dividend is 0
    test("0", "x+1", 3, "0");
    // - a non-monic divisor
    test("x^4+4", "5*x^2+2", 3, "5*x^2+6");
    // - an exponent of two limbs
    test("3*x^4+x^3+16*x^2+5*x+5", "3*x^2+x+1", 100, "x^2+5");
    // - an exponent of more than two limbs, a constant divisor
    test("3*x^2+9*x+6", "3", 200, "x^2+3*x+2");
    // - an exponent of more than two limbs
    test(
        "15*x^5+17*x^4+21*x^3+14*x^2+6*x+2",
        "3*x^2+x+1",
        200,
        "5*x^3+4*x^2+4*x+2",
    );
    // - an exponent of more than two limbs, the divisor's low coefficients are 0
    test("9*x^4+3*x^3", "3*x^3", 200, "3*x+1");
    // - an exponent of more than two limbs, the lowest coefficient is even
    test("x^5+5*x^4+x^3+2*x^2+10*x+2", "x^3+2", 200, "x^2+5*x+1");
}

#[test]
fn test_checked_mod_power_of_2_div_exact() {
    let test = |p, q, pow: u64, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();

        assert_eq!(
            verify_checked(&p, &q, pow).map(|s| s.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - not divisible
    test("x^4+7*x+5", "5*x^2+2", 200, None);
    // - divisible
    test("x^2+3*x+2", "x+1", 200, Some("x+2"));
}

// Every algorithm, at lengths that reach the bidirectional method and the method of Karp and
// Markstein, against the naive version.
#[test]
fn test_mod_power_of_2_div_exact_algorithms() {
    for &(q_len, b_len) in &[(10, 5), (100, 50), (300, 150), (150, 300)] {
        for (pow, low) in [(3, 1u32), (Limb::WIDTH, 2), (Limb::WIDTH << 1, 0), (140, 1), (400, 2)] {
            let qs = natural_mod_power_of_2_generated_coefficients(q_len, pow);
            let mut bs = natural_mod_power_of_2_generated_coefficients(b_len, pow);
            *bs.last_mut().unwrap() = Natural::ONE;
            bs[0] = Natural::from(low);
            let q = NaturalPolynomial::from_coefficients_asc(qs);
            let b = NaturalPolynomial::from_coefficients_asc(bs);
            let p = (&b).mod_power_of_2_mul(&q, pow);
            verify_exact(&p, &b, pow);
        }
    }
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_fail_1() {
    // The divisor is 0.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_exact(NaturalPolynomial::ZERO, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_fail_2() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("2*x^2+x")
        .unwrap()
        .mod_power_of_2_div_exact(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_fail_3() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_exact(NaturalPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_exact_assign_fail() {
    let mut p = NaturalPolynomial::from_str("2*x^2+x").unwrap();
    p.mod_power_of_2_div_exact_assign(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_div_exact_fail() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("2*x^2+x")
        .unwrap()
        .checked_mod_power_of_2_div_exact(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
fn mod_power_of_2_div_exact_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_6().test_properties(
        |(p, q, pow)| {
            verify_exact(&p, &q, pow);
        },
    );

    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, pow)| {
            verify_checked(&p, &q, pow);
            // The product of the polynomials is divisible by each.
            let pq = (&p).mod_power_of_2_mul(&q, pow);
            assert_eq!(verify_exact(&pq, &q, pow), p);
        },
    );

    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, _, pow)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!(
            (&p).mod_power_of_2_div_exact(NaturalPolynomial::one().mod_power_of_2(pow), pow),
            p
        );
    });
}
