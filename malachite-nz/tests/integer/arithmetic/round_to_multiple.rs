use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use std::str::FromStr;

#[test]
fn test_round_to_multiple() {
    let test = |s, t, rm, quotient| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n.round_to_multiple_assign(v.clone(), rm);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let r = u.clone().round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);

        let r = u.clone().round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);

        let r = (&u).round_to_multiple(v.clone(), rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);

        let r = (&u).round_to_multiple(&v, rm);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), quotient);
    };
    test("0", "1", RoundingMode::Down, "0");
    test("0", "1", RoundingMode::Floor, "0");
    test("0", "1", RoundingMode::Up, "0");
    test("0", "1", RoundingMode::Ceiling, "0");
    test("0", "1", RoundingMode::Nearest, "0");
    test("0", "1", RoundingMode::Exact, "0");

    test("0", "123", RoundingMode::Down, "0");
    test("0", "123", RoundingMode::Floor, "0");
    test("0", "123", RoundingMode::Up, "0");
    test("0", "123", RoundingMode::Ceiling, "0");
    test("0", "123", RoundingMode::Nearest, "0");
    test("0", "123", RoundingMode::Exact, "0");

    test("1", "1", RoundingMode::Down, "1");
    test("1", "1", RoundingMode::Floor, "1");
    test("1", "1", RoundingMode::Up, "1");
    test("1", "1", RoundingMode::Ceiling, "1");
    test("1", "1", RoundingMode::Nearest, "1");
    test("1", "1", RoundingMode::Exact, "1");

    test("123", "1", RoundingMode::Down, "123");
    test("123", "1", RoundingMode::Floor, "123");
    test("123", "1", RoundingMode::Up, "123");
    test("123", "1", RoundingMode::Ceiling, "123");
    test("123", "1", RoundingMode::Nearest, "123");
    test("123", "1", RoundingMode::Exact, "123");

    test("123", "2", RoundingMode::Down, "122");
    test("123", "2", RoundingMode::Floor, "122");
    test("123", "2", RoundingMode::Up, "124");
    test("123", "2", RoundingMode::Ceiling, "124");
    test("123", "2", RoundingMode::Nearest, "124");

    test("125", "2", RoundingMode::Down, "124");
    test("125", "2", RoundingMode::Floor, "124");
    test("125", "2", RoundingMode::Up, "126");
    test("125", "2", RoundingMode::Ceiling, "126");
    test("125", "2", RoundingMode::Nearest, "124");

    test("123", "123", RoundingMode::Down, "123");
    test("123", "123", RoundingMode::Floor, "123");
    test("123", "123", RoundingMode::Up, "123");
    test("123", "123", RoundingMode::Ceiling, "123");
    test("123", "123", RoundingMode::Nearest, "123");
    test("123", "123", RoundingMode::Exact, "123");

    test("123", "456", RoundingMode::Down, "0");
    test("123", "456", RoundingMode::Floor, "0");
    test("123", "456", RoundingMode::Up, "456");
    test("123", "456", RoundingMode::Ceiling, "456");
    test("123", "456", RoundingMode::Nearest, "0");

    test("1000000000000", "1", RoundingMode::Down, "1000000000000");
    test("1000000000000", "1", RoundingMode::Floor, "1000000000000");
    test("1000000000000", "1", RoundingMode::Up, "1000000000000");
    test("1000000000000", "1", RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", "1", RoundingMode::Nearest, "1000000000000");
    test("1000000000000", "1", RoundingMode::Exact, "1000000000000");

    test("1000000000000", "3", RoundingMode::Down, "999999999999");
    test("1000000000000", "3", RoundingMode::Floor, "999999999999");
    test("1000000000000", "3", RoundingMode::Up, "1000000000002");
    test("1000000000000", "3", RoundingMode::Ceiling, "1000000000002");
    test("1000000000000", "3", RoundingMode::Nearest, "999999999999");

    test("999999999999", "2", RoundingMode::Down, "999999999998");
    test("999999999999", "2", RoundingMode::Floor, "999999999998");
    test("999999999999", "2", RoundingMode::Up, "1000000000000");
    test("999999999999", "2", RoundingMode::Ceiling, "1000000000000");
    test("999999999999", "2", RoundingMode::Nearest, "1000000000000");

    test("1000000000001", "2", RoundingMode::Down, "1000000000000");
    test("1000000000001", "2", RoundingMode::Floor, "1000000000000");
    test("1000000000001", "2", RoundingMode::Up, "1000000000002");
    test("1000000000001", "2", RoundingMode::Ceiling, "1000000000002");
    test("1000000000001", "2", RoundingMode::Nearest, "1000000000000");

    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Down,
        "999999999999996832276305",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Floor,
        "999999999999996832276305",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Up,
        "1000000000000001127243600",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Ceiling,
        "1000000000000001127243600",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Nearest,
        "1000000000000001127243600",
    );

    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Down,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Floor,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Up,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Ceiling,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Nearest,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Exact,
        "1000000000000000000000000",
    );

    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Down,
        "999999999999999999999999",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Floor,
        "999999999999999999999999",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Up,
        "1000000000001000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Ceiling,
        "1000000000001000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Nearest,
        "999999999999999999999999",
    );

    test(
        "2999999999999999999999999",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2000000000000000000000000",
    );
    test(
        "3000000000000000000000000",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "4000000000000000000000000",
    );
    test(
        "3000000000000000000000001",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "4000000000000000000000000",
    );

    test("0", "-1", RoundingMode::Down, "0");
    test("0", "-1", RoundingMode::Floor, "0");
    test("0", "-1", RoundingMode::Up, "0");
    test("0", "-1", RoundingMode::Ceiling, "0");
    test("0", "-1", RoundingMode::Nearest, "0");
    test("0", "-1", RoundingMode::Exact, "0");

    test("0", "-123", RoundingMode::Down, "0");
    test("0", "-123", RoundingMode::Floor, "0");
    test("0", "-123", RoundingMode::Up, "0");
    test("0", "-123", RoundingMode::Ceiling, "0");
    test("0", "-123", RoundingMode::Nearest, "0");
    test("0", "-123", RoundingMode::Exact, "0");

    test("1", "-1", RoundingMode::Down, "1");
    test("1", "-1", RoundingMode::Floor, "1");
    test("1", "-1", RoundingMode::Up, "1");
    test("1", "-1", RoundingMode::Ceiling, "1");
    test("1", "-1", RoundingMode::Nearest, "1");
    test("1", "-1", RoundingMode::Exact, "1");

    test("123", "-1", RoundingMode::Down, "123");
    test("123", "-1", RoundingMode::Floor, "123");
    test("123", "-1", RoundingMode::Up, "123");
    test("123", "-1", RoundingMode::Ceiling, "123");
    test("123", "-1", RoundingMode::Nearest, "123");
    test("123", "-1", RoundingMode::Exact, "123");

    test("123", "-2", RoundingMode::Down, "122");
    test("123", "-2", RoundingMode::Floor, "122");
    test("123", "-2", RoundingMode::Up, "124");
    test("123", "-2", RoundingMode::Ceiling, "124");
    test("123", "-2", RoundingMode::Nearest, "124");

    test("125", "-2", RoundingMode::Down, "124");
    test("125", "-2", RoundingMode::Floor, "124");
    test("125", "-2", RoundingMode::Up, "126");
    test("125", "-2", RoundingMode::Ceiling, "126");
    test("125", "-2", RoundingMode::Nearest, "124");

    test("123", "-123", RoundingMode::Down, "123");
    test("123", "-123", RoundingMode::Floor, "123");
    test("123", "-123", RoundingMode::Up, "123");
    test("123", "-123", RoundingMode::Ceiling, "123");
    test("123", "-123", RoundingMode::Nearest, "123");
    test("123", "-123", RoundingMode::Exact, "123");

    test("123", "-456", RoundingMode::Down, "0");
    test("123", "-456", RoundingMode::Floor, "0");
    test("123", "-456", RoundingMode::Up, "456");
    test("123", "-456", RoundingMode::Ceiling, "456");
    test("123", "-456", RoundingMode::Nearest, "0");

    test("1000000000000", "-1", RoundingMode::Down, "1000000000000");
    test("1000000000000", "-1", RoundingMode::Floor, "1000000000000");
    test("1000000000000", "-1", RoundingMode::Up, "1000000000000");
    test(
        "1000000000000",
        "-1",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1000000000000",
        "-1",
        RoundingMode::Nearest,
        "1000000000000",
    );
    test("1000000000000", "-1", RoundingMode::Exact, "1000000000000");

    test("1000000000000", "-3", RoundingMode::Down, "999999999999");
    test("1000000000000", "-3", RoundingMode::Floor, "999999999999");
    test("1000000000000", "-3", RoundingMode::Up, "1000000000002");
    test(
        "1000000000000",
        "-3",
        RoundingMode::Ceiling,
        "1000000000002",
    );
    test("1000000000000", "-3", RoundingMode::Nearest, "999999999999");

    test("999999999999", "-2", RoundingMode::Down, "999999999998");
    test("999999999999", "-2", RoundingMode::Floor, "999999999998");
    test("999999999999", "-2", RoundingMode::Up, "1000000000000");
    test("999999999999", "-2", RoundingMode::Ceiling, "1000000000000");
    test("999999999999", "-2", RoundingMode::Nearest, "1000000000000");

    test("1000000000001", "-2", RoundingMode::Down, "1000000000000");
    test("1000000000001", "-2", RoundingMode::Floor, "1000000000000");
    test("1000000000001", "-2", RoundingMode::Up, "1000000000002");
    test(
        "1000000000001",
        "-2",
        RoundingMode::Ceiling,
        "1000000000002",
    );
    test(
        "1000000000001",
        "-2",
        RoundingMode::Nearest,
        "1000000000000",
    );

    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Down,
        "999999999999996832276305",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Floor,
        "999999999999996832276305",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Up,
        "1000000000000001127243600",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Ceiling,
        "1000000000000001127243600",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Nearest,
        "1000000000000001127243600",
    );

    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Down,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Floor,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Up,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Ceiling,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Nearest,
        "1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Exact,
        "1000000000000000000000000",
    );

    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Down,
        "999999999999999999999999",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Floor,
        "999999999999999999999999",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Up,
        "1000000000001000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Ceiling,
        "1000000000001000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Nearest,
        "999999999999999999999999",
    );

    test(
        "2999999999999999999999999",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "2000000000000000000000000",
    );
    test(
        "3000000000000000000000000",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "4000000000000000000000000",
    );
    test(
        "3000000000000000000000001",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "4000000000000000000000000",
    );

    test("-1", "1", RoundingMode::Down, "-1");
    test("-1", "1", RoundingMode::Floor, "-1");
    test("-1", "1", RoundingMode::Up, "-1");
    test("-1", "1", RoundingMode::Ceiling, "-1");
    test("-1", "1", RoundingMode::Nearest, "-1");
    test("-1", "1", RoundingMode::Exact, "-1");

    test("-123", "1", RoundingMode::Down, "-123");
    test("-123", "1", RoundingMode::Floor, "-123");
    test("-123", "1", RoundingMode::Up, "-123");
    test("-123", "1", RoundingMode::Ceiling, "-123");
    test("-123", "1", RoundingMode::Nearest, "-123");
    test("-123", "1", RoundingMode::Exact, "-123");

    test("-123", "2", RoundingMode::Down, "-122");
    test("-123", "2", RoundingMode::Floor, "-124");
    test("-123", "2", RoundingMode::Up, "-124");
    test("-123", "2", RoundingMode::Ceiling, "-122");
    test("-123", "2", RoundingMode::Nearest, "-124");

    test("-125", "2", RoundingMode::Down, "-124");
    test("-125", "2", RoundingMode::Floor, "-126");
    test("-125", "2", RoundingMode::Up, "-126");
    test("-125", "2", RoundingMode::Ceiling, "-124");
    test("-125", "2", RoundingMode::Nearest, "-124");

    test("-123", "123", RoundingMode::Down, "-123");
    test("-123", "123", RoundingMode::Floor, "-123");
    test("-123", "123", RoundingMode::Up, "-123");
    test("-123", "123", RoundingMode::Ceiling, "-123");
    test("-123", "123", RoundingMode::Nearest, "-123");
    test("-123", "123", RoundingMode::Exact, "-123");

    test("-123", "456", RoundingMode::Down, "0");
    test("-123", "456", RoundingMode::Floor, "-456");
    test("-123", "456", RoundingMode::Up, "-456");
    test("-123", "456", RoundingMode::Ceiling, "0");
    test("-123", "456", RoundingMode::Nearest, "0");

    test("-1000000000000", "1", RoundingMode::Down, "-1000000000000");
    test("-1000000000000", "1", RoundingMode::Floor, "-1000000000000");
    test("-1000000000000", "1", RoundingMode::Up, "-1000000000000");
    test(
        "-1000000000000",
        "1",
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-1000000000000",
        "1",
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test("-1000000000000", "1", RoundingMode::Exact, "-1000000000000");

    test("-1000000000000", "3", RoundingMode::Down, "-999999999999");
    test("-1000000000000", "3", RoundingMode::Floor, "-1000000000002");
    test("-1000000000000", "3", RoundingMode::Up, "-1000000000002");
    test(
        "-1000000000000",
        "3",
        RoundingMode::Ceiling,
        "-999999999999",
    );
    test(
        "-1000000000000",
        "3",
        RoundingMode::Nearest,
        "-999999999999",
    );

    test("-999999999999", "2", RoundingMode::Down, "-999999999998");
    test("-999999999999", "2", RoundingMode::Floor, "-1000000000000");
    test("-999999999999", "2", RoundingMode::Up, "-1000000000000");
    test("-999999999999", "2", RoundingMode::Ceiling, "-999999999998");
    test(
        "-999999999999",
        "2",
        RoundingMode::Nearest,
        "-1000000000000",
    );

    test("-1000000000001", "2", RoundingMode::Down, "-1000000000000");
    test("-1000000000001", "2", RoundingMode::Floor, "-1000000000002");
    test("-1000000000001", "2", RoundingMode::Up, "-1000000000002");
    test(
        "-1000000000001",
        "2",
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-1000000000001",
        "2",
        RoundingMode::Nearest,
        "-1000000000000",
    );

    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Down,
        "-999999999999996832276305",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Floor,
        "-1000000000000001127243600",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Up,
        "-1000000000000001127243600",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Ceiling,
        "-999999999999996832276305",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Nearest,
        "-1000000000000001127243600",
    );

    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Down,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Floor,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Up,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Ceiling,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Nearest,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Exact,
        "-1000000000000000000000000",
    );

    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Down,
        "-999999999999999999999999",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Floor,
        "-1000000000001000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Up,
        "-1000000000001000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Ceiling,
        "-999999999999999999999999",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Nearest,
        "-999999999999999999999999",
    );

    test(
        "-2999999999999999999999999",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "-2000000000000000000000000",
    );
    test(
        "-3000000000000000000000000",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "-4000000000000000000000000",
    );
    test(
        "-3000000000000000000000001",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "-4000000000000000000000000",
    );

    test("-1", "-1", RoundingMode::Down, "-1");
    test("-1", "-1", RoundingMode::Floor, "-1");
    test("-1", "-1", RoundingMode::Up, "-1");
    test("-1", "-1", RoundingMode::Ceiling, "-1");
    test("-1", "-1", RoundingMode::Nearest, "-1");
    test("-1", "-1", RoundingMode::Exact, "-1");

    test("-123", "-1", RoundingMode::Down, "-123");
    test("-123", "-1", RoundingMode::Floor, "-123");
    test("-123", "-1", RoundingMode::Up, "-123");
    test("-123", "-1", RoundingMode::Ceiling, "-123");
    test("-123", "-1", RoundingMode::Nearest, "-123");
    test("-123", "-1", RoundingMode::Exact, "-123");

    test("-123", "-2", RoundingMode::Down, "-122");
    test("-123", "-2", RoundingMode::Floor, "-124");
    test("-123", "-2", RoundingMode::Up, "-124");
    test("-123", "-2", RoundingMode::Ceiling, "-122");
    test("-123", "-2", RoundingMode::Nearest, "-124");

    test("-125", "-2", RoundingMode::Down, "-124");
    test("-125", "-2", RoundingMode::Floor, "-126");
    test("-125", "-2", RoundingMode::Up, "-126");
    test("-125", "-2", RoundingMode::Ceiling, "-124");
    test("-125", "-2", RoundingMode::Nearest, "-124");

    test("-123", "-123", RoundingMode::Down, "-123");
    test("-123", "-123", RoundingMode::Floor, "-123");
    test("-123", "-123", RoundingMode::Up, "-123");
    test("-123", "-123", RoundingMode::Ceiling, "-123");
    test("-123", "-123", RoundingMode::Nearest, "-123");
    test("-123", "-123", RoundingMode::Exact, "-123");

    test("-123", "-456", RoundingMode::Down, "0");
    test("-123", "-456", RoundingMode::Floor, "-456");
    test("-123", "-456", RoundingMode::Up, "-456");
    test("-123", "-456", RoundingMode::Ceiling, "0");
    test("-123", "-456", RoundingMode::Nearest, "0");

    test("-1000000000000", "-1", RoundingMode::Down, "-1000000000000");
    test(
        "-1000000000000",
        "-1",
        RoundingMode::Floor,
        "-1000000000000",
    );
    test("-1000000000000", "-1", RoundingMode::Up, "-1000000000000");
    test(
        "-1000000000000",
        "-1",
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-1000000000000",
        "-1",
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-1000000000000",
        "-1",
        RoundingMode::Exact,
        "-1000000000000",
    );

    test("-1000000000000", "-3", RoundingMode::Down, "-999999999999");
    test(
        "-1000000000000",
        "-3",
        RoundingMode::Floor,
        "-1000000000002",
    );
    test("-1000000000000", "-3", RoundingMode::Up, "-1000000000002");
    test(
        "-1000000000000",
        "-3",
        RoundingMode::Ceiling,
        "-999999999999",
    );
    test(
        "-1000000000000",
        "-3",
        RoundingMode::Nearest,
        "-999999999999",
    );

    test("-999999999999", "-2", RoundingMode::Down, "-999999999998");
    test("-999999999999", "-2", RoundingMode::Floor, "-1000000000000");
    test("-999999999999", "-2", RoundingMode::Up, "-1000000000000");
    test(
        "-999999999999",
        "-2",
        RoundingMode::Ceiling,
        "-999999999998",
    );
    test(
        "-999999999999",
        "-2",
        RoundingMode::Nearest,
        "-1000000000000",
    );

    test("-1000000000001", "-2", RoundingMode::Down, "-1000000000000");
    test(
        "-1000000000001",
        "-2",
        RoundingMode::Floor,
        "-1000000000002",
    );
    test("-1000000000001", "-2", RoundingMode::Up, "-1000000000002");
    test(
        "-1000000000001",
        "-2",
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-1000000000001",
        "-2",
        RoundingMode::Nearest,
        "-1000000000000",
    );

    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Down,
        "-999999999999996832276305",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Floor,
        "-1000000000000001127243600",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Up,
        "-1000000000000001127243600",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Ceiling,
        "-999999999999996832276305",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Nearest,
        "-1000000000000001127243600",
    );

    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Down,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Floor,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Up,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Ceiling,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Nearest,
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Exact,
        "-1000000000000000000000000",
    );

    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Down,
        "-999999999999999999999999",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Floor,
        "-1000000000001000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Up,
        "-1000000000001000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Ceiling,
        "-999999999999999999999999",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Nearest,
        "-999999999999999999999999",
    );

    test(
        "-2999999999999999999999999",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "-2000000000000000000000000",
    );
    test(
        "-3000000000000000000000000",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "-4000000000000000000000000",
    );
    test(
        "-3000000000000000000000001",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "-4000000000000000000000000",
    );

    test("0", "0", RoundingMode::Floor, "0");
    test("0", "0", RoundingMode::Ceiling, "0");
    test("0", "0", RoundingMode::Down, "0");
    test("0", "0", RoundingMode::Up, "0");
    test("0", "0", RoundingMode::Nearest, "0");
    test("0", "0", RoundingMode::Exact, "0");

    test("2", "0", RoundingMode::Floor, "0");
    test("2", "0", RoundingMode::Down, "0");
    test("2", "0", RoundingMode::Nearest, "0");
    test("-2", "0", RoundingMode::Ceiling, "0");
    test("-2", "0", RoundingMode::Down, "0");
    test("-2", "0", RoundingMode::Nearest, "0");
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_1() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_2() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(Integer::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_3() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(Integer::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_fail_4() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(Integer::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_1() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(&Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_2() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(&Integer::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_3() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(&Integer::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_assign_ref_fail_4() {
    let mut n = Integer::from(10);
    n.round_to_multiple_assign(&Integer::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_1() {
    Integer::from(10).round_to_multiple(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_2() {
    Integer::from(10).round_to_multiple(Integer::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_3() {
    Integer::from(10).round_to_multiple(Integer::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_fail_4() {
    Integer::from(10).round_to_multiple(Integer::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_1() {
    Integer::from(10).round_to_multiple(&Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_2() {
    Integer::from(10).round_to_multiple(&Integer::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_3() {
    Integer::from(10).round_to_multiple(&Integer::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_val_ref_fail_4() {
    Integer::from(10).round_to_multiple(&Integer::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_1() {
    (&Integer::from(10)).round_to_multiple(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_2() {
    (&Integer::from(10)).round_to_multiple(Integer::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_3() {
    (&Integer::from(10)).round_to_multiple(Integer::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_val_fail_4() {
    (&Integer::from(10)).round_to_multiple(Integer::ZERO, RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_1() {
    (&Integer::from(10)).round_to_multiple(&Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_2() {
    (&Integer::from(10)).round_to_multiple(&Integer::ZERO, RoundingMode::Ceiling);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_3() {
    (&Integer::from(10)).round_to_multiple(&Integer::ZERO, RoundingMode::Up);
}

#[test]
#[should_panic]
fn round_to_multiple_ref_ref_fail_4() {
    (&Integer::from(10)).round_to_multiple(&Integer::ZERO, RoundingMode::Exact);
}
