use malachite_base::num::arithmetic::traits::{
    Abs, Parity, PowerOf2, RoundToMultiple, RoundToMultipleOfPowerOf2,
    RoundToMultipleOfPowerOf2Assign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::signed_rounding_mode_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{
    integer_rounding_mode_pair_gen, integer_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_q::test_util::generators::{
    rational_signed_pair_gen_var_1, rational_signed_pair_gen_var_3,
    rational_signed_rounding_mode_triple_gen_var_1,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_round_to_multiple_of_power_of_2() {
    let test = |s, v: i64, rm: RoundingMode, out| {
        let u = Rational::from_str(s).unwrap();

        let mut n = u.clone();
        n.round_to_multiple_of_power_of_2_assign(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).round_to_multiple_of_power_of_2(v, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 0, RoundingMode::Down, "0");
    test("0", 0, RoundingMode::Up, "0");
    test("0", 0, RoundingMode::Floor, "0");
    test("0", 0, RoundingMode::Ceiling, "0");
    test("0", 0, RoundingMode::Nearest, "0");
    test("0", 0, RoundingMode::Exact, "0");

    test("0", 10, RoundingMode::Down, "0");
    test("0", 10, RoundingMode::Up, "0");
    test("0", 10, RoundingMode::Floor, "0");
    test("0", 10, RoundingMode::Ceiling, "0");
    test("0", 10, RoundingMode::Nearest, "0");
    test("0", 10, RoundingMode::Exact, "0");

    test("123", 0, RoundingMode::Down, "123");
    test("123", 0, RoundingMode::Up, "123");
    test("123", 0, RoundingMode::Floor, "123");
    test("123", 0, RoundingMode::Ceiling, "123");
    test("123", 0, RoundingMode::Nearest, "123");
    test("123", 0, RoundingMode::Exact, "123");

    test("123", 2, RoundingMode::Down, "120");
    test("123", 2, RoundingMode::Up, "124");
    test("123", 2, RoundingMode::Floor, "120");
    test("123", 2, RoundingMode::Ceiling, "124");
    test("123", 2, RoundingMode::Nearest, "124");

    test("123", -2, RoundingMode::Down, "123");
    test("123", -2, RoundingMode::Up, "123");
    test("123", -2, RoundingMode::Floor, "123");
    test("123", -2, RoundingMode::Ceiling, "123");
    test("123", -2, RoundingMode::Nearest, "123");
    test("123", -2, RoundingMode::Exact, "123");

    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Down,
        "0",
    );
    test("884279719003555/281474976710656", 2, RoundingMode::Up, "4");
    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Floor,
        "0",
    );
    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Ceiling,
        "4",
    );
    test(
        "884279719003555/281474976710656",
        2,
        RoundingMode::Nearest,
        "4",
    );

    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Down,
        "25/8",
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Up,
        "51/16",
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Floor,
        "25/8",
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Ceiling,
        "51/16",
    );
    test(
        "884279719003555/281474976710656",
        -4,
        RoundingMode::Nearest,
        "25/8",
    );

    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Down,
        "201/64",
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Up,
        "3217/1024",
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Floor,
        "201/64",
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Ceiling,
        "3217/1024",
    );
    test(
        "884279719003555/281474976710656",
        -10,
        RoundingMode::Nearest,
        "3217/1024",
    );

    test("-123", 0, RoundingMode::Down, "-123");
    test("-123", 0, RoundingMode::Up, "-123");
    test("-123", 0, RoundingMode::Floor, "-123");
    test("-123", 0, RoundingMode::Ceiling, "-123");
    test("-123", 0, RoundingMode::Nearest, "-123");
    test("-123", 0, RoundingMode::Exact, "-123");

    test("-123", 2, RoundingMode::Down, "-120");
    test("-123", 2, RoundingMode::Up, "-124");
    test("-123", 2, RoundingMode::Floor, "-124");
    test("-123", 2, RoundingMode::Ceiling, "-120");
    test("-123", 2, RoundingMode::Nearest, "-124");

    test("-123", -2, RoundingMode::Down, "-123");
    test("-123", -2, RoundingMode::Up, "-123");
    test("-123", -2, RoundingMode::Floor, "-123");
    test("-123", -2, RoundingMode::Ceiling, "-123");
    test("-123", -2, RoundingMode::Nearest, "-123");
    test("-123", -2, RoundingMode::Exact, "-123");

    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Down,
        "0",
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Up,
        "-4",
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Floor,
        "-4",
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Ceiling,
        "0",
    );
    test(
        "-884279719003555/281474976710656",
        2,
        RoundingMode::Nearest,
        "-4",
    );

    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Down,
        "-25/8",
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Up,
        "-51/16",
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Floor,
        "-51/16",
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Ceiling,
        "-25/8",
    );
    test(
        "-884279719003555/281474976710656",
        -4,
        RoundingMode::Nearest,
        "-25/8",
    );

    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Down,
        "-201/64",
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Up,
        "-3217/1024",
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Floor,
        "-3217/1024",
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Ceiling,
        "-201/64",
    );
    test(
        "-884279719003555/281474976710656",
        -10,
        RoundingMode::Nearest,
        "-3217/1024",
    );
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_1() {
    Rational::from(-123).round_to_multiple_of_power_of_2_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_2() {
    Rational::from(-123).round_to_multiple_of_power_of_2_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_3() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_assign_fail_4() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2_assign(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_1() {
    Rational::from(-123).round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_2() {
    Rational::from(-123).round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_3() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_fail_4() {
    Rational::from_str("-1000000000001")
        .unwrap()
        .round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_1() {
    (&Rational::from(-123)).round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_2() {
    (&Rational::from(-123)).round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_3() {
    (&Rational::from_str("-1000000000001").unwrap())
        .round_to_multiple_of_power_of_2(1, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_of_power_of_2_ref_fail_4() {
    (&Rational::from_str("-1000000000001").unwrap())
        .round_to_multiple_of_power_of_2(100, RoundingMode::Exact);
}

#[test]
fn round_to_multiple_of_power_of_2_properties() {
    rational_signed_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        let r = (&n).round_to_multiple_of_power_of_2(pow, rm);
        assert!(r.is_valid());

        let r_alt = n.clone().round_to_multiple_of_power_of_2(pow, rm);
        assert!(r_alt.is_valid());
        assert_eq!(r_alt, r);

        let mut mut_n = n.clone();
        mut_n.round_to_multiple_of_power_of_2_assign(pow, rm);
        assert!(mut_n.is_valid());
        assert_eq!(mut_n, r);

        assert!((&r >> pow).is_integer());
        assert_eq!(-(-&n).round_to_multiple_of_power_of_2(pow, -rm), r);
        assert!((&r - &n).abs() <= Rational::power_of_2(pow));
        assert_eq!((&n).round_to_multiple(Rational::power_of_2(pow), rm), r);
        match rm {
            RoundingMode::Floor => assert!(r <= n),
            RoundingMode::Ceiling => assert!(r >= n),
            RoundingMode::Down => assert!(r.le_abs(&n)),
            RoundingMode::Up => assert!(r.ge_abs(&n)),
            RoundingMode::Exact => assert_eq!(r, n),
            RoundingMode::Nearest => {
                let k = Rational::power_of_2(pow);
                let closest;
                let second_closest;
                if r <= n {
                    closest = &n - &r;
                    second_closest = &r + k - n;
                } else {
                    closest = &r - &n;
                    second_closest = n + k - &r;
                }
                assert!(closest <= second_closest);
                if closest == second_closest {
                    assert!(Integer::exact_from(r >> pow).even());
                }
            }
        }
    });

    rational_signed_pair_gen_var_1().test_properties(|(n, pow)| {
        let rounded: Rational = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest),
            rounded
        );
        assert_eq!(
            (&rounded).round_to_multiple_of_power_of_2(pow, RoundingMode::Exact),
            rounded
        );
    });

    rational_signed_pair_gen_var_3().test_properties(|(n, pow)| {
        let floor = (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Floor);
        let ceiling = &floor + Rational::power_of_2(pow);
        assert_eq!(
            (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Ceiling),
            ceiling
        );
        if n >= 0 {
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                ceiling
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
                floor
            );
        } else {
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Up),
                floor
            );
            assert_eq!(
                (&n).round_to_multiple_of_power_of_2(pow, RoundingMode::Down),
                ceiling
            );
        }
        let nearest = n.round_to_multiple_of_power_of_2(pow, RoundingMode::Nearest);
        assert!(nearest == ceiling || nearest == floor);
    });

    integer_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        assert_eq!(Rational::from(&n).round_to_multiple_of_power_of_2(0, rm), n);
    });

    signed_rounding_mode_pair_gen().test_properties(|(pow, rm)| {
        assert_eq!(
            Rational::ZERO.round_to_multiple_of_power_of_2(pow, rm),
            0u32
        );
    });

    integer_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, pow, rm)| {
        assert_eq!(
            (&n).round_to_multiple_of_power_of_2(pow, rm),
            Rational::from(n).round_to_multiple_of_power_of_2(i64::exact_from(pow), rm)
        )
    });
}
