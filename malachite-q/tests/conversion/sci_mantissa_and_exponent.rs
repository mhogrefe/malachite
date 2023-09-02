use malachite_base::assert_panic;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode;
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
use std::cmp::Ordering;
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
    test("3", RoundingMode::Floor, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Down, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Ceiling, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Up, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Nearest, Some((1.5, 1, Ordering::Equal)));
    test("3", RoundingMode::Exact, Some((1.5, 1, Ordering::Equal)));

    test(
        "123",
        RoundingMode::Floor,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Down,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Ceiling,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Up,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Nearest,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "123",
        RoundingMode::Exact,
        Some((1.921875, 6, Ordering::Equal)),
    );

    test(
        "1000000000",
        RoundingMode::Floor,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Down,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Ceiling,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Up,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Nearest,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "1000000000",
        RoundingMode::Exact,
        Some((1.8626451, 29, Ordering::Equal)),
    );

    test(
        "16777216",
        RoundingMode::Floor,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Down,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Ceiling,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Up,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Nearest,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "16777216",
        RoundingMode::Exact,
        Some((1.0, 24, Ordering::Equal)),
    );

    test(
        "16777218",
        RoundingMode::Floor,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Down,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Ceiling,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Up,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Nearest,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "16777218",
        RoundingMode::Exact,
        Some((1.0000001, 24, Ordering::Equal)),
    );

    test(
        "16777217",
        RoundingMode::Floor,
        Some((1.0, 24, Ordering::Less)),
    );
    test(
        "16777217",
        RoundingMode::Down,
        Some((1.0, 24, Ordering::Less)),
    );
    test(
        "16777217",
        RoundingMode::Ceiling,
        Some((1.0000001, 24, Ordering::Greater)),
    );
    test(
        "16777217",
        RoundingMode::Up,
        Some((1.0000001, 24, Ordering::Greater)),
    );
    test(
        "16777217",
        RoundingMode::Nearest,
        Some((1.0, 24, Ordering::Less)),
    );
    test("16777217", RoundingMode::Exact, None);

    test(
        "33554432",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Ceiling,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Up,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "33554432",
        RoundingMode::Exact,
        Some((1.0, 25, Ordering::Equal)),
    );

    test(
        "33554436",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "33554436",
        RoundingMode::Exact,
        Some((1.0000001, 25, Ordering::Equal)),
    );

    test(
        "33554440",
        RoundingMode::Floor,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Down,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "33554440",
        RoundingMode::Exact,
        Some((1.0000002, 25, Ordering::Equal)),
    );

    test(
        "33554433",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554433",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554433",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554433",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554433",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Less)),
    );
    test("33554433", RoundingMode::Exact, None);

    test(
        "33554434",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554434",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554434",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554434",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554434",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Less)),
    );
    test("33554434", RoundingMode::Exact, None);

    test(
        "33554435",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554435",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "33554435",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554435",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "33554435",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test("33554435", RoundingMode::Exact, None);

    test(
        "33554437",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554437",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554437",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554437",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554437",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test("33554437", RoundingMode::Exact, None);

    test(
        "33554438",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554438",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554438",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554438",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554438",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test("33554438", RoundingMode::Exact, None);

    test(
        "33554439",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554439",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "33554439",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554439",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "33554439",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test("33554439", RoundingMode::Exact, None);

    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Floor,
        Some((1.9999998, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Down,
        Some((1.9999998, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Ceiling,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Up,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Exact,
        None,
    );

    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Floor,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Down,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Ceiling,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Up,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Exact,
        Some((1.9999999, 127, Ordering::Equal)),
    );

    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Floor,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Down,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Ceiling,
        Some((1.0, 128, Ordering::Greater)),
    );

    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Up,
        Some((1.0, 128, Ordering::Greater)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Exact,
        None,
    );

    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        Some((1.6704779, 172, Ordering::Less)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        Some((1.6704779, 172, Ordering::Less)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Exact,
        None,
    );

    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Floor,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Down,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Ceiling,
        Some((1.8920968, 458, Ordering::Greater)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Up,
        Some((1.8920968, 458, Ordering::Greater)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Nearest,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Exact,
        None,
    );

    test(
        "1/3",
        RoundingMode::Floor,
        Some((1.3333333, -2, Ordering::Less)),
    );
    test(
        "1/3",
        RoundingMode::Ceiling,
        Some((1.3333334, -2, Ordering::Greater)),
    );
    test(
        "1/3",
        RoundingMode::Down,
        Some((1.3333333, -2, Ordering::Less)),
    );
    test(
        "1/3",
        RoundingMode::Up,
        Some((1.3333334, -2, Ordering::Greater)),
    );
    test(
        "1/3",
        RoundingMode::Nearest,
        Some((1.3333334, -2, Ordering::Greater)),
    );
    test("1/3", RoundingMode::Exact, None);

    test(
        "1/1024",
        RoundingMode::Floor,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "1/1024",
        RoundingMode::Ceiling,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "1/1024",
        RoundingMode::Down,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "1/1024",
        RoundingMode::Up,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "1/1024",
        RoundingMode::Nearest,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "1/1024",
        RoundingMode::Exact,
        Some((1.0, -10, Ordering::Equal)),
    );

    test(
        "22/7",
        RoundingMode::Floor,
        Some((1.5714285, 1, Ordering::Less)),
    );
    test(
        "22/7",
        RoundingMode::Ceiling,
        Some((1.5714287, 1, Ordering::Greater)),
    );
    test(
        "22/7",
        RoundingMode::Down,
        Some((1.5714285, 1, Ordering::Less)),
    );
    test(
        "22/7",
        RoundingMode::Up,
        Some((1.5714287, 1, Ordering::Greater)),
    );
    test(
        "22/7",
        RoundingMode::Nearest,
        Some((1.5714285, 1, Ordering::Less)),
    );
    test("22/7", RoundingMode::Exact, None);

    test(
        "936851431250/1397",
        RoundingMode::Floor,
        Some((1.2491207, 29, Ordering::Less)),
    );
    test(
        "936851431250/1397",
        RoundingMode::Ceiling,
        Some((1.2491208, 29, Ordering::Greater)),
    );
    test(
        "936851431250/1397",
        RoundingMode::Down,
        Some((1.2491207, 29, Ordering::Less)),
    );
    test(
        "936851431250/1397",
        RoundingMode::Up,
        Some((1.2491208, 29, Ordering::Greater)),
    );
    test(
        "936851431250/1397",
        RoundingMode::Nearest,
        Some((1.2491208, 29, Ordering::Greater)),
    );
    test("936851431250/1397", RoundingMode::Exact, None);

    test(
        "1073741823/1099511627776",
        RoundingMode::Floor,
        Some((1.9999999, -11, Ordering::Less)),
    );
    test(
        "1073741823/1099511627776",
        RoundingMode::Ceiling,
        Some((1.0, -10, Ordering::Greater)),
    );
    test(
        "1073741823/1099511627776",
        RoundingMode::Down,
        Some((1.9999999, -11, Ordering::Less)),
    );
    test(
        "1073741823/1099511627776",
        RoundingMode::Up,
        Some((1.0, -10, Ordering::Greater)),
    );
    test(
        "1073741823/1099511627776",
        RoundingMode::Nearest,
        Some((1.0, -10, Ordering::Greater)),
    );
    test("1073741823/1099511627776", RoundingMode::Exact, None);

    test("-3", RoundingMode::Floor, Some((1.5, 1, Ordering::Equal)));
    test("-3", RoundingMode::Down, Some((1.5, 1, Ordering::Equal)));
    test("-3", RoundingMode::Ceiling, Some((1.5, 1, Ordering::Equal)));
    test("-3", RoundingMode::Up, Some((1.5, 1, Ordering::Equal)));
    test("-3", RoundingMode::Nearest, Some((1.5, 1, Ordering::Equal)));
    test("-3", RoundingMode::Exact, Some((1.5, 1, Ordering::Equal)));

    test(
        "-123",
        RoundingMode::Floor,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "-123",
        RoundingMode::Down,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "-123",
        RoundingMode::Ceiling,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "-123",
        RoundingMode::Up,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "-123",
        RoundingMode::Nearest,
        Some((1.921875, 6, Ordering::Equal)),
    );
    test(
        "-123",
        RoundingMode::Exact,
        Some((1.921875, 6, Ordering::Equal)),
    );

    test(
        "-1000000000",
        RoundingMode::Floor,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "-1000000000",
        RoundingMode::Down,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "-1000000000",
        RoundingMode::Ceiling,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "-1000000000",
        RoundingMode::Up,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "-1000000000",
        RoundingMode::Nearest,
        Some((1.8626451, 29, Ordering::Equal)),
    );
    test(
        "-1000000000",
        RoundingMode::Exact,
        Some((1.8626451, 29, Ordering::Equal)),
    );

    test(
        "-16777216",
        RoundingMode::Floor,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "-16777216",
        RoundingMode::Down,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "-16777216",
        RoundingMode::Ceiling,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "-16777216",
        RoundingMode::Up,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "-16777216",
        RoundingMode::Nearest,
        Some((1.0, 24, Ordering::Equal)),
    );
    test(
        "-16777216",
        RoundingMode::Exact,
        Some((1.0, 24, Ordering::Equal)),
    );

    test(
        "-16777218",
        RoundingMode::Floor,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "-16777218",
        RoundingMode::Down,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "-16777218",
        RoundingMode::Ceiling,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "-16777218",
        RoundingMode::Up,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "-16777218",
        RoundingMode::Nearest,
        Some((1.0000001, 24, Ordering::Equal)),
    );
    test(
        "-16777218",
        RoundingMode::Exact,
        Some((1.0000001, 24, Ordering::Equal)),
    );

    test(
        "-16777217",
        RoundingMode::Floor,
        Some((1.0, 24, Ordering::Less)),
    );
    test(
        "-16777217",
        RoundingMode::Down,
        Some((1.0, 24, Ordering::Less)),
    );
    test(
        "-16777217",
        RoundingMode::Ceiling,
        Some((1.0000001, 24, Ordering::Greater)),
    );
    test(
        "-16777217",
        RoundingMode::Up,
        Some((1.0000001, 24, Ordering::Greater)),
    );
    test(
        "-16777217",
        RoundingMode::Nearest,
        Some((1.0, 24, Ordering::Less)),
    );
    test("-16777217", RoundingMode::Exact, None);

    test(
        "-33554432",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "-33554432",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "-33554432",
        RoundingMode::Ceiling,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "-33554432",
        RoundingMode::Up,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "-33554432",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Equal)),
    );
    test(
        "-33554432",
        RoundingMode::Exact,
        Some((1.0, 25, Ordering::Equal)),
    );

    test(
        "-33554436",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "-33554436",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "-33554436",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "-33554436",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "-33554436",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Equal)),
    );
    test(
        "-33554436",
        RoundingMode::Exact,
        Some((1.0000001, 25, Ordering::Equal)),
    );

    test(
        "-33554440",
        RoundingMode::Floor,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "-33554440",
        RoundingMode::Down,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "-33554440",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "-33554440",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "-33554440",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Equal)),
    );
    test(
        "-33554440",
        RoundingMode::Exact,
        Some((1.0000002, 25, Ordering::Equal)),
    );

    test(
        "-33554433",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "-33554433",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "-33554433",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "-33554433",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "-33554433",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Less)),
    );
    test("-33554433", RoundingMode::Exact, None);

    test(
        "-33554434",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "-33554434",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "-33554434",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "-33554434",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "-33554434",
        RoundingMode::Nearest,
        Some((1.0, 25, Ordering::Less)),
    );
    test("-33554434", RoundingMode::Exact, None);

    test(
        "-33554435",
        RoundingMode::Floor,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "-33554435",
        RoundingMode::Down,
        Some((1.0, 25, Ordering::Less)),
    );
    test(
        "-33554435",
        RoundingMode::Ceiling,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "-33554435",
        RoundingMode::Up,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test(
        "-33554435",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Greater)),
    );
    test("-33554435", RoundingMode::Exact, None);

    test(
        "-33554437",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "-33554437",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "-33554437",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "-33554437",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "-33554437",
        RoundingMode::Nearest,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test("-33554437", RoundingMode::Exact, None);

    test(
        "-33554438",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "-33554438",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "-33554438",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "-33554438",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "-33554438",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test("-33554438", RoundingMode::Exact, None);

    test(
        "-33554439",
        RoundingMode::Floor,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "-33554439",
        RoundingMode::Down,
        Some((1.0000001, 25, Ordering::Less)),
    );
    test(
        "-33554439",
        RoundingMode::Ceiling,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "-33554439",
        RoundingMode::Up,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test(
        "-33554439",
        RoundingMode::Nearest,
        Some((1.0000002, 25, Ordering::Greater)),
    );
    test("-33554439", RoundingMode::Exact, None);

    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Floor,
        Some((1.9999998, 127, Ordering::Less)),
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Down,
        Some((1.9999998, 127, Ordering::Less)),
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Ceiling,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Up,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Greater)),
    );
    test(
        "-340282346638528859811704183484516925439",
        RoundingMode::Exact,
        None,
    );

    test(
        "-340282346638528859811704183484516925440",
        RoundingMode::Floor,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        RoundingMode::Down,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        RoundingMode::Ceiling,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        RoundingMode::Up,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Equal)),
    );
    test(
        "-340282346638528859811704183484516925440",
        RoundingMode::Exact,
        Some((1.9999999, 127, Ordering::Equal)),
    );

    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Floor,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Down,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Ceiling,
        Some((1.0, 128, Ordering::Greater)),
    );

    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Up,
        Some((1.0, 128, Ordering::Greater)),
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Nearest,
        Some((1.9999999, 127, Ordering::Less)),
    );
    test(
        "-340282346638528859811704183484516925441",
        RoundingMode::Exact,
        None,
    );

    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        Some((1.6704779, 172, Ordering::Less)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        Some((1.6704779, 172, Ordering::Less)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        Some((1.670478, 172, Ordering::Greater)),
    );
    test(
        "-10000000000000000000000000000000000000000000000000000",
        RoundingMode::Exact,
        None,
    );

    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Floor,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Down,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Ceiling,
        Some((1.8920968, 458, Ordering::Greater)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Up,
        Some((1.8920968, 458, Ordering::Greater)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Nearest,
        Some((1.8920966, 458, Ordering::Less)),
    );
    test(
        "-14082550970654138785851080671018547544414440081606160064666506533805114137489745015963441\
        66687102119468305028824490080062160433429798263165",
        RoundingMode::Exact,
        None,
    );

    test(
        "-1/3",
        RoundingMode::Floor,
        Some((1.3333333, -2, Ordering::Less)),
    );
    test(
        "-1/3",
        RoundingMode::Ceiling,
        Some((1.3333334, -2, Ordering::Greater)),
    );
    test(
        "-1/3",
        RoundingMode::Down,
        Some((1.3333333, -2, Ordering::Less)),
    );
    test(
        "-1/3",
        RoundingMode::Up,
        Some((1.3333334, -2, Ordering::Greater)),
    );
    test(
        "-1/3",
        RoundingMode::Nearest,
        Some((1.3333334, -2, Ordering::Greater)),
    );
    test("-1/3", RoundingMode::Exact, None);

    test(
        "-1/1024",
        RoundingMode::Floor,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "-1/1024",
        RoundingMode::Ceiling,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "-1/1024",
        RoundingMode::Down,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "-1/1024",
        RoundingMode::Up,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "-1/1024",
        RoundingMode::Nearest,
        Some((1.0, -10, Ordering::Equal)),
    );
    test(
        "-1/1024",
        RoundingMode::Exact,
        Some((1.0, -10, Ordering::Equal)),
    );

    test(
        "-22/7",
        RoundingMode::Floor,
        Some((1.5714285, 1, Ordering::Less)),
    );
    test(
        "-22/7",
        RoundingMode::Ceiling,
        Some((1.5714287, 1, Ordering::Greater)),
    );
    test(
        "-22/7",
        RoundingMode::Down,
        Some((1.5714285, 1, Ordering::Less)),
    );
    test(
        "-22/7",
        RoundingMode::Up,
        Some((1.5714287, 1, Ordering::Greater)),
    );
    test(
        "-22/7",
        RoundingMode::Nearest,
        Some((1.5714285, 1, Ordering::Less)),
    );
    test("-22/7", RoundingMode::Exact, None);

    test(
        "-936851431250/1397",
        RoundingMode::Floor,
        Some((1.2491207, 29, Ordering::Less)),
    );
    test(
        "-936851431250/1397",
        RoundingMode::Ceiling,
        Some((1.2491208, 29, Ordering::Greater)),
    );
    test(
        "-936851431250/1397",
        RoundingMode::Down,
        Some((1.2491207, 29, Ordering::Less)),
    );
    test(
        "-936851431250/1397",
        RoundingMode::Up,
        Some((1.2491208, 29, Ordering::Greater)),
    );
    test(
        "-936851431250/1397",
        RoundingMode::Nearest,
        Some((1.2491208, 29, Ordering::Greater)),
    );
    test("-936851431250/1397", RoundingMode::Exact, None);

    test(
        "-1073741823/1099511627776",
        RoundingMode::Floor,
        Some((1.9999999, -11, Ordering::Less)),
    );
    test(
        "-1073741823/1099511627776",
        RoundingMode::Ceiling,
        Some((1.0, -10, Ordering::Greater)),
    );
    test(
        "-1073741823/1099511627776",
        RoundingMode::Down,
        Some((1.9999999, -11, Ordering::Less)),
    );
    test(
        "-1073741823/1099511627776",
        RoundingMode::Up,
        Some((1.0, -10, Ordering::Greater)),
    );
    test(
        "-1073741823/1099511627776",
        RoundingMode::Nearest,
        Some((1.0, -10, Ordering::Greater)),
    );
    test("-1073741823/1099511627776", RoundingMode::Exact, None);
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
            n.sci_mantissa_and_exponent_round(RoundingMode::Nearest)
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
            if rm == RoundingMode::Exact {
                assert_eq!(x.partial_cmp_abs(&n), Some(Ordering::Equal));
            }
            assert_eq!(x.partial_cmp_abs(&n), Some(o));
            match rm {
                RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
                RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
                RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
                _ => {}
            }
        }
    });

    rational_gen_var_1().test_properties(|n| {
        let (floor_mantissa, floor_exponent, floor_o) = n
            .sci_mantissa_and_exponent_round_ref::<T>(RoundingMode::Floor)
            .unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round_ref::<T>(RoundingMode::Down)
                .unwrap(),
            (floor_mantissa, floor_exponent, floor_o)
        );
        let (ceiling_mantissa, ceiling_exponent, ceiling_o) = n
            .sci_mantissa_and_exponent_round_ref::<T>(RoundingMode::Ceiling)
            .unwrap();
        assert_eq!(
            n.sci_mantissa_and_exponent_round_ref::<T>(RoundingMode::Up)
                .unwrap(),
            (ceiling_mantissa, ceiling_exponent, ceiling_o)
        );
        let (nearest_mantissa, nearest_exponent, nearest_o) = n
            .sci_mantissa_and_exponent_round_ref::<T>(RoundingMode::Nearest)
            .unwrap();
        if let Some((mantissa, exponent, o)) =
            n.sci_mantissa_and_exponent_round_ref::<T>(RoundingMode::Exact)
        {
            assert_eq!(o, Ordering::Equal);
            assert_eq!(floor_mantissa, mantissa);
            assert_eq!(ceiling_mantissa, mantissa);
            assert_eq!(nearest_mantissa, mantissa);
            assert_eq!(floor_exponent, exponent);
            assert_eq!(ceiling_exponent, exponent);
            assert_eq!(nearest_exponent, exponent);
        } else {
            assert_eq!(floor_o, Ordering::Less);
            assert_eq!(ceiling_o, Ordering::Greater);
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
