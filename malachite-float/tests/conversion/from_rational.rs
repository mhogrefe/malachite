// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::conversion::from_rational::{
    from_rational_prec_round_direct, from_rational_prec_round_ref_direct,
    from_rational_prec_round_ref_using_div, from_rational_prec_round_using_div,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::generators::rational_unsigned_rounding_mode_triple_gen_var_1;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float, emulate_rational_to_float_fn};
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::Rational;
use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::cmp::Ordering::*;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_from_rational_prec() {
    fn float_helper<T: PrimitiveFloat>(x: &Rational)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float> + RoundingFrom<&'a Rational>,
        Rational: TryFrom<T>,
    {
        let xf = emulate_rational_to_float_fn::<T, _>(Float::from_rational_prec_ref, x);
        assert_eq!(NiceFloat(xf), NiceFloat(T::rounding_from(x, Nearest).0));
    }

    let test = |s, prec, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (x, o) = Float::from_rational_prec(u.clone(), prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_rational_prec_ref(&u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_direct(u.clone(), prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_direct(&u, prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_using_div(u.clone(), prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_using_div(&u, prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Rational::from(&u));
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        float_helper::<f32>(&u);
        float_helper::<f64>(&u);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 100, "0.0", "0x0.0", Equal);
    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0000", "0x1.000#10", Equal);
    test(
        "1",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1/2", 1, "0.50", "0x0.8#1", Equal);
    test("1/2", 10, "0.50000", "0x0.800#10", Equal);
    test(
        "1/2",
        100,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("1/3", 1, "0.25", "0x0.4#1", Less);
    test("1/3", 10, "0.33350", "0x0.556#10", Greater);
    test(
        "1/3",
        100,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test("22/7", 1, "4.0", "0x4.0#1", Greater);
    test("22/7", 10, "3.1445", "0x3.25#10", Greater);
    test(
        "22/7",
        100,
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        Greater,
    );

    let test_big = |u: Rational, prec, out, out_hex, out_o| {
        let (x, o) = Float::from_rational_prec(u.clone(), prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_rational_prec_ref(&u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_direct(u.clone(), prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_direct(&u, prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_using_div(u.clone(), prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_using_div(&u, prec, Nearest);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Rational::from(&u));
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        float_helper::<f32>(&u);
        float_helper::<f64>(&u);
    };

    test_big(
        Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL),
        10,
        "1.4013e-45",
        "0x8.00E-38#10",
        Equal,
    );
    test_big(
        Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL) >> 1,
        10,
        "7.0065e-46",
        "0x4.00E-38#10",
        Equal,
    );
    test_big(
        (Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL) >> 1) + Rational::power_of_2(-1000i64),
        10,
        "7.0065e-46",
        "0x4.00E-38#10",
        Less,
    );
    test_big(
        (Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL) >> 1) - Rational::power_of_2(-1000i64),
        10,
        "7.0065e-46",
        "0x4.00E-38#10",
        Greater,
    );

    test_big(
        Rational::exact_from(f64::MIN_POSITIVE_SUBNORMAL),
        10,
        "4.9407e-324",
        "0x4.00E-269#10",
        Equal,
    );
    test_big(
        Rational::exact_from(f64::MIN_POSITIVE_SUBNORMAL) >> 1,
        10,
        "2.4703e-324",
        "0x2.00E-269#10",
        Equal,
    );
    test_big(
        (Rational::exact_from(f64::MIN_POSITIVE_SUBNORMAL) >> 1) + Rational::power_of_2(-2000i64),
        10,
        "2.4703e-324",
        "0x2.00E-269#10",
        Less,
    );
    test_big(
        (Rational::exact_from(f64::MIN_POSITIVE_SUBNORMAL) >> 1) - Rational::power_of_2(-2000i64),
        10,
        "2.4703e-324",
        "0x2.00E-269#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(1000i64),
        10,
        "1.0715e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        10,
        "9.3326e-302",
        "0x1.000E-250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        1,
        "9.3e-302",
        "0x1.0E-250#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );

    test_big(
        -Rational::power_of_2(1000i64),
        10,
        "-1.0715e301",
        "-0x1.000E+250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        1,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );

    test_big(
        -Rational::power_of_2(-1000i64),
        10,
        "-9.3326e-302",
        "-0x1.000E-250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );

    test_big(
        -Rational::power_of_2(-1000i64),
        1,
        "-9.3e-302",
        "-0x1.0E-250#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
}

#[test]
fn from_rational_prec_fail() {
    assert_panic!(Float::from_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::from_rational_prec(Rational::ONE, 0));
    assert_panic!(Float::from_rational_prec(Rational::NEGATIVE_ONE, 0));
}

#[test]
fn from_rational_prec_ref_fail() {
    assert_panic!(Float::from_rational_prec_ref(&Rational::ZERO, 0));
    assert_panic!(Float::from_rational_prec_ref(&Rational::ONE, 0));
    assert_panic!(Float::from_rational_prec_ref(&Rational::NEGATIVE_ONE, 0));
}

#[test]
fn test_from_rational_prec_round() {
    let test = |s, prec, rm, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (x, o) = Float::from_rational_prec_round(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_rational_prec_round_ref(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_direct(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_using_div(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_direct(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_using_div(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Rational::from(&u), rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(o, out_o);
        }
    };
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);

    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);

    test("0", 100, Floor, "0.0", "0x0.0", Equal);
    test("0", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 100, Down, "0.0", "0x0.0", Equal);
    test("0", 100, Up, "0.0", "0x0.0", Equal);
    test("0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("0", 100, Exact, "0.0", "0x0.0", Equal);

    test("1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Exact, "1.0000", "0x1.000#10", Equal);

    test(
        "1",
        100,
        Floor,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Ceiling,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Down,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Up,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Exact,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test("1/2", 1, Floor, "0.50", "0x0.8#1", Equal);
    test("1/2", 1, Ceiling, "0.50", "0x0.8#1", Equal);
    test("1/2", 1, Down, "0.50", "0x0.8#1", Equal);
    test("1/2", 1, Up, "0.50", "0x0.8#1", Equal);
    test("1/2", 1, Nearest, "0.50", "0x0.8#1", Equal);
    test("1/2", 1, Exact, "0.50", "0x0.8#1", Equal);

    test("1/2", 10, Floor, "0.50000", "0x0.800#10", Equal);
    test("1/2", 10, Ceiling, "0.50000", "0x0.800#10", Equal);
    test("1/2", 10, Down, "0.50000", "0x0.800#10", Equal);
    test("1/2", 10, Up, "0.50000", "0x0.800#10", Equal);
    test("1/2", 10, Nearest, "0.50000", "0x0.800#10", Equal);
    test("1/2", 10, Exact, "0.50000", "0x0.800#10", Equal);

    test(
        "1/2",
        100,
        Floor,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Down,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Up,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Exact,
        "0.50000000000000000000000000000000",
        "0x0.8000000000000000000000000#100",
        Equal,
    );

    test("1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Down, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Up, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.25", "0x0.4#1", Less);

    test("1/3", 10, Floor, "0.33301", "0x0.554#10", Less);
    test("1/3", 10, Ceiling, "0.33350", "0x0.556#10", Greater);
    test("1/3", 10, Down, "0.33301", "0x0.554#10", Less);
    test("1/3", 10, Up, "0.33350", "0x0.556#10", Greater);
    test("1/3", 10, Nearest, "0.33350", "0x0.556#10", Greater);

    test(
        "1/3",
        100,
        Floor,
        "0.33333333333333333333333333333307",
        "0x0.55555555555555555555555550#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "0.33333333333333333333333333333307",
        "0x0.55555555555555555555555550#100",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Greater,
    );

    test("22/7", 1, Floor, "2.0", "0x2.0#1", Less);
    test("22/7", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("22/7", 1, Down, "2.0", "0x2.0#1", Less);
    test("22/7", 1, Up, "4.0", "0x4.0#1", Greater);
    test("22/7", 1, Nearest, "4.0", "0x4.0#1", Greater);

    test("22/7", 10, Floor, "3.1406", "0x3.24#10", Less);
    test("22/7", 10, Ceiling, "3.1445", "0x3.25#10", Greater);
    test("22/7", 10, Down, "3.1406", "0x3.24#10", Less);
    test("22/7", 10, Up, "3.1445", "0x3.25#10", Greater);
    test("22/7", 10, Nearest, "3.1445", "0x3.25#10", Greater);

    test(
        "22/7",
        100,
        Floor,
        "3.1428571428571428571428571428553",
        "0x3.2492492492492492492492490#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Down,
        "3.1428571428571428571428571428553",
        "0x3.2492492492492492492492490#100",
        Less,
    );
    test(
        "22/7",
        100,
        Up,
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        Greater,
    );

    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Down, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Up, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Exact, "-1.0", "-0x1.0#1", Equal);

    test("-1", 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 10, Ceiling, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 10, Down, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 10, Up, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 10, Exact, "-1.0000", "-0x1.000#10", Equal);

    test(
        "-1",
        100,
        Floor,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Ceiling,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Down,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Up,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Exact,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );

    test("-1/2", 1, Floor, "-0.50", "-0x0.8#1", Equal);
    test("-1/2", 1, Ceiling, "-0.50", "-0x0.8#1", Equal);
    test("-1/2", 1, Down, "-0.50", "-0x0.8#1", Equal);
    test("-1/2", 1, Up, "-0.50", "-0x0.8#1", Equal);
    test("-1/2", 1, Nearest, "-0.50", "-0x0.8#1", Equal);
    test("-1/2", 1, Exact, "-0.50", "-0x0.8#1", Equal);

    test("-1/2", 10, Floor, "-0.50000", "-0x0.800#10", Equal);
    test("-1/2", 10, Ceiling, "-0.50000", "-0x0.800#10", Equal);
    test("-1/2", 10, Down, "-0.50000", "-0x0.800#10", Equal);
    test("-1/2", 10, Up, "-0.50000", "-0x0.800#10", Equal);
    test("-1/2", 10, Nearest, "-0.50000", "-0x0.800#10", Equal);
    test("-1/2", 10, Exact, "-0.50000", "-0x0.800#10", Equal);

    test(
        "-1/2",
        100,
        Floor,
        "-0.50000000000000000000000000000000",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Ceiling,
        "-0.50000000000000000000000000000000",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Down,
        "-0.50000000000000000000000000000000",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Up,
        "-0.50000000000000000000000000000000",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Nearest,
        "-0.50000000000000000000000000000000",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Exact,
        "-0.50000000000000000000000000000000",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );

    test("-1/3", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Up, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4#1", Greater);

    test("-1/3", 10, Floor, "-0.33350", "-0x0.556#10", Less);
    test("-1/3", 10, Ceiling, "-0.33301", "-0x0.554#10", Greater);
    test("-1/3", 10, Down, "-0.33301", "-0x0.554#10", Greater);
    test("-1/3", 10, Up, "-0.33350", "-0x0.556#10", Less);
    test("-1/3", 10, Nearest, "-0.33350", "-0x0.556#10", Less);

    test(
        "-1/3",
        100,
        Floor,
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        Less,
    );
    test(
        "-1/3",
        100,
        Ceiling,
        "-0.33333333333333333333333333333307",
        "-0x0.55555555555555555555555550#100",
        Greater,
    );
    test(
        "-1/3",
        100,
        Down,
        "-0.33333333333333333333333333333307",
        "-0x0.55555555555555555555555550#100",
        Greater,
    );
    test(
        "-1/3",
        100,
        Up,
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        Less,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        Less,
    );

    test("-22/7", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("-22/7", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("-22/7", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("-22/7", 1, Up, "-4.0", "-0x4.0#1", Less);
    test("-22/7", 1, Nearest, "-4.0", "-0x4.0#1", Less);

    test("-22/7", 10, Floor, "-3.1445", "-0x3.25#10", Less);
    test("-22/7", 10, Ceiling, "-3.1406", "-0x3.24#10", Greater);
    test("-22/7", 10, Down, "-3.1406", "-0x3.24#10", Greater);
    test("-22/7", 10, Up, "-3.1445", "-0x3.25#10", Less);
    test("-22/7", 10, Nearest, "-3.1445", "-0x3.25#10", Less);

    test(
        "-22/7",
        100,
        Floor,
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        Less,
    );
    test(
        "-22/7",
        100,
        Ceiling,
        "-3.1428571428571428571428571428553",
        "-0x3.2492492492492492492492490#100",
        Greater,
    );
    test(
        "-22/7",
        100,
        Down,
        "-3.1428571428571428571428571428553",
        "-0x3.2492492492492492492492490#100",
        Greater,
    );
    test(
        "-22/7",
        100,
        Up,
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        Less,
    );

    let test_big = |u: Rational, prec, rm, out, out_hex, out_o| {
        let (x, o) = Float::from_rational_prec_round(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_rational_prec_round_ref(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_direct(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_using_div(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_direct(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = from_rational_prec_round_ref_using_div(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Rational::from(&u), rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(o, out_o);
        }
    };
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Floor,
        "1.0715e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Ceiling,
        "1.0715e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Down,
        "1.0715e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Up,
        "1.0715e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Nearest,
        "1.0715e301",
        "0x1.000E+250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Exact,
        "1.0715e301",
        "0x1.000E+250#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Floor,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Ceiling,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Down,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Up,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Nearest,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Exact,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Nearest,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Floor,
        "9.3326e-302",
        "0x1.000E-250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Ceiling,
        "9.3326e-302",
        "0x1.000E-250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Down,
        "9.3326e-302",
        "0x1.000E-250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Up,
        "9.3326e-302",
        "0x1.000E-250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "9.3326e-302",
        "0x1.000E-250#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Exact,
        "9.3326e-302",
        "0x1.000E-250#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Floor,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Ceiling,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Down,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Up,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Nearest,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Exact,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Floor,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Down,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Exact,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Floor,
        "9.3e-302",
        "0x1.0E-250#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Ceiling,
        "9.3e-302",
        "0x1.0E-250#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Down,
        "9.3e-302",
        "0x1.0E-250#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Up,
        "9.3e-302",
        "0x1.0E-250#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Nearest,
        "9.3e-302",
        "0x1.0E-250#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Exact,
        "9.3e-302",
        "0x1.0E-250#1",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Floor,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Ceiling,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Down,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Up,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Exact,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Floor,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Down,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Exact,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );

    test_big(
        -Rational::power_of_2(1000i64),
        10,
        Floor,
        "-1.0715e301",
        "-0x1.000E+250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(1000i64),
        10,
        Ceiling,
        "-1.0715e301",
        "-0x1.000E+250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(1000i64),
        10,
        Down,
        "-1.0715e301",
        "-0x1.000E+250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(1000i64),
        10,
        Up,
        "-1.0715e301",
        "-0x1.000E+250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(1000i64),
        10,
        Nearest,
        "-1.0715e301",
        "-0x1.000E+250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(1000i64),
        10,
        Exact,
        "-1.0715e301",
        "-0x1.000E+250#10",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Ceiling,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Down,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Floor,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Ceiling,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Down,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Up,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Nearest,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Exact,
        "-1.0493e323228496",
        "-0x4.00E+268435455#10",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        1,
        Ceiling,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        1,
        Down,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Ceiling,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Down,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Ceiling,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Down,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Nearest,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );

    test_big(
        -Rational::power_of_2(-1000i64),
        10,
        Floor,
        "-9.3326e-302",
        "-0x1.000E-250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        10,
        Ceiling,
        "-9.3326e-302",
        "-0x1.000E-250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        10,
        Down,
        "-9.3326e-302",
        "-0x1.000E-250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        10,
        Up,
        "-9.3326e-302",
        "-0x1.000E-250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "-9.3326e-302",
        "-0x1.000E-250#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        10,
        Exact,
        "-9.3326e-302",
        "-0x1.000E-250#10",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Floor,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Ceiling,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Down,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Up,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Nearest,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Exact,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Ceiling,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Down,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Exact,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );

    test_big(
        -Rational::power_of_2(-1000i64),
        1,
        Floor,
        "-9.3e-302",
        "-0x1.0E-250#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        1,
        Ceiling,
        "-9.3e-302",
        "-0x1.0E-250#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        1,
        Down,
        "-9.3e-302",
        "-0x1.0E-250#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        1,
        Up,
        "-9.3e-302",
        "-0x1.0E-250#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        1,
        Nearest,
        "-9.3e-302",
        "-0x1.0E-250#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(-1000i64),
        1,
        Exact,
        "-9.3e-302",
        "-0x1.0E-250#1",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Floor,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Down,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Up,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Nearest,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Exact,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Ceiling,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Down,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Exact,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Equal,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );

    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
}

#[test]
fn from_rational_prec_round_fail() {
    assert_panic!(Float::from_rational_prec_round(Rational::ZERO, 0, Floor));
    assert_panic!(Float::from_rational_prec_round(Rational::ONE, 0, Floor));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from(-123),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from_signeds(-1i8, 3),
        100,
        Exact
    ));
}

#[test]
fn from_rational_prec_round_ref_fail() {
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from(-123),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from_signeds(-1i8, 3),
        100,
        Exact
    ));
}

#[allow(clippy::needless_borrow)]
#[test]
fn test_try_from_rational() {
    let test = |s, out, out_hex| {
        let x = Rational::from_str(s).unwrap();

        let of = Float::try_from(x.clone());
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);

        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);
    };
    test("0", Ok("0.0"), Ok("0x0.0"));
    test("1", Ok("1.0"), Ok("0x1.0#1"));
    test("1/2", Ok("0.50"), Ok("0x0.8#1"));
    test("117/256", Ok("0.4570"), Ok("0x0.75#7"));
    test(
        "6369051672525773/4503599627370496",
        Ok("1.4142135623730951"),
        Ok("0x1.6a09e667f3bcd#53"),
    );
    test(
        "884279719003555/281474976710656",
        Ok("3.1415926535897931"),
        Ok("0x3.243f6a8885a3#50"),
    );
    test(
        "6121026514868073/2251799813685248",
        Ok("2.7182818284590451"),
        Ok("0x2.b7e151628aed2#53"),
    );
    test("-1", Ok("-1.0"), Ok("-0x1.0#1"));
    test("-1/2", Ok("-0.50"), Ok("-0x0.8#1"));
    test("-117/256", Ok("-0.4570"), Ok("-0x0.75#7"));
    test(
        "-6369051672525773/4503599627370496",
        Ok("-1.4142135623730951"),
        Ok("-0x1.6a09e667f3bcd#53"),
    );
    test(
        "-884279719003555/281474976710656",
        Ok("-3.1415926535897931"),
        Ok("-0x3.243f6a8885a3#50"),
    );
    test(
        "-6121026514868073/2251799813685248",
        Ok("-2.7182818284590451"),
        Ok("-0x2.b7e151628aed2#53"),
    );

    test(
        "1/3",
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test(
        "22/7",
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test(
        "-1/3",
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test(
        "-22/7",
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );

    let test_big = |x: Rational, out, out_hex| {
        let of = Float::try_from(x.clone());
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);

        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);
    };
    test_big(
        Rational::power_of_2(1000i64),
        Ok("1.1e301"),
        Ok("0x1.0E+250#1"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        Err(&&FloatConversionError::Overflow),
        Err(&&FloatConversionError::Overflow),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        Ok("1.0e323228496"),
        Ok("0x4.0E+268435455#1"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        Ok("1.6e323228496"),
        Ok("0x6.0E+268435455#2"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3073u16, 2048),
        Ok("1.5744e323228496"),
        Ok("0x6.008E+268435455#12"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3071u16, 2048),
        Ok("1.5734e323228496"),
        Ok("0x5.ff8E+268435455#12"),
    );

    test_big(
        Rational::power_of_2(-1000i64),
        Ok("9.3e-302"),
        Ok("0x1.0E-250#1"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Ok("4.8e-323228497"),
        Ok("0x2.0E-268435456#1"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        Ok("2.4e-323228497"),
        Ok("0x1.0E-268435456#1"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        Err(&&FloatConversionError::Underflow),
        Err(&&FloatConversionError::Underflow),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        Err(&&FloatConversionError::Underflow),
        Err(&&FloatConversionError::Underflow),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        Err(&&FloatConversionError::Underflow),
        Err(&&FloatConversionError::Underflow),
    );

    test_big(
        Rational::power_of_2(1000i64),
        Ok("1.1e301"),
        Ok("0x1.0E+250#1"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        Err(&&FloatConversionError::Overflow),
        Err(&&FloatConversionError::Overflow),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        Ok("1.0e323228496"),
        Ok("0x4.0E+268435455#1"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        Ok("1.6e323228496"),
        Ok("0x6.0E+268435455#2"),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );

    test_big(
        -Rational::power_of_2(-1000i64),
        Ok("-9.3e-302"),
        Ok("-0x1.0E-250#1"),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        Ok("-4.8e-323228497"),
        Ok("-0x2.0E-268435456#1"),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        Ok("-2.4e-323228497"),
        Ok("-0x1.0E-268435456#1"),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        Err(&&FloatConversionError::Underflow),
        Err(&&FloatConversionError::Underflow),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        Err(&&FloatConversionError::Underflow),
        Err(&&FloatConversionError::Underflow),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        Err(&&FloatConversionError::Underflow),
        Err(&&FloatConversionError::Underflow),
    );

    test_big(
        -Rational::power_of_2(1000i64),
        Ok("-1.1e301"),
        Ok("-0x1.0E+250#1"),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        Err(&&FloatConversionError::Overflow),
        Err(&&FloatConversionError::Overflow),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        Ok("-1.0e323228496"),
        Ok("-0x4.0E+268435455#1"),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        Ok("-1.6e323228496"),
        Ok("-0x6.0E+268435455#2"),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        Err(&&FloatConversionError::Inexact),
        Err(&&FloatConversionError::Inexact),
    );
}

#[test]
fn test_convertible_from_rational() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(Float::convertible_from(&x), out);
    };
    test("0", true);
    test("1", true);
    test("1/2", true);
    test("117/256", true);
    test("6369051672525773/4503599627370496", true);
    test("884279719003555/281474976710656", true);
    test("6121026514868073/2251799813685248", true);
    test("-1", true);
    test("-1/2", true);
    test("-117/256", true);
    test("-6369051672525773/4503599627370496", true);
    test("-884279719003555/281474976710656", true);
    test("-6121026514868073/2251799813685248", true);

    test("1/3", false);
    test("22/7", false);
    test("-1/3", false);
    test("-22/7", false);

    let test_big = |x: Rational, out| {
        assert_eq!(Float::convertible_from(&x), out);
    };
    test_big(Rational::power_of_2(1000i64), true);
    test_big(Rational::power_of_2(i64::from(Float::MAX_EXPONENT)), false);
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        true,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        true,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        false,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        false,
    );

    test_big(Rational::power_of_2(-1000i64), true);
    test_big(Rational::power_of_2(i64::from(Float::MIN_EXPONENT)), true);
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        true,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        false,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        false,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        false,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        false,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        false,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        false,
    );

    test_big(-Rational::power_of_2(1000i64), true);
    test_big(-Rational::power_of_2(i64::from(Float::MAX_EXPONENT)), false);
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        true,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(3u8, 2),
        true,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        false,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        false,
    );

    test_big(-Rational::power_of_2(-1000i64), true);
    test_big(-Rational::power_of_2(i64::from(Float::MIN_EXPONENT)), true);
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        true,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        false,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        false,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        false,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        false,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        false,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        false,
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn from_rational_prec_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float> + RoundingFrom<&'a Rational>,
    Rational: TryFrom<T>,
{
    rational_gen().test_properties(|x| {
        let xf = emulate_rational_to_float_fn::<T, _>(Float::from_rational_prec_ref, &x);
        assert_eq!(NiceFloat(xf), NiceFloat(T::rounding_from(&x, Nearest).0));
    });
}

#[test]
fn from_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (float_x, o) = Float::from_rational_prec(x.clone(), prec);
        assert!(float_x.is_valid());

        let (float_x_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_x.partial_cmp(&x), Some(o));

        let (float_x_alt, o_alt) = from_rational_prec_round_direct(x.clone(), prec, Nearest);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        let (float_x_alt, o_alt) = from_rational_prec_round_using_div(x.clone(), prec, Nearest);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        let (float_x_alt, o_alt) = from_rational_prec_round_ref_direct(&x, prec, Nearest);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        let (float_x_alt, o_alt) = from_rational_prec_round_ref_using_div(&x, prec, Nearest);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Rational::from(&x));
        assert_eq!(
            ComparableFloatRef(&float_x),
            ComparableFloatRef(&Float::from(&rug_x))
        );

        assert_eq!(x == 0u32, float_x == 0u32);
        assert_eq!(
            float_x.get_prec(),
            if x == 0u32 { None } else { Some(prec) }
        );
        if x != 0u32 {
            assert!(
                (Rational::exact_from(&float_x) - &x)
                    .le_abs(&(Rational::exact_from(float_x.ulp().unwrap()) >> 1))
            );
        }
    });

    apply_fn_to_primitive_floats!(from_rational_prec_properties_helper);
}

#[test]
fn from_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (float_x, o) = Float::from_rational_prec_round(x.clone(), prec, rm);
        assert!(float_x.is_valid());

        let (float_x_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_x.partial_cmp(&x), Some(o));
        match (x >= 0, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }

        let (float_x_alt, o_alt) = from_rational_prec_round_direct(x.clone(), prec, rm);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        let (float_x_alt, o_alt) = from_rational_prec_round_using_div(x.clone(), prec, rm);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        let (float_x_alt, o_alt) = from_rational_prec_round_ref_direct(&x, prec, rm);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        let (float_x_alt, o_alt) = from_rational_prec_round_ref_using_div(&x, prec, rm);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, rug_o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Rational::from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&float_x),
                ComparableFloatRef(&Float::from(&rug_x))
            );
            assert_eq!(rug_o, o);
        }

        assert_eq!(x == 0u32, float_x == 0u32);
        assert_eq!(
            float_x.get_prec(),
            if x == 0u32 { None } else { Some(prec) }
        );
        if x != 0u32 {
            assert!(
                (Rational::exact_from(&float_x) - &x)
                    .le_abs(&Rational::exact_from(float_x.ulp().unwrap()))
            );
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let floor = Float::from_rational_prec_round_ref(&x, prec, Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= x);
        if r_floor != 0u32 {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > x);
        }
        let (floor_x_alt, o_alt) =
            Float::from_rational_prec_round_ref(&x, prec, if x >= 0 { Down } else { Up });
        assert_eq!(
            ComparableFloatRef(&floor_x_alt),
            ComparableFloatRef(&floor.0)
        );
        assert_eq!(o_alt, floor.1);

        let ceiling = Float::from_rational_prec_round_ref(&x, prec, Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= x);
        if r_ceiling != 0u32 {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < x);
        }
        let (ceiling_x_alt, o_alt) =
            Float::from_rational_prec_round_ref(&x, prec, if x >= 0 { Up } else { Down });
        assert_eq!(
            ComparableFloatRef(&ceiling_x_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(o_alt, ceiling.1);

        let nearest = Float::from_rational_prec_round_ref(&x, prec, Nearest);
        let r_nearest = Rational::exact_from(&nearest.0);
        assert!(
            ComparableFloatRef(&nearest.0) == ComparableFloatRef(&floor.0) && nearest.1 == floor.1
                || ComparableFloatRef(&nearest.0) == ComparableFloatRef(&ceiling.0)
                    && nearest.1 == ceiling.1
        );
        if r_nearest != 0u32 {
            assert!((r_nearest - x).le_abs(&(Rational::exact_from(nearest.0.ulp().unwrap()) >> 1)));
        }
    });
}

#[test]
fn float_try_from_rational_properties() {
    rational_gen().test_properties(|x| {
        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        assert_eq!(
            Float::try_from(x.clone()).map(ComparableFloat),
            of.clone().map(ComparableFloat)
        );
        if let Ok(f) = of {
            assert_eq!(-x, -f);
        }
    });
}

#[test]
fn float_convertible_from_rational_properties() {
    rational_gen().test_properties(|x| {
        assert_eq!(Float::convertible_from(&x), Float::try_from(&x).is_ok());
    });

    integer_gen().test_properties(|n| {
        assert!(Float::convertible_from(&Rational::from(n)));
    });
}
