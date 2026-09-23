// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, ModPowerOf2, ModPowerOf2IsReduced, PowerOf2, RemPowerOf2,
    RemPowerOf2Assign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_unsigned_pair_gen_var_1,
};

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = (&p).mod_power_of_2(pow);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.mod_power_of_2(pow), q);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    // Modulo 1 every coefficient is zero, so the whole polynomial is.
    test("x^2-3*x-2", 0, "0");
    // Negative coefficients become non-negative.
    test("x^2-3*x-2", 2, "x^2+x+2");
    test("-1", 1, "1");
    test("-1", 8, "255");
    test("-x", 64, "18446744073709551615*x");
    // Non-negative polynomials behave as NaturalPolynomials do.
    test("x^2+3*x+2", 1, "x^2+x");
    // Reducing the leading coefficient to zero lowers the degree, whatever its sign.
    test("4*x^2+3", 2, "3");
    test("-4*x^2+3", 2, "3");
    test("-4*x^2-4*x-8", 2, "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3-4*x^2+2", 2, "x^3+2");
    // A power too wide for a `u64` still does something here.
    test(
        "-340282366920938463463374607431768211457*x",
        64,
        "18446744073709551615*x",
    );
    test(
        "-340282366920938463463374607431768211457*x",
        200,
        "1606938044258990275541621809974241664058739619175361067089919*x",
    );
}

#[test]
fn mod_power_of_2_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, pow)| {
        let q = (&p).mod_power_of_2(pow);
        assert!(q.is_valid());
        assert_eq!(p.clone().mod_power_of_2(pow), q);

        // The result is reduced, and reducing it again changes nothing.
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert_eq!((&q).mod_power_of_2(pow), q);

        // Reducing never lengthens the coefficient list.
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // Coefficient by coefficient, this is the `Integer` operation, and each result is congruent
        // to the original coefficient.
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let r = q.coefficient(u64::try_from(i).unwrap());
            assert_eq!(*r, c.mod_power_of_2(pow));
            assert!((Integer::from(r) - c).divisible_by_power_of_2(pow));
        }

        // On a polynomial with no negative coefficients, this is the `NaturalPolynomial` operation.
        if let Ok(n) = NaturalPolynomial::try_from(&p) {
            assert_eq!(n.mod_power_of_2(pow), q);
        }
    });

    integer_polynomial_gen().test_properties(|p| {
        // Modulo 1 everything vanishes.
        assert_eq!((&p).mod_power_of_2(0), NaturalPolynomial::ZERO);
        // Powers wider than a word still reduce every coefficient.
        for pow in [64, 100, 200] {
            let q = (&p).mod_power_of_2(pow);
            assert!(q.mod_power_of_2_is_reduced(pow));
            for (i, c) in p.coefficients_asc().iter().enumerate() {
                assert_eq!(
                    *q.coefficient(u64::try_from(i).unwrap()),
                    c.mod_power_of_2(pow)
                );
            }
        }
    });
}

#[test]
fn test_rem_power_of_2() {
    let test = |s, pow, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = (&p).rem_power_of_2(pow);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone().rem_power_of_2(pow), q);
        let mut r = p;
        r.rem_power_of_2_assign(pow);
        assert!(r.is_valid());
        assert_eq!(r, q);
    };
    test("0", 0, "0");
    test("0", 10, "0");
    // Modulo 1 every coefficient is zero, so the whole polynomial is.
    test("x^2-7*x-2", 0, "0");
    // Each remainder keeps its coefficient's sign.
    test("x^2-7*x-2", 2, "x^2-3*x-2");
    test("-1", 1, "-1");
    test("-1", 8, "-1");
    test("-257*x+257", 8, "-x+1");
    // Non-negative polynomials behave as NaturalPolynomials do.
    test("x^2+3*x+2", 1, "x^2+x");
    // Reducing the leading coefficient to zero lowers the degree, whatever its sign.
    test("-4*x^2-3", 2, "-3");
    test("-4*x^2-4*x-8", 2, "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3-4*x^2-2", 2, "x^3-2");
    // A power too wide for a `u64` still does something here.
    test("-340282366920938463463374607431768211457*x", 64, "-x");
    test(
        "-340282366920938463463374607431768211457*x",
        200,
        "-340282366920938463463374607431768211457*x",
    );
}

#[test]
fn rem_power_of_2_properties() {
    integer_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, pow)| {
        let q = (&p).rem_power_of_2(pow);
        assert!(q.is_valid());
        assert_eq!(p.clone().rem_power_of_2(pow), q);
        let mut r = p.clone();
        r.rem_power_of_2_assign(pow);
        assert_eq!(r, q);

        // Reducing again changes nothing, and never lengthens the coefficient list.
        assert_eq!((&q).rem_power_of_2(pow), q);
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // This is `%` by the power of 2.
        assert_eq!(&p % Integer::power_of_2(pow), q);

        // Coefficient by coefficient, this is the `Integer` operation: each remainder is smaller
        // than 2^pow in absolute value, has its coefficient's sign unless it is zero, and is
        // congruent to its coefficient.
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let r = q.coefficient(u64::try_from(i).unwrap());
            assert_eq!(*r, c.rem_power_of_2(pow));
            assert!(r.significant_bits() <= pow);
            assert!(*r == 0u32 || (*r > 0u32) == (*c > 0u32));
            assert!((r - c).divisible_by_power_of_2(pow));
        }

        // `mod_power_of_2` and `rem_power_of_2` agree up to a multiple of 2^pow.
        assert_eq!((&q).mod_power_of_2(pow), (&p).mod_power_of_2(pow));

        // On a polynomial with no negative coefficients, this is `mod_power_of_2`.
        if let Ok(n) = NaturalPolynomial::try_from(&p) {
            assert_eq!(IntegerPolynomial::from(n.mod_power_of_2(pow)), q);
        }
    });

    integer_polynomial_gen().test_properties(|p| {
        // Modulo 1 everything vanishes.
        assert_eq!((&p).rem_power_of_2(0), IntegerPolynomial::ZERO);
        // Powers wider than a word still reduce every coefficient.
        for pow in [64, 100, 200] {
            let q = (&p).rem_power_of_2(pow);
            assert_eq!(&p % Integer::power_of_2(pow), q);
        }
    });
}
