// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_1, integer_rounding_mode_pair_gen_var_1,
};
use malachite_q::conversion::primitive_float_from_rational::FloatFromRationalError;
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_4, rational_gen_var_5, rational_gen_var_6,
    rational_rounding_mode_pair_gen_var_5,
};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_f32_rounding_from_rational() {
    let max = Rational::exact_from(f32::MAX_FINITE);
    let test = |s: &str, rm: RoundingMode, out, o_out| {
        let u = Rational::from_str(s).unwrap();
        let (f, o) = f32::rounding_from(&u, rm);
        assert_eq!(NiceFloat(f), NiceFloat(out));
        assert_eq!(o, o_out);
        let (f, o) = f32::rounding_from(u.clone(), rm);
        assert_eq!(NiceFloat(f), NiceFloat(out));
        assert_eq!(o, o_out);
        if rm == Down && u.lt_abs(&max) && NiceFloat(out) != NiceFloat(-0.0) {
            assert_eq!(
                NiceFloat(rug::Rational::from_str(s).unwrap().to_f32()),
                NiceFloat(out)
            );
        }
    };
    test("3", Exact, 3.0, Equal);
    test("-3", Exact, -3.0, Equal);
    test("123", Exact, 123.0, Equal);
    test("-123", Exact, -123.0, Equal);
    test("0", Exact, 0.0, Equal);
    test("1000000000", Exact, 1.0e9, Equal);
    test("-1000000000", Exact, -1.0e9, Equal);
    test("16777216", Exact, 1.6777216e7, Equal);
    test("-16777216", Exact, -1.6777216e7, Equal);
    test("16777218", Exact, 1.6777218e7, Equal);
    test("-16777218", Exact, -1.6777218e7, Equal);

    test("16777217", Floor, 1.6777216e7, Less);
    test("16777217", Down, 1.6777216e7, Less);
    test("16777217", Ceiling, 1.6777218e7, Greater);
    test("16777217", Up, 1.6777218e7, Greater);
    test("16777217", Nearest, 1.6777216e7, Less);

    test("-16777217", Floor, -1.6777218e7, Less);
    test("-16777217", Down, -1.6777216e7, Greater);
    test("-16777217", Ceiling, -1.6777216e7, Greater);
    test("-16777217", Up, -1.6777218e7, Less);
    test("-16777217", Nearest, -1.6777216e7, Greater);

    test("33554432", Exact, 3.3554432e7, Equal);
    test("-33554432", Exact, -3.3554432e7, Equal);
    test("33554436", Exact, 3.3554436e7, Equal);
    test("-33554436", Exact, -3.3554436e7, Equal);

    test("33554433", Floor, 3.3554432e7, Less);
    test("33554433", Down, 3.3554432e7, Less);
    test("33554433", Ceiling, 3.3554436e7, Greater);
    test("33554433", Up, 3.3554436e7, Greater);
    test("33554433", Nearest, 3.3554432e7, Less);

    test("-33554433", Floor, -3.3554436e7, Less);
    test("-33554433", Down, -3.3554432e7, Greater);
    test("-33554433", Ceiling, -3.3554432e7, Greater);
    test("-33554433", Up, -3.3554436e7, Less);
    test("-33554433", Nearest, -3.3554432e7, Greater);

    test("33554434", Nearest, 3.3554432e7, Less);
    test("-33554434", Nearest, -3.3554432e7, Greater);
    test("33554435", Nearest, 3.3554436e7, Greater);
    test("-33554435", Nearest, -3.3554436e7, Less);

    test(
        "340282346638528859811704183484516925439",
        Floor,
        3.4028233e38,
        Less,
    );
    test(
        "340282346638528859811704183484516925439",
        Down,
        3.4028233e38,
        Less,
    );
    test(
        "340282346638528859811704183484516925439",
        Ceiling,
        3.4028235e38,
        Greater,
    );
    test(
        "340282346638528859811704183484516925439",
        Up,
        3.4028235e38,
        Greater,
    );
    test(
        "340282346638528859811704183484516925439",
        Nearest,
        3.4028235e38,
        Greater,
    );

    test(
        "-340282346638528859811704183484516925439",
        Floor,
        -3.4028235e38,
        Less,
    );
    test(
        "-340282346638528859811704183484516925439",
        Down,
        -3.4028233e38,
        Greater,
    );
    test(
        "-340282346638528859811704183484516925439",
        Ceiling,
        -3.4028233e38,
        Greater,
    );
    test(
        "-340282346638528859811704183484516925439",
        Up,
        -3.4028235e38,
        Less,
    );
    test(
        "-340282346638528859811704183484516925439",
        Nearest,
        -3.4028235e38,
        Less,
    );

    test(
        "340282346638528859811704183484516925440",
        Exact,
        3.4028235e38,
        Equal,
    );
    test(
        "-340282346638528859811704183484516925440",
        Exact,
        -3.4028235e38,
        Equal,
    );

    test(
        "340282346638528859811704183484516925441",
        Floor,
        3.4028235e38,
        Less,
    );
    test(
        "340282346638528859811704183484516925441",
        Down,
        3.4028235e38,
        Less,
    );
    test(
        "340282346638528859811704183484516925441",
        Nearest,
        3.4028235e38,
        Less,
    );
    test(
        "340282346638528859811704183484516925441",
        Ceiling,
        f32::INFINITY,
        Greater,
    );
    test(
        "340282346638528859811704183484516925441",
        Up,
        f32::INFINITY,
        Greater,
    );

    test(
        "-340282346638528859811704183484516925441",
        Floor,
        f32::NEGATIVE_INFINITY,
        Less,
    );
    test(
        "-340282346638528859811704183484516925441",
        Down,
        -3.4028235e38,
        Greater,
    );
    test(
        "-340282346638528859811704183484516925441",
        Nearest,
        -3.4028235e38,
        Greater,
    );
    test(
        "-340282346638528859811704183484516925441",
        Ceiling,
        -3.4028235e38,
        Greater,
    );
    test(
        "-340282346638528859811704183484516925441",
        Up,
        f32::NEGATIVE_INFINITY,
        Less,
    );

    test(
        "10000000000000000000000000000000000000000000000000000",
        Floor,
        3.4028235e38,
        Less,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Down,
        3.4028235e38,
        Less,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Nearest,
        3.4028235e38,
        Less,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Ceiling,
        f32::INFINITY,
        Greater,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Up,
        f32::INFINITY,
        Greater,
    );

    test(
        "-10000000000000000000000000000000000000000000000000000",
        Floor,
        f32::NEGATIVE_INFINITY,
        Less,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Down,
        -3.4028235e38,
        Greater,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Nearest,
        -3.4028235e38,
        Greater,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Ceiling,
        -3.4028235e38,
        Greater,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Up,
        f32::NEGATIVE_INFINITY,
        Less,
    );

    test("1125899873419263", Floor, 1.12589984e15, Less);
    test("1125899873419263", Down, 1.12589984e15, Less);
    test("1125899873419263", Ceiling, 1.1258999e15, Greater);
    test("1125899873419263", Up, 1.1258999e15, Greater);
    test("1125899873419263", Nearest, 1.1258999e15, Greater);

    test("-1125899873419263", Floor, -1.1258999e15, Less);
    test("-1125899873419263", Down, -1.12589984e15, Greater);
    test("-1125899873419263", Ceiling, -1.12589984e15, Greater);
    test("-1125899873419263", Up, -1.1258999e15, Less);
    test("-1125899873419263", Nearest, -1.1258999e15, Less);

    test("1/2", Floor, 0.5, Equal);
    test("1/2", Down, 0.5, Equal);
    test("1/2", Ceiling, 0.5, Equal);
    test("1/2", Up, 0.5, Equal);
    test("1/2", Nearest, 0.5, Equal);
    test("1/2", Exact, 0.5, Equal);

    test("-1/2", Floor, -0.5, Equal);
    test("-1/2", Down, -0.5, Equal);
    test("-1/2", Ceiling, -0.5, Equal);
    test("-1/2", Up, -0.5, Equal);
    test("-1/2", Nearest, -0.5, Equal);
    test("-1/2", Exact, -0.5, Equal);

    test("1/3", Floor, 0.3333333, Less);
    test("1/3", Down, 0.3333333, Less);
    test("1/3", Ceiling, 0.33333334, Greater);
    test("1/3", Up, 0.33333334, Greater);
    test("1/3", Nearest, 0.33333334, Greater);

    test("-1/3", Floor, -0.33333334, Less);
    test("-1/3", Down, -0.3333333, Greater);
    test("-1/3", Ceiling, -0.3333333, Greater);
    test("-1/3", Up, -0.33333334, Less);
    test("-1/3", Nearest, -0.33333334, Less);

    // subnormal
    test(
        "1/10000000000000000000000000000000000000000",
        Floor,
        1.0e-40,
        Less,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        Down,
        1.0e-40,
        Less,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        Ceiling,
        1.00001e-40,
        Greater,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        Up,
        1.00001e-40,
        Greater,
    );
    test(
        "1/10000000000000000000000000000000000000000",
        Nearest,
        1.0e-40,
        Less,
    );

    test(
        "-1/10000000000000000000000000000000000000000",
        Floor,
        -1.00001e-40,
        Less,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        Down,
        -1.0e-40,
        Greater,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        Ceiling,
        -1.0e-40,
        Greater,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        Up,
        -1.00001e-40,
        Less,
    );
    test(
        "-1/10000000000000000000000000000000000000000",
        Nearest,
        -1.0e-40,
        Greater,
    );

    // less than subnormal
    test(
        "1/100000000000000000000000000000000000000000000000000",
        Floor,
        0.0,
        Less,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        Down,
        0.0,
        Less,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        Ceiling,
        1.0e-45,
        Greater,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        Up,
        1.0e-45,
        Greater,
    );
    test(
        "1/100000000000000000000000000000000000000000000000000",
        Nearest,
        0.0,
        Less,
    );

    test(
        "-1/100000000000000000000000000000000000000000000000000",
        Floor,
        -1.0e-45,
        Less,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        Down,
        -0.0,
        Greater,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        Ceiling,
        -0.0,
        Greater,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        Up,
        -1.0e-45,
        Less,
    );
    test(
        "-1/100000000000000000000000000000000000000000000000000",
        Nearest,
        -0.0,
        Greater,
    );

    // half of smallest positive
    test(
        "1/1427247692705959881058285969449495136382746624",
        Floor,
        0.0,
        Less,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        Down,
        0.0,
        Less,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        Ceiling,
        1.0e-45,
        Greater,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        Up,
        1.0e-45,
        Greater,
    );
    test(
        "1/1427247692705959881058285969449495136382746624",
        Nearest,
        0.0,
        Less,
    );

    test(
        "-1/1427247692705959881058285969449495136382746624",
        Floor,
        -1.0e-45,
        Less,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        Down,
        -0.0,
        Greater,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        Ceiling,
        -0.0,
        Greater,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        Up,
        -1.0e-45,
        Less,
    );
    test(
        "-1/1427247692705959881058285969449495136382746624",
        Nearest,
        -0.0,
        Greater,
    );

    // just over half of smallest positive; Nearest rounds up
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        Floor,
        0.0, Less
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        Down,
        0.0, Less
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        Ceiling,
        1.0e-45, Greater
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        Up,
        1.0e-45, Greater
    );
    test(
        "88819109620612751463292030150471001/126765060022822940149670320537600000000000000000000000\
        000000000000000000000000000",
        Nearest,
        1.0e-45, Greater
    );

    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        Floor,
        -1.0e-45, Less
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        Down,
        -0.0, Greater
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        Ceiling,
        -0.0, Greater
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        Up,
        -1.0e-45, Less
    );
    test(
        "-88819109620612751463292030150471001/12676506002282294014967032053760000000000000000000000\
        0000000000000000000000000000",
        Nearest,
        -1.0e-45, Less
    );

    // halfway between max subnormal and min normal
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        Floor,
        1.1754942e-38,
        Less,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        Down,
        1.1754942e-38,
        Less,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        Ceiling,
        1.1754944e-38,
        Greater,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        Up,
        1.1754944e-38,
        Greater,
    );
    test(
        "16777215/1427247692705959881058285969449495136382746624",
        Nearest,
        1.1754944e-38,
        Greater,
    );
}

#[test]
fn f32_rounding_from_rational_fail() {
    assert_panic!(f32::rounding_from(
        Rational::from_str("340282346638528859811704183484516925439").unwrap(),
        Exact,
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("340282346638528859811704183484516925441").unwrap(),
        Exact,
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("16777217").unwrap(),
        Exact
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        Exact,
    ));
    assert_panic!(f32::rounding_from(
        Rational::from_str("1/10").unwrap(),
        Exact
    ));
}

#[test]
fn f32_rounding_from_rational_ref_fail() {
    assert_panic!(f32::rounding_from(
        &Rational::from_str("340282346638528859811704183484516925439").unwrap(),
        Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("340282346638528859811704183484516925441").unwrap(),
        Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("16777217").unwrap(),
        Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        Exact,
    ));
    assert_panic!(f32::rounding_from(
        &Rational::from_str("1/10").unwrap(),
        Exact
    ));
}

#[test]
fn test_f64_rounding_from_rational() {
    let test = |s: &str, rm: RoundingMode, out, o_out| {
        let u = Rational::from_str(s).unwrap();
        let (f, o) = f64::rounding_from(&u, rm);
        assert_eq!(NiceFloat(f), NiceFloat(out));
        assert_eq!(o, o_out);
        let (f, o) = f64::rounding_from(u, rm);
        assert_eq!(NiceFloat(f), NiceFloat(out));
        assert_eq!(o, o_out);
        if rm == Down {
            assert_eq!(
                NiceFloat(rug::Rational::from_str(s).unwrap().to_f64()),
                NiceFloat(out)
            );
        }
    };
    test("3", Exact, 3.0, Equal);
    test("-3", Exact, -3.0, Equal);
    test("123", Exact, 123.0, Equal);
    test("-123", Exact, -123.0, Equal);
    test("0", Exact, 0.0, Equal);
    test("100000000000000000000", Exact, 1.0e20, Equal);
    test("-100000000000000000000", Exact, -1.0e20, Equal);
    test("9007199254740992", Exact, 9.007199254740992e15, Equal);
    test("-9007199254740992", Exact, -9.007199254740992e15, Equal);
    test("9007199254740994", Exact, 9.007199254740994e15, Equal);
    test("-9007199254740994", Exact, -9.007199254740994e15, Equal);

    test("9007199254740993", Floor, 9.007199254740992e15, Less);
    test("9007199254740993", Down, 9.007199254740992e15, Less);
    test("9007199254740993", Ceiling, 9.007199254740994e15, Greater);
    test("9007199254740993", Up, 9.007199254740994e15, Greater);
    test("9007199254740993", Nearest, 9.007199254740992e15, Less);

    test("-9007199254740993", Floor, -9.007199254740994e15, Less);
    test("-9007199254740993", Down, -9.007199254740992e15, Greater);
    test("-9007199254740993", Ceiling, -9.007199254740992e15, Greater);
    test("-9007199254740993", Up, -9.007199254740994e15, Less);
    test("-9007199254740993", Nearest, -9.007199254740992e15, Greater);

    test("18014398509481984", Exact, 1.8014398509481984e16, Equal);
    test("-18014398509481984", Exact, -1.8014398509481984e16, Equal);
    test("18014398509481988", Exact, 1.8014398509481988e16, Equal);
    test("-18014398509481988", Exact, -1.8014398509481988e16, Equal);

    test("18014398509481985", Floor, 1.8014398509481984e16, Less);
    test("18014398509481985", Down, 1.8014398509481984e16, Less);
    test("18014398509481985", Ceiling, 1.8014398509481988e16, Greater);
    test("18014398509481985", Up, 1.8014398509481988e16, Greater);
    test("18014398509481985", Nearest, 1.8014398509481984e16, Less);

    test("-18014398509481985", Floor, -1.8014398509481988e16, Less);
    test("-18014398509481985", Down, -1.8014398509481984e16, Greater);
    test(
        "-18014398509481985",
        Ceiling,
        -1.8014398509481984e16,
        Greater,
    );
    test("-18014398509481985", Up, -1.8014398509481988e16, Less);
    test(
        "-18014398509481985",
        Nearest,
        -1.8014398509481984e16,
        Greater,
    );

    test("18014398509481986", Nearest, 1.8014398509481984e16, Less);
    test(
        "-18014398509481986",
        Nearest,
        -1.8014398509481984e16,
        Greater,
    );
    test("18014398509481987", Nearest, 1.8014398509481988e16, Greater);
    test("-18014398509481987", Nearest, -1.8014398509481988e16, Less);

    test("1/2", Floor, 0.5, Equal);
    test("1/2", Down, 0.5, Equal);
    test("1/2", Ceiling, 0.5, Equal);
    test("1/2", Up, 0.5, Equal);
    test("1/2", Nearest, 0.5, Equal);
    test("1/2", Exact, 0.5, Equal);

    test("-1/2", Floor, -0.5, Equal);
    test("-1/2", Down, -0.5, Equal);
    test("-1/2", Ceiling, -0.5, Equal);
    test("-1/2", Up, -0.5, Equal);
    test("-1/2", Nearest, -0.5, Equal);
    test("-1/2", Exact, -0.5, Equal);

    test("1/3", Floor, 0.3333333333333333, Less);
    test("1/3", Down, 0.3333333333333333, Less);
    test("1/3", Ceiling, 0.33333333333333337, Greater);
    test("1/3", Up, 0.33333333333333337, Greater);
    test("1/3", Nearest, 0.3333333333333333, Less);

    test("-1/3", Floor, -0.33333333333333337, Less);
    test("-1/3", Down, -0.3333333333333333, Greater);
    test("-1/3", Ceiling, -0.3333333333333333, Greater);
    test("-1/3", Up, -0.33333333333333337, Less);
    test("-1/3", Nearest, -0.3333333333333333, Greater);
}

#[test]
fn f64_rounding_from_rational_fail() {
    assert_panic!(f64::rounding_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(), Exact)
    );
    assert_panic!(f64::rounding_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap(), Exact)
    );
    assert_panic!(f64::rounding_from(
        Rational::from_str("9007199254740993").unwrap(),
        Exact,
    ));
    assert_panic!(f64::rounding_from(
        Rational::from_str("1/10").unwrap(),
        Exact
    ));
}

#[test]
fn f64_rounding_from_rational_ref_fail() {
    assert_panic!(
        f64::rounding_from(&Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(), Exact)
    );
    assert_panic!(
        f64::rounding_from(&Rational::from_str(
        "17976931348623157081452742373170435679807056752584499659891747680315726078002858760589558\
        632766878171540458953514382464234321326889464182768467546703537516986049910576552820762454\
        900903893289440758685084551339423045832369032229481658085593321233482747978262044472316873\
        8177180919299881250404026184124858369").unwrap(), Exact)
    );
    assert_panic!(f64::rounding_from(
        &Rational::from_str("9007199254740993").unwrap(),
        Exact,
    ));
    assert_panic!(f64::rounding_from(
        &Rational::from_str("1/10").unwrap(),
        Exact
    ));
}

#[test]
fn test_f32_try_from_rational() {
    let test = |s: &str, out: Result<f32, FloatFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f32::try_from(&u).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f32::try_from(u.clone()).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f32::convertible_from(u), out.is_ok());
    };
    test("3", Ok(3.0));
    test("-3", Ok(-3.0));
    test("123", Ok(123.0));
    test("-123", Ok(-123.0));
    test("0", Ok(0.0));
    test("1000000000", Ok(1.0e9));
    test("-1000000000", Ok(-1.0e9));
    test("16777216", Ok(1.6777216e7));
    test("-16777216", Ok(-1.6777216e7));
    test("16777218", Ok(1.6777218e7));
    test("-16777218", Ok(-1.6777218e7));
    test("16777217", Err(FloatFromRationalError));
    test("-16777217", Err(FloatFromRationalError));
    test("33554432", Ok(3.3554432e7));
    test("-33554432", Ok(-3.3554432e7));
    test("33554436", Ok(3.3554436e7));
    test("-33554436", Ok(-3.3554436e7));
    test("33554433", Err(FloatFromRationalError));
    test("-33554433", Err(FloatFromRationalError));
    test("33554434", Err(FloatFromRationalError));
    test("-33554434", Err(FloatFromRationalError));
    test("33554435", Err(FloatFromRationalError));
    test("-33554435", Err(FloatFromRationalError));
    test(
        "340282346638528859811704183484516925439",
        Err(FloatFromRationalError),
    );
    test(
        "-340282346638528859811704183484516925439",
        Err(FloatFromRationalError),
    );
    test("340282346638528859811704183484516925440", Ok(3.4028235e38));
    test(
        "-340282346638528859811704183484516925440",
        Ok(-3.4028235e38),
    );
    test(
        "340282346638528859811704183484516925441",
        Err(FloatFromRationalError),
    );
    test(
        "-340282346638528859811704183484516925441",
        Err(FloatFromRationalError),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Err(FloatFromRationalError),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        Err(FloatFromRationalError),
    );

    test("1/2", Ok(0.5));
    test("-1/2", Ok(-0.5));
    test("1/3", Err(FloatFromRationalError));
    test("-1/3", Err(FloatFromRationalError));
    test(
        "1/713623846352979940529142984724747568191373312",
        Ok(f32::MIN_POSITIVE_SUBNORMAL),
    );
    test(
        "-1/713623846352979940529142984724747568191373312",
        Ok(-f32::MIN_POSITIVE_SUBNORMAL),
    );
    test(
        "8388607/713623846352979940529142984724747568191373312",
        Ok(f32::MAX_SUBNORMAL),
    );
    test(
        "-8388607/713623846352979940529142984724747568191373312",
        Ok(-f32::MAX_SUBNORMAL),
    );
    test(
        "1/85070591730234615865843651857942052864",
        Ok(f32::MIN_POSITIVE_NORMAL),
    );
    test(
        "-1/85070591730234615865843651857942052864",
        Ok(-f32::MIN_POSITIVE_NORMAL),
    );
}

#[test]
fn test_f64_try_from_rational() {
    let test = |s: &str, out: Result<f64, FloatFromRationalError>| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f64::try_from(&u).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f64::try_from(u.clone()).map(NiceFloat), out.map(NiceFloat));
        assert_eq!(f64::convertible_from(u), out.is_ok());
    };
    test("3", Ok(3.0));
    test("-3", Ok(-3.0));
    test("123", Ok(123.0));
    test("-123", Ok(-123.0));
    test("0", Ok(0.0));
    test("1000000000", Ok(1.0e9));
    test("-1000000000", Ok(-1.0e9));
    test("9007199254740992", Ok(9.007199254740992e15));
    test("-9007199254740992", Ok(-9.007199254740992e15));
    test("9007199254740994", Ok(9.007199254740994e15));
    test("-9007199254740994", Ok(-9.007199254740994e15));
    test("9007199254740993", Err(FloatFromRationalError));
    test("-9007199254740993", Err(FloatFromRationalError));
    test("18014398509481984", Ok(1.8014398509481984e16));
    test("-18014398509481984", Ok(-1.8014398509481984e16));
    test("18014398509481988", Ok(1.8014398509481988e16));
    test("-18014398509481988", Ok(-1.8014398509481988e16));
    test("18014398509481985", Err(FloatFromRationalError));
    test("-18014398509481985", Err(FloatFromRationalError));
    test("18014398509481986", Err(FloatFromRationalError));
    test("-18014398509481986", Err(FloatFromRationalError));
    test("18014398509481987", Err(FloatFromRationalError));
    test("-18014398509481987", Err(FloatFromRationalError));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", Err(FloatFromRationalError));
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367", Err(FloatFromRationalError));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", Ok(1.7976931348623157e308));
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858368", Ok(-1.7976931348623157e308));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", Err(FloatFromRationalError));
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369", Err(FloatFromRationalError));

    test("1/2", Ok(0.5));
    test("-1/2", Ok(-0.5));
    test("1/3", Err(FloatFromRationalError));
    test("-1/3", Err(FloatFromRationalError));
}

#[test]
fn test_f32_exact_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(NiceFloat(f32::exact_from(&u)), NiceFloat(out));
        assert_eq!(NiceFloat(f32::exact_from(u)), NiceFloat(out));
    };
    test("3", 3.0);
    test("-3", -3.0);
    test("123", 123.0);
    test("-123", -123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("-1000000000", -1.0e9);
    test("16777216", 1.6777216e7);
    test("-16777216", -1.6777216e7);
    test("16777218", 1.6777218e7);
    test("-16777218", -1.6777218e7);
    test("33554432", 3.3554432e7);
    test("-33554432", -3.3554432e7);
    test("33554436", 3.3554436e7);
    test("-33554436", -3.3554436e7);
    test("340282346638528859811704183484516925440", 3.4028235e38);
    test("-340282346638528859811704183484516925440", -3.4028235e38);
}

#[test]
fn f32_exact_from_rational_fail() {
    assert_panic!(f32::exact_from(Rational::from_str("16777217").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-16777217").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("33554433").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-33554433").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("33554434").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-33554434").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("33554435").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-33554435").unwrap()));
    assert_panic!(f32::exact_from(
        Rational::from_str("340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(
        Rational::from_str("-10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(Rational::from_str("1/3").unwrap()));
    assert_panic!(f32::exact_from(Rational::from_str("-1/3").unwrap()));
}

#[test]
fn f32_exact_from_rational_ref_fail() {
    assert_panic!(f32::exact_from(&Rational::from_str("16777217").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-16777217").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("33554433").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-33554433").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("33554434").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-33554434").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("33554435").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-33554435").unwrap()));
    assert_panic!(f32::exact_from(
        &Rational::from_str("340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-340282346638528859811704183484516925439").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-340282346638528859811704183484516925441").unwrap()
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(
        &Rational::from_str("-10000000000000000000000000000000000000000000000000000").unwrap(),
    ));
    assert_panic!(f32::exact_from(&Rational::from_str("1/3").unwrap()));
    assert_panic!(f32::exact_from(&Rational::from_str("-1/3").unwrap()));
}

#[test]
fn test_f64_exact_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(NiceFloat(f64::exact_from(&u)), NiceFloat(out));
        assert_eq!(NiceFloat(f64::exact_from(u)), NiceFloat(out));
    };
    test("3", 3.0);
    test("-3", -3.0);
    test("123", 123.0);
    test("-123", -123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("-1000000000", -1.0e9);
    test("9007199254740992", 9.007199254740992e15);
    test("-9007199254740992", -9.007199254740992e15);
    test("9007199254740994", 9.007199254740994e15);
    test("-9007199254740994", -9.007199254740994e15);
    test("18014398509481984", 1.8014398509481984e16);
    test("-18014398509481984", -1.8014398509481984e16);
    test("18014398509481988", 1.8014398509481988e16);
    test("-18014398509481988", -1.8014398509481988e16);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", 1.7976931348623157e308);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858368", -1.7976931348623157e308);
}

#[test]
fn f64_exact_from_rational_fail() {
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(
        Rational::from_str("-18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str("1/3").unwrap()));
    assert_panic!(f64::exact_from(Rational::from_str("-1/3").unwrap()));
}

#[test]
fn f64_exact_from_rational_ref_fail() {
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481983").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481985").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481986").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(
        &Rational::from_str("-18014398509481987").unwrap()
    ));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str("1/3").unwrap()));
    assert_panic!(f64::exact_from(&Rational::from_str("-1/3").unwrap()));
}

#[test]
fn test_f32_convertible_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f32::convertible_from(&u), out);
        assert_eq!(f32::convertible_from(u), out);
    };
    test("3", true);
    test("-3", true);
    test("123", true);
    test("-123", true);
    test("0", true);
    test("1000000000", true);
    test("-1000000000", true);
    test("16777216", true);
    test("-16777216", true);
    test("16777218", true);
    test("-16777218", true);
    test("16777217", false);
    test("-16777217", false);
    test("33554432", true);
    test("-33554432", true);
    test("33554436", true);
    test("-33554436", true);
    test("33554433", false);
    test("-33554433", false);
    test("33554434", false);
    test("-33554434", false);
    test("33554435", false);
    test("-33554435", false);
    test("340282346638528859811704183484516925439", false);
    test("-340282346638528859811704183484516925439", false);
    test("340282346638528859811704183484516925440", true);
    test("-340282346638528859811704183484516925440", true);
    test("340282346638528859811704183484516925441", false);
    test("-340282346638528859811704183484516925441", false);
    test(
        "10000000000000000000000000000000000000000000000000000",
        false,
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        false,
    );
    test("1/3", false);
    test("-1/3", false);
}

#[test]
fn test_f64_convertible_from_rational() {
    let test = |s: &str, out| {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(f64::convertible_from(&u), out);
        assert_eq!(f64::convertible_from(u), out);
    };
    test("3", true);
    test("-3", true);
    test("123", true);
    test("-123", true);
    test("0", true);
    test("1000000000", true);
    test("-1000000000", true);
    test("9007199254740992", true);
    test("-9007199254740992", true);
    test("9007199254740994", true);
    test("-9007199254740994", true);
    test("9007199254740993", false);
    test("-9007199254740993", false);
    test("18014398509481984", true);
    test("-18014398509481984", true);
    test("18014398509481988", true);
    test("-18014398509481988", true);
    test("18014398509481985", false);
    test("-18014398509481985", false);
    test("18014398509481986", false);
    test("-18014398509481986", false);
    test("18014398509481987", false);
    test("-18014398509481987", false);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", false);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858367", false);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", true);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858368", true);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", false);
    test(
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858369", false);

    test("1/3", false);
    test("-1/3", false);
}

#[allow(clippy::trait_duplication_in_bounds)]
fn float_rounding_from_rational_properties_helper<
    T: for<'a> ConvertibleFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a Rational>
        + PartialOrd<Rational>
        + PrimitiveFloat
        + RoundingFrom<Rational>
        + for<'a> RoundingFrom<&'a Integer>
        + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Rational: TryFrom<T>,
{
    rational_rounding_mode_pair_gen_var_5::<T>().test_properties(|(x, rm)| {
        let (f, o) = T::rounding_from(&x, rm);
        let neg_f = if x == 0 { T::ZERO } else { -f };
        let (f_alt, o_alt) = T::rounding_from(-&x, -rm);
        assert_eq!(NiceFloat(f_alt), NiceFloat(neg_f));
        assert_eq!(o_alt, o.reverse());
        let (f_alt, o_alt) = T::rounding_from(&x, rm);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);

        assert_eq!(f.partial_cmp(&x), Some(o));
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
    });

    rational_gen_var_4::<T>().test_properties(|n| {
        let (f, o) = T::rounding_from(&n, Exact);
        assert_eq!(o, Equal);
        let (f_alt, o_alt) = T::rounding_from(&n, Floor);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, Equal);
        let (f_alt, o_alt) = T::rounding_from(&n, Down);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, Equal);
        let (f_alt, o_alt) = T::rounding_from(&n, Ceiling);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, Equal);
        let (f_alt, o_alt) = T::rounding_from(&n, Up);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, Equal);
        let (f_alt, o_alt) = T::rounding_from(&n, Nearest);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, Equal);

        assert_eq!(Rational::exact_from(f), n);
    });

    rational_gen_var_5::<T>().test_properties(|n| {
        let f_below = T::rounding_from(&n, Floor);
        assert_eq!(f_below.1, Less);
        let f_above = (f_below.0.next_higher(), Greater);
        if f_below.0.is_finite() {
            assert!(Rational::exact_from(f_below.0) < n);
        }
        if f_above.0.is_finite() {
            assert!(Rational::exact_from(f_above.0) > n);
        }
        let (f, o) = T::rounding_from(&n, Ceiling);
        assert_eq!(NiceFloat(f), NiceFloat(f_above.0));
        assert_eq!(o, Greater);
        if n >= 0 {
            let (f, o) = T::rounding_from(&n, Down);
            assert_eq!(NiceFloat(f), NiceFloat(f_below.0));
            assert_eq!(o, Less);
            let (f, o) = T::rounding_from(&n, Up);
            assert_eq!(NiceFloat(f), NiceFloat(f_above.0));
            assert_eq!(o, Greater);
        } else {
            let (f, o) = T::rounding_from(&n, Down);
            assert_eq!(NiceFloat(f), NiceFloat(f_above.0));
            assert_eq!(o, Greater);
            let (f, o) = T::rounding_from(&n, Up);
            assert_eq!(NiceFloat(f), NiceFloat(f_below.0));
            assert_eq!(o, Less);
        }
        let (f, o) = T::rounding_from(&n, Nearest);
        assert!(
            (NiceFloat(f), o) == (NiceFloat(f_below.0), f_below.1)
                || (NiceFloat(f), o) == (NiceFloat(f_above.0), f_above.1)
        );
        if f_below.0.is_finite() && f_above.0.is_finite() {
            let below_diff = &n - Rational::exact_from(f_below.0);
            let above_diff = Rational::exact_from(f_above.0) - &n;
            if NiceFloat(f) == NiceFloat(f_below.0) {
                assert!(below_diff <= above_diff);
            } else {
                assert!(below_diff >= above_diff);
            }
        }
    });

    rational_gen_var_6::<T>().test_properties(|n| {
        let floor = T::rounding_from(&n, Floor);
        assert_eq!(floor.1, Less);
        let ceiling = (floor.0.next_higher(), Greater);
        let nearest = T::rounding_from(&n, Nearest);
        assert_eq!(
            (NiceFloat(nearest.0), nearest.1),
            if floor.0.to_bits().even() {
                (NiceFloat(floor.0), floor.1)
            } else {
                (NiceFloat(ceiling.0), ceiling.1)
            }
        );
    });

    integer_rounding_mode_pair_gen_var_1::<T>().test_properties(|(n, rm)| {
        let r: Rational = ExactFrom::exact_from(&n);
        let (f, o) = T::rounding_from(r, rm);
        let (f_alt, o_alt) = T::rounding_from(&n, rm);
        assert_eq!(NiceFloat(f), NiceFloat(f_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn float_rounding_from_rational_properties() {
    apply_fn_to_primitive_floats!(float_rounding_from_rational_properties_helper);

    let max = Rational::exact_from(f32::MAX_FINITE);
    rational_gen().test_properties(|x| {
        if x.lt_abs(&max) {
            let f = f32::rounding_from(&x, Down).0;
            if NiceFloat(f) != NiceFloat(-0.0) {
                assert_eq!(NiceFloat(f), NiceFloat(rug::Rational::from(&x).to_f32()));
            }
        }
        assert_eq!(
            NiceFloat(f64::rounding_from(&x, Down).0),
            NiceFloat(rug::Rational::from(&x).to_f64())
        );
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn float_try_from_rational_properties_helper<
    T: TryFrom<Rational, Error = FloatFromRationalError>
        + for<'a> TryFrom<&'a Integer>
        + for<'a> TryFrom<&'a Rational, Error = FloatFromRationalError>
        + for<'a> ConvertibleFrom<&'a Rational>
        + PrimitiveFloat
        + for<'a> RoundingFrom<&'a Rational>,
>()
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
    Rational: TryFrom<T>,
{
    rational_gen().test_properties(|n| {
        let of = T::try_from(&n);
        assert_eq!(
            T::try_from(n.clone()).map(NiceFloat),
            of.map(|f| NiceFloat(f))
        );
        assert_eq!(
            T::try_from(-&n).map(NiceFloat),
            of.map(|f| NiceFloat(if n == 0 { T::ZERO } else { -f }))
        );
    });

    rational_gen_var_4::<T>().test_properties(|n| {
        let f = T::exact_from(&n);
        assert_eq!(NiceFloat(f), NiceFloat(T::rounding_from(&n, Exact).0));
        assert_eq!(Rational::exact_from(f), n);
    });

    rational_gen_var_5::<T>().test_properties(|n| {
        assert!(T::try_from(n).is_err());
    });

    rational_gen_var_6::<T>().test_properties(|n| {
        assert!(T::try_from(n).is_err());
    });

    integer_gen().test_properties(|n| {
        if let Ok(f) = T::try_from(&n) {
            let rn: Rational = From::from(&n);
            assert_eq!(NiceFloat(f), NiceFloat(T::exact_from(rn)));
        }
    });

    integer_gen_var_1::<T>().test_properties(|n| {
        let rn: Rational = From::from(&n);
        assert_eq!(NiceFloat(T::exact_from(&n)), NiceFloat(T::exact_from(rn)));
    });
}

#[test]
fn float_try_from_rational_properties() {
    apply_fn_to_primitive_floats!(float_try_from_rational_properties_helper);
}

#[allow(clippy::trait_duplication_in_bounds)]
fn float_convertible_from_rational_properties_helper<
    T: ConvertibleFrom<Rational>
        + for<'a> ConvertibleFrom<&'a Integer>
        + for<'a> ConvertibleFrom<&'a Rational>
        + PrimitiveFloat,
>()
where
    Rational: TryFrom<T>,
{
    rational_gen().test_properties(|n| {
        assert_eq!(T::convertible_from(&n), T::convertible_from(-n));
    });

    rational_gen_var_4::<T>().test_properties(|n| {
        assert!(T::convertible_from(n));
    });

    rational_gen_var_5::<T>().test_properties(|n| {
        assert!(!T::convertible_from(n));
    });

    rational_gen_var_6::<T>().test_properties(|n| {
        assert!(!T::convertible_from(n));
    });

    integer_gen().test_properties(|n| {
        let rn: Rational = ExactFrom::exact_from(&n);
        assert_eq!(T::convertible_from(&n), T::convertible_from(&rn));
    });
}

#[test]
fn float_convertible_from_rational_properties() {
    apply_fn_to_primitive_floats!(float_convertible_from_rational_properties_helper);
}
