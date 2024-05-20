// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    primitive_float_signed_pair_gen_var_1, primitive_float_signed_pair_gen_var_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    natural_gen_var_2, natural_rounding_mode_pair_gen_var_2,
};
use malachite_q::test_util::generators::{
    rational_gen_var_1, rational_rounding_mode_pair_gen_var_4,
};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_sci_mantissa_and_exponent() {
    let test = |s: &str, mantissa: f32, exponent: i64| {
        let n = Rational::from_str(s).unwrap();
        let (actual_mantissa, actual_exponent) = (&n).sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
        assert_eq!(actual_exponent, exponent);
        let (actual_mantissa, actual_exponent) = n.clone().sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
        assert_eq!(actual_exponent, exponent);
        assert_eq!(NiceFloat((&n).sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(
            SciMantissaAndExponent::<f32, i64, Rational>::sci_exponent(&n),
            exponent
        );
        assert_eq!(NiceFloat(n.clone().sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(
            SciMantissaAndExponent::<f32, i64, Rational>::sci_exponent(n.clone()),
            exponent
        );

        let (actual_mantissa, actual_exponent) = (-n).sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(actual_mantissa), NiceFloat(mantissa));
        assert_eq!(actual_exponent, exponent);
    };
    test("3", 1.5, 1);
    test("123", 1.921875, 6);
    test("1000000000", 1.8626451, 29);

    test("16777216", 1.0, 24);
    test("16777218", 1.0000001, 24);
    test("16777217", 1.0, 24);

    test("33554432", 1.0, 25);
    test("33554436", 1.0000001, 25);
    test("33554440", 1.0000002, 25);

    test("33554433", 1.0, 25);
    test("33554434", 1.0, 25);
    test("33554435", 1.0000001, 25);
    test("33554437", 1.0000001, 25);
    test("33554438", 1.0000002, 25);
    test("33554439", 1.0000002, 25);
    test("340282346638528859811704183484516925439", 1.9999999, 127);
    test("340282346638528859811704183484516925440", 1.9999999, 127);
    test("340282346638528859811704183484516925441", 1.9999999, 127);
    test(
        "10000000000000000000000000000000000000000000000000000",
        1.670478,
        172,
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        1.8920966,
        458,
    );

    test("1/3", 1.3333334, -2);
    test("1/1024", 1.0, -10);
    test("22/7", 1.5714285, 1);
    test("936851431250/1397", 1.2491208, 29);
}

#[test]
fn test_sci_mantissa_and_exponent_round() {
    let test = |n: &str, rm: RoundingMode, out: Option<(f32, i64, Ordering)>| {
        let r = Rational::from_str(n).unwrap();
        let actual_out = r.clone().sci_mantissa_and_exponent_round(rm);
        assert_eq!(
            actual_out.map(|(m, e, o)| (NiceFloat(m), e, o)),
            out.map(|(m, e, o)| (NiceFloat(m), e, o))
        );
        let actual_out = r.sci_mantissa_and_exponent_round_ref(rm);
        assert_eq!(
            actual_out.map(|(m, e, o)| (NiceFloat(m), e, o)),
            out.map(|(m, e, o)| (NiceFloat(m), e, o))
        );
        let actual_out = (-r).sci_mantissa_and_exponent_round(rm);
        assert_eq!(
            actual_out.map(|(m, e, o)| (NiceFloat(m), e, o)),
            out.map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    };
    test("3", Floor, Some((1.5, 1, Equal)));
    test("3", Down, Some((1.5, 1, Equal)));
    test("3", Ceiling, Some((1.5, 1, Equal)));
    test("3", Up, Some((1.5, 1, Equal)));
    test("3", Nearest, Some((1.5, 1, Equal)));
    test("3", Exact, Some((1.5, 1, Equal)));

    test("123", Floor, Some((1.921875, 6, Equal)));
    test("123", Down, Some((1.921875, 6, Equal)));
    test("123", Ceiling, Some((1.921875, 6, Equal)));
    test("123", Up, Some((1.921875, 6, Equal)));
    test("123", Nearest, Some((1.921875, 6, Equal)));
    test("123", Exact, Some((1.921875, 6, Equal)));

    test("1000000000", Floor, Some((1.8626451, 29, Equal)));
    test("1000000000", Down, Some((1.8626451, 29, Equal)));
    test("1000000000", Ceiling, Some((1.8626451, 29, Equal)));
    test("1000000000", Up, Some((1.8626451, 29, Equal)));
    test("1000000000", Nearest, Some((1.8626451, 29, Equal)));
    test("1000000000", Exact, Some((1.8626451, 29, Equal)));

    test("16777216", Floor, Some((1.0, 24, Equal)));
    test("16777216", Down, Some((1.0, 24, Equal)));
    test("16777216", Ceiling, Some((1.0, 24, Equal)));
    test("16777216", Up, Some((1.0, 24, Equal)));
    test("16777216", Nearest, Some((1.0, 24, Equal)));
    test("16777216", Exact, Some((1.0, 24, Equal)));

    test("16777218", Floor, Some((1.0000001, 24, Equal)));
    test("16777218", Down, Some((1.0000001, 24, Equal)));
    test("16777218", Ceiling, Some((1.0000001, 24, Equal)));
    test("16777218", Up, Some((1.0000001, 24, Equal)));
    test("16777218", Nearest, Some((1.0000001, 24, Equal)));
    test("16777218", Exact, Some((1.0000001, 24, Equal)));

    test("16777217", Floor, Some((1.0, 24, Less)));
    test("16777217", Down, Some((1.0, 24, Less)));
    test("16777217", Ceiling, Some((1.0000001, 24, Greater)));
    test("16777217", Up, Some((1.0000001, 24, Greater)));
    test("16777217", Nearest, Some((1.0, 24, Less)));
    test("16777217", Exact, None);

    test("33554432", Floor, Some((1.0, 25, Equal)));
    test("33554432", Down, Some((1.0, 25, Equal)));
    test("33554432", Ceiling, Some((1.0, 25, Equal)));
    test("33554432", Up, Some((1.0, 25, Equal)));
    test("33554432", Nearest, Some((1.0, 25, Equal)));
    test("33554432", Exact, Some((1.0, 25, Equal)));

    test("33554436", Floor, Some((1.0000001, 25, Equal)));
    test("33554436", Down, Some((1.0000001, 25, Equal)));
    test("33554436", Ceiling, Some((1.0000001, 25, Equal)));
    test("33554436", Up, Some((1.0000001, 25, Equal)));
    test("33554436", Nearest, Some((1.0000001, 25, Equal)));
    test("33554436", Exact, Some((1.0000001, 25, Equal)));

    test("33554440", Floor, Some((1.0000002, 25, Equal)));
    test("33554440", Down, Some((1.0000002, 25, Equal)));
    test("33554440", Ceiling, Some((1.0000002, 25, Equal)));
    test("33554440", Up, Some((1.0000002, 25, Equal)));
    test("33554440", Nearest, Some((1.0000002, 25, Equal)));
    test("33554440", Exact, Some((1.0000002, 25, Equal)));

    test("33554433", Floor, Some((1.0, 25, Less)));
    test("33554433", Down, Some((1.0, 25, Less)));
    test("33554433", Ceiling, Some((1.0000001, 25, Greater)));
    test("33554433", Up, Some((1.0000001, 25, Greater)));
    test("33554433", Nearest, Some((1.0, 25, Less)));
    test("33554433", Exact, None);

    test("33554434", Floor, Some((1.0, 25, Less)));
    test("33554434", Down, Some((1.0, 25, Less)));
    test("33554434", Ceiling, Some((1.0000001, 25, Greater)));
    test("33554434", Up, Some((1.0000001, 25, Greater)));
    test("33554434", Nearest, Some((1.0, 25, Less)));
    test("33554434", Exact, None);

    test("33554435", Floor, Some((1.0, 25, Less)));
    test("33554435", Down, Some((1.0, 25, Less)));
    test("33554435", Ceiling, Some((1.0000001, 25, Greater)));
    test("33554435", Up, Some((1.0000001, 25, Greater)));
    test("33554435", Nearest, Some((1.0000001, 25, Greater)));
    test("33554435", Exact, None);

    test("33554437", Floor, Some((1.0000001, 25, Less)));
    test("33554437", Down, Some((1.0000001, 25, Less)));
    test("33554437", Ceiling, Some((1.0000002, 25, Greater)));
    test("33554437", Up, Some((1.0000002, 25, Greater)));
    test("33554437", Nearest, Some((1.0000001, 25, Less)));
    test("33554437", Exact, None);

    test("33554438", Floor, Some((1.0000001, 25, Less)));
    test("33554438", Down, Some((1.0000001, 25, Less)));
    test("33554438", Ceiling, Some((1.0000002, 25, Greater)));
    test("33554438", Up, Some((1.0000002, 25, Greater)));
    test("33554438", Nearest, Some((1.0000002, 25, Greater)));
    test("33554438", Exact, None);

    test("33554439", Floor, Some((1.0000001, 25, Less)));
    test("33554439", Down, Some((1.0000001, 25, Less)));
    test("33554439", Ceiling, Some((1.0000002, 25, Greater)));
    test("33554439", Up, Some((1.0000002, 25, Greater)));
    test("33554439", Nearest, Some((1.0000002, 25, Greater)));
    test("33554439", Exact, None);

    test(
        "340282346638528859811704183484516925439",
        Floor,
        Some((1.9999998, 127, Less)),
    );
    test(
        "340282346638528859811704183484516925439",
        Down,
        Some((1.9999998, 127, Less)),
    );
    test(
        "340282346638528859811704183484516925439",
        Ceiling,
        Some((1.9999999, 127, Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        Up,
        Some((1.9999999, 127, Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        Nearest,
        Some((1.9999999, 127, Greater)),
    );
    test("340282346638528859811704183484516925439", Exact, None);

    test(
        "340282346638528859811704183484516925440",
        Floor,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        Down,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        Ceiling,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        Up,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        Nearest,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        Exact,
        Some((1.9999999, 127, Equal)),
    );

    test(
        "340282346638528859811704183484516925441",
        Floor,
        Some((1.9999999, 127, Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        Down,
        Some((1.9999999, 127, Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        Ceiling,
        Some((1.0, 128, Greater)),
    );

    test(
        "340282346638528859811704183484516925441",
        Up,
        Some((1.0, 128, Greater)),
    );
    test(
        "340282346638528859811704183484516925441",
        Nearest,
        Some((1.9999999, 127, Less)),
    );
    test("340282346638528859811704183484516925441", Exact, None);

    test(
        "10000000000000000000000000000000000000000000000000000",
        Floor,
        Some((1.6704779, 172, Less)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Down,
        Some((1.6704779, 172, Less)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Ceiling,
        Some((1.670478, 172, Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Up,
        Some((1.670478, 172, Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Nearest,
        Some((1.670478, 172, Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Exact,
        None,
    );

    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Floor,
        Some((1.8920966, 458, Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Down,
        Some((1.8920966, 458, Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Ceiling,
        Some((1.8920968, 458, Greater)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Up,
        Some((1.8920968, 458, Greater)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Nearest,
        Some((1.8920966, 458, Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Exact,
        None,
    );

    test("1/3", Floor, Some((1.3333333, -2, Less)));
    test("1/3", Ceiling, Some((1.3333334, -2, Greater)));
    test("1/3", Down, Some((1.3333333, -2, Less)));
    test("1/3", Up, Some((1.3333334, -2, Greater)));
    test("1/3", Nearest, Some((1.3333334, -2, Greater)));
    test("1/3", Exact, None);

    test("1/1024", Floor, Some((1.0, -10, Equal)));
    test("1/1024", Ceiling, Some((1.0, -10, Equal)));
    test("1/1024", Down, Some((1.0, -10, Equal)));
    test("1/1024", Up, Some((1.0, -10, Equal)));
    test("1/1024", Nearest, Some((1.0, -10, Equal)));
    test("1/1024", Exact, Some((1.0, -10, Equal)));

    test("22/7", Floor, Some((1.5714285, 1, Less)));
    test("22/7", Ceiling, Some((1.5714287, 1, Greater)));
    test("22/7", Down, Some((1.5714285, 1, Less)));
    test("22/7", Up, Some((1.5714287, 1, Greater)));
    test("22/7", Nearest, Some((1.5714285, 1, Less)));
    test("22/7", Exact, None);

    test("936851431250/1397", Floor, Some((1.2491207, 29, Less)));
    test("936851431250/1397", Ceiling, Some((1.2491208, 29, Greater)));
    test("936851431250/1397", Down, Some((1.2491207, 29, Less)));
    test("936851431250/1397", Up, Some((1.2491208, 29, Greater)));
    test("936851431250/1397", Nearest, Some((1.2491208, 29, Greater)));
    test("936851431250/1397", Exact, None);

    test(
        "1073741823/1099511627776",
        Floor,
        Some((1.9999999, -11, Less)),
    );
    test(
        "1073741823/1099511627776",
        Ceiling,
        Some((1.0, -10, Greater)),
    );
    test(
        "1073741823/1099511627776",
        Down,
        Some((1.9999999, -11, Less)),
    );
    test("1073741823/1099511627776", Up, Some((1.0, -10, Greater)));
    test(
        "1073741823/1099511627776",
        Nearest,
        Some((1.0, -10, Greater)),
    );
    test("1073741823/1099511627776", Exact, None);

    test("-3", Floor, Some((1.5, 1, Equal)));
    test("-3", Down, Some((1.5, 1, Equal)));
    test("-3", Ceiling, Some((1.5, 1, Equal)));
    test("-3", Up, Some((1.5, 1, Equal)));
    test("-3", Nearest, Some((1.5, 1, Equal)));
    test("-3", Exact, Some((1.5, 1, Equal)));

    test("-123", Floor, Some((1.921875, 6, Equal)));
    test("-123", Down, Some((1.921875, 6, Equal)));
    test("-123", Ceiling, Some((1.921875, 6, Equal)));
    test("-123", Up, Some((1.921875, 6, Equal)));
    test("-123", Nearest, Some((1.921875, 6, Equal)));
    test("-123", Exact, Some((1.921875, 6, Equal)));

    test("-1000000000", Floor, Some((1.8626451, 29, Equal)));
    test("-1000000000", Down, Some((1.8626451, 29, Equal)));
    test("-1000000000", Ceiling, Some((1.8626451, 29, Equal)));
    test("-1000000000", Up, Some((1.8626451, 29, Equal)));
    test("-1000000000", Nearest, Some((1.8626451, 29, Equal)));
    test("-1000000000", Exact, Some((1.8626451, 29, Equal)));

    test("-16777216", Floor, Some((1.0, 24, Equal)));
    test("-16777216", Down, Some((1.0, 24, Equal)));
    test("-16777216", Ceiling, Some((1.0, 24, Equal)));
    test("-16777216", Up, Some((1.0, 24, Equal)));
    test("-16777216", Nearest, Some((1.0, 24, Equal)));
    test("-16777216", Exact, Some((1.0, 24, Equal)));

    test("-16777218", Floor, Some((1.0000001, 24, Equal)));
    test("-16777218", Down, Some((1.0000001, 24, Equal)));
    test("-16777218", Ceiling, Some((1.0000001, 24, Equal)));
    test("-16777218", Up, Some((1.0000001, 24, Equal)));
    test("-16777218", Nearest, Some((1.0000001, 24, Equal)));
    test("-16777218", Exact, Some((1.0000001, 24, Equal)));

    test("-16777217", Floor, Some((1.0, 24, Less)));
    test("-16777217", Down, Some((1.0, 24, Less)));
    test("-16777217", Ceiling, Some((1.0000001, 24, Greater)));
    test("-16777217", Up, Some((1.0000001, 24, Greater)));
    test("-16777217", Nearest, Some((1.0, 24, Less)));
    test("-16777217", Exact, None);

    test("-33554432", Floor, Some((1.0, 25, Equal)));
    test("-33554432", Down, Some((1.0, 25, Equal)));
    test("-33554432", Ceiling, Some((1.0, 25, Equal)));
    test("-33554432", Up, Some((1.0, 25, Equal)));
    test("-33554432", Nearest, Some((1.0, 25, Equal)));
    test("-33554432", Exact, Some((1.0, 25, Equal)));

    test("-33554436", Floor, Some((1.0000001, 25, Equal)));
    test("-33554436", Down, Some((1.0000001, 25, Equal)));
    test("-33554436", Ceiling, Some((1.0000001, 25, Equal)));
    test("-33554436", Up, Some((1.0000001, 25, Equal)));
    test("-33554436", Nearest, Some((1.0000001, 25, Equal)));
    test("-33554436", Exact, Some((1.0000001, 25, Equal)));

    test("-33554440", Floor, Some((1.0000002, 25, Equal)));
    test("-33554440", Down, Some((1.0000002, 25, Equal)));
    test("-33554440", Ceiling, Some((1.0000002, 25, Equal)));
    test("-33554440", Up, Some((1.0000002, 25, Equal)));
    test("-33554440", Nearest, Some((1.0000002, 25, Equal)));
    test("-33554440", Exact, Some((1.0000002, 25, Equal)));

    test("-33554433", Floor, Some((1.0, 25, Less)));
    test("-33554433", Down, Some((1.0, 25, Less)));
    test("-33554433", Ceiling, Some((1.0000001, 25, Greater)));
    test("-33554433", Up, Some((1.0000001, 25, Greater)));
    test("-33554433", Nearest, Some((1.0, 25, Less)));
    test("-33554433", Exact, None);

    test("-33554434", Floor, Some((1.0, 25, Less)));
    test("-33554434", Down, Some((1.0, 25, Less)));
    test("-33554434", Ceiling, Some((1.0000001, 25, Greater)));
    test("-33554434", Up, Some((1.0000001, 25, Greater)));
    test("-33554434", Nearest, Some((1.0, 25, Less)));
    test("-33554434", Exact, None);

    test("-33554435", Floor, Some((1.0, 25, Less)));
    test("-33554435", Down, Some((1.0, 25, Less)));
    test("-33554435", Ceiling, Some((1.0000001, 25, Greater)));
    test("-33554435", Up, Some((1.0000001, 25, Greater)));
    test("-33554435", Nearest, Some((1.0000001, 25, Greater)));
    test("-33554435", Exact, None);

    test("-33554437", Floor, Some((1.0000001, 25, Less)));
    test("-33554437", Down, Some((1.0000001, 25, Less)));
    test("-33554437", Ceiling, Some((1.0000002, 25, Greater)));
    test("-33554437", Up, Some((1.0000002, 25, Greater)));
    test("-33554437", Nearest, Some((1.0000001, 25, Less)));
    test("-33554437", Exact, None);

    test("-33554438", Floor, Some((1.0000001, 25, Less)));
    test("-33554438", Down, Some((1.0000001, 25, Less)));
    test("-33554438", Ceiling, Some((1.0000002, 25, Greater)));
    test("-33554438", Up, Some((1.0000002, 25, Greater)));
    test("-33554438", Nearest, Some((1.0000002, 25, Greater)));
    test("-33554438", Exact, None);

    test("-33554439", Floor, Some((1.0000001, 25, Less)));
    test("-33554439", Down, Some((1.0000001, 25, Less)));
    test("-33554439", Ceiling, Some((1.0000002, 25, Greater)));
    test("-33554439", Up, Some((1.0000002, 25, Greater)));
    test("-33554439", Nearest, Some((1.0000002, 25, Greater)));
    test("-33554439", Exact, None);

    test(
        "-340282346638528859811704183484516925439",
        Floor,
        Some((1.9999998, 127, Less)),
    );
    test(
        "-340282346638528859811704183484516925439",
        Down,
        Some((1.9999998, 127, Less)),
    );
    test(
        "-340282346638528859811704183484516925439",
        Ceiling,
        Some((1.9999999, 127, Greater)),
    );
    test(
        "-340282346638528859811704183484516925439",
        Up,
        Some((1.9999999, 127, Greater)),
    );
    test(
        "-340282346638528859811704183484516925439",
        Nearest,
        Some((1.9999999, 127, Greater)),
    );
    test("-340282346638528859811704183484516925439", Exact, None);

    test(
        "-340282346638528859811704183484516925440",
        Floor,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        Down,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        Ceiling,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        Up,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        Nearest,
        Some((1.9999999, 127, Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        Exact,
        Some((1.9999999, 127, Equal)),
    );

    test(
        "-340282346638528859811704183484516925441",
        Floor,
        Some((1.9999999, 127, Less)),
    );
    test(
        "-340282346638528859811704183484516925441",
        Down,
        Some((1.9999999, 127, Less)),
    );
    test(
        "-340282346638528859811704183484516925441",
        Ceiling,
        Some((1.0, 128, Greater)),
    );

    test(
        "-340282346638528859811704183484516925441",
        Up,
        Some((1.0, 128, Greater)),
    );
    test(
        "-340282346638528859811704183484516925441",
        Nearest,
        Some((1.9999999, 127, Less)),
    );
    test("-340282346638528859811704183484516925441", Exact, None);

    test(
        "-10000000000000000000000000000000000000000000000000000",
        Floor,
        Some((1.6704779, 172, Less)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Down,
        Some((1.6704779, 172, Less)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Ceiling,
        Some((1.670478, 172, Greater)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Up,
        Some((1.670478, 172, Greater)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Nearest,
        Some((1.670478, 172, Greater)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Exact,
        None,
    );

    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Floor,
        Some((1.8920966, 458, Less)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Down,
        Some((1.8920966, 458, Less)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Ceiling,
        Some((1.8920968, 458, Greater)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Up,
        Some((1.8920968, 458, Greater)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Nearest,
        Some((1.8920966, 458, Less)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        Exact,
        None,
    );

    test("-1/3", Floor, Some((1.3333333, -2, Less)));
    test("-1/3", Ceiling, Some((1.3333334, -2, Greater)));
    test("-1/3", Down, Some((1.3333333, -2, Less)));
    test("-1/3", Up, Some((1.3333334, -2, Greater)));
    test("-1/3", Nearest, Some((1.3333334, -2, Greater)));
    test("-1/3", Exact, None);

    test("-1/1024", Floor, Some((1.0, -10, Equal)));
    test("-1/1024", Ceiling, Some((1.0, -10, Equal)));
    test("-1/1024", Down, Some((1.0, -10, Equal)));
    test("-1/1024", Up, Some((1.0, -10, Equal)));
    test("-1/1024", Nearest, Some((1.0, -10, Equal)));
    test("-1/1024", Exact, Some((1.0, -10, Equal)));

    test("-22/7", Floor, Some((1.5714285, 1, Less)));
    test("-22/7", Ceiling, Some((1.5714287, 1, Greater)));
    test("-22/7", Down, Some((1.5714285, 1, Less)));
    test("-22/7", Up, Some((1.5714287, 1, Greater)));
    test("-22/7", Nearest, Some((1.5714285, 1, Less)));
    test("-22/7", Exact, None);

    test("-936851431250/1397", Floor, Some((1.2491207, 29, Less)));
    test(
        "-936851431250/1397",
        Ceiling,
        Some((1.2491208, 29, Greater)),
    );
    test("-936851431250/1397", Down, Some((1.2491207, 29, Less)));
    test("-936851431250/1397", Up, Some((1.2491208, 29, Greater)));
    test(
        "-936851431250/1397",
        Nearest,
        Some((1.2491208, 29, Greater)),
    );
    test("-936851431250/1397", Exact, None);

    test(
        "-1073741823/1099511627776",
        Floor,
        Some((1.9999999, -11, Less)),
    );
    test(
        "-1073741823/1099511627776",
        Ceiling,
        Some((1.0, -10, Greater)),
    );
    test(
        "-1073741823/1099511627776",
        Down,
        Some((1.9999999, -11, Less)),
    );
    test("-1073741823/1099511627776", Up, Some((1.0, -10, Greater)));
    test(
        "-1073741823/1099511627776",
        Nearest,
        Some((1.0, -10, Greater)),
    );
    test("-1073741823/1099511627776", Exact, None);
}

#[test]
fn test_from_sci_mantissa_and_exponent() {
    let test = |mantissa: f32, exponent: i64, out: Option<&str>| {
        assert_eq!(
            <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
                mantissa, exponent
            ),
            out.map(|s| Rational::from_str(s).unwrap())
        );
        assert_eq!(
            <Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
                mantissa, exponent
            ),
            out.map(|s| Rational::from_str(s).unwrap())
        );
    };
    test(1.5, 1, Some("3"));
    test(1.51, 1, Some("6333399/2097152"));
    test(1.921875, 6, Some("123"));
    test(
        1.670478,
        172,
        Some("10000000254586612611935772707803116801852191350456320"),
    );

    test(2.0, 1, None);
    test(10.0, 1, None);
    test(0.5, 1, None);
}

fn from_sci_mantissa_and_exponent_fail_helper<T: PrimitiveFloat>()
where
    Rational: SciMantissaAndExponent<T, i64>,
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    assert_panic!(
        <&Rational as SciMantissaAndExponent<T, i64, _>>::from_sci_mantissa_and_exponent(
            T::ZERO,
            0
        )
    );
    assert_panic!(
        <Rational as SciMantissaAndExponent<T, i64, _>>::from_sci_mantissa_and_exponent(T::ZERO, 0)
    );
}

#[test]
fn from_sci_mantissa_and_exponent_fail() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_fail_helper);
}

fn sci_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>()
where
    Rational: SciMantissaAndExponent<T, i64>,
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
    for<'a> &'a Natural: SciMantissaAndExponent<T, u64, Natural>,
{
    rational_gen_var_1().test_properties(|n| {
        let (mantissa, exponent) = (&n).sci_mantissa_and_exponent();
        assert_eq!(NiceFloat((&n).sci_mantissa()), NiceFloat(mantissa));
        assert_eq!((&n).sci_exponent(), exponent);
        let (mantissa_alt, exponent_alt) = n.clone().sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(mantissa_alt), NiceFloat(mantissa));
        assert_eq!(exponent_alt, exponent);
        assert_eq!(NiceFloat(n.clone().sci_mantissa()), NiceFloat(mantissa));
        assert_eq!(n.clone().sci_exponent(), exponent);

        assert!(mantissa >= T::ONE);
        assert!(mantissa < T::TWO);
        assert_eq!(
            n.sci_mantissa_and_exponent_round(Nearest)
                .map(|(m, e, _): (T, i64, Ordering)| (NiceFloat(m), e)),
            Some((NiceFloat(mantissa), exponent))
        );
    });

    natural_gen_var_2().test_properties(|x| {
        let (mantissa_1, exponent_1) = x.sci_mantissa_and_exponent();
        let (mantissa_2, exponent_2) = (&Rational::from(x)).sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(mantissa_1), NiceFloat(mantissa_2));
        assert_eq!(i64::exact_from(exponent_1), exponent_2);
    });
}

#[test]
fn sci_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_properties_helper);
}

fn sci_mantissa_and_exponent_round_properties_helper<T: PrimitiveFloat>()
where
    Rational: SciMantissaAndExponent<T, i64>,
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    rational_rounding_mode_pair_gen_var_4().test_properties(|(n, rm)| {
        let result = n.sci_mantissa_and_exponent_round_ref::<T>(rm);
        assert_eq!(
            n.clone()
                .sci_mantissa_and_exponent_round::<T>(rm)
                .map(|(m, e, o)| (NiceFloat(m), e, o)),
            result.map(|(m, e, o)| (NiceFloat(m), e, o))
        );
        if let Some((mantissa, exponent, o)) = result {
            assert!(mantissa >= T::ONE);
            assert!(mantissa < T::TWO);
            let x = <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(
                mantissa, exponent,
            )
            .unwrap();
            if rm == Exact {
                assert_eq!(x.partial_cmp_abs(&n), Some(Equal));
            }
            assert_eq!(x.partial_cmp_abs(&n), Some(o));
            match rm {
                Floor | Down => assert_ne!(o, Greater),
                Ceiling | Up => assert_ne!(o, Less),
                Exact => assert_eq!(o, Equal),
                _ => {}
            }
        }
    });

    rational_gen_var_1().test_properties(|n| {
        let (floor_mantissa, floor_exponent, floor_o) =
            n.sci_mantissa_and_exponent_round_ref::<T>(Floor).unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round_ref::<T>(Down).unwrap(),
            (floor_mantissa, floor_exponent, floor_o)
        );
        let (ceiling_mantissa, ceiling_exponent, ceiling_o) =
            n.sci_mantissa_and_exponent_round_ref::<T>(Ceiling).unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round_ref::<T>(Up).unwrap(),
            (ceiling_mantissa, ceiling_exponent, ceiling_o)
        );
        let (nearest_mantissa, nearest_exponent, nearest_o) =
            n.sci_mantissa_and_exponent_round_ref::<T>(Nearest).unwrap();
        if let Some((mantissa, exponent, o)) = n.sci_mantissa_and_exponent_round_ref::<T>(Exact) {
            assert_eq!(o, Equal);
            assert_eq!(floor_mantissa, mantissa);
            assert_eq!(ceiling_mantissa, mantissa);
            assert_eq!(nearest_mantissa, mantissa);
            assert_eq!(floor_exponent, exponent);
            assert_eq!(ceiling_exponent, exponent);
            assert_eq!(nearest_exponent, exponent);
        } else {
            assert_eq!(floor_o, Less);
            assert_eq!(ceiling_o, Greater);
            assert_ne!(
                (floor_mantissa, floor_exponent),
                (ceiling_mantissa, ceiling_exponent)
            );
            assert!(
                (nearest_mantissa, nearest_exponent, nearest_o)
                    == (floor_mantissa, floor_exponent, floor_o)
                    || (nearest_mantissa, nearest_exponent, nearest_o)
                        == (ceiling_mantissa, ceiling_exponent, ceiling_o)
            );
            if ceiling_mantissa == T::ONE {
                assert_eq!(floor_mantissa, T::TWO.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent - 1);
            } else {
                assert_eq!(floor_mantissa, ceiling_mantissa.next_lower());
                assert_eq!(floor_exponent, ceiling_exponent);
            }
        }
    });

    natural_rounding_mode_pair_gen_var_2().test_properties(|(x, rm)| {
        assert_eq!(
            x.sci_mantissa_and_exponent_round(rm)
                .map(|(m, e, o): (T, u64, Ordering)| (NiceFloat(m), i64::exact_from(e), o)),
            Rational::from(x)
                .sci_mantissa_and_exponent_round(rm)
                .map(|(m, e, o)| (NiceFloat(m), e, o))
        );
    });
}

#[test]
fn sci_mantissa_and_exponent_round_properties() {
    apply_fn_to_primitive_floats!(sci_mantissa_and_exponent_round_properties_helper);
}

fn from_sci_mantissa_and_exponent_properties_helper<T: PrimitiveFloat>()
where
    Rational: SciMantissaAndExponent<T, i64>,
    for<'a> &'a Rational: SciMantissaAndExponent<T, i64, Rational>,
{
    primitive_float_signed_pair_gen_var_1::<T, i64>().test_properties(|(m, e)| {
        let on =
            <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e);
        assert_eq!(
            <Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e),
            on
        );
        assert_eq!(on.is_some(), m >= T::ONE && m < T::TWO);
    });

    primitive_float_signed_pair_gen_var_2::<T>().test_properties(|(m, e)| {
        let x =
            <&Rational as SciMantissaAndExponent<_, _, _>>::from_sci_mantissa_and_exponent(m, e)
                .unwrap();
        assert!(m >= T::ONE && m < T::TWO);
        let (m_alt, e_alt) = (&x).sci_mantissa_and_exponent();
        assert_eq!(NiceFloat(m_alt), NiceFloat(m));
        assert_eq!(e_alt, e);
    });
}

#[test]
fn from_sci_mantissa_and_exponent_properties() {
    apply_fn_to_primitive_floats!(from_sci_mantissa_and_exponent_properties_helper);
}
