// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Height, ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::u64_polynomial_gen;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_unsigned_pair_gen_var_1,
};

#[test]
fn test_mod_power_of_2() {
    let test = |s, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        // by reference
        let q = (&p).mod_power_of_2(pow);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        // by value
        let q = p.clone().mod_power_of_2(pow);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        // in place
        let mut q = p;
        q.mod_power_of_2_assign(pow);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 8, "0");
    test("x^2+3*x+2", 1, "x^2+x");
    test("x^2+3*x+2", 2, "x^2+3*x+2");
    // Modulo 2^0 every coefficient is zero, so the whole polynomial is.
    test("x^2+3*x+2", 0, "0");
}

#[test]
fn test_mod_power_of_2_beyond_a_word() {
    // A `Natural` coefficient can be larger than any power of 2, so unlike a `U64Polynomial` there
    // is no width past which this stops doing anything.
    let test = |s, pow, out| {
        assert_eq!(
            NaturalPolynomial::from_str(s)
                .unwrap()
                .mod_power_of_2(pow)
                .to_string(),
            out
        );
    };
    // 2^128 + 1 modulo 2^64 is 1.
    test("340282366920938463463374607431768211457*x", 64, "x");
    // 2^128 modulo 2^64 is 0, which drops the term and leaves the zero polynomial.
    test("340282366920938463463374607431768211456*x", 64, "0");
    // 2^128 modulo 2^128 is 0; modulo 2^129 it is unchanged.
    test("340282366920938463463374607431768211456", 128, "0");
    test(
        "340282366920938463463374607431768211456",
        129,
        "340282366920938463463374607431768211456",
    );
}

#[test]
fn test_mod_power_of_2_lowers_the_degree() {
    let test = |s, pow, out| {
        assert_eq!(
            NaturalPolynomial::from_str(s)
                .unwrap()
                .mod_power_of_2(pow)
                .to_string(),
            out
        );
    };
    test("4*x^2+3", 2, "3");
    test("4*x^2+4*x+3", 2, "3");
    // Every coefficient can vanish at once.
    test("4*x^2+4*x+4", 2, "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3+4*x^2+3", 2, "x^3+3");
    test("8*x^5+8*x^4+8*x^3+1", 3, "1");
}

#[test]
fn mod_power_of_2_properties() {
    natural_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, pow)| {
        let q = (&p).mod_power_of_2(pow);
        assert!(q.is_valid());
        // The three forms agree.
        assert_eq!(p.clone().mod_power_of_2(pow), q);
        let mut r = p.clone();
        r.mod_power_of_2_assign(pow);
        assert_eq!(r, q);

        // The result is reduced, and reducing it again changes nothing.
        assert!(q.mod_power_of_2_is_reduced(pow));
        assert_eq!((&q).mod_power_of_2(pow), q);

        // Reducing never raises the degree, and never lengthens the coefficient list.
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // Coefficient by coefficient, this is the `Natural` operation.
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            assert_eq!(
                *q.coefficient(u64::try_from(i).unwrap()),
                c.mod_power_of_2(pow)
            );
        }

        // A polynomial that is already reduced is left alone.
        assert_eq!(p.mod_power_of_2_is_reduced(pow), p == q);
    });

    natural_polynomial_gen().test_properties(|p| {
        // Modulo 2^0 everything vanishes.
        assert_eq!((&p).mod_power_of_2(0), NaturalPolynomial::ZERO);
        // Reducing to the height's width leaves the polynomial alone, and to one bit less does not,
        // unless it was zero already.
        let bits = p.height_significant_bits();
        assert_eq!((&p).mod_power_of_2(bits), p);
        if bits != 0 {
            assert_ne!((&p).mod_power_of_2(bits - 1), p);
        }
    });

    u64_polynomial_gen().test_properties(|p| {
        // The `u64` and `Natural` polynomials agree, the conversion changing no coefficient.
        let q = NaturalPolynomial::from(p.clone());
        for pow in 0..8 {
            assert_eq!(
                NaturalPolynomial::from((&p).mod_power_of_2(pow)),
                (&q).mod_power_of_2(pow)
            );
        }
    });
}
