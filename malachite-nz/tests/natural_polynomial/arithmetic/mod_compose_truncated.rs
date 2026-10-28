// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Mod, ModIsReduced};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{
    ModCompose, ModComposeTruncated, ModComposeTruncatedAssign, Polynomial,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_2,
    natural_polynomial_unsigned_natural_triple_gen_var_3,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_compose_truncated::*;
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

// Checks the composition modulo m as power series of `p` and `q`, truncated to `len` coefficients,
// in every way of calling it, against the naive version, each algorithm over `Natural`s, the
// truncation of the full composition, and, when m fits in a limb, the composition of the
// `UnsignedPolynomial`s of limbs, and returns it.
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    let r = p.mod_compose_truncated(q, len, m);
    assert!(r.is_valid());
    assert!(r.mod_is_reduced(m));
    assert_eq!(
        p.clone().mod_compose_truncated(q.clone(), len, m.clone()),
        r
    );
    assert_eq!(p.clone().mod_compose_truncated(q.clone(), len, m), r);
    assert_eq!(p.clone().mod_compose_truncated(q, len, m.clone()), r);
    assert_eq!(p.clone().mod_compose_truncated(q, len, m), r);
    assert_eq!(p.mod_compose_truncated(q.clone(), len, m.clone()), r);
    assert_eq!(p.mod_compose_truncated(q.clone(), len, m), r);
    assert_eq!(p.mod_compose_truncated(q, len, m.clone()), r);
    let mut s = p.clone();
    s.mod_compose_truncated_assign(q.clone(), len, m.clone());
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_compose_truncated_assign(q.clone(), len, m);
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_compose_truncated_assign(q, len, m.clone());
    assert_eq!(s, r);
    let mut s = p.clone();
    s.mod_compose_truncated_assign(q, len, m);
    assert_eq!(s, r);
    assert_eq!(mod_compose_truncated_naive(p, q, len, m), r);
    assert_eq!(mod_compose_truncated_horner_wrapper(p, q, len, m), r);
    assert_eq!(mod_compose_truncated_brent_kung_wrapper(p, q, len, m), r);
    if p.len() > 2 {
        assert_eq!(
            mod_compose_truncated_divide_and_conquer_wrapper(p, q, len, m),
            r
        );
    }
    assert_eq!(p.mod_compose(q, m).truncate(len), r);
    if m.significant_bits() <= Limb::WIDTH {
        assert_eq!(
            to_limbs(&r),
            to_limbs(p).mod_compose_truncated(to_limbs(q), len, Limb::wrapping_from(m))
        );
    }
    r
}

#[test]
fn test_mod_compose_truncated() {
    let test = |p, q, len, m, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify(&p, &q, len, &m).to_string(), out);
    };
    // - the length is 0
    test("3*x^2+2*x+1", "x", 0, "10000000000000000000009", "0");
    // - the outer polynomial is 0
    test("0", "2*x", 5, "10000000000000000000009", "0");
    // - the inner polynomial is 0: the constant coefficient
    test("3*x^2+2*x+1", "0", 5, "10000000000000000000009", "1");
    // - a constant outer polynomial
    test("5", "3*x^2+2*x", 4, "10000000000000000000009", "5");
    // - length 1: the constant coefficient
    test("3*x+5", "x", 1, "10000000000000000000009", "5");
    // - m fits in a limb
    test("x^3+3*x^2+x+2", "x^2+5*x", 4, "7", "x^3+6*x^2+5*x+2");
    // - a coefficient vanishes modulo m
    test("x^2+2*x+1", "x^2+2*x", 3, "6", "4*x+1");
    // - a monomial inner polynomial of degree 1
    test(
        "x^3+3*x^2+x+2",
        "3*x",
        4,
        "10000000000000000000009",
        "27*x^3+27*x^2+3*x+2",
    );
    // - a monomial inner polynomial of higher degree
    test(
        "x^3+3*x^2+x+2",
        "10000000000000000000008*x^2",
        7,
        "10000000000000000000009",
        "10000000000000000000008*x^6+3*x^4+10000000000000000000008*x^2+2",
    );
    // - the inner polynomial's valuation truncates the outer polynomial
    test(
        "7*x^8+6*x^7+5*x^6+4*x^5+3*x^4+2*x^3+x^2+3*x+1",
        "x^3+x^2",
        7,
        "10000000000000000000009",
        "3*x^6+2*x^5+x^4+3*x^3+3*x^2+1",
    );
    // - Horner's rule
    test(
        "x^3+3*x^2+x+2",
        "x^2+5*x",
        4,
        "10000000000000000000009",
        "155*x^3+76*x^2+5*x+2",
    );
    // - the coefficients wrap around modulo m
    test(
        "10000000000000000000008*x^3+10000000000000000000007*x^2+10000000000000000000006*x+1000000\
        0000000000000005",
        "10000000000000000000008*x^2+10000000000000000000008*x",
        5,
        "10000000000000000000009",
        "x^4+10000000000000000000006*x^3+x^2+3*x+10000000000000000000005",
    );
    // - divide and conquer
    test(
        "9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "10000000000000000000008*x^2+3*x",
        10,
        "10000000000000000000009",
        "9999999999999999879679*x^9+26195*x^8+8850*x^7+2939*x^6+954*x^5+300*x^4+90*x^3+25*x^2+6*x+\
        1",
    );
    // - the Brent-Kung algorithm
    test(
        "12*x^11+11*x^10+10*x^9+9*x^8+8*x^7+7*x^6+6*x^5+5*x^4+4*x^3+3*x^2+2*x+1",
        "6*x^5+5*x^4+4*x^3+3*x^2+10000000000000000000008*x",
        12,
        "10000000000000000000009",
        "9999999999999999997309*x^11+5176*x^10+9999999999999999999003*x^9+1212*x^8+999999999999999\
        9999715*x^7+269*x^6+9999999999999999999937*x^5+54*x^4+9999999999999999999995*x^3+9*x^2+100\
        00000000000000000007*x+1",
    );
}

// Every algorithm, at lengths that reach each of them, against the naive version, for moduli that
// fit in a limb and that do not.
#[test]
fn test_mod_compose_truncated_algorithms() {
    for &(len1, len2, n) in
        &[(8, 2, 8), (9, 3, 10), (16, 16, 16), (40, 3, 100), (50, 50, 50), (64, 9, 64)]
    {
        for m in
            ["7", "18446744073709551557", "18446744073709551629", "1267650600228229401496703205653"]
        {
            let m = Natural::from_str(m).unwrap();
            let p = NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(
                len1, &m,
            ));
            let mut ys = natural_mod_generated_coefficients(len2, &m);
            ys[0] = Natural::ZERO;
            let q = NaturalPolynomial::from_coefficients_asc(ys);
            verify(&p, &q, u64::exact_from(n), &m);
        }
    }
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_1() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1")
        .unwrap()
        .mod_compose_truncated(
            NaturalPolynomial::from_str("x").unwrap(),
            3,
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_2() {
    // A coefficient of the inner polynomial is not reduced.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_compose_truncated(
            NaturalPolynomial::from_str("7*x").unwrap(),
            3,
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_3() {
    // The modulus is zero.
    NaturalPolynomial::ZERO.mod_compose_truncated(NaturalPolynomial::ZERO, 3, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_4() {
    // The inner polynomial has a nonzero constant coefficient.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_compose_truncated(
            NaturalPolynomial::from_str("x+1").unwrap(),
            3,
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_compose_truncated_fail_5() {
    // The inner polynomial has a nonzero constant coefficient, even when the length is 0.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_compose_truncated(
            NaturalPolynomial::from_str("x+1").unwrap(),
            0,
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_compose_truncated_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mod_compose_truncated_assign(
        NaturalPolynomial::from_str("x+1").unwrap(),
        3,
        Natural::from(7u32),
    );
}

#[test]
fn mod_compose_truncated_properties() {
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_2().test_properties(
        |(p, q, len, m)| {
            let r = verify(&p, &q, len, &m);
            assert!(r.len() <= len);
            // Composition is associative: (p(q))(q) = p(q(q)).
            assert_eq!(
                (&r).mod_compose_truncated(&q, len, &m),
                (&p).mod_compose_truncated((&q).mod_compose_truncated(&q, len, &m), len, &m)
            );
            // Only the first `len` coefficients of each polynomial matter.
            assert_eq!(
                p.truncate(len)
                    .mod_compose_truncated(q.truncate(len), len, &m),
                r
            );
        },
    );

    natural_polynomial_unsigned_natural_triple_gen_var_3().test_properties(|(p, len, m)| {
        let x = NaturalPolynomial::x().mod_op(&m);
        // Composing with x truncates.
        assert_eq!(verify(&p, &x, len, &m), p.truncate(len));
        // Composing with 0 gives the constant coefficient.
        assert_eq!(
            verify(&p, &NaturalPolynomial::ZERO, len, &m),
            NaturalPolynomial::from(p.coefficient(0).clone()).truncate(len)
        );
        // Composing x with a polynomial truncates it.
        if *p.coefficient(0) == 0u32 {
            assert_eq!(verify(&x, &p, len, &m), p.truncate(len));
        }
    });
}
