// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{BalancedMod, DivisibleBy, Mod, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_natural_pair_gen_var_1,
};

#[test]
fn test_balanced_mod() {
    let test = |s, m, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference.
        let q = (&p).balanced_mod(&m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).balanced_mod(m.clone()), q);
        assert_eq!(p.clone().balanced_mod(&m), q);
        assert_eq!(p.balanced_mod(m), q);
    };
    test("0", "1", "0");
    test("0", "7", "0");
    // Modulo 1 every coefficient is zero, so the whole polynomial is.
    test("x^2+27*x+23", "1", "0");
    // Each coefficient goes to the representative closest to zero, which may be negative.
    test("x^2+27*x+23", "10", "x^2-3*x+3");
    test("9", "10", "-1");
    // Half the modulus is positive.
    test("5*x+15", "10", "5*x+5");
    test("x^2+x+1", "2", "x^2+x+1");
    // With an odd modulus there is no tie.
    test("2*x+3", "5", "2*x-2");
    // Reducing the leading coefficient to zero lowers the degree.
    test("10*x^2+7*x+5", "10", "-3*x+5");
    test("6*x^2+3*x+9", "3", "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3+6*x^2+2", "3", "x^3-1");
    // Coefficients and divisor of many limbs.
    test(
        "1000000000000000000000000*x+1",
        "1234567890987",
        "530068894399*x+1",
    );
    test(
        "1000000000000000000000000*x+1",
        "1000000000000000000000001",
        "-x+1",
    );
}

#[test]
#[should_panic]
fn balanced_mod_fail() {
    let _ = NaturalPolynomial::from_str("x")
        .unwrap()
        .balanced_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_ref_fail() {
    let _ = (&NaturalPolynomial::from_str("x").unwrap()).balanced_mod(&Natural::ZERO);
}

// The zero polynomial has no coefficients, so a zero divisor is never reached by the
// coefficient-wise loop; only the explicit check makes these panic rather than quietly returning
// zero.
#[test]
#[should_panic]
fn balanced_mod_zero_polynomial_fail() {
    let _ = NaturalPolynomial::ZERO.balanced_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_zero_polynomial_fail() {
    let _ = (&NaturalPolynomial::ZERO).balanced_mod(&Natural::ZERO);
}

#[test]
fn balanced_mod_properties() {
    natural_polynomial_natural_pair_gen_var_1().test_properties(|(p, m)| {
        let q = (&p).balanced_mod(&m);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!((&p).balanced_mod(m.clone()), q);
        assert_eq!(p.clone().balanced_mod(&m), q);
        assert_eq!(p.clone().balanced_mod(m.clone()), q);

        // This is the `IntegerPolynomial` operation on the same polynomial.
        let m_i = Integer::from(&m);
        assert_eq!(IntegerPolynomial::from(p.clone()).balanced_mod(&m_i), q);

        // Reducing never lengthens the coefficient list.
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // Coefficient by coefficient, this is the `Natural` operation: each result is congruent to
        // its coefficient and lies in (-m/2, m/2].
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let r = q.coefficient(u64::try_from(i).unwrap());
            assert_eq!(*r, c.balanced_mod(&m));
            assert!((r - Integer::from(c)).divisible_by(&m_i));
            // m >= 2|r|, and at equality the remainder is the positive one.
            match m.cmp_double(&r.unsigned_abs()) {
                Less => panic!("remainder too large"),
                Equal => assert!(*r > 0u32),
                Greater => {}
            }
        }

        // Reducing the result into [0, m) gives the ordinary remainder.
        assert_eq!((&q).mod_op(&m), &p % &m);
    });

    natural_polynomial_gen().test_properties(|p| {
        // Modulo 1 everything vanishes.
        assert_eq!((&p).balanced_mod(Natural::ONE), IntegerPolynomial::ZERO);
        // Modulo 2 every odd coefficient becomes 1 and every even one 0, which is `%`.
        assert_eq!(
            (&p).balanced_mod(Natural::TWO),
            IntegerPolynomial::from(&p % Natural::TWO)
        );
    });
}
