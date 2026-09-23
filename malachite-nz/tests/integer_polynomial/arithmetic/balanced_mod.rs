// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    BalancedMod, BalancedModAssign, DivisibleBy, Mod, UnsignedAbs,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen_var_1,
};

#[test]
fn test_balanced_mod() {
    let test = |s, m, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let m = Integer::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let q = (&p).balanced_mod(&m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).balanced_mod(m.clone()), q);
        assert_eq!(p.clone().balanced_mod(&m), q);
        assert_eq!(p.clone().balanced_mod(m.clone()), q);
        let mut r = p.clone();
        r.balanced_mod_assign(&m);
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p;
        r.balanced_mod_assign(m);
        assert_eq!(r, q);
    };
    test("0", "1", "0");
    test("0", "-7", "0");
    // Modulo 1 or -1 every coefficient is zero, so the whole polynomial is.
    test("x^2+27*x-23", "1", "0");
    test("x^2+27*x-23", "-1", "0");
    // Each coefficient goes to the representative closest to zero.
    test("x^2+27*x-23", "10", "x^2-3*x-3");
    test("x^2+27*x-23", "-10", "x^2-3*x-3");
    // Half the modulus is positive, whichever side it comes from.
    test("5*x-5", "10", "5*x+5");
    test("x^2+x+1", "2", "x^2+x+1");
    test("-x^2-x-1", "2", "x^2+x+1");
    // With an odd modulus there is no tie.
    test("2*x-2", "5", "2*x-2");
    test("3*x-3", "5", "-2*x+2");
    // Reducing the leading coefficient to zero lowers the degree.
    test("10*x^2+7*x+5", "-10", "-3*x+5");
    test("-6*x^2-3*x-9", "3", "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3-6*x^2+2", "3", "x^3-1");
    // Coefficients and divisor of many limbs.
    test(
        "1000000000000000000000000*x+1",
        "1234567890987",
        "530068894399*x+1",
    );
}

#[test]
#[should_panic]
fn balanced_mod_fail() {
    let _ = IntegerPolynomial::from_str("x")
        .unwrap()
        .balanced_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_ref_fail() {
    let _ = (&IntegerPolynomial::from_str("x").unwrap()).balanced_mod(&Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x").unwrap();
    p.balanced_mod_assign(Integer::ZERO);
}

// The zero polynomial has no coefficients, so a zero divisor is never reached by the
// coefficient-wise loop; only the explicit check makes these panic rather than quietly returning
// zero.
#[test]
#[should_panic]
fn balanced_mod_zero_polynomial_fail() {
    let _ = IntegerPolynomial::ZERO.balanced_mod(Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_ref_zero_polynomial_fail() {
    let _ = (&IntegerPolynomial::ZERO).balanced_mod(&Integer::ZERO);
}

#[test]
#[should_panic]
fn balanced_mod_assign_zero_polynomial_fail() {
    let mut p = IntegerPolynomial::ZERO;
    p.balanced_mod_assign(&Integer::ZERO);
}

#[test]
fn balanced_mod_properties() {
    integer_polynomial_integer_pair_gen_var_1().test_properties(|(p, m)| {
        let q = (&p).balanced_mod(&m);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!((&p).balanced_mod(m.clone()), q);
        assert_eq!(p.clone().balanced_mod(&m), q);
        assert_eq!(p.clone().balanced_mod(m.clone()), q);
        let mut r = p.clone();
        r.balanced_mod_assign(&m);
        assert_eq!(r, q);
        let mut r = p.clone();
        r.balanced_mod_assign(m.clone());
        assert_eq!(r, q);

        // The sign of the modulus makes no difference.
        assert_eq!((&p).balanced_mod(-&m), q);

        // Reducing again changes nothing, and never lengthens the coefficient list.
        assert_eq!((&q).balanced_mod(&m), q);
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // Coefficient by coefficient, this is the `Integer` operation: each result is congruent to
        // its coefficient and lies in (-|m|/2, |m|/2].
        let abs_m = (&m).unsigned_abs();
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let r = q.coefficient(u64::try_from(i).unwrap());
            assert_eq!(*r, c.balanced_mod(&m));
            assert!((r - c).divisible_by(&m));
            let abs_r = r.unsigned_abs();
            // |m| >= 2|r|, and at equality the remainder is the positive one.
            match abs_m.cmp_double(&abs_r) {
                Less => panic!("remainder too large"),
                Equal => assert!(*r > 0u32),
                Greater => {}
            }
        }

        // Reducing the result into [0, |m|) gives the same as reducing the original.
        assert_eq!((&q).mod_op(&abs_m), (&p).mod_op(&abs_m));
    });

    integer_polynomial_gen().test_properties(|p| {
        // Modulo 1 or -1 everything vanishes.
        assert_eq!((&p).balanced_mod(Integer::ONE), IntegerPolynomial::ZERO);
        assert_eq!(
            (&p).balanced_mod(Integer::NEGATIVE_ONE),
            IntegerPolynomial::ZERO
        );
        // Modulo 2 every odd coefficient becomes 1 and every even one 0, which is `mod_op`.
        assert_eq!(
            (&p).balanced_mod(Integer::TWO),
            IntegerPolynomial::from((&p).mod_op(Natural::TWO))
        );
    });
}
