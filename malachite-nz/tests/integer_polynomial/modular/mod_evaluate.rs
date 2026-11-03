// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::Mod;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::ModEvaluate;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_polynomial_unsigned_unsigned_triple_gen_var_1;
use malachite_nz::test_util::integer_polynomial::modular::mod_evaluate::mod_evaluate_u64_naive;

#[test]
fn test_mod_evaluate_u64() {
    let test = |s, x: u64, m: u64, out: u64| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        assert_eq!((&p).mod_evaluate(x, m), out);
        assert_eq!(mod_evaluate_u64_naive(&p, x, m), out);
    };
    // - the zero polynomial
    test("0", 0, 1, 0);
    test("0", 3, 7, 0);
    // - x == 0, so only the constant term is reduced
    // A negative coefficient is reduced into [0, m).
    test("-1", 0, 7, 6);
    test("-7", 0, 7, 0);
    test("-5*x^2+3*x-7", 0, 11, 4);
    // Everything is 0 mod 1.
    test("x^3-1", 0, 1, 0);
    // - Horner, since the polynomial is short
    test("-5*x^2+3*x-7", 6, 11, 7);
    // The coefficients need not be reduced.
    test("100*x+1", 3, 10, 1);
    // - lazy Shoup
    test(
        "-1000000000000000000000000*x^2+999999999999999999999*x-1",
        123456789,
        1000000007,
        204882185,
    );
    // - Shoup, not lazy
    test(
        "-340282366920938463463374607431768211456*x^3+12345",
        9223372036854775782,
        9223372036854775783,
        14845,
    );
    // - Horner, since the top bit of m is set
    test(
        "-x^4+x^3-x^2+x-1",
        18446744073709551556,
        18446744073709551557,
        18446744073709551552,
    );
}

#[test]
#[should_panic]
fn mod_evaluate_u64_fail_1() {
    // m is 0.
    (&IntegerPolynomial::from_str("x+1").unwrap()).mod_evaluate(0, 0);
}

#[test]
#[should_panic]
fn mod_evaluate_u64_fail_2() {
    // x is not reduced.
    (&IntegerPolynomial::from_str("x+1").unwrap()).mod_evaluate(7, 7);
}

#[test]
fn mod_evaluate_u64_properties() {
    integer_polynomial_unsigned_unsigned_triple_gen_var_1().test_properties(|(p, x, m)| {
        let y = (&p).mod_evaluate(x, m);
        assert!(y < m);
        assert_eq!(mod_evaluate_u64_naive(&p, x, m), y);

        // Reducing the coefficients first, into a `NaturalPolynomial`, gives the same value.
        let m_natural = Natural::from(m);
        assert_eq!(
            (&p).mod_op(m_natural.clone())
                .mod_evaluate(Natural::from(x), m_natural),
            y
        );

        // p(0) is the constant term mod m.
        assert_eq!(
            (&p).mod_evaluate(0, m),
            u64::exact_from(&p.coefficient(0).mod_op(Integer::from(m)))
        );
        // Everything is 0 mod 1.
        if m == 1 {
            assert_eq!(y, 0);
        }
    });
}
