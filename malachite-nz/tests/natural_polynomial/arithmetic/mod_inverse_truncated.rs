// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{CoprimeWith, Mod, ModIsReduced};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{
    ModInverseTruncated, ModMulTruncated, ModSquareTruncated, Polynomial,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_unsigned_natural_triple_gen_var_3,
    natural_polynomial_unsigned_natural_triple_gen_var_4,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_inverse_truncated::*;
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

// Checks the inverse modulo m as a power series of `p`, truncated to `len` coefficients, in every
// way of calling it, against the naive version, each algorithm over `Natural`s, and, when m fits in
// a limb, the inverse of the `UnsignedPolynomial` of limbs; checks that it exists exactly when the
// constant coefficient is invertible and that its product with `p` is 1; and returns it.
fn verify(p: &NaturalPolynomial, len: u64, m: &Natural) -> Option<NaturalPolynomial> {
    let r = p.mod_inverse_truncated(len, m);
    assert_eq!(p.clone().mod_inverse_truncated(len, m.clone()), r);
    assert_eq!(p.clone().mod_inverse_truncated(len, m), r);
    assert_eq!(p.mod_inverse_truncated(len, m.clone()), r);
    assert_eq!(mod_inverse_truncated_naive(p, len, m), r);
    assert_eq!(mod_inverse_truncated_basecase_wrapper(p, len, m), r);
    assert_eq!(mod_inverse_truncated_newton_wrapper(p, len, m), r);
    if m.significant_bits() <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).mod_inverse_truncated(len, Limb::wrapping_from(m)),
            r.as_ref().map(to_limbs)
        );
    }
    let invertible =
        len == 0 || *m == 1u32 || (*p.coefficient(0) != 0u32 && p.coefficient(0).coprime_with(m));
    assert_eq!(r.is_some(), invertible);
    if let Some(q) = &r {
        assert!(q.is_valid());
        assert!(q.mod_is_reduced(m));
        assert!(q.len() <= len);
        assert_eq!(
            p.mod_mul_truncated(q, len, m),
            NaturalPolynomial::one().truncate(len).mod_op(m)
        );
    }
    r
}

#[test]
fn test_mod_inverse_truncated() {
    let test = |p, len, m, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(
            verify(&p, len, &m).map(|q| q.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test(
        "3*x+5",
        0,
        "1361129467683753853853498429727072845827",
        Some("0"),
    );
    // - modulo 1
    test("0", 5, "1", Some("0"));
    // - the polynomial is 0
    test("0", 3, "1361129467683753853853498429727072845827", None);
    // - the constant coefficient is 0
    test("3*x", 3, "1361129467683753853853498429727072845827", None);
    // - the constant coefficient is not invertible
    test("x+3", 3, "4083388403051261561560495289181218537475", None);
    // - a constant polynomial
    test(
        "3",
        5,
        "1361129467683753853853498429727072845827",
        Some("907419645122502569235665619818048563885"),
    );
    // - the constant coefficient is 1
    test(
        "x+1",
        5,
        "1361129467683753853853498429727072845827",
        Some(
            "x^4+1361129467683753853853498429727072845826*x^3+x^2+13611294676837538538534984297270\
            72845826*x+1",
        ),
    );
    // - the constant coefficient is not 1
    test(
        "x^2+3*x+2",
        6,
        "1361129467683753853853498429727072845827",
        Some(
            "446620606583731733295679172254195777536*x^5+467888254516290387262140085218681290754*x\
            ^4+425352958651173079329218259289710264320*x^3+510423550381407695195061911147652317186\
            *x^2+340282366920938463463374607431768211456*x+680564733841876926926749214863536422914\
            ",
        ),
    );
    // - length 1: only the constant coefficient
    test(
        "3*x+5",
        1,
        "1361129467683753853853498429727072845827",
        Some("544451787073501541541399371890829138331"),
    );
    // - only the first coefficients are read
    test(
        "5*x^4+x+3",
        2,
        "1361129467683753853853498429727072845827",
        Some("151236607520417094872610936636341427314*x+907419645122502569235665619818048563885"),
    );
    // - the inverse has zeros at the end
    test(
        "x^2+1",
        4,
        "1361129467683753853853498429727072845827",
        Some("1361129467683753853853498429727072845826*x^2+1"),
    );
    // - m fits in a limb
    test("x^2+3*x+2", 6, "7", Some("6*x^4+3*x^3+x+4"));
    // - m fits in two limbs
    test(
        "x^2+3*x+2",
        6,
        "1267650600228229401496703205653",
        Some(
            "59421121885698253195157962764*x^5+1148808356456832895106387280124*x^4+237684487542793\
            012780631851059*x^3+792281625142643375935439503534*x^2+950737950171172051122527404239*\
            x+633825300114114700748351602827",
        ),
    );
}

// Every algorithm, at lengths that reach Newton's iteration from the dispatcher, against the naive
// version, for moduli that fit in a limb, in two limbs, and in neither.
#[test]
fn test_mod_inverse_truncated_algorithms() {
    for &(p_len, len) in &[(21, 21), (30, 60), (60, 60), (100, 250)] {
        for m in [
            "7",
            "18446744073709551557",
            "1267650600228229401496703205653",
            "1361129467683753853853498429727072845827",
            "1797693134862315907729305190789024733617976978942306572734300811577326758055009631327\
            0847732242298596011106407",
        ] {
            let m = Natural::from_str(m).unwrap();
            let mut xs = natural_mod_generated_coefficients(p_len, &m);
            xs[0] = Natural::ONE;
            let p = NaturalPolynomial::from_coefficients_asc(xs);
            assert!(verify(&p, u64::exact_from(len), &m).is_some());
        }
    }
}

#[test]
#[should_panic]
fn mod_inverse_truncated_fail_1() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1")
        .unwrap()
        .mod_inverse_truncated(3, Natural::from(7u32));
}

#[test]
#[should_panic]
fn mod_inverse_truncated_fail_2() {
    // The modulus is zero.
    NaturalPolynomial::ZERO.mod_inverse_truncated(3, Natural::ZERO);
}

#[test]
fn mod_inverse_truncated_properties() {
    natural_polynomial_unsigned_natural_triple_gen_var_3().test_properties(|(p, len, m)| {
        let Some(q) = verify(&p, len, &m) else {
            return;
        };
        // The inverse of the inverse is the truncated polynomial.
        assert_eq!(verify(&q, len, &m), Some(p.truncate(len)));
        // Inverting to fewer coefficients truncates the inverse.
        let half = len >> 1;
        assert_eq!(verify(&p, half, &m), Some(q.truncate(half)));
        // The inverse of the square is the square of the inverse.
        assert_eq!(
            (&p).mod_square_truncated(len, &m)
                .mod_inverse_truncated(len, &m),
            Some(q.mod_square_truncated(len, &m))
        );
    });

    natural_polynomial_unsigned_natural_triple_gen_var_4().test_properties(|(p, len, m)| {
        verify(&p, len, &m);
    });
}
