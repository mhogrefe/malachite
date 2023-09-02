use malachite_base::num::arithmetic::traits::{Ceiling, Floor, Parity};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::ToDebugString;
use malachite_float::test_util::common::{parse_hex_string, rug_round_try_from_rounding_mode};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_4, float_rounding_mode_pair_gen_var_2,
};
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::Rational;
use std::cmp::Ordering;
use std::panic::catch_unwind;

#[test]
fn test_try_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let on = Integer::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Integer::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("NaN", "NaN", "Err(FloatInfiniteOrNan)");
    test("Infinity", "Infinity", "Err(FloatInfiniteOrNan)");
    test("-Infinity", "-Infinity", "Err(FloatInfiniteOrNan)");
    test("0.0", "0x0.0", "Ok(0)");
    test("-0.0", "-0x0.0", "Ok(0)");

    test("1.0", "0x1.0#1", "Ok(1)");
    test("2.0", "0x2.0#1", "Ok(2)");
    test("0.5", "0x0.8#1", "Err(FloatNonIntegerOrOutOfRange)");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "Err(FloatNonIntegerOrOutOfRange)",
    );
    test("123.0", "0x7b.0#7", "Ok(123)");
    test("1000000000000.0", "0xe8d4a51000.0#40", "Ok(1000000000000)");

    test("-1.0", "-0x1.0#1", "Ok(-1)");
    test("-2.0", "-0x2.0#1", "Ok(-2)");
    test("-0.5", "-0x0.8#1", "Err(FloatNonIntegerOrOutOfRange)");
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "Err(FloatNonIntegerOrOutOfRange)",
    );
    test("-123.0", "-0x7b.0#7", "Ok(-123)");
    test(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        "Ok(-1000000000000)",
    );
}

#[test]
fn test_convertible_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(Integer::convertible_from(&x), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("123.0", "0x7b.0#7", true);
    test("1000000000000.0", "0xe8d4a51000.0#40", true);

    test("-1.0", "-0x1.0#1", true);
    test("-2.0", "-0x2.0#1", true);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-123.0", "-0x7b.0#7", true);
    test("-1000000000000.0", "-0xe8d4a51000.0#40", true);
}

#[test]
fn test_rounding_from_float() {
    let test = |s, s_hex, rm, out, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (n, o) = Integer::rounding_from(x.clone(), rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o, o_out);

        let (n, o) = Integer::rounding_from(&x, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rn, ro) = rug::Float::exact_from(&x).to_integer_round(rm).unwrap();
            assert_eq!(Integer::from(&rn), n);
            assert_eq!(ro, o);
        }
    };
    test("0.0", "0x0.0", RoundingMode::Floor, "0", Ordering::Equal);
    test("0.0", "0x0.0", RoundingMode::Ceiling, "0", Ordering::Equal);
    test("0.0", "0x0.0", RoundingMode::Down, "0", Ordering::Equal);
    test("0.0", "0x0.0", RoundingMode::Up, "0", Ordering::Equal);
    test("0.0", "0x0.0", RoundingMode::Nearest, "0", Ordering::Equal);
    test("0.0", "0x0.0", RoundingMode::Exact, "0", Ordering::Equal);

    test("-0.0", "-0x0.0", RoundingMode::Floor, "0", Ordering::Equal);
    test(
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "0",
        Ordering::Equal,
    );
    test("-0.0", "-0x0.0", RoundingMode::Down, "0", Ordering::Equal);
    test("-0.0", "-0x0.0", RoundingMode::Up, "0", Ordering::Equal);
    test(
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "0",
        Ordering::Equal,
    );
    test("-0.0", "-0x0.0", RoundingMode::Exact, "0", Ordering::Equal);

    test("1.0", "0x1.0#1", RoundingMode::Floor, "1", Ordering::Equal);
    test(
        "1.0",
        "0x1.0#1",
        RoundingMode::Ceiling,
        "1",
        Ordering::Equal,
    );
    test("1.0", "0x1.0#1", RoundingMode::Down, "1", Ordering::Equal);
    test("1.0", "0x1.0#1", RoundingMode::Up, "1", Ordering::Equal);
    test(
        "1.0",
        "0x1.0#1",
        RoundingMode::Nearest,
        "1",
        Ordering::Equal,
    );
    test("1.0", "0x1.0#1", RoundingMode::Exact, "1", Ordering::Equal);

    test("2.0", "0x2.0#1", RoundingMode::Floor, "2", Ordering::Equal);
    test(
        "2.0",
        "0x2.0#1",
        RoundingMode::Ceiling,
        "2",
        Ordering::Equal,
    );
    test("2.0", "0x2.0#1", RoundingMode::Down, "2", Ordering::Equal);
    test("2.0", "0x2.0#1", RoundingMode::Up, "2", Ordering::Equal);
    test(
        "2.0",
        "0x2.0#1",
        RoundingMode::Nearest,
        "2",
        Ordering::Equal,
    );
    test("2.0", "0x2.0#1", RoundingMode::Exact, "2", Ordering::Equal);

    test("0.5", "0x0.8#1", RoundingMode::Floor, "0", Ordering::Less);
    test(
        "0.5",
        "0x0.8#1",
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test("0.5", "0x0.8#1", RoundingMode::Down, "0", Ordering::Less);
    test("0.5", "0x0.8#1", RoundingMode::Up, "1", Ordering::Greater);
    test("0.5", "0x0.8#1", RoundingMode::Nearest, "0", Ordering::Less);

    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        RoundingMode::Floor,
        "0",
        Ordering::Less,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        RoundingMode::Down,
        "0",
        Ordering::Less,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        RoundingMode::Up,
        "1",
        Ordering::Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        RoundingMode::Nearest,
        "0",
        Ordering::Less,
    );

    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Floor,
        "0",
        Ordering::Less,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Ceiling,
        "1",
        Ordering::Greater,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Down,
        "0",
        Ordering::Less,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Up,
        "1",
        Ordering::Greater,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Nearest,
        "1",
        Ordering::Greater,
    );

    test("1.5", "0x1.8#2", RoundingMode::Floor, "1", Ordering::Less);
    test(
        "1.5",
        "0x1.8#2",
        RoundingMode::Ceiling,
        "2",
        Ordering::Greater,
    );
    test("1.5", "0x1.8#2", RoundingMode::Down, "1", Ordering::Less);
    test("1.5", "0x1.8#2", RoundingMode::Up, "2", Ordering::Greater);
    test(
        "1.5",
        "0x1.8#2",
        RoundingMode::Nearest,
        "2",
        Ordering::Greater,
    );

    test("2.5", "0x2.8#3", RoundingMode::Floor, "2", Ordering::Less);
    test(
        "2.5",
        "0x2.8#3",
        RoundingMode::Ceiling,
        "3",
        Ordering::Greater,
    );
    test("2.5", "0x2.8#3", RoundingMode::Down, "2", Ordering::Less);
    test("2.5", "0x2.8#3", RoundingMode::Up, "3", Ordering::Greater);
    test("2.5", "0x2.8#3", RoundingMode::Nearest, "2", Ordering::Less);

    test(
        "123.0",
        "0x7b.0#7",
        RoundingMode::Floor,
        "123",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        RoundingMode::Ceiling,
        "123",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        RoundingMode::Down,
        "123",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        RoundingMode::Up,
        "123",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        RoundingMode::Nearest,
        "123",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        RoundingMode::Exact,
        "123",
        Ordering::Equal,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Floor,
        "-1",
        Ordering::Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Ceiling,
        "-1",
        Ordering::Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Down,
        "-1",
        Ordering::Equal,
    );
    test("-1.0", "-0x1.0#1", RoundingMode::Up, "-1", Ordering::Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Nearest,
        "-1",
        Ordering::Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Exact,
        "-1",
        Ordering::Equal,
    );

    test(
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Floor,
        "-2",
        Ordering::Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Ceiling,
        "-2",
        Ordering::Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Down,
        "-2",
        Ordering::Equal,
    );
    test("-2.0", "-0x2.0#1", RoundingMode::Up, "-2", Ordering::Equal);
    test(
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Nearest,
        "-2",
        Ordering::Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        RoundingMode::Exact,
        "-2",
        Ordering::Equal,
    );

    test(
        "-0.5",
        "-0x0.8#1",
        RoundingMode::Floor,
        "-1",
        Ordering::Less,
    );
    test(
        "-0.5",
        "-0x0.8#1",
        RoundingMode::Ceiling,
        "0",
        Ordering::Greater,
    );
    test(
        "-0.5",
        "-0x0.8#1",
        RoundingMode::Down,
        "0",
        Ordering::Greater,
    );
    test("-0.5", "-0x0.8#1", RoundingMode::Up, "-1", Ordering::Less);
    test(
        "-0.5",
        "-0x0.8#1",
        RoundingMode::Nearest,
        "0",
        Ordering::Greater,
    );

    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        RoundingMode::Floor,
        "-1",
        Ordering::Less,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        RoundingMode::Ceiling,
        "0",
        Ordering::Greater,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        RoundingMode::Down,
        "0",
        Ordering::Greater,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        RoundingMode::Up,
        "-1",
        Ordering::Less,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        RoundingMode::Nearest,
        "0",
        Ordering::Greater,
    );

    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Floor,
        "-1",
        Ordering::Less,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Ceiling,
        "0",
        Ordering::Greater,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Down,
        "0",
        Ordering::Greater,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Up,
        "-1",
        Ordering::Less,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        RoundingMode::Nearest,
        "-1",
        Ordering::Less,
    );

    test(
        "-1.5",
        "-0x1.8#2",
        RoundingMode::Floor,
        "-2",
        Ordering::Less,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        RoundingMode::Ceiling,
        "-1",
        Ordering::Greater,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        RoundingMode::Down,
        "-1",
        Ordering::Greater,
    );
    test("-1.5", "-0x1.8#2", RoundingMode::Up, "-2", Ordering::Less);
    test(
        "-1.5",
        "-0x1.8#2",
        RoundingMode::Nearest,
        "-2",
        Ordering::Less,
    );

    test(
        "-2.5",
        "-0x2.8#3",
        RoundingMode::Floor,
        "-3",
        Ordering::Less,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        RoundingMode::Ceiling,
        "-2",
        Ordering::Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        RoundingMode::Down,
        "-2",
        Ordering::Greater,
    );
    test("-2.5", "-0x2.8#3", RoundingMode::Up, "-3", Ordering::Less);
    test(
        "-2.5",
        "-0x2.8#3",
        RoundingMode::Nearest,
        "-2",
        Ordering::Greater,
    );

    test(
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Floor,
        "-123",
        Ordering::Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Ceiling,
        "-123",
        Ordering::Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Down,
        "-123",
        Ordering::Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Up,
        "-123",
        Ordering::Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Nearest,
        "-123",
        Ordering::Equal,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        RoundingMode::Exact,
        "-123",
        Ordering::Equal,
    );
}

#[test]
fn integer_rounding_from_float_fail() {
    assert_panic!(Integer::rounding_from(Float::NAN, RoundingMode::Floor));
    assert_panic!(Integer::rounding_from(Float::NAN, RoundingMode::Ceiling));
    assert_panic!(Integer::rounding_from(Float::NAN, RoundingMode::Down));
    assert_panic!(Integer::rounding_from(Float::NAN, RoundingMode::Up));
    assert_panic!(Integer::rounding_from(Float::NAN, RoundingMode::Nearest));
    assert_panic!(Integer::rounding_from(Float::NAN, RoundingMode::Exact));

    assert_panic!(Integer::rounding_from(Float::INFINITY, RoundingMode::Floor));
    assert_panic!(Integer::rounding_from(
        Float::INFINITY,
        RoundingMode::Ceiling
    ));
    assert_panic!(Integer::rounding_from(Float::INFINITY, RoundingMode::Down));
    assert_panic!(Integer::rounding_from(Float::INFINITY, RoundingMode::Up));
    assert_panic!(Integer::rounding_from(
        Float::INFINITY,
        RoundingMode::Nearest
    ));
    assert_panic!(Integer::rounding_from(Float::INFINITY, RoundingMode::Exact));

    assert_panic!(Integer::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(Integer::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Ceiling
    ));
    assert_panic!(Integer::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Down
    ));
    assert_panic!(Integer::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Up
    ));
    assert_panic!(Integer::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Nearest
    ));
    assert_panic!(Integer::rounding_from(
        Float::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(Integer::rounding_from(
        Float::from_unsigned_times_power_of_2(3u8, -1),
        RoundingMode::Exact
    ));
    assert_panic!(Integer::rounding_from(
        Float::from_signed_times_power_of_2(-3i8, -1),
        RoundingMode::Exact
    ));
}

#[test]
fn integer_rounding_from_float_ref_fail() {
    assert_panic!(Integer::rounding_from(&Float::NAN, RoundingMode::Floor));
    assert_panic!(Integer::rounding_from(&Float::NAN, RoundingMode::Ceiling));
    assert_panic!(Integer::rounding_from(&Float::NAN, RoundingMode::Down));
    assert_panic!(Integer::rounding_from(&Float::NAN, RoundingMode::Up));
    assert_panic!(Integer::rounding_from(&Float::NAN, RoundingMode::Nearest));
    assert_panic!(Integer::rounding_from(&Float::NAN, RoundingMode::Exact));

    assert_panic!(Integer::rounding_from(
        &Float::INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(Integer::rounding_from(
        &Float::INFINITY,
        RoundingMode::Ceiling
    ));
    assert_panic!(Integer::rounding_from(&Float::INFINITY, RoundingMode::Down));
    assert_panic!(Integer::rounding_from(&Float::INFINITY, RoundingMode::Up));
    assert_panic!(Integer::rounding_from(
        &Float::INFINITY,
        RoundingMode::Nearest
    ));
    assert_panic!(Integer::rounding_from(
        &Float::INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(Integer::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(Integer::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Ceiling
    ));
    assert_panic!(Integer::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Down
    ));
    assert_panic!(Integer::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Up
    ));
    assert_panic!(Integer::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Nearest
    ));
    assert_panic!(Integer::rounding_from(
        &Float::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(Integer::rounding_from(
        &Float::from_unsigned_times_power_of_2(3u8, -1),
        RoundingMode::Exact
    ));
    assert_panic!(Integer::rounding_from(
        &Float::from_signed_times_power_of_2(-3i8, -1),
        RoundingMode::Exact
    ));
}

#[test]
fn try_from_float_properties() {
    float_gen().test_properties(|x| {
        let integer_x = Integer::try_from(x.clone());
        assert!(integer_x.as_ref().map_or(true, Integer::is_valid));

        let integer_x_alt = Integer::try_from(&x);
        assert!(integer_x_alt.as_ref().map_or(true, Integer::is_valid));
        assert_eq!(integer_x, integer_x_alt);

        assert_eq!(integer_x.is_ok(), Integer::convertible_from(&x));
        if let Ok(n) = integer_x {
            assert_eq!(Integer::exact_from(&x), n);
            assert_eq!(n, x);
            assert_eq!(Float::from(&n), x);
            assert_eq!(Float::from(n), x);
        }
    });
}

#[test]
fn convertible_from_float_properties() {
    float_gen().test_properties(|x| {
        Integer::convertible_from(&x);
    });
}

#[test]
fn rounding_from_float_properties() {
    float_rounding_mode_pair_gen_var_2().test_properties(|(x, rm)| {
        let no = Integer::rounding_from(&x, rm);
        assert_eq!(Integer::rounding_from(x.clone(), rm), no);
        let (n, o) = no;
        assert!((Rational::from(&n) - Rational::exact_from(&x)).lt_abs(&1));

        assert_eq!(n.partial_cmp(&x), Some(o));
        match (x >= 0, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rn, ro) = rug::Float::exact_from(&x).to_integer_round(rm).unwrap();
            assert_eq!(Integer::from(&rn), n);
            assert_eq!(ro, o);
        }
    });

    float_gen_var_4().test_properties(|x| {
        let floor = Integer::rounding_from(&x, RoundingMode::Floor);
        assert_eq!(floor.0, Rational::exact_from(&x).floor());
        assert!(floor.0 <= x);
        assert!(&floor.0 + Integer::ONE > x);
        assert_eq!(
            Integer::rounding_from(
                &x,
                if x >= 0 {
                    RoundingMode::Down
                } else {
                    RoundingMode::Up
                }
            ),
            floor
        );

        let ceiling = Integer::rounding_from(&x, RoundingMode::Ceiling);
        assert_eq!(ceiling.0, Rational::exact_from(&x).ceiling());
        assert!(ceiling.0 >= x);
        assert!(&ceiling.0 - Integer::ONE < x);
        assert_eq!(
            Integer::rounding_from(
                &x,
                if x >= 0 {
                    RoundingMode::Up
                } else {
                    RoundingMode::Down
                }
            ),
            ceiling
        );

        let nearest = Integer::rounding_from(&x, RoundingMode::Nearest);
        assert!(nearest == floor || nearest == ceiling);
        assert!((Rational::from(nearest.0) - Rational::exact_from(x)).le_abs(&Rational::ONE_HALF));
    });

    integer_gen().test_properties(|n| {
        let x = Float::from(&n);
        let no = (n, Ordering::Equal);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Floor), no);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Down), no);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Ceiling), no);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Up), no);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Nearest), no);
        assert_eq!(Integer::rounding_from(&x, RoundingMode::Exact), no);

        let x = Float::from_integer_times_power_of_2((no.0 << 1) | Integer::ONE, -1);
        assert!(Integer::rounding_from(x, RoundingMode::Nearest).0.even());
    });
}
