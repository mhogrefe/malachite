// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::WrappingFrom;

use malachite_base::polynomial::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1,
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5,
    natural_polynomial_natural_unsigned_triple_gen_var_1,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul::*;

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
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    let r = p.checked_mod_power_of_2_div_rem(q, pow);
    assert_eq!(p.clone().checked_mod_power_of_2_div_rem(q.clone(), pow), r);
    assert_eq!(p.clone().checked_mod_power_of_2_div_rem(q, pow), r);
    assert_eq!(p.checked_mod_power_of_2_div_rem(q.clone(), pow), r);
    assert_eq!(mod_power_of_2_div_rem_naive(p, q, pow), r);
    assert_eq!(mod_power_of_2_div_rem_basecase_wrapper(p, q, pow), r);
    assert_eq!(mod_power_of_2_div_rem_newton_wrapper(p, q, pow), r);
    assert_eq!(
        p.checked_mod_power_of_2_quotient(q, pow),
        r.as_ref().map(|(s, _)| s.clone())
    );
    assert_eq!(
        p.checked_mod_power_of_2_rem(q, pow),
        r.as_ref().map(|(_, t)| t.clone())
    );
    if pow <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_power_of_2_div_rem(to_limbs(q), pow),
            r.as_ref().map(|(s, t)| (to_limbs(s), to_limbs(t)))
        );
    }
    assert_eq!(
        r.is_some(),
        pow == 0 || q.coefficients_asc().last().is_some_and(Parity::odd)
    );
    if let Some((s, t)) = &r {
        assert!(s.is_valid());
        assert!(t.is_valid());
        assert!(s.mod_power_of_2_is_reduced(pow));
        assert!(t.mod_power_of_2_is_reduced(pow));
        assert!(*q == NaturalPolynomial::ZERO || t.len() < q.len());
        assert_eq!(p.mod_power_of_2_div_rem(q, pow), (s.clone(), t.clone()));
        assert_eq!(
            p.clone().mod_power_of_2_div_rem(q.clone(), pow),
            (s.clone(), t.clone())
        );
        let mut u = p.clone();
        assert_eq!(u.mod_power_of_2_div_assign_rem(q, pow), *t);
        assert_eq!(u, *s);
        let mut u = p.clone();
        assert_eq!(u.mod_power_of_2_div_assign_rem(q.clone(), pow), *t);
        assert_eq!(u, *s);
        assert_eq!(q.mod_power_of_2_mul(s, pow).mod_power_of_2_add(t, pow), *p);
    }
    r
}

#[test]
fn test_mod_power_of_2_div_rem() {
    let test = |p, q, pow: u64, out: Option<(&str, &str)>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, pow).map(|(s, t)| (s.to_string(), t.to_string())),
            out.map(|(s, t)| (s.to_string(), t.to_string()))
        );
    };
    // - modulo 1
    test("0", "0", 0, Some(("0", "0")));
    // - the divisor is 0
    test("3*x+1", "0", 3, None);
    // - the divisor's leading coefficient is even
    test("x^3+2*x^2+3", "2*x+1", 2, None);
    // - the dividend is shorter than the divisor
    test("5", "3*x+2", 3, Some(("0", "5")));
    // - a non-monic divisor
    test("x^4+7*x+5", "5*x^2+2", 3, Some(("5*x^2+6", "7*x+1")));
    // - an exponent of two limbs
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        100,
        Some((
            "1014120480182583521197362564301*x^2+152118072027387528179604384645",
            "7*x+811296384146066816957890051446",
        )),
    );
    // - an exponent of more than two limbs, the divisor is 0
    test("3*x+1", "0", 200, None);
    // - an exponent of more than two limbs, the dividend is shorter
    test("5", "3*x+2", 200, Some(("0", "5")));
    // - an exponent of more than two limbs, a constant divisor
    test(
        "6*x^2+5",
        "3",
        200,
        Some((
            "2*x^2+535646014752996758513987364113720867507400997927597611767127",
            "0",
        )),
    );
    // - an exponent of more than two limbs
    test(
        "x^4+7*x+5",
        "5*x^2+3",
        200,
        Some((
            "1285550435407192220433569673872930082017762395026234268241101*x^2+19283256531107883306\
            5035451080939512302664359253935140236165",
            "7*x+1028440348325753776346855739098344065614209916020987414592886",
        )),
    );
    // - an exponent of more than two limbs, the divisor's low coefficients are 0
    test("x^3+2*x+1", "x^2", 200, Some(("x", "2*x+1")));
    // - an exponent of more than two limbs, the leading coefficient is even
    test("x^4+7*x+5", "6*x^2+3", 200, None);
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
#[test]
fn test_mod_power_of_2_div_rem_algorithms() {
    for &(p_len, q_len) in &[(10, 5), (500, 250), (300, 2)] {
        for pow in [3, Limb::WIDTH, Limb::WIDTH << 1, 140, 400] {
            let p = NaturalPolynomial::from_coefficients_asc(
                natural_mod_power_of_2_generated_coefficients(p_len, pow),
            );
            let mut ys = natural_mod_power_of_2_generated_coefficients(q_len, pow);
            *ys.last_mut().unwrap() = Natural::ONE;
            let q = NaturalPolynomial::from_coefficients_asc(ys);
            assert!(verify(&p, &q, pow).is_some());
        }
    }
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_fail_1() {
    // The divisor is 0.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_rem(NaturalPolynomial::ZERO, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_fail_2() {
    // The divisor's leading coefficient is not invertible.
    NaturalPolynomial::from_str("x^2+1")
        .unwrap()
        .mod_power_of_2_div_rem(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_rem_fail_3() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_rem(NaturalPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_assign_rem_fail() {
    let mut p = NaturalPolynomial::from_str("x^2+1").unwrap();
    p.mod_power_of_2_div_assign_rem(NaturalPolynomial::from_str("2*x+1").unwrap(), 3);
}

#[test]
#[should_panic]
fn checked_mod_power_of_2_div_rem_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .checked_mod_power_of_2_div_rem(NaturalPolynomial::from_str("x+1").unwrap(), 3);
}

#[test]
fn mod_power_of_2_div_rem_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            verify(&p, &q, pow);
        },
    );

    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_5().test_properties(
        |(p, q, pow)| {
            let s = verify(&p, &q, pow).unwrap().0;
            // Dividing a multiple of the divisor leaves no remainder.
            assert_eq!(
                (&q).mod_power_of_2_mul(&s, pow)
                    .mod_power_of_2_div_rem(&q, pow),
                (s, NaturalPolynomial::ZERO)
            );
            // Dividing the divisor by itself gives 1.
            assert_eq!(
                (&q).mod_power_of_2_div_rem(&q, pow),
                (
                    NaturalPolynomial::one().mod_power_of_2(pow),
                    NaturalPolynomial::ZERO
                )
            );
        },
    );

    natural_polynomial_natural_unsigned_triple_gen_var_1().test_properties(|(p, _, pow)| {
        // Dividing by 1 leaves the polynomial unchanged.
        assert_eq!(
            (&p).mod_power_of_2_div_rem(NaturalPolynomial::one().mod_power_of_2(pow), pow),
            (p, NaturalPolynomial::ZERO)
        );
    });
}
