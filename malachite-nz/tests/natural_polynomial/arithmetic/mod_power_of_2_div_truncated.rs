// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, Parity, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::polynomial::{
    CheckedModDivTruncated, CheckedModPowerOf2DivTruncated, ModPowerOf2DivTruncated,
    ModPowerOf2DivTruncatedAssign, ModPowerOf2InverseTruncated, ModPowerOf2MulTruncated,
    Polynomial,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1,
    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_4,
    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_5,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
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

// Checks the quotient as power series of `p` by `q`, truncated to `len` coefficients, in every way
// of calling it, against the naive version, each algorithm over `Natural`s, and the word-sized
// versions where they apply; checks that it exists exactly when the divisor's constant coefficient
// is invertible, that its product with `q` is `p`, and that it is `p` times the inverse of `q`; and
// returns it.
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    pow: u64,
) -> Option<NaturalPolynomial> {
    let r = p.checked_mod_power_of_2_div_truncated(q, len, pow);
    assert_eq!(
        p.clone()
            .checked_mod_power_of_2_div_truncated(q.clone(), len, pow),
        r
    );
    assert_eq!(
        p.clone().checked_mod_power_of_2_div_truncated(q, len, pow),
        r
    );
    assert_eq!(
        p.checked_mod_power_of_2_div_truncated(q.clone(), len, pow),
        r
    );
    assert_eq!(mod_power_of_2_div_truncated_naive(p, q, len, pow), r);
    assert_eq!(
        mod_power_of_2_div_truncated_basecase_wrapper(p, q, len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_div_truncated_newton_wrapper(p, q, len, pow),
        r
    );
    assert_eq!(
        p.checked_mod_div_truncated(q, len, Natural::power_of_2(pow)),
        r
    );
    if pow <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_power_of_2_div_truncated(to_limbs(q), len, pow),
            r.as_ref().map(to_limbs)
        );
    }
    let invertible = len == 0 || pow == 0 || q.coefficient(0).odd();
    assert_eq!(r.is_some(), invertible);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert!(s.mod_power_of_2_is_reduced(pow));
        assert!(s.len() <= len);
        assert_eq!(p.mod_power_of_2_div_truncated(q, len, pow), *s);
        assert_eq!(
            p.clone().mod_power_of_2_div_truncated(q.clone(), len, pow),
            *s
        );
        let mut t = p.clone();
        t.mod_power_of_2_div_truncated_assign(q, len, pow);
        assert_eq!(t, *s);
        let mut t = p.clone();
        t.mod_power_of_2_div_truncated_assign(q.clone(), len, pow);
        assert_eq!(t, *s);
        assert_eq!(q.mod_power_of_2_mul_truncated(s, len, pow), p.truncate(len));
        assert_eq!(
            p.mod_power_of_2_mul_truncated(
                q.mod_power_of_2_inverse_truncated(len, pow).unwrap(),
                len,
                pow
            ),
            *s
        );
    }
    r
}

#[test]
fn test_mod_power_of_2_div_truncated() {
    let test = |p, q, len, pow, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        assert_eq!(
            verify(&p, &q, len, pow).map(|r| r.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test("3*x+5", "x+1", 0, 140, Some("0"));
    // - modulo 1
    test("0", "0", 5, 0, Some("0"));
    // - the divisor is 0
    test("3*x+5", "0", 3, 140, None);
    // - the divisor's constant coefficient is even
    test("3*x+5", "x+2", 3, 140, None);
    // - the divisor's constant coefficient is 0
    test("3*x+5", "3*x", 3, 140, None);
    // - the dividend is 0
    test("0", "x+1", 4, 140, Some("0"));
    // - a constant divisor
    test(
        "x^2+3*x+2",
        "3",
        4,
        140,
        Some(
            "929197716605442630897321594693681729415851*x^2+x+464598858302721315448660797346840864\
            707926",
        ),
    );
    // - a constant dividend
    test(
        "5",
        "x+1",
        4,
        140,
        Some(
            "1393796574908163946345982392040522594123771*x^3+5*x^2+1393796574908163946345982392040\
            522594123771*x+5",
        ),
    );
    // - the divisor's constant coefficient is 1
    test("x^2+3*x+2", "x+1", 4, 140, Some("x+2"));
    // - the divisor's constant coefficient is not 1
    test(
        "x^2+3*x+2",
        "x^2+2*x+3",
        5,
        140,
        Some(
            "367090455942890915910052975681454510386509*x^4+73991670025988950238120052910793174749\
            7807*x^3+206488381467876140199404798820818162092411*x^2+154866286100907105149553599115\
            613621569309*x+464598858302721315448660797346840864707926",
        ),
    );
    // - only the first coefficients are read
    test(
        "5*x^4+x+3",
        "2*x^3+x+1",
        2,
        140,
        Some("1393796574908163946345982392040522594123774*x+3"),
    );
    // - the quotient has zeros at the end
    test("x^2+2*x+1", "x+1", 4, 140, Some("x+1"));
    // - 2^k fits in a limb
    test(
        "x^2+3*x+2",
        "x^2+2*x+3",
        5,
        3,
        Some("5*x^4+7*x^3+3*x^2+5*x+6"),
    );
    // - 2^k fits in two limbs
    test(
        "x^2+3*x+2",
        "x^2+2*x+3",
        5,
        100,
        Some(
            "130416728418542119495545597261*x^4+203450096332925706413051131727*x^3+469500222306751\
            630183964150139*x^2+985950466844178423386324715293*x+422550200076076467165567735126",
        ),
    );
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
#[test]
fn test_mod_power_of_2_div_truncated_algorithms() {
    for &(p_len, q_len, len) in &[(5, 5, 5), (30, 90, 90), (90, 200, 200), (150, 5, 150)] {
        for pow in [3, Limb::WIDTH, Limb::WIDTH << 1, 140, 400] {
            let n = p_len;
            let p = NaturalPolynomial::from_coefficients_asc(
                natural_mod_power_of_2_generated_coefficients(n, pow),
            );
            let n = q_len;
            let mut ys = natural_mod_power_of_2_generated_coefficients(n, pow);
            ys[0] = Natural::ONE;
            let q = NaturalPolynomial::from_coefficients_asc(ys);
            assert!(verify(&p, &q, u64::exact_from(len), pow).is_some());
        }
    }
}

#[test]
#[should_panic]
fn mod_power_of_2_div_truncated_fail_1() {
    // The divisor's constant coefficient is even.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_div_truncated(NaturalPolynomial::from_str("x+2").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_truncated_fail_2() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_div_truncated(NaturalPolynomial::from_str("x+1").unwrap(), 3, 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_div_truncated_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mod_power_of_2_div_truncated_assign(NaturalPolynomial::from_str("x+2").unwrap(), 3, 3);
}

#[test]
fn mod_power_of_2_div_truncated_properties() {
    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(p, q, len, pow)| {
            verify(&p, &q, len, pow);
        },
    );

    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_4().test_properties(
        |(p, q, len, pow)| {
            let r = verify(&p, &q, len, pow).unwrap();
            // Dividing the product by the divisor gives the dividend back.
            assert_eq!(
                (&r).mod_power_of_2_mul_truncated(&q, len, pow)
                    .mod_power_of_2_div_truncated(&q, len, pow),
                r
            );
            // Dividing by 1 truncates.
            assert_eq!(
                (&p).mod_power_of_2_div_truncated(
                    NaturalPolynomial::one().mod_power_of_2(pow),
                    len,
                    pow
                ),
                p.truncate(len)
            );
        },
    );

    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_5().test_properties(
        |(p, q, len, pow)| {
            verify(&p, &q, len, pow);
        },
    );
}
