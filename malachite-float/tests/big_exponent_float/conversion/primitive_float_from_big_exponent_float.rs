// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::float::conversion::primitive_float_from_float::FloatFromFloatError;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;
use malachite_float::{BigExponentFloat, Float};
use std::cmp::Ordering::{self, *};

const ROUNDING_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_primitive_float_from_big_exponent_float() {
    let test = |x: &str, x_hex: &str, rm: RoundingMode, out: f64, o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        let (y, o_alt) = f64::rounding_from(x, rm);
        assert_eq!(NiceFloat(y), NiceFloat(out));
        assert_eq!(o_alt, o);
    };
    test(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        Nearest,
        f64::INFINITY,
        Greater,
    );
    test(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        Floor,
        f64::MAX,
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Nearest,
        0.0,
        Less,
    );
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Up,
        f64::MIN_POSITIVE_SUBNORMAL,
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Nearest,
        -0.0,
        Greater,
    );

    let test_f32 = |x: &str, x_hex: &str, rm: RoundingMode, out: f32, o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        let (y, o_alt) = f32::rounding_from(x, rm);
        assert_eq!(NiceFloat(y), NiceFloat(out));
        assert_eq!(o_alt, o);
    };
    test_f32(
        "-8.1e330985980541",
        "-0x1.0E+274877906944",
        Ceiling,
        -f32::MAX,
        Greater,
    );

    let test_try_from = |x: &str, x_hex: &str, out: Result<f64, FloatFromFloatError>| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(f64::try_from(x).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f64::convertible_from(x), out.is_ok());
    };
    test_try_from(
        "8.1e330985980541",
        "0x1.0E+274877906944",
        Err(FloatFromFloatError::Overflow),
    );
    test_try_from(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Err(FloatFromFloatError::Underflow),
    );
    test_try_from(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Err(FloatFromFloatError::Underflow),
    );
}

fn primitive_float_from_big_exponent_float_properties_helper<T: PrimitiveFloat>()
where
    for<'a> T: RoundingFrom<&'a BigExponentFloat>
        + RoundingFrom<BigExponentFloat>
        + RoundingFrom<&'a Float>
        + TryFrom<&'a BigExponentFloat, Error = FloatFromFloatError>
        + TryFrom<&'a Float, Error = FloatFromFloatError>
        + ConvertibleFrom<&'a BigExponentFloat>
        + ConvertibleFrom<&'a Float>,
{
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(
            T::try_from(&x).map(NiceFloat),
            T::try_from(&f).map(NiceFloat)
        );
        assert_eq!(T::convertible_from(&x), T::convertible_from(&f));
        for rm in ROUNDING_MODES {
            let (y, o) = T::rounding_from(&x, rm);
            let (z, o_z) = T::rounding_from(&f, rm);
            assert_eq!(NiceFloat(y), NiceFloat(z));
            assert_eq!(o, o_z);
            let (y_alt, o_alt) = T::rounding_from(x.clone(), rm);
            assert_eq!(NiceFloat(y_alt), NiceFloat(y));
            assert_eq!(o_alt, o);
        }
    });
}

#[test]
fn primitive_float_from_big_exponent_float_properties() {
    apply_fn_to_primitive_floats!(primitive_float_from_big_exponent_float_properties_helper);
}
