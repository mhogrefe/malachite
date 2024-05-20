// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen, unsigned_gen_var_18, unsigned_rounding_mode_pair_gen_var_2,
};
use malachite_nz::natural::conversion::primitive_float_from_natural::PrimitiveFloatFromNaturalError;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_var_3, natural_gen_var_4, natural_gen_var_5,
    natural_rounding_mode_pair_gen_var_1,
};
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_f32_rounding_from_natural() {
    let test = |n: &str, rm: RoundingMode, out, o_out| {
        let (x, o) = f32::rounding_from(&Natural::from_str(n).unwrap(), rm);
        assert_eq!(NiceFloat(x), NiceFloat(out));
        assert_eq!(o, o_out);
    };
    test("3", Exact, 3.0, Equal);
    test("123", Exact, 123.0, Equal);
    test("0", Exact, 0.0, Equal);
    test("1000000000", Exact, 1.0e9, Equal);
    test("16777216", Exact, 1.6777216e7, Equal);
    test("16777218", Exact, 1.6777218e7, Equal);

    test("16777217", Floor, 1.6777216e7, Less);
    test("16777217", Down, 1.6777216e7, Less);
    test("16777217", Ceiling, 1.6777218e7, Greater);
    test("16777217", Up, 1.6777218e7, Greater);
    test("16777217", Nearest, 1.6777216e7, Less);

    test("33554432", Exact, 3.3554432e7, Equal);
    test("33554436", Exact, 3.3554436e7, Equal);

    test("33554433", Floor, 3.3554432e7, Less);
    test("33554433", Down, 3.3554432e7, Less);
    test("33554433", Ceiling, 3.3554436e7, Greater);
    test("33554433", Up, 3.3554436e7, Greater);
    test("33554433", Nearest, 3.3554432e7, Less);

    test("33554434", Nearest, 3.3554432e7, Less);
    test("33554435", Nearest, 3.3554436e7, Greater);

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
        "340282346638528859811704183484516925440",
        Exact,
        3.4028235e38,
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

    test("1125899873419263", Floor, 1.12589984e15, Less);
    test("1125899873419263", Down, 1.12589984e15, Less);
    test("1125899873419263", Ceiling, 1.1258999e15, Greater);
    test("1125899873419263", Up, 1.1258999e15, Greater);
    test("1125899873419263", Nearest, 1.1258999e15, Greater);
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_1() {
    f32::rounding_from(
        &Natural::from_str("340282346638528859811704183484516925439").unwrap(),
        Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_2() {
    f32::rounding_from(
        &Natural::from_str("340282346638528859811704183484516925441").unwrap(),
        Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_3() {
    f32::rounding_from(&Natural::from_str("16777217").unwrap(), Exact);
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_4() {
    f32::rounding_from(
        &Natural::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        Exact,
    );
}

#[test]
fn test_f64_rounding_from_natural() {
    let test = |n: &str, rm: RoundingMode, out, o_out| {
        let (x, o) = f64::rounding_from(&Natural::from_str(n).unwrap(), rm);
        assert_eq!(NiceFloat(x), NiceFloat(out));
        assert_eq!(o, o_out);
    };
    test("3", Exact, 3.0, Equal);
    test("123", Exact, 123.0, Equal);
    test("0", Exact, 0.0, Equal);
    test("100000000000000000000", Exact, 1.0e20, Equal);
    test("9007199254740992", Exact, 9.007199254740992e15, Equal);
    test("9007199254740994", Exact, 9.007199254740994e15, Equal);
    test("9007199254740993", Floor, 9.007199254740992e15, Less);

    test("9007199254740993", Down, 9.007199254740992e15, Less);
    test("9007199254740993", Ceiling, 9.007199254740994e15, Greater);
    test("9007199254740993", Up, 9.007199254740994e15, Greater);
    test("9007199254740993", Nearest, 9.007199254740992e15, Less);

    test("18014398509481984", Exact, 1.8014398509481984e16, Equal);
    test("18014398509481988", Exact, 1.8014398509481988e16, Equal);

    test("18014398509481985", Floor, 1.8014398509481984e16, Less);
    test("18014398509481985", Down, 1.8014398509481984e16, Less);
    test("18014398509481985", Ceiling, 1.8014398509481988e16, Greater);
    test("18014398509481985", Up, 1.8014398509481988e16, Greater);
    test("18014398509481985", Nearest, 1.8014398509481984e16, Less);
    test("18014398509481986", Nearest, 1.8014398509481984e16, Less);
    test("18014398509481987", Nearest, 1.8014398509481988e16, Greater);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367",
        Floor,
        1.7976931348623155e308,
        Less,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367",
        Down,
        1.7976931348623155e308,
        Less,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367",
        Ceiling,
        1.7976931348623157e308,
        Greater,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367",
        Up,
        1.7976931348623157e308,
        Greater,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367",
        Nearest,
        1.7976931348623157e308,
        Greater,
    );

    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368",
        Exact,
        1.7976931348623157e308,
        Equal,
    );

    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369",
        Floor,
        1.7976931348623157e308,
        Less,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369",
        Down,
        1.7976931348623157e308,
        Less,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369",
        Nearest,
        1.7976931348623157e308,
        Less,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369",
        Ceiling,
        f64::INFINITY,
        Greater,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369",
        Up,
        f64::INFINITY,
        Greater,
    );
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_1() {
    f64::rounding_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(),
                       Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_2() {
    f64::rounding_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap(),
                       Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_3() {
    f64::rounding_from(&Natural::from_str("9007199254740993").unwrap(), Exact);
}

#[test]
fn test_f32_try_from_natural() {
    let test = |n: &str, out: Result<f32, PrimitiveFloatFromNaturalError>| {
        assert_eq!(
            f32::try_from(&Natural::from_str(n).unwrap()).map(NiceFloat),
            out.map(NiceFloat)
        );
    };
    test("3", Ok(3.0));
    test("123", Ok(123.0));
    test("0", Ok(0.0));
    test("1000000000", Ok(1.0e9));
    test("16777216", Ok(1.6777216e7));
    test("16777218", Ok(1.6777218e7));
    test("16777217", Err(PrimitiveFloatFromNaturalError));
    test("33554432", Ok(3.3554432e7));
    test("33554436", Ok(3.3554436e7));
    test("33554433", Err(PrimitiveFloatFromNaturalError));
    test("33554434", Err(PrimitiveFloatFromNaturalError));
    test("33554435", Err(PrimitiveFloatFromNaturalError));
    test(
        "340282346638528859811704183484516925439",
        Err(PrimitiveFloatFromNaturalError),
    );
    test("340282346638528859811704183484516925440", Ok(3.4028235e38));
    test(
        "340282346638528859811704183484516925441",
        Err(PrimitiveFloatFromNaturalError),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        Err(PrimitiveFloatFromNaturalError),
    );
}

#[test]
fn test_f64_try_from_natural() {
    let test = |n: &str, out: Result<f64, PrimitiveFloatFromNaturalError>| {
        assert_eq!(
            f64::try_from(&Natural::from_str(n).unwrap()).map(NiceFloat),
            out.map(NiceFloat)
        );
    };
    test("3", Ok(3.0));
    test("123", Ok(123.0));
    test("0", Ok(0.0));
    test("1000000000", Ok(1.0e9));
    test("9007199254740992", Ok(9.007199254740992e15));
    test("9007199254740994", Ok(9.007199254740994e15));
    test("9007199254740993", Err(PrimitiveFloatFromNaturalError));
    test("18014398509481984", Ok(1.8014398509481984e16));
    test("18014398509481988", Ok(1.8014398509481988e16));
    test("18014398509481985", Err(PrimitiveFloatFromNaturalError));
    test("18014398509481986", Err(PrimitiveFloatFromNaturalError));
    test("18014398509481987", Err(PrimitiveFloatFromNaturalError));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367",
        Err(PrimitiveFloatFromNaturalError),
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368",
        Ok(1.7976931348623157e308),
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369",
        Err(PrimitiveFloatFromNaturalError),
    );
}

#[test]
fn test_f32_exact_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(
            NiceFloat(f32::exact_from(&Natural::from_str(n).unwrap())),
            NiceFloat(out)
        );
    };
    test("3", 3.0);
    test("123", 123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("16777216", 1.6777216e7);
    test("16777218", 1.6777218e7);
    test("33554432", 3.3554432e7);
    test("33554436", 3.3554436e7);
    test("340282346638528859811704183484516925440", 3.4028235e38);
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_1() {
    f32::exact_from(&Natural::from_str("9007199254740993").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_2() {
    f32::exact_from(&Natural::from_str("18014398509481985").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_3() {
    f32::exact_from(&Natural::from_str("18014398509481986").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_4() {
    f32::exact_from(&Natural::from_str("18014398509481987").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_5() {
    f32::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_6() {
    f32::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap());
}

#[test]
fn test_f64_exact_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(
            NiceFloat(f64::exact_from(&Natural::from_str(n).unwrap())),
            NiceFloat(out)
        );
    };
    test("3", 3.0);
    test("123", 123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("9007199254740992", 9.007199254740992e15);
    test("9007199254740994", 9.007199254740994e15);
    test("18014398509481984", 1.8014398509481984e16);
    test("18014398509481988", 1.8014398509481988e16);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368",
        1.7976931348623157e308,
    );
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_1() {
    f64::exact_from(&Natural::from_str("9007199254740993").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_2() {
    f64::exact_from(&Natural::from_str("18014398509481985").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_3() {
    f64::exact_from(&Natural::from_str("18014398509481986").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_4() {
    f64::exact_from(&Natural::from_str("18014398509481987").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_5() {
    f64::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_6() {
    f64::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap());
}

#[test]
fn test_f32_convertible_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f32::convertible_from(&Natural::from_str(n).unwrap()), out);
    };
    test("3", true);
    test("123", true);
    test("0", true);
    test("1000000000", true);
    test("16777216", true);
    test("16777218", true);
    test("16777217", false);
    test("33554432", true);
    test("33554436", true);
    test("33554433", false);
    test("33554434", false);
    test("33554435", false);
    test("340282346638528859811704183484516925439", false);
    test("340282346638528859811704183484516925440", true);
    test("340282346638528859811704183484516925441", false);
    test(
        "10000000000000000000000000000000000000000000000000000",
        false,
    );
}

#[test]
fn test_f64_convertible_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f64::convertible_from(&Natural::from_str(n).unwrap()), out);
    };
    test("3", true);
    test("123", true);
    test("0", true);
    test("1000000000", true);
    test("9007199254740992", true);
    test("9007199254740994", true);
    test("9007199254740993", false);
    test("18014398509481984", true);
    test("18014398509481988", true);
    test("18014398509481985", false);
    test("18014398509481986", false);
    test("18014398509481987", false);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367",
        false,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368",
        true,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369",
        false,
    );
}

fn float_rounding_from_natural_properties_helper<
    T: for<'a> TryFrom<&'a Natural>
        + for<'a> ConvertibleFrom<&'a Natural>
        + PartialOrd<Natural>
        + PrimitiveFloat
        + for<'a> RoundingFrom<&'a Natural>,
>()
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError> + RoundingFrom<T>,
{
    natural_rounding_mode_pair_gen_var_1::<T>().test_properties(|(n, rm)| {
        let o = T::rounding_from(&n, rm).1;
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
    });

    natural_gen_var_3::<T>().test_properties(|n| {
        let (f, o) = T::rounding_from(&n, Exact);
        assert_eq!(o, Equal);
        let fo = (NiceFloat(f), o);
        let (f_alt, o_alt) = T::rounding_from(&n, Floor);
        assert_eq!((NiceFloat(f_alt), o_alt), fo);
        let (f_alt, o_alt) = T::rounding_from(&n, Ceiling);
        assert_eq!((NiceFloat(f_alt), o_alt), fo);
        let (f_alt, o_alt) = T::rounding_from(&n, Down);
        assert_eq!((NiceFloat(f_alt), o_alt), fo);
        let (f_alt, o_alt) = T::rounding_from(&n, Up);
        assert_eq!((NiceFloat(f_alt), o_alt), fo);
        let (f_alt, o_alt) = T::rounding_from(&n, Nearest);
        assert_eq!((NiceFloat(f_alt), o_alt), fo);
        assert_eq!(Natural::rounding_from(f, Exact), (n, Equal));
    });

    natural_gen_var_4::<T>().test_properties(|n| {
        let f_below = T::rounding_from(&n, Floor);
        assert_eq!(f_below.1, Less);
        let f_above = (NiceFloat(f_below.0.next_higher()), Greater);
        let f_below = (NiceFloat(f_below.0), f_below.1);
        let (f, o) = T::rounding_from(&n, Ceiling);
        assert_eq!((NiceFloat(f), o), f_above);
        let (f, o) = T::rounding_from(&n, Down);
        assert_eq!((NiceFloat(f), o), f_below);
        let (f, o) = T::rounding_from(&n, Up);
        assert_eq!((NiceFloat(f), o), f_above);
        let f_nearest = T::rounding_from(&n, Nearest);
        let f_nearest = (NiceFloat(f_nearest.0), f_nearest.1);
        assert!(f_nearest == f_below || f_nearest == f_above);
    });

    natural_gen_var_5::<T>().test_properties(|n| {
        let floor = T::rounding_from(&n, Floor);
        assert_eq!(floor.1, Less);
        let ceiling = (NiceFloat(floor.0.next_higher()), Greater);
        let floor = (NiceFloat(floor.0), floor.1);
        let nearest = T::rounding_from(&n, Nearest);
        let nearest = (NiceFloat(nearest.0), nearest.1);
        assert_eq!(
            nearest,
            if floor.0 .0.to_bits().even() {
                floor
            } else {
                ceiling
            }
        );
    });

    unsigned_rounding_mode_pair_gen_var_2::<Limb, T>().test_properties(|(u, rm)| {
        let n: Natural = From::from(u);
        let (f, o) = T::rounding_from(u, rm);
        let (f_alt, o_alt) = T::rounding_from(&n, rm);
        assert_eq!(NiceFloat(f), NiceFloat(f_alt));
        assert_eq!(o, o_alt);
        assert_eq!(f.partial_cmp(&n), Some(o));
    });
}

#[test]
fn float_rounding_from_natural_properties() {
    apply_fn_to_primitive_floats!(float_rounding_from_natural_properties_helper);
}

fn float_try_from_natural_properties_helper<
    T: for<'a> TryFrom<&'a Natural>
        + for<'a> ConvertibleFrom<&'a Natural>
        + PrimitiveFloat
        + for<'a> RoundingFrom<&'a Natural>,
>()
where
    Limb: RoundingFrom<T>,
    Natural: TryFrom<T, Error = UnsignedFromFloatError> + RoundingFrom<T>,
    NiceFloat<T>: TryFrom<Limb>,
{
    natural_gen().test_properties(|n| {
        T::try_from(&n).ok();
    });

    natural_gen_var_3::<T>().test_properties(|n| {
        let f = T::exact_from(&n);
        assert_eq!(NiceFloat(f), NiceFloat(T::rounding_from(&n, Exact).0));
        assert_eq!(Natural::rounding_from(f, Exact).0, n);
    });

    natural_gen_var_4::<T>().test_properties(|n| {
        assert!(T::try_from(&n).is_err());
    });

    natural_gen_var_5::<T>().test_properties(|n| {
        assert!(T::try_from(&n).is_err());
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        if let Ok(f) = NiceFloat::<T>::try_from(u) {
            let n: Natural = From::from(u);
            assert_eq!(f, NiceFloat(T::exact_from(&n)));
        }
    });

    unsigned_gen_var_18::<Limb, T>().test_properties(|u| {
        let n: Natural = From::from(u);
        assert_eq!(NiceFloat::<T>::exact_from(u), NiceFloat(T::exact_from(&n)));
    });
}

#[test]
fn float_try_from_natural_properties() {
    apply_fn_to_primitive_floats!(float_try_from_natural_properties_helper);
}

fn float_convertible_from_natural_properties_helper<
    T: for<'a> TryFrom<&'a Natural> + for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>()
where
    Natural: TryFrom<T, Error = UnsignedFromFloatError>,
{
    natural_gen().test_properties(|n| {
        T::convertible_from(&n);
    });

    natural_gen_var_3::<T>().test_properties(|n| {
        assert!(T::convertible_from(&n));
    });

    natural_gen_var_4::<T>().test_properties(|n| {
        assert!(!T::convertible_from(&n));
    });

    natural_gen_var_5::<T>().test_properties(|n| {
        assert!(!T::convertible_from(&n));
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        let n: Natural = From::from(u);
        assert_eq!(T::convertible_from(u), T::convertible_from(&n));
    });
}

#[test]
fn float_convertible_from_natural_properties() {
    apply_fn_to_primitive_floats!(float_convertible_from_natural_properties_helper);
}
