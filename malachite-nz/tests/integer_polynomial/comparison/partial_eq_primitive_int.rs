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
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_polynomial_gen, integer_polynomial_signed_pair_gen,
    integer_polynomial_unsigned_pair_gen,
};

fn test_helper<T: PrimitiveInt + PartialEq<IntegerPolynomial>>(s: &str, c: T, out: bool)
where
    IntegerPolynomial: PartialEq<T>,
{
    let p = IntegerPolynomial::from_str(s).unwrap();
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
    test_helper("1", 1i8, true);
    test_helper("1", -1isize, false);
    test_helper("-1", -1i32, true);
    test_helper("-1", 1i32, false);
    // -1 is not u64::MAX, and nothing negative equals an unsigned value.
    test_helper("-1", u64::MAX, false);
    test_helper("-1", u8::MAX, false);
    test_helper("123", 123u8, true);
    test_helper("123", 123i8, true);
    test_helper("-123", -123i8, true);
    test_helper("-123", 123u8, false);
    test_helper("-128", i8::MIN, true);
    test_helper("128", i8::MIN, false);
    test_helper("255", u8::MAX, true);
    // A constant too large for the type equals none of its values, including the one it would wrap
    // to.
    test_helper("256", 0u8, false);
    test_helper("-129", 127i8, false);
    test_helper("18446744073709551615", u64::MAX, true);
    test_helper("-9223372036854775808", i64::MIN, true);
    test_helper("-9223372036854775809", i64::MAX, false);
    test_helper("-170141183460469231731687303715884105728", i128::MIN, true);
    test_helper("340282366920938463463374607431768211455", u128::MAX, true);
    // No polynomial of positive degree equals a value, whatever its constant term.
    test_helper("x", 0u32, false);
    test_helper("x", 0i32, false);
    test_helper("x-1", -1i64, false);
    test_helper("-2*x^2+3", 3u64, false);
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_unsigned_properties_helper<T>()
where
    T: PrimitiveUnsigned + PartialEq<IntegerPolynomial>,
    IntegerPolynomial: PartialEq<T>,
    Integer: From<T>,
{
    integer_polynomial_unsigned_pair_gen::<T>().test_properties(|(p, c)| {
        let eq = p == c;
        assert_eq!(c == p, eq);
        // Extra refs for type inference: with `IntegerPolynomial: PartialEq<T>` in scope, `p == q`
        // would look for that impl.
        assert_eq!(&p == &IntegerPolynomial::from(Integer::from(c)), eq);
        if eq {
            assert!(p.degree().is_none_or(|d| d == 0));
        }
    });

    unsigned_gen::<T>().test_properties(|c| {
        let p = IntegerPolynomial::from(Integer::from(c));
        assert!(p == c);
        assert!(c == p);
        assert_eq!(IntegerPolynomial::ZERO == c, c == T::ZERO);
        assert_eq!(IntegerPolynomial::one() == c, c == T::ONE);
        assert!(IntegerPolynomial::negative_one() != c);
        assert!(IntegerPolynomial::x() != c);
    });
}

// Comparing with a converted value is the reference the direct comparison is checked against.
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_signed_properties_helper<T>()
where
    T: PrimitiveSigned + PartialEq<IntegerPolynomial>,
    IntegerPolynomial: PartialEq<T>,
    Integer: From<T>,
{
    integer_polynomial_signed_pair_gen::<T>().test_properties(|(p, c)| {
        let eq = p == c;
        assert_eq!(c == p, eq);
        assert_eq!(&p == &IntegerPolynomial::from(Integer::from(c)), eq);
        if eq {
            assert!(p.degree().is_none_or(|d| d == 0));
        }
    });

    signed_gen::<T>().test_properties(|c| {
        let p = IntegerPolynomial::from(Integer::from(c));
        assert!(p == c);
        assert!(c == p);
        assert_eq!(IntegerPolynomial::ZERO == c, c == T::ZERO);
        assert_eq!(IntegerPolynomial::one() == c, c == T::ONE);
        assert_eq!(IntegerPolynomial::negative_one() == c, c == T::NEGATIVE_ONE);
        assert!(IntegerPolynomial::x() != c);
    });
}

#[test]
fn partial_eq_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_eq_unsigned_properties_helper);
    apply_fn_to_signeds!(partial_eq_signed_properties_helper);

    integer_polynomial_gen().test_properties(|p| {
        // The comparisons that stand in for FLINT's `is_zero` and `is_one`.
        assert_eq!(p == 0u32, p == IntegerPolynomial::ZERO);
        assert_eq!(p == 1u32, p == IntegerPolynomial::one());
        assert_eq!(p == -1i32, p == IntegerPolynomial::negative_one());
        // Signed and unsigned comparisons agree on non-negative values.
        assert_eq!(p == 5u64, p == 5i64);
    });
}
