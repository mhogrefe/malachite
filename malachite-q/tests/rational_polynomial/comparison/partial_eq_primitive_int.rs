// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_polynomial_gen, rational_polynomial_signed_pair_gen,
    rational_polynomial_unsigned_pair_gen,
};

fn test_helper<T: PrimitiveInt + PartialEq<RationalPolynomial>>(s: &str, c: T, out: bool)
where
    RationalPolynomial: PartialEq<T>,
{
    let p = RationalPolynomial::from_str(s).unwrap();
    assert_eq!(p == c, out);
    assert_eq!(c == p, out);
}

#[test]
fn test_partial_eq_primitive_int() {
    // The zero polynomial equals 0 and nothing else.
    test_helper("0", 0u8, true);
    test_helper("0", 0i64, true);
    test_helper("0", 1u32, false);
    test_helper("0", -1i32, false);
    test_helper("1", 1u16, true);
    test_helper("1", -1isize, false);
    test_helper("-1", -1i32, true);
    test_helper("-1", u64::MAX, false);
    // A constant that is not an integer equals no value, neither its floor nor its ceiling.
    test_helper("1/2", 0u8, false);
    test_helper("1/2", 1u8, false);
    test_helper("-1/2", -1i8, false);
    test_helper("-1/2", 0i8, false);
    test_helper("4/2", 2u32, true);
    test_helper("123", 123u8, true);
    test_helper("-123", -123i8, true);
    test_helper("-123", 123u8, false);
    test_helper("-128", i8::MIN, true);
    test_helper("256", 0u8, false);
    test_helper("18446744073709551615", u64::MAX, true);
    test_helper("-9223372036854775808", i64::MIN, true);
    test_helper("-170141183460469231731687303715884105728", i128::MIN, true);
    test_helper("340282366920938463463374607431768211455", u128::MAX, true);
    // No polynomial of positive degree equals a value, whatever its constant term.
    test_helper("x", 0u32, false);
    test_helper("1/2*x", 0i32, false);
    test_helper("x-1", -1i64, false);
    test_helper("1/2*x^2+3", 3u64, false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_unsigned_properties_helper<T>()
where
    T: PrimitiveUnsigned + PartialEq<RationalPolynomial>,
    RationalPolynomial: PartialEq<T>,
    IntegerPolynomial: PartialEq<T>,
    Rational: From<T>,
{
    rational_polynomial_unsigned_pair_gen::<T>().test_properties(|(p, c)| {
        let eq = p == c;
        assert_eq!(c == p, eq);
        // Extra refs for type inference: with `RationalPolynomial: PartialEq<T>` in scope, `p == q`
        // would look for that impl.
        assert_eq!(&p == &RationalPolynomial::from(Rational::from(c)), eq);
        assert_eq!(eq, *p.denominator_ref() == 1u32 && *p.numerator_ref() == c);
    });

    unsigned_gen::<T>().test_properties(|c| {
        let p = RationalPolynomial::from(Rational::from(c));
        assert!(p == c);
        assert!(c == p);
        assert_eq!(RationalPolynomial::ZERO == c, c == T::ZERO);
        assert_eq!(RationalPolynomial::one() == c, c == T::ONE);
        assert!(RationalPolynomial::negative_one() != c);
        assert!(RationalPolynomial::one_half() != c);
        assert!(RationalPolynomial::x() != c);
    });
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_signed_properties_helper<T>()
where
    T: PrimitiveSigned + PartialEq<RationalPolynomial>,
    RationalPolynomial: PartialEq<T>,
    IntegerPolynomial: PartialEq<T>,
    Rational: From<T>,
{
    rational_polynomial_signed_pair_gen::<T>().test_properties(|(p, c)| {
        let eq = p == c;
        assert_eq!(c == p, eq);
        assert_eq!(&p == &RationalPolynomial::from(Rational::from(c)), eq);
        assert_eq!(eq, *p.denominator_ref() == 1u32 && *p.numerator_ref() == c);
    });

    signed_gen::<T>().test_properties(|c| {
        let p = RationalPolynomial::from(Rational::from(c));
        assert!(p == c);
        assert!(c == p);
        assert_eq!(RationalPolynomial::ZERO == c, c == T::ZERO);
        assert_eq!(RationalPolynomial::one() == c, c == T::ONE);
        assert_eq!(
            RationalPolynomial::negative_one() == c,
            c == T::NEGATIVE_ONE
        );
        assert!(RationalPolynomial::one_half() != c);
        assert!(RationalPolynomial::x() != c);
    });
}

#[test]
fn partial_eq_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_eq_unsigned_properties_helper);
    apply_fn_to_signeds!(partial_eq_signed_properties_helper);

    rational_polynomial_gen().test_properties(|p| {
        // The comparisons that stand in for FLINT's `is_zero` and `is_one`.
        assert_eq!(p == 0u32, p == RationalPolynomial::ZERO);
        assert_eq!(p == 1u32, p == RationalPolynomial::one());
        assert_eq!(p == -1i32, p == RationalPolynomial::negative_one());
        // Signed and unsigned comparisons agree on non-negative values.
        assert_eq!(p == 5u64, p == 5i64);
    });
}
