// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, big_exponent_float_to_float,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use std::cmp::Ordering::*;
use std::panic::catch_unwind;

const ROUNDING_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_primitive_int_from_big_exponent_float() {
    let big = big_exponent_float_power_of_2(Integer::from(1u64 << 40));
    assert_eq!(u8::rounding_from(&big, Floor), (u8::MAX, Less));
    assert_eq!(
        u64::try_from(&big),
        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
    );
    let neg_big = big_exponent_float_negate(big);
    assert_eq!(i32::rounding_from(&neg_big, Ceiling), (i32::MIN, Greater));
    assert_eq!(
        i32::try_from(&neg_big),
        Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
    );
    let tiny = big_exponent_float_power_of_2(-Integer::from(1u64 << 40));
    assert_eq!(u8::rounding_from(&tiny, Nearest), (0, Less));
    assert_eq!(u8::rounding_from(&tiny, Up), (1, Greater));
    assert_eq!(
        i8::rounding_from(&big_exponent_float_negate(tiny), Floor),
        (-1, Less)
    );
}

fn unsigned_properties_helper<T: PrimitiveUnsigned>()
where
    for<'a> T: RoundingFrom<&'a BigExponentFloat>
        + RoundingFrom<&'a Float>
        + TryFrom<&'a BigExponentFloat, Error = UnsignedFromFloatError>
        + TryFrom<&'a Float, Error = UnsignedFromFloatError>
        + ConvertibleFrom<&'a BigExponentFloat>
        + ConvertibleFrom<&'a Float>,
{
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(T::try_from(&x), T::try_from(&f));
        assert_eq!(T::convertible_from(&x), T::convertible_from(&f));
        for rm in ROUNDING_MODES {
            // - both panic, or both agree
            let ours = catch_unwind(|| T::rounding_from(&x, rm));
            let theirs = catch_unwind(|| T::rounding_from(&f, rm));
            assert_eq!(ours.ok(), theirs.ok());
        }
    });
}

fn signed_properties_helper<T: PrimitiveSigned>()
where
    for<'a> T: RoundingFrom<&'a BigExponentFloat>
        + RoundingFrom<&'a Float>
        + TryFrom<&'a BigExponentFloat, Error = SignedFromFloatError>
        + TryFrom<&'a Float, Error = SignedFromFloatError>
        + ConvertibleFrom<&'a BigExponentFloat>
        + ConvertibleFrom<&'a Float>,
{
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(T::try_from(&x), T::try_from(&f));
        assert_eq!(T::convertible_from(&x), T::convertible_from(&f));
        for rm in ROUNDING_MODES {
            let ours = catch_unwind(|| T::rounding_from(&x, rm));
            let theirs = catch_unwind(|| T::rounding_from(&f, rm));
            assert_eq!(ours.ok(), theirs.ok());
        }
    });
}

#[test]
fn primitive_int_from_big_exponent_float_properties() {
    apply_fn_to_unsigneds!(unsigned_properties_helper);
    apply_fn_to_signeds!(signed_properties_helper);
}
