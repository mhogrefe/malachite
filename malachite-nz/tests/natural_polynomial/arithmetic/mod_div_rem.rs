// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CoprimeWith, Mod, ModAdd, ModIsReduced, ModMul};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1,
    natural_polynomial_natural_polynomial_natural_triple_gen_var_2,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_div_rem::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_mul::*;

// The polynomial with the same coefficients as `p`, which must fit in limbs.
fn to_limbs(p: &NaturalPolynomial) -> UnsignedPolynomial<Limb> {
    UnsignedPolynomial::from_coefficients_asc(
        p.coefficients_asc()
            .iter()
            .map(Limb::wrapping_from)
            .collect(),
    )
}

// Checks the quotient and remainder of `p` by `q` in every way of calling them, against the naive
// version, each algorithm over `Natural`s, the quotient and remainder alone, and the word-sized
// versions where they apply; checks that they exist exactly when the divisor's leading coefficient
// is invertible, that `q` times the quotient plus the remainder is `p`, and that the remainder is
// shorter than `q`; and returns them.
#[allow(clippy::type_complexity)]
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    let r = p.checked_mod_div_rem(q, m);
    assert_eq!(p.clone().checked_mod_div_rem(q.clone(), m), r);
    assert_eq!(p.clone().checked_mod_div_rem(q, m), r);
    assert_eq!(p.checked_mod_div_rem(q.clone(), m), r);
    assert_eq!(p.checked_mod_div_rem(q, m.clone()), r);
    assert_eq!(mod_div_rem_naive(p, q, m), r);
    assert_eq!(mod_div_rem_basecase_wrapper(p, q, m), r);
    assert_eq!(mod_div_rem_newton_wrapper(p, q, m), r);
    assert_eq!(
        p.checked_mod_quotient(q, m),
        r.as_ref().map(|(s, _)| s.clone())
    );
    assert_eq!(p.checked_mod_rem(q, m), r.as_ref().map(|(_, t)| t.clone()));
    if m.significant_bits() <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_div_rem(to_limbs(q), Limb::wrapping_from(m)),
            r.as_ref().map(|(s, t)| (to_limbs(s), to_limbs(t)))
        );
    }
    assert_eq!(
        r.is_some(),
        *m == 1u32
            || q.coefficients_asc()
                .last()
                .is_some_and(|y| y.coprime_with(m))
    );
    if let Some((s, t)) = &r {
        assert!(s.is_valid());
        assert!(t.is_valid());
        assert!(s.mod_is_reduced(m));
        assert!(t.mod_is_reduced(m));
        assert!(*q == NaturalPolynomial::ZERO || t.len() < q.len());
        assert_eq!(p.mod_div_rem(q, m), (s.clone(), t.clone()));
        assert_eq!(p.clone().mod_div_rem(q.clone(), m), (s.clone(), t.clone()));
        let mut u = p.clone();
        assert_eq!(u.mod_div_assign_rem(q, m), *t);
        assert_eq!(u, *s);
        let mut u = p.clone();
        assert_eq!(u.mod_div_assign_rem(q.clone(), m), *t);
        assert_eq!(u, *s);
        assert_eq!(q.mod_mul(s, m).mod_add(t, m), *p);
    }
    r
}

#[test]
fn test_mod_div_rem() {
    let test = |p, q, m: &str, out: Option<(&str, &str)>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(
            verify(&p, &q, &m).map(|(s, t)| (s.to_string(), t.to_string())),
            out.map(|(s, t)| (s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    test("0", "0", "1", Some(("0", "0")));
    // - the divisor is 0
    test("3*x+5", "0", "7", None);
    // - the divisor's leading coefficient is not invertible
    test("x^3+x+1", "2*x+1", "4", None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", "7", Some(("0", "5")));
    // - the dividend is 0
    test("0", "x+1", "7", Some(("0", "0")));
    // - a constant divisor
    test("6*x^2+5", "3", "7", Some(("2*x^2+4", "0")));
    // - a non-monic divisor
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^2+x+1",
        "7",
        Some(("5*x^3+4*x^2+4*x+2", "3*x+6")),
    );
    // - a modulus of two limbs
    test(
        "x^4+3*x+1",
        "5*x^2+7",
        "1267650600228229401496703205653",
        Some((
            "760590360136937640898021923392*x^2+963414456173454345137494436296",
            "3*x+862002408155195993017758179847",
        )),
    );
    // - a modulus of more than two limbs, the divisor is 0
    test(
        "3*x+5",
        "0",
        "1361129467683753853853498429727072845827",
        None,
    );
    // - a modulus of more than two limbs, the dividend is shorter
    test(
        "5",
        "3*x+2",
        "1361129467683753853853498429727072845827",
        Some(("0", "5")),
    );
    // - a modulus of more than two limbs, a constant divisor
    test(
        "6*x^2+5",
        "3",
        "1361129467683753853853498429727072845827",
        Some(("2*x^2+453709822561251284617832809909024281944", "0")),
    );
    // - a modulus of more than two limbs
    test(
        "x^5+3*x^4+2*x+1",
        "3*x^2+x+1",
        "1361129467683753853853498429727072845827",
        Some((
            "907419645122502569235665619818048563885*x^3+151236607520417094872610936636341427315*x^\
            2+100824405013611396581740624424227618209*x+1277109130172411023368714576040216497319",
            "1344325400181485287756541658989701576128*x+84020337511342830484783853686856348509",
        )),
    );
    // - a modulus of more than two limbs, the divisor's low coefficients are 0
    test(
        "x^3+2*x+1",
        "x^2",
        "1361129467683753853853498429727072845827",
        Some(("x", "2*x+1")),
    );
    // - a modulus of more than two limbs, not invertible
    test(
        "x^3+x+1",
        "3*x+1",
        "4083388403051261561560495289181218537481",
        None,
    );
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
#[test]
fn test_mod_div_rem_algorithms() {
    for &(p_len, q_len) in &[(10, 5), (300, 150), (300, 2)] {
        for m in [
            "7",
            "18446744073709551557",
            "1267650600228229401496703205653",
            "1361129467683753853853498429727072845827",
        ] {
            let m = Natural::from_str(m).unwrap();
            let p = NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(
                p_len, &m,
            ));
            let mut ys = natural_mod_generated_coefficients(q_len, &m);
            *ys.last_mut().unwrap() = Natural::ONE;
            let q = NaturalPolynomial::from_coefficients_asc(ys);
            assert!(verify(&p, &q, &m).is_some());
        }
    }
}

#[test]
#[should_panic]
fn mod_div_rem_fail_1() {
    // The divisor is 0.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_div_rem(NaturalPolynomial::ZERO, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_div_rem_fail_2() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("x^2+1").unwrap().mod_div_rem(
        NaturalPolynomial::from_str("2*x+1").unwrap(),
        Natural::from(6u32),
    );
}

#[test]
#[should_panic]
fn mod_div_rem_fail_3() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1").unwrap().mod_div_rem(
        NaturalPolynomial::from_str("x+1").unwrap(),
        Natural::from(7u32),
    );
}

#[test]
#[should_panic]
fn mod_div_assign_rem_fail() {
    let mut p = NaturalPolynomial::from_str("x^2+1").unwrap();
    p.mod_div_assign_rem(
        NaturalPolynomial::from_str("2*x+1").unwrap(),
        Natural::from(6u32),
    );
}

#[test]
#[should_panic]
fn checked_mod_div_rem_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1")
        .unwrap()
        .checked_mod_div_rem(
            NaturalPolynomial::from_str("x+1").unwrap(),
            Natural::from(7u32),
        );
}

#[test]
fn mod_div_rem_properties() {
    natural_polynomial_natural_polynomial_natural_triple_gen_var_1().test_properties(
        |(p, q, m)| {
            verify(&p, &q, &m);
        },
    );

    natural_polynomial_natural_polynomial_natural_triple_gen_var_2().test_properties(
        |(p, q, m)| {
            let s = verify(&p, &q, &m).unwrap().0;
            // Dividing a multiple of the divisor leaves no remainder.
            assert_eq!(
                (&q).mod_mul(&s, &m).mod_div_rem(&q, &m),
                (s, NaturalPolynomial::ZERO)
            );
            // Dividing the divisor by itself gives 1.
            assert_eq!(
                (&q).mod_div_rem(&q, &m),
                (NaturalPolynomial::one().mod_op(&m), NaturalPolynomial::ZERO)
            );
        },
    );

    natural_polynomial_natural_natural_triple_gen_var_1().test_properties(|(p, _, m)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!(
            (&p).mod_div_rem(NaturalPolynomial::one().mod_op(&m), &m),
            (p, NaturalPolynomial::ZERO)
        );
    });
}
