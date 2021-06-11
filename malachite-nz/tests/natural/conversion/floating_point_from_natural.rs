use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::{
    natural_gen, natural_gen_var_3, natural_gen_var_4, natural_gen_var_5,
    natural_rounding_mode_pair_gen_var_1,
};
use std::str::FromStr;

#[test]
fn test_f32_rounding_from_natural() {
    let test = |n: &str, rm: RoundingMode, out| {
        assert_eq!(f32::rounding_from(Natural::from_str(n).unwrap(), rm), out);
        assert_eq!(f32::rounding_from(&Natural::from_str(n).unwrap(), rm), out);
    };
    test("3", RoundingMode::Exact, 3.0);
    test("123", RoundingMode::Exact, 123.0);
    test("0", RoundingMode::Exact, 0.0);
    test("1000000000", RoundingMode::Exact, 1.0e9);
    test("16777216", RoundingMode::Exact, 1.6777216e7);
    test("16777218", RoundingMode::Exact, 1.6777218e7);
    test("16777217", RoundingMode::Floor, 1.6777216e7);
    test("16777217", RoundingMode::Down, 1.6777216e7);
    test("16777217", RoundingMode::Ceiling, 1.6777218e7);
    test("16777217", RoundingMode::Up, 1.6777218e7);
    test("16777217", RoundingMode::Nearest, 1.6777216e7);
    test("33554432", RoundingMode::Exact, 3.3554432e7);
    test("33554436", RoundingMode::Exact, 3.3554436e7);
    test("33554433", RoundingMode::Floor, 3.3554432e7);
    test("33554433", RoundingMode::Down, 3.3554432e7);
    test("33554433", RoundingMode::Ceiling, 3.3554436e7);
    test("33554433", RoundingMode::Up, 3.3554436e7);
    test("33554433", RoundingMode::Nearest, 3.3554432e7);
    test("33554434", RoundingMode::Nearest, 3.3554432e7);
    test("33554435", RoundingMode::Nearest, 3.3554436e7);
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Floor,
        3.4028233e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Down,
        3.4028233e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Ceiling,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Up,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Exact,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Floor,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Down,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Ceiling,
        f32::POSITIVE_INFINITY,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Up,
        f32::POSITIVE_INFINITY,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        f32::POSITIVE_INFINITY,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        f32::POSITIVE_INFINITY,
    );
    test("1125899873419263", RoundingMode::Floor, 1.12589984e15);
    test("1125899873419263", RoundingMode::Down, 1.12589984e15);
    test("1125899873419263", RoundingMode::Ceiling, 1.1258999e15);
    test("1125899873419263", RoundingMode::Up, 1.1258999e15);
    test("1125899873419263", RoundingMode::Nearest, 1.1258999e15);
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_1() {
    f32::rounding_from(
        Natural::from_str("340282346638528859811704183484516925439").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_2() {
    f32::rounding_from(
        Natural::from_str("340282346638528859811704183484516925441").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_3() {
    f32::rounding_from(Natural::from_str("16777217").unwrap(), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_4() {
    f32::rounding_from(
        Natural::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_ref_fail_1() {
    f32::rounding_from(
        &Natural::from_str("340282346638528859811704183484516925439").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_ref_fail_2() {
    f32::rounding_from(
        &Natural::from_str("340282346638528859811704183484516925441").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_ref_fail_3() {
    f32::rounding_from(&Natural::from_str("16777217").unwrap(), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_ref_fail_4() {
    f32::rounding_from(
        &Natural::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
fn test_f64_rounding_from_natural() {
    let test = |n: &str, rm: RoundingMode, out| {
        assert_eq!(f64::rounding_from(Natural::from_str(n).unwrap(), rm), out);
        assert_eq!(f64::rounding_from(&Natural::from_str(n).unwrap(), rm), out);
    };
    test("3", RoundingMode::Exact, 3.0);
    test("123", RoundingMode::Exact, 123.0);
    test("0", RoundingMode::Exact, 0.0);
    test("100000000000000000000", RoundingMode::Exact, 1.0e20);
    test(
        "9007199254740992",
        RoundingMode::Exact,
        9.007199254740992e15,
    );
    test(
        "9007199254740994",
        RoundingMode::Exact,
        9.007199254740994e15,
    );
    test(
        "9007199254740993",
        RoundingMode::Floor,
        9.007199254740992e15,
    );
    test("9007199254740993", RoundingMode::Down, 9.007199254740992e15);
    test(
        "9007199254740993",
        RoundingMode::Ceiling,
        9.007199254740994e15,
    );
    test("9007199254740993", RoundingMode::Up, 9.007199254740994e15);
    test(
        "9007199254740993",
        RoundingMode::Nearest,
        9.007199254740992e15,
    );
    test(
        "18014398509481984",
        RoundingMode::Exact,
        1.8014398509481984e16,
    );
    test(
        "18014398509481988",
        RoundingMode::Exact,
        1.8014398509481988e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Floor,
        1.8014398509481984e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Down,
        1.8014398509481984e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Ceiling,
        1.8014398509481988e16,
    );
    test("18014398509481985", RoundingMode::Up, 1.8014398509481988e16);
    test(
        "18014398509481985",
        RoundingMode::Nearest,
        1.8014398509481984e16,
    );
    test(
        "18014398509481986",
        RoundingMode::Nearest,
        1.8014398509481984e16,
    );
    test(
        "18014398509481987",
        RoundingMode::Nearest,
        1.8014398509481988e16,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Floor, 1.7976931348623155e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Down, 1.7976931348623155e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Ceiling, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Up, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Nearest, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", RoundingMode::Exact, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Floor, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Down, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Nearest, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Ceiling, f64::POSITIVE_INFINITY);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Up, f64::POSITIVE_INFINITY);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_1() {
    f64::rounding_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(),
                       RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_2() {
    f64::rounding_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap(),
                       RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_3() {
    f64::rounding_from(
        Natural::from_str("9007199254740993").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_ref_fail_1() {
    f64::rounding_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(),
                       RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_ref_fail_2() {
    f64::rounding_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap(),
                       RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_ref_fail_3() {
    f64::rounding_from(
        &Natural::from_str("9007199254740993").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
fn test_f32_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f32::from(Natural::from_str(n).unwrap()), out);
        assert_eq!(f32::from(&Natural::from_str(n).unwrap()), out);
    };
    test("3", 3.0);
    test("123", 123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("16777216", 1.6777216e7);
    test("16777218", 1.6777218e7);
    test("16777217", 1.6777216e7);
    test("33554432", 3.3554432e7);
    test("33554436", 3.3554436e7);
    test("33554433", 3.3554432e7);
    test("33554434", 3.3554432e7);
    test("33554435", 3.3554436e7);
    test("340282346638528859811704183484516925439", 3.4028235e38);
    test("340282346638528859811704183484516925440", 3.4028235e38);
    test("340282346638528859811704183484516925441", 3.4028235e38);
    test(
        "10000000000000000000000000000000000000000000000000000",
        3.4028235e38,
    );
}

#[test]
fn test_f64_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f64::from(Natural::from_str(n).unwrap()), out);
        assert_eq!(f64::from(&Natural::from_str(n).unwrap()), out);
    };
    test("3", 3.0);
    test("123", 123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("9007199254740992", 9.007199254740992e15);
    test("9007199254740994", 9.007199254740994e15);
    test("9007199254740993", 9.007199254740992e15);
    test("18014398509481984", 1.8014398509481984e16);
    test("18014398509481988", 1.8014398509481988e16);
    test("18014398509481985", 1.8014398509481984e16);
    test("18014398509481986", 1.8014398509481984e16);
    test("18014398509481987", 1.8014398509481988e16);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", 1.7976931348623157e308);
}

#[test]
fn test_f32_checked_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f32::checked_from(Natural::from_str(n).unwrap()), out);
        assert_eq!(f32::checked_from(&Natural::from_str(n).unwrap()), out);
    };
    test("3", Some(3.0));
    test("123", Some(123.0));
    test("0", Some(0.0));
    test("1000000000", Some(1.0e9));
    test("16777216", Some(1.6777216e7));
    test("16777218", Some(1.6777218e7));
    test("16777217", None);
    test("33554432", Some(3.3554432e7));
    test("33554436", Some(3.3554436e7));
    test("33554433", None);
    test("33554434", None);
    test("33554435", None);
    test("340282346638528859811704183484516925439", None);
    test(
        "340282346638528859811704183484516925440",
        Some(3.4028235e38),
    );
    test("340282346638528859811704183484516925441", None);
    test(
        "10000000000000000000000000000000000000000000000000000",
        None,
    );
}

#[test]
fn test_f64_checked_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f64::checked_from(Natural::from_str(n).unwrap()), out);
        assert_eq!(f64::checked_from(&Natural::from_str(n).unwrap()), out);
    };
    test("3", Some(3.0));
    test("123", Some(123.0));
    test("0", Some(0.0));
    test("1000000000", Some(1.0e9));
    test("9007199254740992", Some(9.007199254740992e15));
    test("9007199254740994", Some(9.007199254740994e15));
    test("9007199254740993", None);
    test("18014398509481984", Some(1.8014398509481984e16));
    test("18014398509481988", Some(1.8014398509481988e16));
    test("18014398509481985", None);
    test("18014398509481986", None);
    test("18014398509481987", None);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", None);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", Some(1.7976931348623157e308));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", None);
}

#[test]
fn test_f32_exact_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f32::exact_from(Natural::from_str(n).unwrap()), out);
        assert_eq!(f32::exact_from(&Natural::from_str(n).unwrap()), out);
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
    f32::exact_from(Natural::from_str("9007199254740993").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_2() {
    f32::exact_from(Natural::from_str("18014398509481985").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_3() {
    f32::exact_from(Natural::from_str("18014398509481986").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_4() {
    f32::exact_from(Natural::from_str("18014398509481987").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_5() {
    f32::exact_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_fail_6() {
    f32::exact_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_ref_fail_1() {
    f32::exact_from(&Natural::from_str("9007199254740993").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_ref_fail_2() {
    f32::exact_from(&Natural::from_str("18014398509481985").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_ref_fail_3() {
    f32::exact_from(&Natural::from_str("18014398509481986").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_ref_fail_4() {
    f32::exact_from(&Natural::from_str("18014398509481987").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_ref_fail_5() {
    f32::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap());
}

#[test]
#[should_panic]
fn f32_exact_from_natural_ref_fail_6() {
    f32::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap());
}

#[test]
fn test_f64_exact_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f64::exact_from(Natural::from_str(n).unwrap()), out);
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
        8177180919299881250404026184124858368", 1.7976931348623157e308);
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_1() {
    f64::exact_from(Natural::from_str("9007199254740993").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_2() {
    f64::exact_from(Natural::from_str("18014398509481985").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_3() {
    f64::exact_from(Natural::from_str("18014398509481986").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_4() {
    f64::exact_from(Natural::from_str("18014398509481987").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_5() {
    f64::exact_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_fail_6() {
    f64::exact_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_ref_fail_1() {
    f64::exact_from(&Natural::from_str("9007199254740993").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_ref_fail_2() {
    f64::exact_from(&Natural::from_str("18014398509481985").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_ref_fail_3() {
    f64::exact_from(&Natural::from_str("18014398509481986").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_ref_fail_4() {
    f64::exact_from(&Natural::from_str("18014398509481987").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_ref_fail_5() {
    f64::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap());
}

#[test]
#[should_panic]
fn f64_exact_from_natural_ref_fail_6() {
    f64::exact_from(&Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap());
}

#[test]
fn test_f32_convertible_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f32::convertible_from(Natural::from_str(n).unwrap()), out);
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
        assert_eq!(f64::convertible_from(Natural::from_str(n).unwrap()), out);
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
        8177180919299881250404026184124858367", false);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", true);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", false);
}

fn float_rounding_from_natural_properties_helper<
    T: for<'a> CheckedFrom<&'a Natural>
        + for<'a> ConvertibleFrom<&'a Natural>
        + PrimitiveFloat
        + RoundingFrom<Natural>
        + for<'a> RoundingFrom<&'a Natural>,
>()
where
    Natural: CheckedFrom<T> + From<T> + From<T::UnsignedOfEqualWidth> + RoundingFrom<T>,
{
    natural_rounding_mode_pair_gen_var_1::<T>().test_properties(|(n, rm)| {
        let f = T::rounding_from(&n, rm);
        assert_eq!(NiceFloat(T::rounding_from(n, rm)), NiceFloat(f));
    });

    natural_gen_var_3::<T>().test_properties(|n| {
        let f = T::rounding_from(n.clone(), RoundingMode::Exact);
        assert_eq!(
            NiceFloat(T::rounding_from(&n, RoundingMode::Exact)),
            NiceFloat(f)
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Floor))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Ceiling))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Down))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Up))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Nearest))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Floor))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Ceiling))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Down))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Up))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Nearest))
        );
        assert_eq!(Natural::rounding_from(f, RoundingMode::Exact), n);
    });

    natural_gen_var_4::<T>().test_properties(|n| {
        let f_below = T::rounding_from(&n, RoundingMode::Floor);
        assert_eq!(
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Floor)),
            NiceFloat(f_below)
        );
        let f_above = f_below.next_higher();
        assert_eq!(
            NiceFloat(f_above),
            NiceFloat(T::rounding_from(&n, RoundingMode::Ceiling))
        );
        assert_eq!(
            NiceFloat(f_above),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Ceiling))
        );
        assert_eq!(
            NiceFloat(f_below),
            NiceFloat(T::rounding_from(&n, RoundingMode::Down))
        );
        assert_eq!(
            NiceFloat(f_below),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Down))
        );
        assert_eq!(
            NiceFloat(f_above),
            NiceFloat(T::rounding_from(&n, RoundingMode::Up))
        );
        assert_eq!(
            NiceFloat(f_above),
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Up))
        );
        let f_nearest = T::rounding_from(&n, RoundingMode::Nearest);
        assert_eq!(
            NiceFloat(T::rounding_from(&n, RoundingMode::Nearest)),
            NiceFloat(f_nearest)
        );
        assert!(
            NiceFloat(f_nearest) == NiceFloat(f_below)
                || NiceFloat(f_nearest) == NiceFloat(f_above)
        );
        assert_ne!(Natural::from(f_nearest), n);
    });

    natural_gen_var_5::<T>().test_properties(|n| {
        let floor = T::rounding_from(&n, RoundingMode::Floor);
        let ceiling = floor.next_higher();
        let nearest = T::rounding_from(n, RoundingMode::Nearest);
        assert_eq!(
            NiceFloat(nearest),
            NiceFloat(if floor.to_bits().even() {
                floor
            } else {
                ceiling
            })
        );
    });
}

#[test]
fn float_rounding_from_natural_properties() {
    apply_fn_to_primitive_floats!(float_rounding_from_natural_properties_helper);
}

fn float_from_natural_properties_helper<
    T: CheckedFrom<Natural>
        + for<'a> CheckedFrom<&'a Natural>
        + for<'a> ConvertibleFrom<&'a Natural>
        + From<Natural>
        + for<'a> From<&'a Natural>
        + PrimitiveFloat
        + RoundingFrom<Natural>
        + for<'a> RoundingFrom<&'a Natural>,
>()
where
    Natural: CheckedFrom<T> + From<T> + From<T::UnsignedOfEqualWidth>,
{
    natural_gen().test_properties(|n| {
        let f = T::from(&n);
        assert_eq!(NiceFloat(T::from(n.clone())), NiceFloat(f));
        assert_eq!(
            NiceFloat(T::rounding_from(n, RoundingMode::Nearest)),
            NiceFloat(f)
        );
    });

    natural_gen_var_3::<T>().test_properties(|n| {
        let f = T::from(&n);
        assert_eq!(NiceFloat(T::from(n.clone())), NiceFloat(f));
        assert_eq!(Natural::from(f), n);
    });

    natural_gen_var_4::<T>().test_properties(|n| {
        let f_below = T::rounding_from(&n, RoundingMode::Floor);
        assert_eq!(
            NiceFloat(T::rounding_from(n.clone(), RoundingMode::Floor)),
            NiceFloat(f_below)
        );
        let f_above = f_below.next_higher();
        let f_nearest = T::from(&n);
        assert_eq!(NiceFloat(T::from(n.clone())), NiceFloat(f_nearest));
        assert!(
            NiceFloat(f_nearest) == NiceFloat(f_below)
                || NiceFloat(f_nearest) == NiceFloat(f_above)
        );
        assert_ne!(Natural::from(f_nearest), n);
    });

    natural_gen_var_5::<T>().test_properties(|n| {
        let floor = T::rounding_from(&n, RoundingMode::Floor);
        let ceiling = floor.next_higher();
        let nearest = T::from(n);
        assert_eq!(
            NiceFloat(nearest),
            NiceFloat(if floor.to_bits().even() {
                floor
            } else {
                ceiling
            })
        );
    });
}

#[test]
fn float_from_natural_properties() {
    apply_fn_to_primitive_floats!(float_from_natural_properties_helper);
}

fn float_checked_from_natural_properties_helper<
    T: CheckedFrom<Natural>
        + for<'a> CheckedFrom<&'a Natural>
        + for<'a> ConvertibleFrom<&'a Natural>
        + PrimitiveFloat
        + for<'a> RoundingFrom<&'a Natural>,
>()
where
    Natural: CheckedFrom<T> + From<T> + From<T::UnsignedOfEqualWidth> + RoundingFrom<T>,
{
    natural_gen().test_properties(|n| {
        let of = T::checked_from(&n);
        assert_eq!(T::checked_from(n).map(NiceFloat), of.map(NiceFloat));
    });

    natural_gen_var_3::<T>().test_properties(|n| {
        let f = T::exact_from(&n);
        assert_eq!(NiceFloat(T::exact_from(n.clone())), NiceFloat(f));
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(&n, RoundingMode::Exact))
        );
        assert_eq!(Natural::rounding_from(f, RoundingMode::Exact), n);
    });

    natural_gen_var_4::<T>().test_properties(|n| {
        assert!(T::checked_from(n).is_none());
    });

    natural_gen_var_5::<T>().test_properties(|n| {
        assert!(T::checked_from(n).is_none());
    });
}

#[test]
fn float_checked_from_natural_properties() {
    apply_fn_to_primitive_floats!(float_checked_from_natural_properties_helper);
}

fn float_convertible_from_natural_properties_helper<
    T: for<'a> CheckedFrom<&'a Natural>
        + ConvertibleFrom<Natural>
        + for<'a> ConvertibleFrom<&'a Natural>
        + PrimitiveFloat,
>()
where
    Natural: CheckedFrom<T> + From<T> + From<T::UnsignedOfEqualWidth>,
{
    natural_gen().test_properties(|n| {
        assert_eq!(T::convertible_from(&n), T::convertible_from(n));
    });

    natural_gen_var_3::<T>().test_properties(|n| {
        assert!(T::convertible_from(n));
    });

    natural_gen_var_4::<T>().test_properties(|n| {
        assert!(!T::convertible_from(n));
    });

    natural_gen_var_5::<T>().test_properties(|n| {
        assert!(!T::convertible_from(n));
    });
}

#[test]
fn float_convertible_from_natural_properties() {
    apply_fn_to_primitive_floats!(float_convertible_from_natural_properties_helper);
}
