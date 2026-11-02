// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::*;
use malachite_base::polynomial::*;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_compositional_inverse_truncated::*;

fn verify(p: &NaturalPolynomial, len: u64, m: &Natural) -> Option<NaturalPolynomial> {
    let r = p.mod_compositional_inverse_truncated(len, m);
    if let Some(r) = &r {
        assert!(r.is_valid());
        assert!(r.mod_is_reduced(m));
    }
    assert_eq!(p.clone().mod_compositional_inverse_truncated(len, m), r);
    assert_eq!(
        p.clone()
            .mod_compositional_inverse_truncated(len, m.clone()),
        r
    );
    assert_eq!(p.mod_compositional_inverse_truncated(len, m.clone()), r);
    assert_eq!(mod_compositional_inverse_truncated_naive(p, len, m), r);
    assert_eq!(mod_compositional_inverse_truncated_newton(p, len, m), r);
    if let Some(r_alt) = mod_compositional_inverse_truncated_lagrange(p, len, m) {
        assert_eq!(Some(r_alt), r);
    }
    if let Some(r_alt) = mod_compositional_inverse_truncated_lagrange_fast(p, len, m) {
        assert_eq!(Some(r_alt), r);
    }
    r
}

fn show(p: Option<NaturalPolynomial>) -> String {
    p.map_or_else(|| "None".to_string(), |p| p.to_string())
}

#[test]
fn test_mod_compositional_inverse_truncated() {
    let test = |p, len, m, out| {
        let p = NaturalPolynomial::from_str(p).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(show(verify(&p, len, &m)), out);
    };
    // - the length is 0
    test("x", 0, "7", "0");
    // - the length is 1
    test("3*x", 1, "7", "0");
    // - the length is 2: the inverse of the coefficient of x
    test("x^2+3*x", 2, "7", "5*x");
    // - modulo 1
    test("0", 5, "1", "0");
    // - the fast Lagrange algorithm
    test("x^2+x", 6, "7", "2*x^4+2*x^3+6*x^2+x");
    // - the fast Lagrange algorithm, with several giant steps
    test("5*x^3+2*x^2+3*x", 8, "7", "4*x^7+4*x^5+4*x^4+2*x^2+5*x");
    // - the polynomial is longer than the result
    test("x^9+x^8+x^7+x^6+x^5+x^4+x^3+x^2+x", 4, "7", "x^3+6*x^2+x");
    // - a linear polynomial
    test("4*x", 6, "7", "2*x");
    // - a division by an index is not possible, so Newton's iteration is used
    test("7*x^3+x^2+3*x", 6, "10", "2*x^5+7*x^3+7*x^2+7*x");
    // - a power of 2 as the modulus
    test(
        "5*x^3+3*x^2+x",
        10,
        "128",
        "43*x^9+35*x^8+42*x^7+58*x^6+8*x^5+68*x^4+13*x^3+125*x^2+x",
    );
    // - a prime modulus less than the length, so Newton's iteration is used
    test("x^2+x", 8, "5", "2*x^7+3*x^6+4*x^5+2*x^3+4*x^2+x");
    // - a composite modulus whose prime factors are all at least the length
    test("3*x^3+x^2+2*x", 6, "49", "31*x^5+9*x^4+6*x^3+6*x^2+25*x");
    // - a composite modulus with a prime factor less than the length
    test(
        "x^2+x",
        10,
        "49",
        "9*x^9+12*x^8+34*x^7+7*x^6+14*x^5+44*x^4+2*x^3+48*x^2+x",
    );
    // - a modulus too large for two limbs, with large prime factors
    test(
        "x^3+2*x^2+x",
        5,
        "392318858461667547569595655490009919272404068553904357377",
        "392318858461667547569595655490009919272404068553904357347*x^4+7*x^3+\
        392318858461667547569595655490009919272404068553904357375*x^2+x",
    );
    // - a modulus too large for two limbs, with a prime factor less than the length
    test(
        "x^2+5*x",
        5,
        "4083388403051261561560495289181218537472",
        "1779181327870271277510646523439727091399*x^4+2625128736553595032696011211508821773370*x^3+\
        1894692219015785364564069814180085401387*x^2+1633355361220504624624198115672487414989*x",
    );
    // - the constant coefficient is not 0
    test("x+1", 5, "7", "None");
    // - the coefficient of x is 0
    test("x^2", 5, "7", "None");
    // - the coefficient of x is not a unit
    test("x^2+2*x", 5, "10", "None");
    // - the polynomial is 0
    test("0", 5, "7", "None");
    // - the polynomial is a nonzero constant
    test("3", 5, "7", "None");
    // - a modulus too large for a limb
    test(
        "7*x^3+5*x^2+3*x",
        6,
        "18446744073709551629",
        "3322344548153246991*x^5+118086153192470838*x^4+7439427651125662797*x^3\
        +17080318586768103360*x^2+6148914691236517210*x",
    );
    // - a large modulus for which a division by an index is not possible
    test(
        "x^2+5*x",
        6,
        "55340232221128654848",
        "32852993605743971830*x^5+46616840735647702727*x^4+31893682633680866362*x^3\
        +16380708737454081835*x^2+33204139332677192909*x",
    );
    // - a modulus of more than two limbs
    test(
        "x^3+x^2+x",
        5,
        "1267650600228229401496703205653",
        "x^3+1267650600228229401496703205652*x^2+x",
    );
}

#[test]
fn mod_compositional_inverse_truncated_properties() {
    natural_polynomial_unsigned_natural_triple_gen_var_5().test_properties(|(p, len, m)| {
        let q = (&p).mod_compositional_inverse_truncated(len, &m).unwrap();
        assert!(q.is_valid());
        assert_eq!(
            p.clone()
                .mod_compositional_inverse_truncated(len, m.clone()),
            Some(q.clone())
        );
        assert_eq!(
            mod_compositional_inverse_truncated_naive(&p, len, &m),
            Some(q.clone())
        );
        assert_eq!(
            mod_compositional_inverse_truncated_newton(&p, len, &m),
            Some(q.clone())
        );
        if let Some(q_alt) = mod_compositional_inverse_truncated_lagrange(&p, len, &m) {
            assert_eq!(q_alt, q);
        }
        if let Some(q_alt) = mod_compositional_inverse_truncated_lagrange_fast(&p, len, &m) {
            assert_eq!(q_alt, q);
        }
        if m != 1u32 && len >= 2 {
            let x = NaturalPolynomial::x();
            assert_eq!((&p).mod_compose_truncated(&q, len, &m), x);
            assert_eq!((&q).mod_compose_truncated(&p, len, &m), x);
        }
    });

    natural_polynomial_unsigned_natural_triple_gen_var_3().test_properties(|(p, len, m)| {
        let q = (&p).mod_compositional_inverse_truncated(len, &m);
        let xs = p.coefficients_asc();
        let reversible = m == 1u32
            || xs.len() >= 2
                && xs[0] == 0u32
                && xs[1] != 0u32
                && (&xs[1]).mod_inverse(&m).is_some();
        assert_eq!(q.is_some(), reversible);
        assert_eq!(mod_compositional_inverse_truncated_naive(&p, len, &m), q);
    });
}
