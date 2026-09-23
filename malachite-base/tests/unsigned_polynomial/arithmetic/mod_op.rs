// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Height, Mod, ModAssign, ModIsReduced, ModPowerOf2, PowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{
    unsigned_polynomial_gen, unsigned_polynomial_unsigned_pair_gen_var_1,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_rem() {
    let test = |s, m, out| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        // by reference
        let q = &p % m;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        // by value
        let q = p.clone() % m;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        // in place
        let mut q = p;
        q %= m;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
    };
    test("0", 1, "0");
    test("0", 7, "0");
    // Every coefficient is taken modulo the divisor.
    test("x^2+4*x+5", 3, "x^2+x+2");
    test("x^2+4*x+5", 7, "x^2+4*x+5");
    // Modulo 1 every coefficient is zero, so the whole polynomial is.
    test("x^2+4*x+5", 1, "0");
    test("18446744073709551615*x+1", 10, "5*x+1");
    // A divisor larger than every coefficient leaves the polynomial alone.
    test("x^2+4*x+5", 18446744073709551615, "x^2+4*x+5");
}

#[test]
fn test_mod_op() {
    // `mod_op` is `%` under the name the mod-family traits use; a `UnsignedPolynomial`'s
    // coefficients are never negative, so there is no case where the two could differ.
    let test = |s, m, out| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        let q = (&p).mod_op(m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        let q = p.clone().mod_op(m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        let mut q = p;
        q.mod_assign(m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
    };
    test("0", 1, "0");
    test("x^2+4*x+5", 3, "x^2+x+2");
    test("x^2+4*x+5", 1, "0");
    test("4*x^2+3", 4, "3");
    test("18446744073709551615*x+1", 10, "5*x+1");
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = UnsignedPolynomial::<u64>::from_str("x").unwrap().mod_op(0);
}

#[test]
#[should_panic]
fn mod_op_ref_fail() {
    let _ = (&UnsignedPolynomial::<u64>::from_str("x").unwrap()).mod_op(0);
}

#[test]
#[should_panic]
fn mod_assign_fail() {
    let mut p = UnsignedPolynomial::<u64>::from_str("x").unwrap();
    p.mod_assign(0);
}

#[test]
fn test_rem_lowers_the_degree() {
    // A leading coefficient that is a multiple of the divisor becomes zero, and a polynomial does
    // not hold trailing zero coefficients, so the degree falls.
    let test = |s, m, out| {
        assert_eq!(
            (UnsignedPolynomial::<u64>::from_str(s).unwrap() % m).to_string(),
            out
        );
    };
    test("4*x^2+3", 4, "3");
    test("6*x^2+3*x+2", 3, "2");
    // Every coefficient can vanish at once, leaving the zero polynomial.
    test("6*x^2+3*x+9", 3, "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3+6*x^2+2", 3, "x^3+2");
    // The degree can fall by more than one.
    test("5*x^5+10*x^4+15*x^3+1", 5, "1");
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = UnsignedPolynomial::<u64>::from_str("x").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_ref_fail() {
    let _ = &UnsignedPolynomial::<u64>::from_str("x").unwrap() % 0;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut p = UnsignedPolynomial::<u64>::from_str("x").unwrap();
    p %= 0;
}

// The zero polynomial has no coefficients, so a zero divisor is never reached by the
// coefficient-wise loop; only the explicit check makes these panic rather than quietly returning
// zero.
#[test]
#[should_panic]
fn rem_zero_polynomial_fail() {
    let _ = UnsignedPolynomial::<u64>::ZERO % 0;
}

#[test]
#[should_panic]
fn rem_ref_zero_polynomial_fail() {
    let _ = &UnsignedPolynomial::<u64>::ZERO % 0;
}

#[test]
#[should_panic]
fn rem_assign_zero_polynomial_fail() {
    let mut p = UnsignedPolynomial::<u64>::ZERO;
    p %= 0;
}

// Reducing modulo 1 is the degenerate case the lint warns about, and asserting that it gives the
// zero polynomial is the point.
#[test]
#[allow(clippy::modulo_one)]
fn rem_properties() {
    unsigned_polynomial_unsigned_pair_gen_var_1().test_properties(|(p, m)| {
        // The generator's second value can be zero, which is not a divisor.
        let m = m + 1;
        let q = &p % m;
        assert!(q.is_valid());
        // The three forms agree.
        assert_eq!(p.clone() % m, q);
        let mut r = p.clone();
        r %= m;
        assert_eq!(r, q);

        // The result is reduced, which is the whole point, and reducing it again changes nothing.
        assert!(q.mod_is_reduced(&m));
        assert_eq!(&q % m, q);

        // Reducing never raises the degree, and never lengthens the coefficient list.
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // Coefficient by coefficient, this is the `u64` operation.
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            assert_eq!(q.coefficient(u64::try_from(i).unwrap()), c % m);
        }

        // A polynomial that is already reduced is left alone.
        assert_eq!(p.mod_is_reduced(&m), p == q);

        // `mod_op` is the same operation, in all three forms.
        assert_eq!((&p).mod_op(m), q);
        assert_eq!(p.clone().mod_op(m), q);
        let mut r = p.clone();
        r.mod_assign(m);
        assert_eq!(r, q);

        // This really is the remainder of a division: with the coefficient-wise quotient, `p` is
        // recovered exactly, coefficient by coefficient. That is what makes it the remainder of
        // dividing by the constant polynomial `m` rather than merely a reduction.
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let i = u64::try_from(i).unwrap();
            assert_eq!(*c, m * (c / m) + q.coefficient(i));
        }
    });

    unsigned_polynomial_gen().test_properties(|p| {
        // Modulo 1 everything vanishes.
        assert_eq!(&p % 1, UnsignedPolynomial::<u64>::ZERO);
        // A divisor above the height leaves the polynomial alone.
        if let Some(above) = p.to_height().checked_add(1) {
            assert_eq!(&p % above, p);
        }
        // For a power-of-2 divisor this is `mod_power_of_2`, which reaches the same answer by
        // masking rather than dividing.
        for pow in 1..8 {
            assert_eq!(&p % u64::power_of_2(pow), (&p).mod_power_of_2(pow));
        }
    });
}
