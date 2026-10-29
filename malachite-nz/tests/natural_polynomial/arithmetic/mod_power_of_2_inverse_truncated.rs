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
    ModInverseTruncated, ModPowerOf2InverseTruncated, ModPowerOf2MulTruncated,
    ModPowerOf2SquareTruncated, Polynomial,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_unsigned_unsigned_triple_gen_var_4,
    natural_polynomial_unsigned_unsigned_triple_gen_var_6,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_inverse_truncated::*;
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

// Checks the inverse modulo 2^k as a power series of `p`, truncated to `len` coefficients, in every
// way of calling it, against the naive version, each algorithm over `Natural`s, the inverse modulo
// 2^k as a general modulus, and, when 2^k fits in a limb, the inverse of the `UnsignedPolynomial`
// of limbs; checks that it exists exactly when the constant coefficient is odd and that its product
// with `p` is 1; and returns it.
fn verify(p: &NaturalPolynomial, len: u64, pow: u64) -> Option<NaturalPolynomial> {
    let r = p.mod_power_of_2_inverse_truncated(len, pow);
    assert_eq!(p.clone().mod_power_of_2_inverse_truncated(len, pow), r);
    assert_eq!(mod_power_of_2_inverse_truncated_naive(p, len, pow), r);
    assert_eq!(
        mod_power_of_2_inverse_truncated_basecase_wrapper(p, len, pow),
        r
    );
    assert_eq!(
        mod_power_of_2_inverse_truncated_newton_wrapper(p, len, pow),
        r
    );
    assert_eq!(p.mod_inverse_truncated(len, Natural::power_of_2(pow)), r);
    if pow <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).mod_power_of_2_inverse_truncated(len, pow),
            r.as_ref().map(to_limbs)
        );
    }
    let invertible = len == 0 || pow == 0 || p.coefficient(0).odd();
    assert_eq!(r.is_some(), invertible);
    if let Some(q) = &r {
        assert!(q.is_valid());
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert!(q.len() <= len);
        assert_eq!(
            p.mod_power_of_2_mul_truncated(q, len, pow),
            NaturalPolynomial::one().truncate(len).mod_power_of_2(pow)
        );
    }
    r
}

#[test]
fn test_mod_power_of_2_inverse_truncated() {
    let test = |p, len, pow, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        assert_eq!(
            verify(&p, len, pow).map(|q| q.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test("3*x+5", 0, 140, Some("0"));
    // - modulo 1
    test("0", 5, 0, Some("0"));
    // - the polynomial is 0
    test("0", 3, 140, None);
    // - the constant coefficient is 0
    test("3*x", 3, 140, None);
    // - the constant coefficient is even
    test("2*x+4", 3, 140, None);
    // - a constant polynomial
    test(
        "3",
        5,
        140,
        Some("929197716605442630897321594693681729415851"),
    );
    // - the constant coefficient is 1
    test(
        "x+1",
        5,
        140,
        Some(
            "x^4+1393796574908163946345982392040522594123775*x^3+x^2+13937965749081639463459823920\
            40522594123775*x+1",
        ),
    );
    // - the constant coefficient is not 1
    test(
        "3*x^2+x+5",
        6,
        140,
        Some(
            "1357312555763367846886429956946469874699992*x^5+1101210797903442170729033768303376091\
            165313*x^4+158335290909567424304903599735803366692461*x^3+4348645313713471512599465063\
            16643049366618*x^2+55751862996326557853839295681620903764951*x+11150372599265311570767\
            85913632418075299021",
        ),
    );
    // - length 1: only the constant coefficient
    test(
        "3*x+5",
        1,
        140,
        Some("1115037259926531157076785913632418075299021"),
    );
    // - only the first coefficients are read
    test(
        "5*x^4+x+3",
        2,
        140,
        Some(
            "1084064002706349736046875193809295350985159*x+929197716605442630897321594693681729415\
            851",
        ),
    );
    // - the inverse has zeros at the end
    test(
        "x^2+1",
        4,
        140,
        Some("1393796574908163946345982392040522594123775*x^2+1"),
    );
    // - 2^k fits in a limb
    test("3*x^2+x+5", 6, 3, Some("x^4+5*x^3+2*x^2+7*x+5"));
    // - 2^k fits in two limbs
    test(
        "3*x^2+x+5",
        6,
        100,
        Some(
            "425200434930953618767630175960*x^5+332225869307814361544255976065*x^4+448241252240701\
            916369234253421*x^3+141976867225561692967630759002*x^2+50706024009129176059868128215*x\
            +1014120480182583521197362564301",
        ),
    );
}

// Every algorithm, at lengths that reach Newton's iteration from the dispatcher, against the naive
// version, for powers of 2 that fit in a limb, in two limbs, and in neither.
#[test]
fn test_mod_power_of_2_inverse_truncated_algorithms() {
    for &(p_len, len) in &[(21, 21), (30, 60), (60, 60), (100, 250)] {
        for pow in [3, Limb::WIDTH, Limb::WIDTH << 1, 140, 400] {
            let mut xs = natural_mod_power_of_2_generated_coefficients(p_len, pow);
            xs[0] = Natural::ONE;
            let p = NaturalPolynomial::from_coefficients_asc(xs);
            assert!(verify(&p, u64::exact_from(len), pow).is_some());
        }
    }
}

#[test]
#[should_panic]
fn mod_power_of_2_inverse_truncated_fail() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("8*x+1")
        .unwrap()
        .mod_power_of_2_inverse_truncated(3, 3);
}

#[test]
fn mod_power_of_2_inverse_truncated_properties() {
    natural_polynomial_unsigned_unsigned_triple_gen_var_4().test_properties(|(p, len, pow)| {
        let Some(q) = verify(&p, len, pow) else {
            return;
        };
        // The inverse of the inverse is the truncated polynomial.
        assert_eq!(verify(&q, len, pow), Some(p.truncate(len)));
        // Inverting to fewer coefficients truncates the inverse.
        let half = len >> 1;
        assert_eq!(verify(&p, half, pow), Some(q.truncate(half)));
        // The inverse of the square is the square of the inverse.
        assert_eq!(
            (&p).mod_power_of_2_square_truncated(len, pow)
                .mod_power_of_2_inverse_truncated(len, pow),
            Some(q.mod_power_of_2_square_truncated(len, pow))
        );
    });

    natural_polynomial_unsigned_unsigned_triple_gen_var_6().test_properties(|(p, len, pow)| {
        verify(&p, len, pow);
    });
}
