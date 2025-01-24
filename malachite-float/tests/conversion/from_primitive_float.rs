// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::{SignificantBits, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_4,
};
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::primitive_float_unsigned_rounding_mode_triple_gen_var_3;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;
use rug::Assign;
use std::cmp::{
    max,
    Ordering::{self, *},
};
use std::panic::catch_unwind;

#[test]
fn test_from_primitive_float() {
    fn test_helper<T: PrimitiveFloat>(u: T, out: &str, out_hex: &str)
    where
        Float: From<T>,
        rug::Float: Assign<T>,
    {
        let x = Float::from(u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let prec = if !u.is_finite() || u == T::ZERO {
            1
        } else {
            let n = u.integer_mantissa();
            u32::exact_from(n.significant_bits() - TrailingZeros::trailing_zeros(n))
        };
        let rug_x = rug::Float::with_val(prec, u);
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    }
    test_helper(f32::NAN, "NaN", "NaN");
    test_helper(f32::INFINITY, "Infinity", "Infinity");
    test_helper(f32::NEGATIVE_INFINITY, "-Infinity", "-Infinity");
    test_helper(f32::ZERO, "0.0", "0x0.0");
    test_helper(f32::NEGATIVE_ZERO, "-0.0", "-0x0.0");

    test_helper(f32::ONE, "1.0", "0x1.0#1");
    test_helper(f32::TWO, "2.0", "0x2.0#1");
    test_helper(f32::ONE_HALF, "0.5", "0x0.8#1");
    test_helper(1.0f32 / 3.0, "0.33333334", "0x0.5555558#24");
    test_helper(std::f32::consts::SQRT_2, "1.4142135", "0x1.6a09e6#24");
    test_helper(std::f32::consts::PI, "3.1415927", "0x3.243f6c#24");
    test_helper(f32::MIN_POSITIVE_SUBNORMAL, "1.0e-45", "0x8.0E-38#1");
    test_helper(f32::MAX_SUBNORMAL, "1.1754942e-38", "0x3.fffff8E-32#23");
    test_helper(f32::MIN_POSITIVE_NORMAL, "1.0e-38", "0x4.0E-32#1");
    test_helper(f32::MAX_FINITE, "3.4028235e38", "0xf.fffffE+31#24");

    test_helper(f32::NEGATIVE_ONE, "-1.0", "-0x1.0#1");
    test_helper(-f32::TWO, "-2.0", "-0x2.0#1");
    test_helper(-f32::ONE_HALF, "-0.5", "-0x0.8#1");
    test_helper(-1.0f32 / 3.0, "-0.33333334", "-0x0.5555558#24");
    test_helper(-std::f32::consts::SQRT_2, "-1.4142135", "-0x1.6a09e6#24");
    test_helper(-std::f32::consts::PI, "-3.1415927", "-0x3.243f6c#24");
    test_helper(-f32::MIN_POSITIVE_SUBNORMAL, "-1.0e-45", "-0x8.0E-38#1");
    test_helper(-f32::MAX_SUBNORMAL, "-1.1754942e-38", "-0x3.fffff8E-32#23");
    test_helper(-f32::MIN_POSITIVE_NORMAL, "-1.0e-38", "-0x4.0E-32#1");
    test_helper(-f32::MAX_FINITE, "-3.4028235e38", "-0xf.fffffE+31#24");

    test_helper(f64::NAN, "NaN", "NaN");
    test_helper(f64::INFINITY, "Infinity", "Infinity");
    test_helper(f64::NEGATIVE_INFINITY, "-Infinity", "-Infinity");
    test_helper(f64::ZERO, "0.0", "0x0.0");
    test_helper(f64::NEGATIVE_ZERO, "-0.0", "-0x0.0");

    test_helper(f64::ONE, "1.0", "0x1.0#1");
    test_helper(f64::TWO, "2.0", "0x2.0#1");
    test_helper(f64::ONE_HALF, "0.5", "0x0.8#1");
    test_helper(1.0f64 / 3.0, "0.33333333333333331", "0x0.55555555555554#53");
    test_helper(
        std::f64::consts::SQRT_2,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
    );
    test_helper(
        std::f64::consts::PI,
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
    );
    test_helper(f64::MIN_POSITIVE_SUBNORMAL, "5.0e-324", "0x4.0E-269#1");
    test_helper(
        f64::MAX_SUBNORMAL,
        "2.2250738585072009e-308",
        "0x3.ffffffffffffcE-256#52",
    );
    test_helper(f64::MIN_POSITIVE_NORMAL, "2.0e-308", "0x4.0E-256#1");
    test_helper(
        f64::MAX_FINITE,
        "1.7976931348623157e308",
        "0xf.ffffffffffff8E+255#53",
    );

    test_helper(f64::NEGATIVE_ONE, "-1.0", "-0x1.0#1");
    test_helper(-f64::TWO, "-2.0", "-0x2.0#1");
    test_helper(-f64::ONE_HALF, "-0.5", "-0x0.8#1");
    test_helper(
        -1.0f64 / 3.0,
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
    );
    test_helper(
        -std::f64::consts::SQRT_2,
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
    );
    test_helper(
        -std::f64::consts::PI,
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
    );
    test_helper(-f64::MIN_POSITIVE_SUBNORMAL, "-5.0e-324", "-0x4.0E-269#1");
    test_helper(
        -f64::MAX_SUBNORMAL,
        "-2.2250738585072009e-308",
        "-0x3.ffffffffffffcE-256#52",
    );
    test_helper(-f64::MIN_POSITIVE_NORMAL, "-2.0e-308", "-0x4.0E-256#1");
    test_helper(
        -f64::MAX_FINITE,
        "-1.7976931348623157e308",
        "-0xf.ffffffffffff8E+255#53",
    );
}

#[test]
fn test_from_primitive_float_prec() {
    fn test_helper<T: PrimitiveFloat>(u: T, prec: u64, out: &str, out_hex: &str, out_o: Ordering)
    where
        Float: From<T>,
        rug::Float: Assign<T>,
    {
        let (x, o) = Float::from_primitive_float_prec(u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(max(1, u32::exact_from(prec)), u);
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    }
    test_helper(f32::NAN, 1, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 10, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 100, "NaN", "NaN", Equal);
    test_helper(f32::INFINITY, 1, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 10, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 100, "Infinity", "Infinity", Equal);
    test_helper(f32::NEGATIVE_INFINITY, 1, "-Infinity", "-Infinity", Equal);
    test_helper(f32::NEGATIVE_INFINITY, 10, "-Infinity", "-Infinity", Equal);
    test_helper(f32::NEGATIVE_INFINITY, 100, "-Infinity", "-Infinity", Equal);
    test_helper(f32::ZERO, 1, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 10, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 100, "0.0", "0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 1, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 10, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 100, "-0.0", "-0x0.0", Equal);

    test_helper(f32::ONE, 1, "1.0", "0x1.0#1", Equal);
    test_helper(f32::ONE, 10, "1.0", "0x1.000#10", Equal);
    test_helper(
        f32::ONE,
        100,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test_helper(1.0f32 / 3.0, 1, "0.2", "0x0.4#1", Less);
    test_helper(1.0f32 / 3.0, 10, "0.3335", "0x0.556#10", Greater);
    test_helper(
        1.0f32 / 3.0,
        100,
        "0.3333333432674407958984375",
        "0x0.55555580000000000000000000#100",
        Equal,
    );

    test_helper(std::f32::consts::PI, 1, "4.0", "0x4.0#1", Greater);
    test_helper(std::f32::consts::PI, 10, "3.141", "0x3.24#10", Less);
    test_helper(
        std::f32::consts::PI,
        100,
        "3.1415927410125732421875",
        "0x3.243f6c0000000000000000000#100",
        Equal,
    );

    test_helper(
        f32::MIN_POSITIVE_SUBNORMAL,
        1,
        "1.0e-45",
        "0x8.0E-38#1",
        Equal,
    );
    test_helper(
        f32::MIN_POSITIVE_SUBNORMAL,
        10,
        "1.401e-45",
        "0x8.00E-38#10",
        Equal,
    );
    test_helper(
        f32::MIN_POSITIVE_SUBNORMAL,
        100,
        "1.40129846432481707092372958329e-45",
        "0x8.000000000000000000000000E-38#100",
        Equal,
    );

    test_helper(f32::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1", Equal);
    test_helper(f32::NEGATIVE_ONE, 10, "-1.0", "-0x1.000#10", Equal);
    test_helper(
        f32::NEGATIVE_ONE,
        100,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );

    test_helper(-1.0f32 / 3.0, 1, "-0.2", "-0x0.4#1", Greater);
    test_helper(-1.0f32 / 3.0, 10, "-0.3335", "-0x0.556#10", Less);
    test_helper(
        -1.0f32 / 3.0,
        100,
        "-0.3333333432674407958984375",
        "-0x0.55555580000000000000000000#100",
        Equal,
    );

    test_helper(-std::f32::consts::PI, 1, "-4.0", "-0x4.0#1", Less);
    test_helper(-std::f32::consts::PI, 10, "-3.141", "-0x3.24#10", Greater);
    test_helper(
        -std::f32::consts::PI,
        100,
        "-3.1415927410125732421875",
        "-0x3.243f6c0000000000000000000#100",
        Equal,
    );

    test_helper(
        -f32::MIN_POSITIVE_SUBNORMAL,
        1,
        "-1.0e-45",
        "-0x8.0E-38#1",
        Equal,
    );
    test_helper(
        -f32::MIN_POSITIVE_SUBNORMAL,
        10,
        "-1.401e-45",
        "-0x8.00E-38#10",
        Equal,
    );
    test_helper(
        -f32::MIN_POSITIVE_SUBNORMAL,
        100,
        "-1.40129846432481707092372958329e-45",
        "-0x8.000000000000000000000000E-38#100",
        Equal,
    );

    test_helper(f64::NAN, 1, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 10, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 100, "NaN", "NaN", Equal);
    test_helper(f64::INFINITY, 1, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 10, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 100, "Infinity", "Infinity", Equal);
    test_helper(f64::NEGATIVE_INFINITY, 1, "-Infinity", "-Infinity", Equal);
    test_helper(f64::NEGATIVE_INFINITY, 10, "-Infinity", "-Infinity", Equal);
    test_helper(f64::NEGATIVE_INFINITY, 100, "-Infinity", "-Infinity", Equal);
    test_helper(f64::ZERO, 1, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 10, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 100, "0.0", "0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 1, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 10, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 100, "-0.0", "-0x0.0", Equal);

    test_helper(f64::ONE, 1, "1.0", "0x1.0#1", Equal);
    test_helper(f64::ONE, 10, "1.0", "0x1.000#10", Equal);
    test_helper(
        f64::ONE,
        100,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test_helper(1.0f64 / 3.0, 1, "0.2", "0x0.4#1", Less);
    test_helper(1.0f64 / 3.0, 10, "0.3335", "0x0.556#10", Greater);
    test_helper(
        1.0f64 / 3.0,
        100,
        "0.3333333333333333148296162562474",
        "0x0.55555555555554000000000000#100",
        Equal,
    );

    test_helper(std::f64::consts::PI, 1, "4.0", "0x4.0#1", Greater);
    test_helper(std::f64::consts::PI, 10, "3.141", "0x3.24#10", Less);
    test_helper(
        std::f64::consts::PI,
        100,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );

    test_helper(
        f64::MIN_POSITIVE_SUBNORMAL,
        1,
        "5.0e-324",
        "0x4.0E-269#1",
        Equal,
    );
    test_helper(
        f64::MIN_POSITIVE_SUBNORMAL,
        10,
        "4.94e-324",
        "0x4.00E-269#10",
        Equal,
    );
    test_helper(
        f64::MIN_POSITIVE_SUBNORMAL,
        100,
        "4.94065645841246544176568792868e-324",
        "0x4.0000000000000000000000000E-269#100",
        Equal,
    );

    test_helper(f64::NEGATIVE_ONE, 1, "-1.0", "-0x1.0#1", Equal);
    test_helper(f64::NEGATIVE_ONE, 10, "-1.0", "-0x1.000#10", Equal);
    test_helper(
        f64::NEGATIVE_ONE,
        100,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );

    test_helper(-1.0f64 / 3.0, 1, "-0.2", "-0x0.4#1", Greater);
    test_helper(-1.0f64 / 3.0, 10, "-0.3335", "-0x0.556#10", Less);
    test_helper(
        -1.0f64 / 3.0,
        100,
        "-0.3333333333333333148296162562474",
        "-0x0.55555555555554000000000000#100",
        Equal,
    );

    test_helper(-std::f64::consts::PI, 1, "-4.0", "-0x4.0#1", Less);
    test_helper(-std::f64::consts::PI, 10, "-3.141", "-0x3.24#10", Greater);
    test_helper(
        -std::f64::consts::PI,
        100,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );

    test_helper(
        -f64::MIN_POSITIVE_SUBNORMAL,
        1,
        "-5.0e-324",
        "-0x4.0E-269#1",
        Equal,
    );
    test_helper(
        -f64::MIN_POSITIVE_SUBNORMAL,
        10,
        "-4.94e-324",
        "-0x4.00E-269#10",
        Equal,
    );
    test_helper(
        -f64::MIN_POSITIVE_SUBNORMAL,
        100,
        "-4.94065645841246544176568792868e-324",
        "-0x4.0000000000000000000000000E-269#100",
        Equal,
    );
}

fn from_primitive_float_prec_fail_helper<T: PrimitiveFloat>()
where
    Float: From<T>,
{
    assert_panic!(Float::from_primitive_float_prec(T::NAN, 0));
    assert_panic!(Float::from_primitive_float_prec(T::INFINITY, 0));
    assert_panic!(Float::from_primitive_float_prec(T::NEGATIVE_INFINITY, 0));
    assert_panic!(Float::from_primitive_float_prec(T::ZERO, 0));
    assert_panic!(Float::from_primitive_float_prec(T::NEGATIVE_ZERO, 0));
    assert_panic!(Float::from_primitive_float_prec(T::ONE, 0));
    assert_panic!(Float::from_primitive_float_prec(T::NEGATIVE_ONE, 0));
}

#[test]
fn from_primitive_float_prec_fail() {
    apply_fn_to_primitive_floats!(from_primitive_float_prec_fail_helper);
}

#[test]
fn test_from_primitive_float_prec_round() {
    fn test_helper<T: PrimitiveFloat>(
        u: T,
        prec: u64,
        rm: RoundingMode,
        out: &str,
        out_hex: &str,
        out_o: Ordering,
    ) where
        rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
    {
        let (x, o) = Float::from_primitive_float_prec_round(u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, rug_o) = rug::Float::with_val_round(max(1, u32::exact_from(prec)), u, rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(rug_o, out_o);
        }
    }
    test_helper(f32::NAN, 1, Floor, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 1, Ceiling, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 1, Down, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 1, Up, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 1, Nearest, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 1, Exact, "NaN", "NaN", Equal);

    test_helper(f32::NAN, 10, Floor, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 10, Ceiling, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 10, Down, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 10, Up, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 10, Nearest, "NaN", "NaN", Equal);
    test_helper(f32::NAN, 10, Exact, "NaN", "NaN", Equal);

    test_helper(f32::INFINITY, 1, Floor, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 1, Ceiling, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 1, Down, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 1, Up, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 1, Nearest, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 1, Exact, "Infinity", "Infinity", Equal);

    test_helper(f32::INFINITY, 10, Floor, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 10, Ceiling, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 10, Down, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 10, Up, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 10, Nearest, "Infinity", "Infinity", Equal);
    test_helper(f32::INFINITY, 10, Exact, "Infinity", "Infinity", Equal);

    test_helper(
        f32::NEGATIVE_INFINITY,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test_helper(
        f32::NEGATIVE_INFINITY,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        10,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        10,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f32::NEGATIVE_INFINITY,
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test_helper(f32::ZERO, 1, Floor, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 1, Ceiling, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 1, Down, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 1, Up, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 1, Nearest, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 1, Exact, "0.0", "0x0.0", Equal);

    test_helper(f32::ZERO, 10, Floor, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 10, Ceiling, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 10, Down, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 10, Up, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 10, Nearest, "0.0", "0x0.0", Equal);
    test_helper(f32::ZERO, 10, Exact, "0.0", "0x0.0", Equal);

    test_helper(f32::NEGATIVE_ZERO, 1, Floor, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 1, Down, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 1, Up, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 1, Nearest, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 1, Exact, "-0.0", "-0x0.0", Equal);

    test_helper(f32::NEGATIVE_ZERO, 10, Floor, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 10, Down, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 10, Up, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test_helper(f32::NEGATIVE_ZERO, 10, Exact, "-0.0", "-0x0.0", Equal);

    test_helper(f32::ONE, 1, Floor, "1.0", "0x1.0#1", Equal);
    test_helper(f32::ONE, 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test_helper(f32::ONE, 1, Down, "1.0", "0x1.0#1", Equal);
    test_helper(f32::ONE, 1, Up, "1.0", "0x1.0#1", Equal);
    test_helper(f32::ONE, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test_helper(f32::ONE, 1, Exact, "1.0", "0x1.0#1", Equal);

    test_helper(f32::ONE, 10, Floor, "1.0", "0x1.000#10", Equal);
    test_helper(f32::ONE, 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test_helper(f32::ONE, 10, Down, "1.0", "0x1.000#10", Equal);
    test_helper(f32::ONE, 10, Up, "1.0", "0x1.000#10", Equal);
    test_helper(f32::ONE, 10, Nearest, "1.0", "0x1.000#10", Equal);
    test_helper(f32::ONE, 10, Exact, "1.0", "0x1.000#10", Equal);

    test_helper(1.0f32 / 3.0, 1, Floor, "0.2", "0x0.4#1", Less);
    test_helper(1.0f32 / 3.0, 1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_helper(1.0f32 / 3.0, 1, Down, "0.2", "0x0.4#1", Less);
    test_helper(1.0f32 / 3.0, 1, Up, "0.5", "0x0.8#1", Greater);
    test_helper(1.0f32 / 3.0, 1, Nearest, "0.2", "0x0.4#1", Less);

    test_helper(1.0f32 / 3.0, 10, Floor, "0.333", "0x0.554#10", Less);
    test_helper(1.0f32 / 3.0, 10, Ceiling, "0.3335", "0x0.556#10", Greater);
    test_helper(1.0f32 / 3.0, 10, Down, "0.333", "0x0.554#10", Less);
    test_helper(1.0f32 / 3.0, 10, Up, "0.3335", "0x0.556#10", Greater);
    test_helper(1.0f32 / 3.0, 10, Nearest, "0.3335", "0x0.556#10", Greater);

    test_helper(std::f32::consts::PI, 1, Floor, "2.0", "0x2.0#1", Less);
    test_helper(std::f32::consts::PI, 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_helper(std::f32::consts::PI, 1, Down, "2.0", "0x2.0#1", Less);
    test_helper(std::f32::consts::PI, 1, Up, "4.0", "0x4.0#1", Greater);
    test_helper(std::f32::consts::PI, 1, Nearest, "4.0", "0x4.0#1", Greater);

    test_helper(std::f32::consts::PI, 10, Floor, "3.141", "0x3.24#10", Less);
    test_helper(
        std::f32::consts::PI,
        10,
        Ceiling,
        "3.145",
        "0x3.25#10",
        Greater,
    );
    test_helper(std::f32::consts::PI, 10, Down, "3.141", "0x3.24#10", Less);
    test_helper(std::f32::consts::PI, 10, Up, "3.145", "0x3.25#10", Greater);
    test_helper(
        std::f32::consts::PI,
        10,
        Nearest,
        "3.141",
        "0x3.24#10",
        Less,
    );

    test_helper(f32::MAX_FINITE, 1, Floor, "2.0e38", "0x8.0E+31#1", Less);
    test_helper(
        f32::MAX_FINITE,
        1,
        Ceiling,
        "3.0e38",
        "0x1.0E+32#1",
        Greater,
    );
    test_helper(f32::MAX_FINITE, 1, Down, "2.0e38", "0x8.0E+31#1", Less);
    test_helper(f32::MAX_FINITE, 1, Up, "3.0e38", "0x1.0E+32#1", Greater);
    test_helper(
        f32::MAX_FINITE,
        1,
        Nearest,
        "3.0e38",
        "0x1.0E+32#1",
        Greater,
    );

    test_helper(f32::MAX_FINITE, 10, Floor, "3.4e38", "0xf.fcE+31#10", Less);
    test_helper(
        f32::MAX_FINITE,
        10,
        Ceiling,
        "3.403e38",
        "0x1.000E+32#10",
        Greater,
    );
    test_helper(f32::MAX_FINITE, 10, Down, "3.4e38", "0xf.fcE+31#10", Less);
    test_helper(
        f32::MAX_FINITE,
        10,
        Up,
        "3.403e38",
        "0x1.000E+32#10",
        Greater,
    );
    test_helper(
        f32::MAX_FINITE,
        10,
        Nearest,
        "3.403e38",
        "0x1.000E+32#10",
        Greater,
    );

    test_helper(f32::NEGATIVE_ONE, 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test_helper(f32::NEGATIVE_ONE, 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test_helper(f32::NEGATIVE_ONE, 1, Down, "-1.0", "-0x1.0#1", Equal);
    test_helper(f32::NEGATIVE_ONE, 1, Up, "-1.0", "-0x1.0#1", Equal);
    test_helper(f32::NEGATIVE_ONE, 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test_helper(f32::NEGATIVE_ONE, 1, Exact, "-1.0", "-0x1.0#1", Equal);

    test_helper(f32::NEGATIVE_ONE, 10, Floor, "-1.0", "-0x1.000#10", Equal);
    test_helper(f32::NEGATIVE_ONE, 10, Ceiling, "-1.0", "-0x1.000#10", Equal);
    test_helper(f32::NEGATIVE_ONE, 10, Down, "-1.0", "-0x1.000#10", Equal);
    test_helper(f32::NEGATIVE_ONE, 10, Up, "-1.0", "-0x1.000#10", Equal);
    test_helper(f32::NEGATIVE_ONE, 10, Nearest, "-1.0", "-0x1.000#10", Equal);
    test_helper(f32::NEGATIVE_ONE, 10, Exact, "-1.0", "-0x1.000#10", Equal);

    test_helper(-1.0f32 / 3.0, 1, Floor, "-0.5", "-0x0.8#1", Less);
    test_helper(-1.0f32 / 3.0, 1, Ceiling, "-0.2", "-0x0.4#1", Greater);
    test_helper(-1.0f32 / 3.0, 1, Down, "-0.2", "-0x0.4#1", Greater);
    test_helper(-1.0f32 / 3.0, 1, Up, "-0.5", "-0x0.8#1", Less);
    test_helper(-1.0f32 / 3.0, 1, Nearest, "-0.2", "-0x0.4#1", Greater);

    test_helper(-1.0f32 / 3.0, 10, Floor, "-0.3335", "-0x0.556#10", Less);
    test_helper(-1.0f32 / 3.0, 10, Ceiling, "-0.333", "-0x0.554#10", Greater);
    test_helper(-1.0f32 / 3.0, 10, Down, "-0.333", "-0x0.554#10", Greater);
    test_helper(-1.0f32 / 3.0, 10, Up, "-0.3335", "-0x0.556#10", Less);
    test_helper(-1.0f32 / 3.0, 10, Nearest, "-0.3335", "-0x0.556#10", Less);

    test_helper(-std::f32::consts::PI, 1, Floor, "-4.0", "-0x4.0#1", Less);
    test_helper(
        -std::f32::consts::PI,
        1,
        Ceiling,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test_helper(-std::f32::consts::PI, 1, Down, "-2.0", "-0x2.0#1", Greater);
    test_helper(-std::f32::consts::PI, 1, Up, "-4.0", "-0x4.0#1", Less);
    test_helper(-std::f32::consts::PI, 1, Nearest, "-4.0", "-0x4.0#1", Less);

    test_helper(
        -std::f32::consts::PI,
        10,
        Floor,
        "-3.145",
        "-0x3.25#10",
        Less,
    );
    test_helper(
        -std::f32::consts::PI,
        10,
        Ceiling,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    test_helper(
        -std::f32::consts::PI,
        10,
        Down,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    test_helper(-std::f32::consts::PI, 10, Up, "-3.145", "-0x3.25#10", Less);
    test_helper(
        -std::f32::consts::PI,
        10,
        Nearest,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );

    test_helper(-f32::MAX_FINITE, 1, Floor, "-3.0e38", "-0x1.0E+32#1", Less);
    test_helper(
        -f32::MAX_FINITE,
        1,
        Ceiling,
        "-2.0e38",
        "-0x8.0E+31#1",
        Greater,
    );
    test_helper(
        -f32::MAX_FINITE,
        1,
        Down,
        "-2.0e38",
        "-0x8.0E+31#1",
        Greater,
    );
    test_helper(-f32::MAX_FINITE, 1, Up, "-3.0e38", "-0x1.0E+32#1", Less);
    test_helper(
        -f32::MAX_FINITE,
        1,
        Nearest,
        "-3.0e38",
        "-0x1.0E+32#1",
        Less,
    );

    test_helper(
        -f32::MAX_FINITE,
        10,
        Floor,
        "-3.403e38",
        "-0x1.000E+32#10",
        Less,
    );
    test_helper(
        -f32::MAX_FINITE,
        10,
        Ceiling,
        "-3.4e38",
        "-0xf.fcE+31#10",
        Greater,
    );
    test_helper(
        -f32::MAX_FINITE,
        10,
        Down,
        "-3.4e38",
        "-0xf.fcE+31#10",
        Greater,
    );
    test_helper(
        -f32::MAX_FINITE,
        10,
        Up,
        "-3.403e38",
        "-0x1.000E+32#10",
        Less,
    );
    test_helper(
        -f32::MAX_FINITE,
        10,
        Nearest,
        "-3.403e38",
        "-0x1.000E+32#10",
        Less,
    );

    test_helper(f64::NAN, 1, Floor, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 1, Ceiling, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 1, Down, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 1, Up, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 1, Nearest, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 1, Exact, "NaN", "NaN", Equal);

    test_helper(f64::NAN, 10, Floor, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 10, Ceiling, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 10, Down, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 10, Up, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 10, Nearest, "NaN", "NaN", Equal);
    test_helper(f64::NAN, 10, Exact, "NaN", "NaN", Equal);

    test_helper(f64::INFINITY, 1, Floor, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 1, Ceiling, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 1, Down, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 1, Up, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 1, Nearest, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 1, Exact, "Infinity", "Infinity", Equal);

    test_helper(f64::INFINITY, 10, Floor, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 10, Ceiling, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 10, Down, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 10, Up, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 10, Nearest, "Infinity", "Infinity", Equal);
    test_helper(f64::INFINITY, 10, Exact, "Infinity", "Infinity", Equal);

    test_helper(
        f64::NEGATIVE_INFINITY,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test_helper(
        f64::NEGATIVE_INFINITY,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        10,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        10,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test_helper(
        f64::NEGATIVE_INFINITY,
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test_helper(f64::ZERO, 1, Floor, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 1, Ceiling, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 1, Down, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 1, Up, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 1, Nearest, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 1, Exact, "0.0", "0x0.0", Equal);

    test_helper(f64::ZERO, 10, Floor, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 10, Ceiling, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 10, Down, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 10, Up, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 10, Nearest, "0.0", "0x0.0", Equal);
    test_helper(f64::ZERO, 10, Exact, "0.0", "0x0.0", Equal);

    test_helper(f64::NEGATIVE_ZERO, 1, Floor, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 1, Down, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 1, Up, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 1, Nearest, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 1, Exact, "-0.0", "-0x0.0", Equal);

    test_helper(f64::NEGATIVE_ZERO, 10, Floor, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 10, Down, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 10, Up, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test_helper(f64::NEGATIVE_ZERO, 10, Exact, "-0.0", "-0x0.0", Equal);

    test_helper(f64::ONE, 1, Floor, "1.0", "0x1.0#1", Equal);
    test_helper(f64::ONE, 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test_helper(f64::ONE, 1, Down, "1.0", "0x1.0#1", Equal);
    test_helper(f64::ONE, 1, Up, "1.0", "0x1.0#1", Equal);
    test_helper(f64::ONE, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test_helper(f64::ONE, 1, Exact, "1.0", "0x1.0#1", Equal);

    test_helper(f64::ONE, 10, Floor, "1.0", "0x1.000#10", Equal);
    test_helper(f64::ONE, 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test_helper(f64::ONE, 10, Down, "1.0", "0x1.000#10", Equal);
    test_helper(f64::ONE, 10, Up, "1.0", "0x1.000#10", Equal);
    test_helper(f64::ONE, 10, Nearest, "1.0", "0x1.000#10", Equal);
    test_helper(f64::ONE, 10, Exact, "1.0", "0x1.000#10", Equal);

    test_helper(1.0f64 / 3.0, 1, Floor, "0.2", "0x0.4#1", Less);
    test_helper(1.0f64 / 3.0, 1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_helper(1.0f64 / 3.0, 1, Down, "0.2", "0x0.4#1", Less);
    test_helper(1.0f64 / 3.0, 1, Up, "0.5", "0x0.8#1", Greater);
    test_helper(1.0f64 / 3.0, 1, Nearest, "0.2", "0x0.4#1", Less);

    test_helper(1.0f64 / 3.0, 10, Floor, "0.333", "0x0.554#10", Less);
    test_helper(1.0f64 / 3.0, 10, Ceiling, "0.3335", "0x0.556#10", Greater);
    test_helper(1.0f64 / 3.0, 10, Down, "0.333", "0x0.554#10", Less);
    test_helper(1.0f64 / 3.0, 10, Up, "0.3335", "0x0.556#10", Greater);
    test_helper(1.0f64 / 3.0, 10, Nearest, "0.3335", "0x0.556#10", Greater);

    test_helper(std::f64::consts::PI, 1, Floor, "2.0", "0x2.0#1", Less);
    test_helper(std::f64::consts::PI, 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_helper(std::f64::consts::PI, 1, Down, "2.0", "0x2.0#1", Less);
    test_helper(std::f64::consts::PI, 1, Up, "4.0", "0x4.0#1", Greater);
    test_helper(std::f64::consts::PI, 1, Nearest, "4.0", "0x4.0#1", Greater);

    test_helper(std::f64::consts::PI, 10, Floor, "3.141", "0x3.24#10", Less);
    test_helper(
        std::f64::consts::PI,
        10,
        Ceiling,
        "3.145",
        "0x3.25#10",
        Greater,
    );
    test_helper(std::f64::consts::PI, 10, Down, "3.141", "0x3.24#10", Less);
    test_helper(std::f64::consts::PI, 10, Up, "3.145", "0x3.25#10", Greater);
    test_helper(
        std::f64::consts::PI,
        10,
        Nearest,
        "3.141",
        "0x3.24#10",
        Less,
    );

    test_helper(f64::MAX_FINITE, 1, Floor, "9.0e307", "0x8.0E+255#1", Less);
    test_helper(
        f64::MAX_FINITE,
        1,
        Ceiling,
        "2.0e308",
        "0x1.0E+256#1",
        Greater,
    );
    test_helper(f64::MAX_FINITE, 1, Down, "9.0e307", "0x8.0E+255#1", Less);
    test_helper(f64::MAX_FINITE, 1, Up, "2.0e308", "0x1.0E+256#1", Greater);
    test_helper(
        f64::MAX_FINITE,
        1,
        Nearest,
        "2.0e308",
        "0x1.0E+256#1",
        Greater,
    );

    test_helper(
        f64::MAX_FINITE,
        10,
        Floor,
        "1.796e308",
        "0xf.fcE+255#10",
        Less,
    );
    test_helper(
        f64::MAX_FINITE,
        10,
        Ceiling,
        "1.798e308",
        "0x1.000E+256#10",
        Greater,
    );
    test_helper(
        f64::MAX_FINITE,
        10,
        Down,
        "1.796e308",
        "0xf.fcE+255#10",
        Less,
    );
    test_helper(
        f64::MAX_FINITE,
        10,
        Up,
        "1.798e308",
        "0x1.000E+256#10",
        Greater,
    );
    test_helper(
        f64::MAX_FINITE,
        10,
        Nearest,
        "1.798e308",
        "0x1.000E+256#10",
        Greater,
    );

    test_helper(f64::NEGATIVE_ONE, 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test_helper(f64::NEGATIVE_ONE, 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test_helper(f64::NEGATIVE_ONE, 1, Down, "-1.0", "-0x1.0#1", Equal);
    test_helper(f64::NEGATIVE_ONE, 1, Up, "-1.0", "-0x1.0#1", Equal);
    test_helper(f64::NEGATIVE_ONE, 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test_helper(f64::NEGATIVE_ONE, 1, Exact, "-1.0", "-0x1.0#1", Equal);

    test_helper(f64::NEGATIVE_ONE, 10, Floor, "-1.0", "-0x1.000#10", Equal);
    test_helper(f64::NEGATIVE_ONE, 10, Ceiling, "-1.0", "-0x1.000#10", Equal);
    test_helper(f64::NEGATIVE_ONE, 10, Down, "-1.0", "-0x1.000#10", Equal);
    test_helper(f64::NEGATIVE_ONE, 10, Up, "-1.0", "-0x1.000#10", Equal);
    test_helper(f64::NEGATIVE_ONE, 10, Nearest, "-1.0", "-0x1.000#10", Equal);
    test_helper(f64::NEGATIVE_ONE, 10, Exact, "-1.0", "-0x1.000#10", Equal);

    test_helper(-1.0f64 / 3.0, 1, Floor, "-0.5", "-0x0.8#1", Less);
    test_helper(-1.0f64 / 3.0, 1, Ceiling, "-0.2", "-0x0.4#1", Greater);
    test_helper(-1.0f64 / 3.0, 1, Down, "-0.2", "-0x0.4#1", Greater);
    test_helper(-1.0f64 / 3.0, 1, Up, "-0.5", "-0x0.8#1", Less);
    test_helper(-1.0f64 / 3.0, 1, Nearest, "-0.2", "-0x0.4#1", Greater);

    test_helper(-1.0f64 / 3.0, 10, Floor, "-0.3335", "-0x0.556#10", Less);
    test_helper(-1.0f64 / 3.0, 10, Ceiling, "-0.333", "-0x0.554#10", Greater);
    test_helper(-1.0f64 / 3.0, 10, Down, "-0.333", "-0x0.554#10", Greater);
    test_helper(-1.0f64 / 3.0, 10, Up, "-0.3335", "-0x0.556#10", Less);
    test_helper(-1.0f64 / 3.0, 10, Nearest, "-0.3335", "-0x0.556#10", Less);

    test_helper(-std::f64::consts::PI, 1, Floor, "-4.0", "-0x4.0#1", Less);
    test_helper(
        -std::f64::consts::PI,
        1,
        Ceiling,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test_helper(-std::f64::consts::PI, 1, Down, "-2.0", "-0x2.0#1", Greater);
    test_helper(-std::f64::consts::PI, 1, Up, "-4.0", "-0x4.0#1", Less);
    test_helper(-std::f64::consts::PI, 1, Nearest, "-4.0", "-0x4.0#1", Less);

    test_helper(
        -std::f64::consts::PI,
        10,
        Floor,
        "-3.145",
        "-0x3.25#10",
        Less,
    );
    test_helper(
        -std::f64::consts::PI,
        10,
        Ceiling,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    test_helper(
        -std::f64::consts::PI,
        10,
        Down,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    test_helper(-std::f64::consts::PI, 10, Up, "-3.145", "-0x3.25#10", Less);
    test_helper(
        -std::f64::consts::PI,
        10,
        Nearest,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );

    test_helper(
        -f64::MAX_FINITE,
        1,
        Floor,
        "-2.0e308",
        "-0x1.0E+256#1",
        Less,
    );
    test_helper(
        -f64::MAX_FINITE,
        1,
        Ceiling,
        "-9.0e307",
        "-0x8.0E+255#1",
        Greater,
    );
    test_helper(
        -f64::MAX_FINITE,
        1,
        Down,
        "-9.0e307",
        "-0x8.0E+255#1",
        Greater,
    );
    test_helper(-f64::MAX_FINITE, 1, Up, "-2.0e308", "-0x1.0E+256#1", Less);
    test_helper(
        -f64::MAX_FINITE,
        1,
        Nearest,
        "-2.0e308",
        "-0x1.0E+256#1",
        Less,
    );

    test_helper(
        -f64::MAX_FINITE,
        10,
        Floor,
        "-1.798e308",
        "-0x1.000E+256#10",
        Less,
    );
    test_helper(
        -f64::MAX_FINITE,
        10,
        Ceiling,
        "-1.796e308",
        "-0xf.fcE+255#10",
        Greater,
    );
    test_helper(
        -f64::MAX_FINITE,
        10,
        Down,
        "-1.796e308",
        "-0xf.fcE+255#10",
        Greater,
    );
    test_helper(
        -f64::MAX_FINITE,
        10,
        Up,
        "-1.798e308",
        "-0x1.000E+256#10",
        Less,
    );
    test_helper(
        -f64::MAX_FINITE,
        10,
        Nearest,
        "-1.798e308",
        "-0x1.000E+256#10",
        Less,
    );
}

fn from_primitive_float_prec_round_fail_helper<T: PrimitiveFloat>()
where
    Float: From<T>,
{
    assert_panic!(Float::from_primitive_float_prec_round(T::NAN, 0, Floor));
    assert_panic!(Float::from_primitive_float_prec_round(
        T::INFINITY,
        0,
        Floor
    ));
    assert_panic!(Float::from_primitive_float_prec_round(
        T::NEGATIVE_INFINITY,
        0,
        Floor
    ));
    assert_panic!(Float::from_primitive_float_prec_round(T::ZERO, 0, Floor));
    assert_panic!(Float::from_primitive_float_prec_round(
        T::NEGATIVE_ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::from_primitive_float_prec_round(T::ONE, 0, Floor));
    assert_panic!(Float::from_primitive_float_prec_round(
        T::NEGATIVE_ONE,
        0,
        Floor
    ));

    assert_panic!(Float::from_primitive_float_prec_round(
        T::from(1.0f32) / T::from(3.0f32),
        1,
        Exact
    ));
    assert_panic!(Float::from_primitive_float_prec_round(
        T::from(-1.0f32) / T::from(3.0f32),
        1,
        Exact
    ));
}

#[test]
fn from_primitive_float_prec_round_fail() {
    apply_fn_to_primitive_floats!(from_primitive_float_prec_round_fail_helper);
}

fn from_primitive_float_prec_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    Rational: TryFrom<T>,
    rug::Float: Assign<T>,
{
    primitive_float_unsigned_pair_gen_var_4::<T, u64>().test_properties(|(x, prec)| {
        let (float_x, o) = Float::from_primitive_float_prec(x, prec);
        assert!(float_x.is_valid());

        assert_eq!(
            float_x.partial_cmp(&x),
            if x.is_nan() { None } else { Some(o) }
        );

        let rug_x = rug::Float::with_val(u32::exact_from(prec), x);
        assert_eq!(
            ComparableFloatRef(&float_x),
            ComparableFloatRef(&Float::from(&rug_x))
        );

        if let Ok(r_x) = Rational::try_from(x) {
            let (float_x_alt, o_alt) = Float::from_rational_prec(r_x, prec);
            assert_eq!(
                ComparableFloatRef(&float_x_alt),
                ComparableFloatRef(&float_x.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, o);
        }

        assert_eq!(
            float_x.get_prec(),
            if x.is_finite() && x != T::ZERO {
                Some(prec)
            } else {
                None
            }
        );

        let (float_x_alt, o_alt) = Float::from_primitive_float_prec(x, prec);
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o_alt, o);

        let (float_x_alt, o_alt) = Float::from_primitive_float_prec_round(x, prec, Nearest);
        assert_eq!(ComparableFloat(float_x_alt), ComparableFloat(float_x));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn from_primitive_float_prec_properties() {
    apply_fn_to_primitive_floats!(from_primitive_float_prec_properties_helper);
}

fn from_primitive_float_prec_round_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: PartialOrd<T> + TryFrom<T>,
    rug::Float: AssignRound<T, Round = Round, Ordering = Ordering>,
{
    primitive_float_unsigned_rounding_mode_triple_gen_var_3::<T>().test_properties(
        |(x, prec, rm)| {
            let (float_x, o) = Float::from_primitive_float_prec_round(x, prec, rm);
            assert!(float_x.is_valid());

            assert_eq!(
                float_x.partial_cmp(&x),
                if x.is_nan() { None } else { Some(o) }
            );
            match (x >= T::ZERO, rm) {
                (_, Floor) | (true, Down) | (false, Up) => {
                    assert_ne!(o, Greater);
                }
                (_, Ceiling) | (true, Up) | (false, Down) => {
                    assert_ne!(o, Less);
                }
                (_, Exact) => assert_eq!(o, Equal),
                _ => {}
            }

            if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_x, rug_o) = rug::Float::with_val_round(u32::exact_from(prec), x, rm);
                let float_x_alt: Float = From::from(&rug_x);
                assert_eq!(
                    ComparableFloatRef(&float_x),
                    ComparableFloatRef(&float_x_alt)
                );
                assert_eq!(rug_o, o);
            }

            if let Ok(r_x) = Rational::try_from(x) {
                let (float_x_alt, o_alt) = Float::from_rational_prec_round(r_x, prec, rm);
                assert_eq!(
                    ComparableFloatRef(&float_x_alt),
                    ComparableFloatRef(&float_x.abs_negative_zero_ref())
                );
                assert_eq!(o_alt, o);
            }

            assert_eq!(
                float_x.get_prec(),
                if x.is_finite() && x != T::ZERO {
                    Some(prec)
                } else {
                    None
                }
            );
        },
    );

    primitive_float_unsigned_pair_gen_var_4::<T, u64>().test_properties(|(x, prec)| {
        let floor = Float::from_primitive_float_prec_round(x, prec, Floor);
        if x.is_nan() {
            assert!(floor.0.is_nan());
        } else {
            let or_floor: Result<Rational, _> = TryFrom::try_from(&floor.0);
            if let Ok(r_floor) = or_floor {
                assert!(r_floor <= x);
                if r_floor != T::ZERO {
                    let rulp: Rational = ExactFrom::exact_from(floor.0.ulp().unwrap());
                    assert!(r_floor + rulp > x);
                }
                let (floor_x_alt, o_alt) = Float::from_primitive_float_prec_round(
                    x,
                    prec,
                    if x >= T::ZERO { Down } else { Up },
                );
                assert_eq!(
                    ComparableFloatRef(&floor_x_alt),
                    ComparableFloatRef(&floor.0)
                );
                assert_eq!(o_alt, floor.1);
            }
        }

        let ceiling = Float::from_primitive_float_prec_round(x, prec, Ceiling);
        if x.is_nan() {
            assert!(ceiling.0.is_nan());
        } else {
            let or_ceiling: Result<Rational, _> = TryFrom::try_from(&ceiling.0);
            if let Ok(r_ceiling) = or_ceiling {
                assert!(r_ceiling >= x);
                if r_ceiling != T::ZERO {
                    let rulp: Rational = ExactFrom::exact_from(ceiling.0.ulp().unwrap());
                    assert!(r_ceiling - rulp < x);
                }
                let (ceiling_x_alt, o_alt) = Float::from_primitive_float_prec_round(
                    x,
                    prec,
                    if x >= T::ZERO { Up } else { Down },
                );
                assert_eq!(
                    ComparableFloatRef(&ceiling_x_alt),
                    ComparableFloatRef(&ceiling.0)
                );
                assert_eq!(o_alt, ceiling.1);
            }
        }

        let nearest = Float::from_primitive_float_prec_round(x, prec, Nearest);
        assert!(
            ComparableFloatRef(&nearest.0) == ComparableFloatRef(&floor.0) && nearest.1 == floor.1
                || ComparableFloatRef(&nearest.0) == ComparableFloatRef(&ceiling.0)
                    && nearest.1 == ceiling.1
        );
        let or_nearest: Result<Rational, _> = TryFrom::try_from(&nearest.0);
        if let Ok(r_nearest) = or_nearest {
            if r_nearest != T::ZERO {
                let rulp: Rational = ExactFrom::exact_from(nearest.0.ulp().unwrap());
                assert!((r_nearest - Rational::exact_from(x)).le_abs(&(rulp >> 1)));
            }
        }
    });
}

#[test]
fn from_primitive_float_prec_round_properties() {
    apply_fn_to_primitive_floats!(from_primitive_float_prec_round_properties_helper);
}

#[allow(clippy::type_repetition_in_bounds)]
fn from_primitive_float_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T>,
    rug::Float: Assign<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let float_x = Float::from(x);
        assert!(float_x.is_valid());

        let expected_prec = if !x.is_finite() || x == T::ZERO {
            None
        } else {
            let n = x.integer_mantissa();
            Some(n.significant_bits() - TrailingZeros::trailing_zeros(n))
        };
        let rug_x = rug::Float::with_val(expected_prec.map_or(1, u32::exact_from), x);
        assert_eq!(
            ComparableFloatRef(&float_x),
            ComparableFloatRef(&From::<&rug::Float>::from(&rug_x))
        );

        assert_eq!(float_x.get_prec(), expected_prec);
        assert_eq!(NiceFloat(T::exact_from(&float_x)), NiceFloat(x));
        let (f, o) = Float::from_primitive_float_prec(x, expected_prec.unwrap_or(1));
        assert_eq!(ComparableFloat(f), ComparableFloat(float_x));
        assert_eq!(o, Equal);
    });
}

#[test]
fn from_primitive_float_properties() {
    apply_fn_to_primitive_floats!(from_primitive_float_properties_helper);
}
