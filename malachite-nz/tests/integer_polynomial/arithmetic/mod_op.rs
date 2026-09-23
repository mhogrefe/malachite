// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    Abs, DivisibleBy, Mod, ModIsReduced, ModPowerOf2, PowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_integer_pair_gen_var_1,
    integer_polynomial_natural_pair_gen_var_1, integer_polynomial_unsigned_pair_gen,
    integer_polynomial_unsigned_polynomial_pair_gen,
};

#[test]
fn test_mod_op() {
    let test = |s, m, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        // All four combinations of value and reference.
        let q = (&p).mod_op(&m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!((&p).mod_op(m.clone()), q);
        assert_eq!(p.clone().mod_op(&m), q);
        assert_eq!(p.mod_op(m), q);
    };
    test("0", "1", "0");
    test("0", "7", "0");
    // Modulo 1 every coefficient is zero, so the whole polynomial is.
    test("x^2-4*x-5", "1", "0");
    // Negative coefficients become non-negative.
    test("x^2-4*x-5", "3", "x^2+2*x+1");
    test("-1", "10", "9");
    test("-x", "18446744073709551616", "18446744073709551615*x");
    // Non-negative polynomials behave as NaturalPolynomials do.
    test("x^2+4*x+5", "3", "x^2+x+2");
    // A negative multiple of the modulus becomes zero, not the modulus.
    test("-6*x+1", "3", "1");
    // Reducing the leading coefficient to zero lowers the degree, whatever its sign.
    test("-6*x^2+3*x-1", "3", "2");
    test("-6*x^2-3*x-9", "3", "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3-6*x^2+2", "3", "x^3+2");
    // Coefficients and divisor of many limbs.
    test(
        "-1000000000000000000000000*x+1",
        "1234567890987",
        "704498996588*x+1",
    );
    // A divisor larger than every coefficient leaves the non-negative ones alone.
    test(
        "x^2-4*x+5",
        "1000000000000000000000000",
        "x^2+999999999999999999999996*x+5",
    );
}

#[test]
#[should_panic]
fn mod_op_fail() {
    let _ = IntegerPolynomial::from_str("x")
        .unwrap()
        .mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_val_ref_fail() {
    let _ = IntegerPolynomial::from_str("x")
        .unwrap()
        .mod_op(&Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_val_fail() {
    let _ = (&IntegerPolynomial::from_str("x").unwrap()).mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_ref_fail() {
    let _ = (&IntegerPolynomial::from_str("x").unwrap()).mod_op(&Natural::ZERO);
}

// The zero polynomial has no coefficients, so a zero divisor is never reached by the
// coefficient-wise loop; only the explicit check makes these panic rather than quietly returning
// zero.
#[test]
#[should_panic]
fn mod_op_zero_polynomial_fail() {
    let _ = IntegerPolynomial::ZERO.mod_op(Natural::ZERO);
}

#[test]
#[should_panic]
fn mod_op_ref_zero_polynomial_fail() {
    let _ = (&IntegerPolynomial::ZERO).mod_op(&Natural::ZERO);
}

#[test]
fn mod_op_properties() {
    integer_polynomial_natural_pair_gen_var_1().test_properties(|(p, m)| {
        let q = (&p).mod_op(&m);
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!((&p).mod_op(m.clone()), q);
        assert_eq!(p.clone().mod_op(&m), q);
        assert_eq!(p.clone().mod_op(m.clone()), q);

        // The result is reduced, and reducing it again changes nothing.
        assert!(q.mod_is_reduced(&m));
        assert_eq!(&q % &m, q);

        // Reducing never lengthens the coefficient list.
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // Coefficient by coefficient, this is the `Integer` operation, and each result is congruent
        // to the original coefficient.
        let m_i = Integer::from(&m);
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let r = q.coefficient(u64::try_from(i).unwrap());
            assert_eq!(Integer::from(r), c.mod_op(&m_i));
            assert!((Integer::from(r) - c).divisible_by(&m_i));
        }

        // On a polynomial with no negative coefficients, this is the `NaturalPolynomial` operation.
        if let Ok(n) = NaturalPolynomial::try_from(&p) {
            assert_eq!(n % &m, q);
        }
    });

    integer_polynomial_gen().test_properties(|p| {
        // Modulo 1 everything vanishes.
        assert_eq!((&p).mod_op(Natural::ONE), NaturalPolynomial::ZERO);
        // For a power-of-2 divisor this is `mod_power_of_2`.
        for pow in [1, 7, 64, 100] {
            assert_eq!(
                (&p).mod_op(Natural::power_of_2(pow)),
                (&p).mod_power_of_2(pow)
            );
        }
    });
}

#[test]
fn test_mod_op_unsigned() {
    fn test<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(s: &str, m: T, out: &str)
    where
        Natural: From<T>,
    {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q: UnsignedPolynomial<T> = (&p).mod_op(m);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.mod_op(m), q);
    }
    test("0", 1u8, "0");
    test("0", 7u64, "0");
    test("x^2-4*x-5", 3u32, "x^2+2*x+1");
    test("x^2-4*x-5", 1u16, "0");
    // Negative coefficients become non-negative, and stay within the modulus type.
    test("-1", 255u8, "254");
    test("-1", u64::MAX, "18446744073709551614");
    test("-x", u128::MAX, "340282366920938463463374607431768211454*x");
    // Coefficients far larger than the modulus type are reduced into it.
    test("-1000000000001*x^2+2000000000003*x-5", 7u8, "5*x^2+5*x+2");
    test(
        "-100000000000000000000*x+1",
        u64::MAX,
        "10680464442257309690*x+1",
    );
    // Reducing the leading coefficient to zero lowers the degree.
    test("-1024*x^2-3", 4usize, "1");
    test("-4294967296*x^2+4294967296", 65536u32, "0");
}

#[test]
#[should_panic]
fn mod_op_unsigned_fail() {
    let _: UnsignedPolynomial<u8> = IntegerPolynomial::from_str("x").unwrap().mod_op(0u8);
}

#[test]
#[should_panic]
fn mod_op_unsigned_ref_fail() {
    let _: UnsignedPolynomial<u64> = (&IntegerPolynomial::from_str("-x").unwrap()).mod_op(0u64);
}

#[test]
#[should_panic]
fn mod_op_unsigned_zero_polynomial_fail() {
    let _: UnsignedPolynomial<u32> = (&IntegerPolynomial::ZERO).mod_op(0u32);
}

fn mod_op_unsigned_properties_helper<
    T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural> + for<'a> TryFrom<&'a Natural>,
>()
where
    Natural: From<T> + PartialEq<T>,
    Integer: From<T>,
{
    integer_polynomial_unsigned_pair_gen::<T>().test_properties(|(p, m)| {
        if m == T::ZERO {
            return;
        }
        let q: UnsignedPolynomial<T> = (&p).mod_op(m);
        assert!(q.is_valid());
        assert_eq!(p.clone().mod_op(m), q);

        // The result is reduced, and reducing it again changes nothing.
        assert!(q.mod_is_reduced(&m));
        assert_eq!(&q % m, q);

        // Apart from its type, the result is the reduction modulo the Natural with the same value.
        let r = (&p).mod_op(Natural::from(m));
        assert!(r == q);
        assert_eq!(UnsignedPolynomial::<T>::try_from(&r), Ok(q.clone()));

        // On a polynomial with no negative coefficients, this is the `NaturalPolynomial` operation.
        if let Ok(n) = NaturalPolynomial::try_from(&p) {
            assert_eq!(n % m, q);
        }
    });

    integer_polynomial_unsigned_polynomial_pair_gen::<T>().test_properties(|(_, q)| {
        // On a polynomial that both types can hold, this is the `UnsignedPolynomial` operation.
        for m in [T::ONE, T::TWO, T::MAX] {
            assert_eq!(IntegerPolynomial::from(q.clone()).mod_op(m), &q % m);
        }
    });
}

#[test]
fn mod_op_unsigned_properties() {
    apply_fn_to_unsigneds!(mod_op_unsigned_properties_helper);
}

#[test]
fn test_rem() {
    let test = |s, m, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let m = Integer::from_str(m).unwrap();
        // All four combinations of value and reference, and in place with both.
        let q = &p % &m;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(&p % m.clone(), q);
        assert_eq!(p.clone() % &m, q);
        assert_eq!(p.clone() % m.clone(), q);
        let mut r = p.clone();
        r %= &m;
        assert!(r.is_valid());
        assert_eq!(r, q);
        let mut r = p;
        r %= m;
        assert_eq!(r, q);
    };
    test("0", "1", "0");
    test("0", "-7", "0");
    // Modulo 1 or -1 every coefficient is zero, so the whole polynomial is.
    test("x^2-4*x-5", "1", "0");
    test("x^2-4*x-5", "-1", "0");
    // Each remainder keeps its coefficient's sign, and the modulus's sign makes no difference.
    test("x^2-4*x-5", "3", "x^2-x-2");
    test("x^2-4*x-5", "-3", "x^2-x-2");
    test("-1", "10", "-1");
    test("-11*x+11", "10", "-x+1");
    // Non-negative polynomials behave as NaturalPolynomials do.
    test("x^2+4*x+5", "3", "x^2+x+2");
    // Reducing the leading coefficient to zero lowers the degree, whatever its sign.
    test("-6*x^2+3*x-1", "3", "-1");
    test("-6*x^2-3*x-9", "-3", "0");
    // Only the leading ones are dropped; an interior zero stays.
    test("x^3-6*x^2-2", "3", "x^3-2");
    // Coefficients and divisor of many limbs.
    test(
        "-1000000000000000000000000*x+1",
        "1234567890987",
        "-530068894399*x+1",
    );
    // A divisor larger than every coefficient leaves the polynomial alone.
    test("x^2-4*x+5", "-1000000000000000000000000", "x^2-4*x+5");
}

#[test]
#[should_panic]
fn rem_fail() {
    let _ = IntegerPolynomial::from_str("x").unwrap() % Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_val_ref_fail() {
    let _ = IntegerPolynomial::from_str("x").unwrap() % &Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_val_fail() {
    let _ = &IntegerPolynomial::from_str("x").unwrap() % Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_ref_fail() {
    let _ = &IntegerPolynomial::from_str("x").unwrap() % &Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_fail() {
    let mut p = IntegerPolynomial::from_str("x").unwrap();
    p %= Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_ref_fail() {
    let mut p = IntegerPolynomial::from_str("x").unwrap();
    p %= &Integer::ZERO;
}

// The zero polynomial has no coefficients, so a zero divisor is never reached by the
// coefficient-wise loop; only the explicit check makes these panic rather than quietly returning
// zero.
#[test]
#[should_panic]
fn rem_zero_polynomial_fail() {
    let _ = IntegerPolynomial::ZERO % Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_ref_zero_polynomial_fail() {
    let _ = &IntegerPolynomial::ZERO % &Integer::ZERO;
}

#[test]
#[should_panic]
fn rem_assign_zero_polynomial_fail() {
    let mut p = IntegerPolynomial::ZERO;
    p %= &Integer::ZERO;
}

#[test]
fn rem_properties() {
    integer_polynomial_integer_pair_gen_var_1().test_properties(|(p, m)| {
        let q = &p % &m;
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(&p % m.clone(), q);
        assert_eq!(p.clone() % &m, q);
        assert_eq!(p.clone() % m.clone(), q);
        let mut r = p.clone();
        r %= &m;
        assert_eq!(r, q);
        let mut r = p.clone();
        r %= m.clone();
        assert_eq!(r, q);

        // The sign of the modulus makes no difference.
        assert_eq!(&p % (-&m), q);

        // Reducing again changes nothing, and never lengthens the coefficient list.
        assert_eq!(&q % &m, q);
        assert!(q.coefficients_asc().len() <= p.coefficients_asc().len());

        // Coefficient by coefficient, this is the `Integer` operation: each remainder is smaller
        // than the modulus in absolute value, has its coefficient's sign unless it is zero, and
        // with the truncated quotient gives the coefficient back exactly.
        let abs_m = (&m).abs();
        for (i, c) in p.coefficients_asc().iter().enumerate() {
            let r = q.coefficient(u64::try_from(i).unwrap());
            assert_eq!(*r, c % &m);
            assert!(r.unsigned_abs_ref() < abs_m.unsigned_abs_ref());
            assert!(*r == 0u32 || (*r > 0u32) == (*c > 0u32));
            assert_eq!(*c, &m * (c / &m) + r);
        }

        // `mod_op` and `%` agree up to a multiple of the modulus: reducing the remainder into [0,
        // |m|) gives the same result as reducing the original polynomial.
        let abs_m = m.unsigned_abs();
        assert_eq!((&q).mod_op(&abs_m), (&p).mod_op(&abs_m));

        // On a polynomial with no negative coefficients, this is the `NaturalPolynomial` operation.
        if let Ok(n) = NaturalPolynomial::try_from(&p) {
            assert_eq!(IntegerPolynomial::from(n % &abs_m), q);
        }
    });

    integer_polynomial_gen().test_properties(|p| {
        // Modulo 1 or -1 everything vanishes.
        assert_eq!(&p % Integer::ONE, IntegerPolynomial::ZERO);
        assert_eq!(&p % Integer::NEGATIVE_ONE, IntegerPolynomial::ZERO);
    });
}
