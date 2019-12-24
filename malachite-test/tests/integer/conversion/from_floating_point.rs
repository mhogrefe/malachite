use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_base::round::RoundingMode;
use malachite_nz::integer::Integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    finite_f32s, finite_f64s, pairs_of_finite_f32_and_rounding_mode_var_2,
    pairs_of_finite_f64_and_rounding_mode_var_2,
};
use malachite_test::inputs::integer::{
    f32s_exactly_equal_to_integer, f32s_var_4, f32s_var_5, f64s_exactly_equal_to_integer,
    f64s_var_4, f64s_var_5,
};

#[test]
fn test_rounding_from_f32() {
    let test = |f: f32, rm: RoundingMode, out| {
        let x = Integer::rounding_from(f, rm);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(0.0, RoundingMode::Exact, "0");
    test(-0.0, RoundingMode::Exact, "0");
    test(123.0, RoundingMode::Exact, "123");
    test(-123.0, RoundingMode::Exact, "-123");
    test(1.0e9, RoundingMode::Exact, "1000000000");
    test(-1.0e9, RoundingMode::Exact, "-1000000000");
    test(4294967295.0, RoundingMode::Exact, "4294967296");
    test(-4294967295.0, RoundingMode::Exact, "-4294967296");
    test(4294967296.0, RoundingMode::Exact, "4294967296");
    test(-4294967296.0, RoundingMode::Exact, "-4294967296");
    test(
        18446744073709551615.0,
        RoundingMode::Exact,
        "18446744073709551616",
    );
    test(
        -18446744073709551615.0,
        RoundingMode::Exact,
        "-18446744073709551616",
    );
    test(
        18446744073709551616.0,
        RoundingMode::Exact,
        "18446744073709551616",
    );
    test(
        -18446744073709551616.0,
        RoundingMode::Exact,
        "-18446744073709551616",
    );
    test(1.0e20, RoundingMode::Exact, "100000002004087734272");
    test(-1.0e20, RoundingMode::Exact, "-100000002004087734272");
    test(1.23e20, RoundingMode::Exact, "122999999650278146048");
    test(-1.23e20, RoundingMode::Exact, "-122999999650278146048");
    test(1.6777216e7, RoundingMode::Exact, "16777216");
    test(-1.6777216e7, RoundingMode::Exact, "-16777216");
    test(1.6777218e7, RoundingMode::Exact, "16777218");
    test(-1.6777218e7, RoundingMode::Exact, "-16777218");
    test(123.1, RoundingMode::Floor, "123");
    test(123.1, RoundingMode::Down, "123");
    test(123.1, RoundingMode::Ceiling, "124");
    test(123.1, RoundingMode::Up, "124");
    test(123.1, RoundingMode::Nearest, "123");
    test(-123.1, RoundingMode::Floor, "-124");
    test(-123.1, RoundingMode::Down, "-123");
    test(-123.1, RoundingMode::Ceiling, "-123");
    test(-123.1, RoundingMode::Up, "-124");
    test(-123.1, RoundingMode::Nearest, "-123");
    test(123.9, RoundingMode::Floor, "123");
    test(123.9, RoundingMode::Down, "123");
    test(123.9, RoundingMode::Ceiling, "124");
    test(123.9, RoundingMode::Up, "124");
    test(123.9, RoundingMode::Nearest, "124");
    test(-123.9, RoundingMode::Floor, "-124");
    test(-123.9, RoundingMode::Down, "-123");
    test(-123.9, RoundingMode::Ceiling, "-123");
    test(-123.9, RoundingMode::Up, "-124");
    test(-123.9, RoundingMode::Nearest, "-124");
    test(123.5, RoundingMode::Nearest, "124");
    test(-123.5, RoundingMode::Nearest, "-124");
    test(124.5, RoundingMode::Nearest, "124");
    test(-124.5, RoundingMode::Nearest, "-124");
    test(-0.99, RoundingMode::Ceiling, "0");
    test(-0.99, RoundingMode::Down, "0");
    test(-0.499, RoundingMode::Nearest, "0");
    test(-0.5, RoundingMode::Nearest, "0");
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_1() {
    Integer::rounding_from(f32::NAN, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_2() {
    Integer::rounding_from(f32::POSITIVE_INFINITY, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_3() {
    Integer::rounding_from(f32::NEGATIVE_INFINITY, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn rounding_from_f32_fail_4() {
    Integer::rounding_from(123.1, RoundingMode::Exact);
}

#[test]
fn test_rounding_from_f64() {
    let test = |f: f64, rm: RoundingMode, out| {
        let x = Integer::rounding_from(f, rm);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(0.0, RoundingMode::Exact, "0");
    test(-0.0, RoundingMode::Exact, "0");
    test(123.0, RoundingMode::Exact, "123");
    test(-123.0, RoundingMode::Exact, "-123");
    test(1.0e9, RoundingMode::Exact, "1000000000");
    test(-1.0e9, RoundingMode::Exact, "-1000000000");
    test(4294967295.0, RoundingMode::Exact, "4294967295");
    test(-4294967295.0, RoundingMode::Exact, "-4294967295");
    test(4294967296.0, RoundingMode::Exact, "4294967296");
    test(-4294967296.0, RoundingMode::Exact, "-4294967296");
    test(
        18446744073709551615.0,
        RoundingMode::Exact,
        "18446744073709551616",
    );
    test(
        -18446744073709551615.0,
        RoundingMode::Exact,
        "-18446744073709551616",
    );
    test(
        18446744073709551616.0,
        RoundingMode::Exact,
        "18446744073709551616",
    );
    test(
        -18446744073709551616.0,
        RoundingMode::Exact,
        "-18446744073709551616",
    );
    test(1.0e20, RoundingMode::Exact, "100000000000000000000");
    test(-1.0e20, RoundingMode::Exact, "-100000000000000000000");
    test(1.23e20, RoundingMode::Exact, "123000000000000000000");
    test(-1.23e20, RoundingMode::Exact, "-123000000000000000000");
    test(1.0e100, RoundingMode::Exact,
        "100000000000000001590289110975991804683608085639452813897813275577478387721703810608134699\
        85856815104");
    test(-1.0e100, RoundingMode::Exact,
        "-10000000000000000159028911097599180468360808563945281389781327557747838772170381060813469\
        985856815104");
    test(1.23e100, RoundingMode::Exact,
        "123000000000000008366862950845375853795062237854139353014252897832358837028676639186389822\
        00322686976");
    test(-1.23e100, RoundingMode::Exact,
        "-12300000000000000836686295084537585379506223785413935301425289783235883702867663918638982\
        200322686976");
    test(
        9.007199254740992e15,
        RoundingMode::Exact,
        "9007199254740992",
    );
    test(
        -9.007199254740992e15,
        RoundingMode::Exact,
        "-9007199254740992",
    );
    test(
        9.007199254740994e15,
        RoundingMode::Exact,
        "9007199254740994",
    );
    test(
        -9.007199254740994e15,
        RoundingMode::Exact,
        "-9007199254740994",
    );
    test(123.1, RoundingMode::Floor, "123");
    test(123.1, RoundingMode::Down, "123");
    test(123.1, RoundingMode::Ceiling, "124");
    test(123.1, RoundingMode::Up, "124");
    test(123.1, RoundingMode::Nearest, "123");
    test(-123.1, RoundingMode::Floor, "-124");
    test(-123.1, RoundingMode::Down, "-123");
    test(-123.1, RoundingMode::Ceiling, "-123");
    test(-123.1, RoundingMode::Up, "-124");
    test(-123.1, RoundingMode::Nearest, "-123");
    test(123.9, RoundingMode::Floor, "123");
    test(123.9, RoundingMode::Down, "123");
    test(123.9, RoundingMode::Ceiling, "124");
    test(123.9, RoundingMode::Up, "124");
    test(123.9, RoundingMode::Nearest, "124");
    test(-123.9, RoundingMode::Floor, "-124");
    test(-123.9, RoundingMode::Down, "-123");
    test(-123.9, RoundingMode::Ceiling, "-123");
    test(-123.9, RoundingMode::Up, "-124");
    test(-123.9, RoundingMode::Nearest, "-124");
    test(123.5, RoundingMode::Nearest, "124");
    test(-123.5, RoundingMode::Nearest, "-124");
    test(124.5, RoundingMode::Nearest, "124");
    test(-124.5, RoundingMode::Nearest, "-124");
    test(-0.99, RoundingMode::Ceiling, "0");
    test(-0.99, RoundingMode::Down, "0");
    test(-0.499, RoundingMode::Nearest, "0");
    test(-0.5, RoundingMode::Nearest, "0");
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_1() {
    Integer::rounding_from(f64::NAN, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_2() {
    Integer::rounding_from(f64::POSITIVE_INFINITY, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_3() {
    Integer::rounding_from(f64::NEGATIVE_INFINITY, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn rounding_from_f64_fail_4() {
    Integer::rounding_from(123.1, RoundingMode::Exact);
}

#[test]
fn test_from_f32() {
    let test = |f: f32, out| {
        let x = Integer::from(f);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(0.0, "0");
    test(-0.0, "0");
    test(123.0, "123");
    test(-123.0, "-123");
    test(1.0e9, "1000000000");
    test(-1.0e9, "-1000000000");
    test(4294967295.0, "4294967296");
    test(-4294967295.0, "-4294967296");
    test(4294967296.0, "4294967296");
    test(-4294967296.0, "-4294967296");
    test(18446744073709551615.0, "18446744073709551616");
    test(-18446744073709551615.0, "-18446744073709551616");
    test(18446744073709551616.0, "18446744073709551616");
    test(-18446744073709551616.0, "-18446744073709551616");
    test(1.0e20, "100000002004087734272");
    test(-1.0e20, "-100000002004087734272");
    test(1.23e20, "122999999650278146048");
    test(-1.23e20, "-122999999650278146048");
    test(123.1, "123");
    test(-123.1, "-123");
    test(123.9, "124");
    test(-123.9, "-124");
    test(123.5, "124");
    test(-123.5, "-124");
    test(124.5, "124");
    test(-124.5, "-124");
    test(-0.499, "0");
    test(-0.5, "0");
    test(f32::MIN_POSITIVE, "0");
    test(-f32::MIN_POSITIVE, "0");
    test(f32::MAX_SUBNORMAL, "0");
    test(-f32::MAX_SUBNORMAL, "0");
    test(f32::MIN_POSITIVE_NORMAL, "0");
    test(-f32::MIN_POSITIVE_NORMAL, "0");
    test(f32::MAX_FINITE, "340282346638528859811704183484516925440");
    test(f32::MIN_FINITE, "-340282346638528859811704183484516925440");
}

#[test]
#[should_panic]
fn from_f32_fail_1() {
    Integer::from(f32::NAN);
}

#[test]
#[should_panic]
fn from_f32_fail_2() {
    Integer::from(f32::POSITIVE_INFINITY);
}

#[test]
#[should_panic]
fn from_f32_fail_3() {
    Integer::from(f32::NEGATIVE_INFINITY);
}

#[test]
fn test_from_f64() {
    let test = |f: f64, out| {
        let x = Integer::from(f);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(0.0, "0");
    test(-0.0, "0");
    test(123.0, "123");
    test(-123.0, "-123");
    test(1.0e9, "1000000000");
    test(-1.0e9, "-1000000000");
    test(4294967295.0, "4294967295");
    test(-4294967295.0, "-4294967295");
    test(4294967296.0, "4294967296");
    test(-4294967296.0, "-4294967296");
    test(18446744073709551615.0, "18446744073709551616");
    test(-18446744073709551615.0, "-18446744073709551616");
    test(18446744073709551616.0, "18446744073709551616");
    test(-18446744073709551616.0, "-18446744073709551616");
    test(1.0e20, "100000000000000000000");
    test(-1.0e20, "-100000000000000000000");
    test(1.23e20, "123000000000000000000");
    test(-1.23e20, "-123000000000000000000");
    test(
        1.0e100,
        "100000000000000001590289110975991804683608085639452813897813275577478387721703810608134699\
        85856815104",
    );
    test(
        -1.0e100,
        "-10000000000000000159028911097599180468360808563945281389781327557747838772170381060813469\
        985856815104",
    );
    test(
        1.23e100,
        "123000000000000008366862950845375853795062237854139353014252897832358837028676639186389822\
        00322686976",
    );
    test(
        -1.23e100,
        "-12300000000000000836686295084537585379506223785413935301425289783235883702867663918638982\
        200322686976",
    );
    test(123.1, "123");
    test(-123.1, "-123");
    test(123.9, "124");
    test(-123.9, "-124");
    test(123.5, "124");
    test(-123.5, "-124");
    test(124.5, "124");
    test(-124.5, "-124");
    test(-0.499, "0");
    test(-0.5, "0");
    test(f64::MIN_POSITIVE, "0");
    test(-f64::MIN_POSITIVE, "0");
    test(f64::MAX_SUBNORMAL, "0");
    test(-f64::MAX_SUBNORMAL, "0");
    test(f64::MIN_POSITIVE_NORMAL, "0");
    test(-f64::MIN_POSITIVE_NORMAL, "0");
    test(f64::MAX_FINITE,
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368");
    test(f64::MIN_FINITE,
        "-17976931348623157081452742373170435679807056752584499659891747680315726078002853876058955\
        8632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245\
        4900903893289440758685084551339423045832369032229481658085593321233482747978262041447231687\
        38177180919299881250404026184124858368");
}

#[test]
#[should_panic]
fn from_f64_fail_1() {
    Integer::from(f64::NAN);
}

#[test]
#[should_panic]
fn from_f64_fail_2() {
    Integer::from(f64::POSITIVE_INFINITY);
}

#[test]
#[should_panic]
fn from_f64_fail_3() {
    Integer::from(f64::NEGATIVE_INFINITY);
}

#[test]
fn test_checked_from_f32() {
    let test = |f: f32, out| {
        let on = Integer::checked_from(f);
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(f32::NAN, "None");
    test(f32::POSITIVE_INFINITY, "None");
    test(f32::NEGATIVE_INFINITY, "None");
    test(0.0, "Some(0)");
    test(-0.0, "Some(0)");
    test(123.0, "Some(123)");
    test(-123.0, "Some(-123)");
    test(1.0e9, "Some(1000000000)");
    test(-1.0e9, "Some(-1000000000)");
    test(4294967295.0, "Some(4294967296)");
    test(-4294967295.0, "Some(-4294967296)");
    test(4294967296.0, "Some(4294967296)");
    test(-4294967296.0, "Some(-4294967296)");
    test(18446744073709551615.0, "Some(18446744073709551616)");
    test(-18446744073709551615.0, "Some(-18446744073709551616)");
    test(18446744073709551616.0, "Some(18446744073709551616)");
    test(-18446744073709551616.0, "Some(-18446744073709551616)");
    test(1.0e20, "Some(100000002004087734272)");
    test(-1.0e20, "Some(-100000002004087734272)");
    test(1.23e20, "Some(122999999650278146048)");
    test(-1.23e20, "Some(-122999999650278146048)");
    test(123.1, "None");
    test(-123.1, "None");
    test(123.5, "None");
    test(-123.5, "None");
    test(124.5, "None");
    test(-124.5, "None");
    test(f32::MIN_POSITIVE, "None");
    test(-f32::MIN_POSITIVE, "None");
    test(f32::MAX_SUBNORMAL, "None");
    test(-f32::MAX_SUBNORMAL, "None");
    test(f32::MIN_POSITIVE_NORMAL, "None");
    test(-f32::MIN_POSITIVE_NORMAL, "None");
    test(
        f32::MAX_FINITE,
        "Some(340282346638528859811704183484516925440)",
    );
    test(
        f32::MIN_FINITE,
        "Some(-340282346638528859811704183484516925440)",
    );
}

#[test]
fn test_checked_from_f64() {
    let test = |f: f64, out| {
        let on = Integer::checked_from(f);
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test(f64::NAN, "None");
    test(f64::POSITIVE_INFINITY, "None");
    test(f64::NEGATIVE_INFINITY, "None");
    test(0.0, "Some(0)");
    test(-0.0, "Some(0)");
    test(123.0, "Some(123)");
    test(-123.0, "Some(-123)");
    test(1.0e9, "Some(1000000000)");
    test(-1.0e9, "Some(-1000000000)");
    test(4294967295.0, "Some(4294967295)");
    test(-4294967295.0, "Some(-4294967295)");
    test(4294967296.0, "Some(4294967296)");
    test(-4294967296.0, "Some(-4294967296)");
    test(18446744073709551615.0, "Some(18446744073709551616)");
    test(-18446744073709551615.0, "Some(-18446744073709551616)");
    test(18446744073709551616.0, "Some(18446744073709551616)");
    test(-18446744073709551616.0, "Some(-18446744073709551616)");
    test(1.0e20, "Some(100000000000000000000)");
    test(-1.0e20, "Some(-100000000000000000000)");
    test(1.23e20, "Some(123000000000000000000)");
    test(-1.23e20, "Some(-123000000000000000000)");
    test(
        1.0e100,
        "Some(1000000000000000015902891109759918046836080856394528138978132755774783877217038106081\
        3469985856815104)",
    );
    test(
        -1.0e100,
        "Some(-100000000000000001590289110975991804683608085639452813897813275577478387721703810608\
        13469985856815104)",
    );
    test(
        1.23e100,
        "Some(1230000000000000083668629508453758537950622378541393530142528978323588370286766391863\
        8982200322686976)",
    );
    test(
        -1.23e100,
        "Some(-123000000000000008366862950845375853795062237854139353014252897832358837028676639186\
        38982200322686976)",
    );
    test(123.1, "None");
    test(-123.1, "None");
    test(123.5, "None");
    test(-123.5, "None");
    test(124.5, "None");
    test(-124.5, "None");
    test(f64::MIN_POSITIVE, "None");
    test(-f64::MIN_POSITIVE, "None");
    test(f64::MAX_SUBNORMAL, "None");
    test(-f64::MAX_SUBNORMAL, "None");
    test(f64::MIN_POSITIVE_NORMAL, "None");
    test(-f64::MIN_POSITIVE_NORMAL, "None");
    test(f64::MAX_FINITE,
        "Some(1797693134862315708145274237317043567980705675258449965989174768031572607800285387605\
        8955863276687817154045895351438246423432132688946418276846754670353751698604991057655128207\
        6245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723\
        168738177180919299881250404026184124858368)");
    test(f64::MIN_FINITE,
        "Some(-179769313486231570814527423731704356798070567525844996598917476803157260780028538760\
        5895586327668781715404589535143824642343213268894641827684675467035375169860499105765512820\
        7624549009038932894407586850845513394230458323690322294816580855933212334827479782620414472\
        3168738177180919299881250404026184124858368)");
}

#[test]
fn test_convertible_from_f32() {
    let test = |f: f32, out| {
        assert_eq!(Integer::convertible_from(f), out);
    };
    test(f32::NAN, false);
    test(f32::POSITIVE_INFINITY, false);
    test(f32::NEGATIVE_INFINITY, false);
    test(0.0, true);
    test(-0.0, true);
    test(123.0, true);
    test(-123.0, true);
    test(1.0e9, true);
    test(-1.0e9, true);
    test(4294967295.0, true);
    test(-4294967295.0, true);
    test(4294967296.0, true);
    test(-4294967296.0, true);
    test(18446744073709551615.0, true);
    test(-18446744073709551615.0, true);
    test(18446744073709551616.0, true);
    test(-18446744073709551616.0, true);
    test(1.0e20, true);
    test(-1.0e20, true);
    test(1.23e20, true);
    test(-1.23e20, true);
    test(123.1, false);
    test(-123.1, false);
    test(123.5, false);
    test(-123.5, false);
    test(124.5, false);
    test(-124.5, false);
    test(f32::MIN_POSITIVE, false);
    test(-f32::MIN_POSITIVE, false);
    test(f32::MAX_SUBNORMAL, false);
    test(-f32::MAX_SUBNORMAL, false);
    test(f32::MIN_POSITIVE_NORMAL, false);
    test(-f32::MIN_POSITIVE_NORMAL, false);
    test(f32::MAX_FINITE, true);
    test(f32::MIN_FINITE, true);
}

#[test]
fn test_convertible_from_f64() {
    let test = |f: f64, out| {
        assert_eq!(Integer::convertible_from(f), out);
    };
    test(f64::NAN, false);
    test(f64::POSITIVE_INFINITY, false);
    test(f64::NEGATIVE_INFINITY, false);
    test(0.0, true);
    test(-0.0, true);
    test(123.0, true);
    test(-123.0, true);
    test(1.0e9, true);
    test(-1.0e9, true);
    test(4294967295.0, true);
    test(-4294967295.0, true);
    test(4294967296.0, true);
    test(-4294967296.0, true);
    test(18446744073709551615.0, true);
    test(-18446744073709551615.0, true);
    test(18446744073709551616.0, true);
    test(-18446744073709551616.0, true);
    test(1.0e20, true);
    test(-1.0e20, true);
    test(1.23e20, true);
    test(-1.23e20, true);
    test(1.0e100, true);
    test(-1.0e100, true);
    test(1.23e100, true);
    test(-1.23e100, true);
    test(123.1, false);
    test(-123.1, false);
    test(123.5, false);
    test(-123.5, false);
    test(124.5, false);
    test(-124.5, false);
    test(f64::MIN_POSITIVE, false);
    test(-f64::MIN_POSITIVE, false);
    test(f64::MAX_SUBNORMAL, false);
    test(-f64::MAX_SUBNORMAL, false);
    test(f64::MIN_POSITIVE_NORMAL, false);
    test(-f64::MIN_POSITIVE_NORMAL, false);
    test(f64::MAX_FINITE, true);
    test(f64::MIN_FINITE, true);
}

macro_rules! float_properties {
    (
        $f: ident,
        $finite_floats: ident,
        $pairs_of_float_and_rounding_mode_var_2: ident,
        $floats_exactly_equal_to_integer: ident,
        $floats_var_4: ident,
        $floats_var_5: ident,
        $rounding_from_float_properties: ident,
        $from_float_properties: ident,
        $checked_from_float_properties: ident,
        $convertible_from_float_properties: ident,
    ) => {
        #[test]
        fn $rounding_from_float_properties() {
            test_properties($pairs_of_float_and_rounding_mode_var_2, |&(f, rm)| {
                let n = Integer::rounding_from(f, rm);
                assert!(n.is_valid());
                assert_eq!(Integer::rounding_from(-f, -rm), -n);
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                let n = Integer::rounding_from(f, RoundingMode::Exact);
                assert!(n.is_valid());
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Floor));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Ceiling));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Down));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Up));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Nearest));
                assert_eq!($f::rounding_from(n, RoundingMode::Exact), f);
            });

            test_properties($floats_var_4, |&f| {
                let n_floor = Integer::rounding_from(f, RoundingMode::Floor);
                assert!(n_floor.is_valid());
                let n_ceiling = &n_floor + Integer::ONE;
                assert_eq!(n_ceiling, Integer::rounding_from(f, RoundingMode::Ceiling));
                if f >= 0.0 {
                    assert_eq!(n_floor, Integer::rounding_from(f, RoundingMode::Down));
                    assert_eq!(n_ceiling, Integer::rounding_from(f, RoundingMode::Up));
                } else {
                    assert_eq!(n_ceiling, Integer::rounding_from(f, RoundingMode::Down));
                    assert_eq!(n_floor, Integer::rounding_from(f, RoundingMode::Up));
                }
                let n_nearest = Integer::rounding_from(f, RoundingMode::Nearest);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
                assert_ne!($f::from(n_nearest), f);
            });

            test_properties($floats_var_5, |&f| {
                let floor = Integer::rounding_from(f, RoundingMode::Floor);
                let ceiling = &floor + Integer::ONE;
                let nearest = Integer::rounding_from(f, RoundingMode::Nearest);
                assert_eq!(nearest, if floor.even() { floor } else { ceiling });
            });
        }

        #[test]
        fn $from_float_properties() {
            test_properties($finite_floats, |&f| {
                let n = Integer::from(f);
                assert!(n.is_valid());
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Nearest));
                assert_eq!(Integer::from(-f), -n);
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                let n = Integer::from(f);
                assert!(n.is_valid());
                assert_eq!($f::from(n), f);
            });

            test_properties($floats_var_4, |&f| {
                let n_floor = Integer::rounding_from(f, RoundingMode::Floor);
                assert!(n_floor.is_valid());
                let n_ceiling = &n_floor + Integer::ONE;
                let n_nearest = Integer::from(f);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
            });

            test_properties($floats_var_5, |&f| {
                let floor = Integer::rounding_from(f, RoundingMode::Floor);
                let ceiling = &floor + Integer::ONE;
                let nearest = Integer::from(f);
                assert_eq!(nearest, if floor.even() { floor } else { ceiling });
            });
        }

        #[test]
        fn $checked_from_float_properties() {
            test_properties($finite_floats, |&f| {
                let on = Integer::checked_from(f);
                assert!(on.as_ref().map_or(true, |n| n.is_valid()));
                assert_eq!(Integer::checked_from(-f), on.map(|n| -n));
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                let n = Integer::checked_from(f).unwrap();
                assert!(n.is_valid());
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Exact));
                assert_eq!($f::rounding_from(n, RoundingMode::Exact), f);
            });

            test_properties($floats_var_4, |&f| {
                assert!(Integer::checked_from(f).is_none());
            });

            test_properties($floats_var_5, |&f| {
                assert!(Integer::checked_from(f).is_none());
            });
        }

        #[test]
        fn $convertible_from_float_properties() {
            test_properties($finite_floats, |&f| {
                Integer::convertible_from(f);
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                assert!(Integer::convertible_from(f));
            });

            test_properties($floats_var_4, |&f| {
                assert!(!Integer::convertible_from(f));
            });

            test_properties($floats_var_5, |&f| {
                assert!(!Integer::convertible_from(f));
            });
        }
    };
}

float_properties!(
    f32,
    finite_f32s,
    pairs_of_finite_f32_and_rounding_mode_var_2,
    f32s_exactly_equal_to_integer,
    f32s_var_4,
    f32s_var_5,
    rounding_from_f32_properties,
    from_f32_properties,
    checked_from_f32_properties,
    convertible_from_f32_properties,
);
float_properties!(
    f64,
    finite_f64s,
    pairs_of_finite_f64_and_rounding_mode_var_2,
    f64s_exactly_equal_to_integer,
    f64s_var_4,
    f64s_var_5,
    rounding_from_f64_properties,
    from_f64_properties,
    checked_from_f64_properties,
    convertible_from_f64_properties,
);
