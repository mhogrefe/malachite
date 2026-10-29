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
    CheckedModDivTruncated, ModDivTruncated, ModDivTruncatedAssign, ModInverseTruncated,
    ModMulTruncated, Polynomial,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1,
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4,
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_5,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_div_truncated::*;
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

// Checks the quotient as power series of `p` by `q`, truncated to `len` coefficients, in every way
// of calling it, against the naive version, each algorithm over `Natural`s, and the word-sized
// versions where they apply; checks that it exists exactly when the divisor's constant coefficient
// is invertible, that its product with `q` is `p`, and that it is `p` times the inverse of `q`; and
// returns it.
fn verify(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    len: u64,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    let r = p.checked_mod_div_truncated(q, len, m);
    assert_eq!(p.clone().checked_mod_div_truncated(q.clone(), len, m), r);
    assert_eq!(p.clone().checked_mod_div_truncated(q, len, m), r);
    assert_eq!(p.checked_mod_div_truncated(q.clone(), len, m), r);
    assert_eq!(mod_div_truncated_naive(p, q, len, m), r);
    assert_eq!(mod_div_truncated_basecase_wrapper(p, q, len, m), r);
    assert_eq!(mod_div_truncated_newton_wrapper(p, q, len, m), r);
    if m.significant_bits() <= Limb::WIDTH {
        assert_eq!(
            to_limbs(p).checked_mod_div_truncated(to_limbs(q), len, Limb::wrapping_from(m)),
            r.as_ref().map(to_limbs)
        );
    }
    let invertible =
        len == 0 || *m == 1u32 || (*q.coefficient(0) != 0u32 && q.coefficient(0).coprime_with(m));
    assert_eq!(r.is_some(), invertible);
    if let Some(s) = &r {
        assert!(s.is_valid());
        assert!(s.mod_is_reduced(m));
        assert!(s.len() <= len);
        assert_eq!(p.mod_div_truncated(q, len, m), *s);
        assert_eq!(p.clone().mod_div_truncated(q.clone(), len, m), *s);
        let mut t = p.clone();
        t.mod_div_truncated_assign(q, len, m);
        assert_eq!(t, *s);
        let mut t = p.clone();
        t.mod_div_truncated_assign(q.clone(), len, m);
        assert_eq!(t, *s);
        assert_eq!(q.mod_mul_truncated(s, len, m), p.truncate(len));
        assert_eq!(
            p.mod_mul_truncated(q.mod_inverse_truncated(len, m).unwrap(), len, m),
            *s
        );
    }
    r
}

#[test]
fn test_mod_div_truncated() {
    let test = |p, q, len, m, out: Option<&str>| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let q = NaturalPolynomial::from_str(q).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(
            verify(&p, &q, len, &m).map(|r| r.to_string()),
            out.map(ToString::to_string)
        );
    };
    // - the length is 0
    test(
        "3*x+5",
        "x+1",
        0,
        "1361129467683753853853498429727072845827",
        Some("0"),
    );
    // - modulo 1
    test("0", "0", 5, "1", Some("0"));
    // - the divisor is 0
    test(
        "3*x+5",
        "0",
        3,
        "1361129467683753853853498429727072845827",
        None,
    );
    // - the divisor's constant coefficient is not invertible
    test(
        "3*x+5",
        "x+3",
        3,
        "4083388403051261561560495289181218537475",
        None,
    );
    // - the divisor's constant coefficient is 0
    test(
        "3*x+5",
        "3*x",
        3,
        "1361129467683753853853498429727072845827",
        None,
    );
    // - the dividend is 0
    test(
        "0",
        "x+1",
        4,
        "1361129467683753853853498429727072845827",
        Some("0"),
    );
    // - a constant divisor
    test(
        "x^2+3*x+2",
        "3",
        4,
        "1361129467683753853853498429727072845827",
        Some(
            "907419645122502569235665619818048563885*x^2+x+453709822561251284617832809909024281943",
        ),
    );
    // - a constant dividend
    test(
        "5",
        "x+1",
        4,
        "1361129467683753853853498429727072845827",
        Some(
            "1361129467683753853853498429727072845822*x^3+5*x^2+1361129467683753853853498429727072\
            845822*x+5",
        ),
    );
    // - the divisor's constant coefficient is 1
    test(
        "x^2+3*x+2",
        "x+1",
        4,
        "1361129467683753853853498429727072845827",
        Some("x+2"),
    );
    // - the divisor's constant coefficient is not 1
    test(
        "x^2+3*x+2",
        "x^2+2*x+3",
        5,
        "1361129467683753853853498429727072845827",
        Some(
            "677764055924832165910589753073974544630*x^4+621750497583936945587400517282736978958*x\
            ^3+806595240108891172653924995393820945675*x^2+604946430081668379490443746545365709257\
            *x+453709822561251284617832809909024281943",
        ),
    );
    // - only the first coefficients are read
    test(
        "5*x^4+x+3",
        "2*x^3+x+1",
        2,
        "1361129467683753853853498429727072845827",
        Some("1361129467683753853853498429727072845825*x+3"),
    );
    // - the quotient has zeros at the end
    test(
        "x^2+2*x+1",
        "x+1",
        4,
        "1361129467683753853853498429727072845827",
        Some("x+1"),
    );
    // - m fits in a limb
    test("x^2+3*x+2", "x^2+2*x+3", 5, "7", Some("6*x^4+5*x^3+6*x+3"));
    // - m fits in two limbs
    test(
        "x^2+3*x+2",
        "x^2+2*x+3",
        5,
        "1267650600228229401496703205653",
        Some(
            "370383508708659619367349496302*x^4+172150081512475597734120188422*x^3+107985051130552\
            8749423117545556*x^2+1126800533536203912441513960581*x+845100400152152934331135470436",
        ),
    );
}

// Every algorithm, at lengths that reach the method of Karp and Markstein from the dispatcher,
// against the naive version.
#[test]
fn test_mod_div_truncated_algorithms() {
    for &(p_len, q_len, len) in &[(5, 5, 5), (30, 90, 90), (90, 120, 120), (150, 5, 150)] {
        for m in [
            "7",
            "18446744073709551557",
            "1267650600228229401496703205653",
            "1361129467683753853853498429727072845827",
        ] {
            let m = Natural::from_str(m).unwrap();
            let n = p_len;
            let p =
                NaturalPolynomial::from_coefficients_asc(natural_mod_generated_coefficients(n, &m));
            let n = q_len;
            let mut ys = natural_mod_generated_coefficients(n, &m);
            ys[0] = Natural::ONE;
            let q = NaturalPolynomial::from_coefficients_asc(ys);
            assert!(verify(&p, &q, u64::exact_from(len), &m).is_some());
        }
    }
}

#[test]
#[should_panic]
fn mod_div_truncated_fail_1() {
    // The divisor's constant coefficient is not invertible.
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_div_truncated(
            NaturalPolynomial::from_str("x+2").unwrap(),
            3,
            Natural::from(6u32),
        );
}

#[test]
#[should_panic]
fn mod_div_truncated_fail_2() {
    // A coefficient is not reduced.
    NaturalPolynomial::from_str("7*x+1")
        .unwrap()
        .mod_div_truncated(
            NaturalPolynomial::from_str("x+1").unwrap(),
            3,
            Natural::from(7u32),
        );
}

#[test]
#[should_panic]
fn mod_div_truncated_fail_3() {
    // The modulus is zero.
    NaturalPolynomial::ZERO.mod_div_truncated(NaturalPolynomial::ZERO, 3, Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_div_truncated_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mod_div_truncated_assign(
        NaturalPolynomial::from_str("x+2").unwrap(),
        3,
        Natural::from(6u32),
    );
}

#[test]
fn mod_div_truncated_properties() {
    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(p, q, len, m)| {
            verify(&p, &q, len, &m);
        },
    );

    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_4().test_properties(
        |(p, q, len, m)| {
            let r = verify(&p, &q, len, &m).unwrap();
            // Dividing the product by the divisor gives the dividend back.
            assert_eq!(
                (&r).mod_mul_truncated(&q, len, &m)
                    .mod_div_truncated(&q, len, &m),
                r
            );
            // Dividing by 1 truncates.
            assert_eq!(
                (&p).mod_div_truncated(NaturalPolynomial::one().mod_op(&m), len, &m),
                p.truncate(len)
            );
        },
    );

    natural_polynomial_pair_unsigned_natural_quadruple_gen_var_5().test_properties(
        |(p, q, len, m)| {
            verify(&p, &q, len, &m);
        },
    );
}
