// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::{Shl, ShlAssign};
use core::str::FromStr;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{Evaluate, Polynomial};
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_signed_pair_gen_var_1,
    rational_polynomial_unsigned_pair_gen_var_3, rational_signed_pair_gen_var_1,
    rational_unsigned_pair_gen_var_1,
};
use malachite_q::test_util::rational_polynomial::arithmetic::shl::shl_naive;

fn test_shl_unsigned_helper<T: PrimitiveUnsigned>()
where
    RationalPolynomial: Shl<T, Output = RationalPolynomial> + ShlAssign<T>,
    for<'a> &'a RationalPolynomial: Shl<T, Output = RationalPolynomial>,
    Rational: Shl<T, Output = Rational>,
{
    let test = |s, bits: u8, out| {
        let bits = T::from(bits);
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = &p << bits;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone() << bits, q);
        let mut r = p.clone();
        r <<= bits;
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(shl_naive(&p, bits), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, "0");
    test("0", 10, "0");
    // Shifting by 0 changes nothing.
    test("1/3*x^2-3/4*x+5", 0, "1/3*x^2-3/4*x+5");
    // Factors of 2 in the denominator are cancelled first.
    test("1/3*x^2-3/4*x+5", 2, "4/3*x^2-3*x+20");
    test("1/8*x+1/2", 2, "1/2*x+2");
    test("1/8*x+1/2", 3, "x+4");
    test("1/8*x+1/2", 5, "4*x+16");
    test("-x", 1, "-2*x");
    test(
        "1/3*x^2-3/4*x+5",
        100,
        "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
        6338253001141147007483516026880",
    );
}

fn test_shl_signed_helper<T: PrimitiveSigned>()
where
    RationalPolynomial: Shl<T, Output = RationalPolynomial> + ShlAssign<T>,
    for<'a> &'a RationalPolynomial: Shl<T, Output = RationalPolynomial>,
    Rational: Shl<T, Output = Rational>,
{
    let test = |s, bits: i8, out| {
        let bits = T::from(bits);
        let p = RationalPolynomial::from_str(s).unwrap();
        let q = &p << bits;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), out);
        assert_eq!(p.clone() << bits, q);
        let mut r = p.clone();
        r <<= bits;
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(shl_naive(&p, bits), q);
    };
    // The zero polynomial stays zero.
    test("0", 0, "0");
    test("0", 10, "0");
    // Shifting by 0 changes nothing.
    test("1/3*x^2-3/4*x+5", 0, "1/3*x^2-3/4*x+5");
    // Factors of 2 in the denominator are cancelled first.
    test("1/3*x^2-3/4*x+5", 2, "4/3*x^2-3*x+20");
    test("1/8*x+1/2", 2, "1/2*x+2");
    test("1/8*x+1/2", 3, "x+4");
    test("1/8*x+1/2", 5, "4*x+16");
    test("-x", 1, "-2*x");
    test(
        "1/3*x^2-3/4*x+5",
        100,
        "1267650600228229401496703205376/3*x^2-950737950171172051122527404032*x+\
        6338253001141147007483516026880",
    );
    // A negative shift divides; factors of 2 shared by the numerator's coefficients are cancelled
    // first.
    test("1/3*x^2-3/4*x+5", -2, "1/12*x^2-3/16*x+5/4");
    test("4*x+16", -2, "x+4");
    test("4*x+16", -3, "1/2*x+2");
    test("4*x+16", -5, "1/8*x+1/2");
    test("6*x-2", -1, "3*x-1");
    test("6*x-2", -2, "3/2*x-1/2");
    test("-x", -1, "-1/2*x");
    test(
        "1/3*x^2-3/4*x+5",
        -100,
        "1/3802951800684688204490109616128*x^2-3/5070602400912917605986812821504*x+\
        5/1267650600228229401496703205376",
    );
}

#[test]
fn test_shl() {
    apply_fn_to_unsigneds!(test_shl_unsigned_helper);
    apply_fn_to_signeds!(test_shl_signed_helper);
}

fn shl_unsigned_properties_helper<T: PrimitiveUnsigned>()
where
    RationalPolynomial: Shl<T, Output = RationalPolynomial> + ShlAssign<T>,
    for<'a> &'a RationalPolynomial: Shl<T, Output = RationalPolynomial>,
    Rational: Shl<T, Output = Rational>,
    u64: ExactFrom<T>,
{
    rational_polynomial_unsigned_pair_gen_var_3::<T>().test_properties(|(p, bits)| {
        let q = &p << bits;
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone() << bits, q);
        let mut r = p.clone();
        r <<= bits;
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(shl_naive(&p, bits), q);

        // Every coefficient is shifted, and the degree is unchanged.
        assert_eq!(q.degree(), p.degree());
        for i in 0..p.len() {
            assert_eq!(q.coefficient(i), p.coefficient(i) << bits);
        }
        // Shifting by the same amount as a u64 gives the same result.
        assert_eq!(
            <&RationalPolynomial as Shl<u64>>::shl(&p, u64::exact_from(bits)),
            q
        );
        // Evaluation: q(-1/3) = p(-1/3) << bits.
        let x = Rational::from_signeds(-1i32, 3);
        assert_eq!((&q).evaluate(&x), (&p).evaluate(&x) << bits);
        // It commutes with negation.
        assert_eq!(-&p << bits, -&q);
    });

    rational_polynomial_gen().test_properties(|p| {
        // Shifting by 0 changes nothing.
        assert_eq!(&p << T::ZERO, p);
    });

    rational_unsigned_pair_gen_var_1::<T>().test_properties(|(x, bits)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            RationalPolynomial::from_coefficients_asc(vec![x.clone()]) << bits,
            RationalPolynomial::from_coefficients_asc(vec![x << bits])
        );
    });
}

fn shl_signed_properties_helper<T: PrimitiveSigned>()
where
    RationalPolynomial: Shl<T, Output = RationalPolynomial> + ShlAssign<T>,
    for<'a> &'a RationalPolynomial: Shl<T, Output = RationalPolynomial>,
    Rational: Shl<T, Output = Rational>,
    i64: ExactFrom<T>,
{
    rational_polynomial_signed_pair_gen_var_1::<T>().test_properties(|(p, bits)| {
        let q = &p << bits;
        assert!(q.is_valid());
        // The forms agree.
        assert_eq!(p.clone() << bits, q);
        let mut r = p.clone();
        r <<= bits;
        assert!(r.is_valid());
        assert_eq!(r, q);
        assert_eq!(shl_naive(&p, bits), q);

        // Every coefficient is shifted, and the degree is unchanged.
        assert_eq!(q.degree(), p.degree());
        for i in 0..p.len() {
            assert_eq!(q.coefficient(i), p.coefficient(i) << bits);
        }
        // Shifting by the same amount as an i64 gives the same result.
        assert_eq!(
            <&RationalPolynomial as Shl<i64>>::shl(&p, i64::exact_from(bits)),
            q
        );
        // Shifting back by the opposite amount recovers the polynomial.
        if let Some(neg_bits) = bits.checked_neg() {
            assert_eq!(&q << neg_bits, p);
        }
        // Evaluation: q(-1/3) = p(-1/3) << bits.
        let x = Rational::from_signeds(-1i32, 3);
        assert_eq!((&q).evaluate(&x), (&p).evaluate(&x) << bits);
        // It commutes with negation.
        assert_eq!(-&p << bits, -&q);
    });

    rational_polynomial_gen().test_properties(|p| {
        // Shifting by 0 changes nothing.
        assert_eq!(&p << T::ZERO, p);
    });

    rational_signed_pair_gen_var_1::<T>().test_properties(|(x, bits)| {
        // Shifting a constant polynomial shifts the constant.
        assert_eq!(
            RationalPolynomial::from_coefficients_asc(vec![x.clone()]) << bits,
            RationalPolynomial::from_coefficients_asc(vec![x << bits])
        );
    });
}

#[test]
fn shl_properties() {
    apply_fn_to_unsigneds!(shl_unsigned_properties_helper);
    apply_fn_to_signeds!(shl_signed_properties_helper);
}
