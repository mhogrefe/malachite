use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use num::{BigInt, Integer as NumInteger};
use rug::ops::DivRounding;
use std::str::FromStr;

#[test]
fn test_div_round() {
    let test = |s, t, rm, quotient| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n.div_round_assign(v.clone(), rm);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.div_round_assign(&v, rm);
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = u.clone().div_round(v.clone(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = u.clone().div_round(&v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&u).div_round(v.clone(), rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = (&u).div_round(&v, rm);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        match rm {
            RoundingMode::Down => {
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_trunc(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            RoundingMode::Floor => {
                assert_eq!(
                    BigInt::from_str(s)
                        .unwrap()
                        .div_floor(&BigInt::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_floor(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            RoundingMode::Ceiling => {
                assert_eq!(
                    rug::Integer::from_str(s)
                        .unwrap()
                        .div_ceil(rug::Integer::from_str(t).unwrap())
                        .to_string(),
                    quotient
                );
            }
            _ => {}
        }
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

    test("123", "2", RoundingMode::Down, "61");
    test("123", "2", RoundingMode::Floor, "61");
    test("123", "2", RoundingMode::Up, "62");
    test("123", "2", RoundingMode::Ceiling, "62");
    test("123", "2", RoundingMode::Nearest, "62");

    test("125", "2", RoundingMode::Down, "62");
    test("125", "2", RoundingMode::Floor, "62");
    test("125", "2", RoundingMode::Up, "63");
    test("125", "2", RoundingMode::Ceiling, "63");
    test("125", "2", RoundingMode::Nearest, "62");

    test("123", "123", RoundingMode::Down, "1");
    test("123", "123", RoundingMode::Floor, "1");
    test("123", "123", RoundingMode::Up, "1");
    test("123", "123", RoundingMode::Ceiling, "1");
    test("123", "123", RoundingMode::Nearest, "1");
    test("123", "123", RoundingMode::Exact, "1");

    test("123", "456", RoundingMode::Down, "0");
    test("123", "456", RoundingMode::Floor, "0");
    test("123", "456", RoundingMode::Up, "1");
    test("123", "456", RoundingMode::Ceiling, "1");
    test("123", "456", RoundingMode::Nearest, "0");

    test("1000000000000", "1", RoundingMode::Down, "1000000000000");
    test("1000000000000", "1", RoundingMode::Floor, "1000000000000");
    test("1000000000000", "1", RoundingMode::Up, "1000000000000");
    test("1000000000000", "1", RoundingMode::Ceiling, "1000000000000");
    test("1000000000000", "1", RoundingMode::Nearest, "1000000000000");
    test("1000000000000", "1", RoundingMode::Exact, "1000000000000");

    test("1000000000000", "3", RoundingMode::Down, "333333333333");
    test("1000000000000", "3", RoundingMode::Floor, "333333333333");
    test("1000000000000", "3", RoundingMode::Up, "333333333334");
    test("1000000000000", "3", RoundingMode::Ceiling, "333333333334");
    test("1000000000000", "3", RoundingMode::Nearest, "333333333333");

    test("999999999999", "2", RoundingMode::Down, "499999999999");
    test("999999999999", "2", RoundingMode::Floor, "499999999999");
    test("999999999999", "2", RoundingMode::Up, "500000000000");
    test("999999999999", "2", RoundingMode::Ceiling, "500000000000");
    test("999999999999", "2", RoundingMode::Nearest, "500000000000");

    test("1000000000001", "2", RoundingMode::Down, "500000000000");
    test("1000000000001", "2", RoundingMode::Floor, "500000000000");
    test("1000000000001", "2", RoundingMode::Up, "500000000001");
    test("1000000000001", "2", RoundingMode::Ceiling, "500000000001");
    test("1000000000001", "2", RoundingMode::Nearest, "500000000000");

    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Down,
        "232830643708079",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Floor,
        "232830643708079",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Up,
        "232830643708080",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Ceiling,
        "232830643708080",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        RoundingMode::Nearest,
        "232830643708080",
    );

    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000000",
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Down,
        "999999999999",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Floor,
        "999999999999",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "1000000000000000000000000",
        "1000000000001",
        RoundingMode::Nearest,
        "999999999999",
    );

    test(
        "2999999999999999999999999",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "1",
    );
    test(
        "3000000000000000000000000",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
    );
    test(
        "3000000000000000000000001",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
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

    test("1", "-1", RoundingMode::Down, "-1");
    test("1", "-1", RoundingMode::Floor, "-1");
    test("1", "-1", RoundingMode::Up, "-1");
    test("1", "-1", RoundingMode::Ceiling, "-1");
    test("1", "-1", RoundingMode::Nearest, "-1");
    test("1", "-1", RoundingMode::Exact, "-1");

    test("123", "-1", RoundingMode::Down, "-123");
    test("123", "-1", RoundingMode::Floor, "-123");
    test("123", "-1", RoundingMode::Up, "-123");
    test("123", "-1", RoundingMode::Ceiling, "-123");
    test("123", "-1", RoundingMode::Nearest, "-123");
    test("123", "-1", RoundingMode::Exact, "-123");

    test("123", "-2", RoundingMode::Down, "-61");
    test("123", "-2", RoundingMode::Floor, "-62");
    test("123", "-2", RoundingMode::Up, "-62");
    test("123", "-2", RoundingMode::Ceiling, "-61");
    test("123", "-2", RoundingMode::Nearest, "-62");

    test("125", "-2", RoundingMode::Down, "-62");
    test("125", "-2", RoundingMode::Floor, "-63");
    test("125", "-2", RoundingMode::Up, "-63");
    test("125", "-2", RoundingMode::Ceiling, "-62");
    test("125", "-2", RoundingMode::Nearest, "-62");

    test("123", "-123", RoundingMode::Down, "-1");
    test("123", "-123", RoundingMode::Floor, "-1");
    test("123", "-123", RoundingMode::Up, "-1");
    test("123", "-123", RoundingMode::Ceiling, "-1");
    test("123", "-123", RoundingMode::Nearest, "-1");
    test("123", "-123", RoundingMode::Exact, "-1");

    test("123", "-456", RoundingMode::Down, "0");
    test("123", "-456", RoundingMode::Floor, "-1");
    test("123", "-456", RoundingMode::Up, "-1");
    test("123", "-456", RoundingMode::Ceiling, "0");
    test("123", "-456", RoundingMode::Nearest, "0");

    test("1000000000000", "-1", RoundingMode::Down, "-1000000000000");
    test("1000000000000", "-1", RoundingMode::Floor, "-1000000000000");
    test("1000000000000", "-1", RoundingMode::Up, "-1000000000000");
    test(
        "1000000000000",
        "-1",
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "1000000000000",
        "-1",
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test("1000000000000", "-1", RoundingMode::Exact, "-1000000000000");

    test("1000000000000", "-3", RoundingMode::Down, "-333333333333");
    test("1000000000000", "-3", RoundingMode::Floor, "-333333333334");
    test("1000000000000", "-3", RoundingMode::Up, "-333333333334");
    test(
        "1000000000000",
        "-3",
        RoundingMode::Ceiling,
        "-333333333333",
    );
    test(
        "1000000000000",
        "-3",
        RoundingMode::Nearest,
        "-333333333333",
    );

    test("999999999999", "-2", RoundingMode::Down, "-499999999999");
    test("999999999999", "-2", RoundingMode::Floor, "-500000000000");
    test("999999999999", "-2", RoundingMode::Up, "-500000000000");
    test("999999999999", "-2", RoundingMode::Ceiling, "-499999999999");
    test("999999999999", "-2", RoundingMode::Nearest, "-500000000000");

    test("1000000000001", "-2", RoundingMode::Down, "-500000000000");
    test("1000000000001", "-2", RoundingMode::Floor, "-500000000001");
    test("1000000000001", "-2", RoundingMode::Up, "-500000000001");
    test(
        "1000000000001",
        "-2",
        RoundingMode::Ceiling,
        "-500000000000",
    );
    test(
        "1000000000001",
        "-2",
        RoundingMode::Nearest,
        "-500000000000",
    );

    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Down,
        "-232830643708079",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Floor,
        "-232830643708080",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Up,
        "-232830643708080",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Ceiling,
        "-232830643708079",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        RoundingMode::Nearest,
        "-232830643708080",
    );

    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Exact,
        "-1000000000000",
    );

    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Down,
        "-999999999999",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Ceiling,
        "-999999999999",
    );
    test(
        "1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Nearest,
        "-999999999999",
    );

    test(
        "2999999999999999999999999",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "-1",
    );
    test(
        "3000000000000000000000000",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "-2",
    );
    test(
        "3000000000000000000000001",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "-2",
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

    test("-123", "2", RoundingMode::Down, "-61");
    test("-123", "2", RoundingMode::Floor, "-62");
    test("-123", "2", RoundingMode::Up, "-62");
    test("-123", "2", RoundingMode::Ceiling, "-61");
    test("-123", "2", RoundingMode::Nearest, "-62");

    test("-125", "2", RoundingMode::Down, "-62");
    test("-125", "2", RoundingMode::Floor, "-63");
    test("-125", "2", RoundingMode::Up, "-63");
    test("-125", "2", RoundingMode::Ceiling, "-62");
    test("-125", "2", RoundingMode::Nearest, "-62");

    test("-123", "123", RoundingMode::Down, "-1");
    test("-123", "123", RoundingMode::Floor, "-1");
    test("-123", "123", RoundingMode::Up, "-1");
    test("-123", "123", RoundingMode::Ceiling, "-1");
    test("-123", "123", RoundingMode::Nearest, "-1");
    test("-123", "123", RoundingMode::Exact, "-1");

    test("-123", "456", RoundingMode::Down, "0");
    test("-123", "456", RoundingMode::Floor, "-1");
    test("-123", "456", RoundingMode::Up, "-1");
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

    test("-1000000000000", "3", RoundingMode::Down, "-333333333333");
    test("-1000000000000", "3", RoundingMode::Floor, "-333333333334");
    test("-1000000000000", "3", RoundingMode::Up, "-333333333334");
    test(
        "-1000000000000",
        "3",
        RoundingMode::Ceiling,
        "-333333333333",
    );
    test(
        "-1000000000000",
        "3",
        RoundingMode::Nearest,
        "-333333333333",
    );

    test("-999999999999", "2", RoundingMode::Down, "-499999999999");
    test("-999999999999", "2", RoundingMode::Floor, "-500000000000");
    test("-999999999999", "2", RoundingMode::Up, "-500000000000");
    test("-999999999999", "2", RoundingMode::Ceiling, "-499999999999");
    test("-999999999999", "2", RoundingMode::Nearest, "-500000000000");

    test("-1000000000001", "2", RoundingMode::Down, "-500000000000");
    test("-1000000000001", "2", RoundingMode::Floor, "-500000000001");
    test("-1000000000001", "2", RoundingMode::Up, "-500000000001");
    test(
        "-1000000000001",
        "2",
        RoundingMode::Ceiling,
        "-500000000000",
    );
    test(
        "-1000000000001",
        "2",
        RoundingMode::Nearest,
        "-500000000000",
    );

    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Down,
        "-232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Floor,
        "-232830643708080",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Up,
        "-232830643708080",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Ceiling,
        "-232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        RoundingMode::Nearest,
        "-232830643708080",
    );

    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Down,
        "-1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Ceiling,
        "-1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Nearest,
        "-1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000",
        RoundingMode::Exact,
        "-1000000000000",
    );

    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Down,
        "-999999999999",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Floor,
        "-1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Up,
        "-1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Ceiling,
        "-999999999999",
    );
    test(
        "-1000000000000000000000000",
        "1000000000001",
        RoundingMode::Nearest,
        "-999999999999",
    );

    test(
        "-2999999999999999999999999",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "-1",
    );
    test(
        "-3000000000000000000000000",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "-2",
    );
    test(
        "-3000000000000000000000001",
        "2000000000000000000000000",
        RoundingMode::Nearest,
        "-2",
    );

    test("-1", "-1", RoundingMode::Down, "1");
    test("-1", "-1", RoundingMode::Floor, "1");
    test("-1", "-1", RoundingMode::Up, "1");
    test("-1", "-1", RoundingMode::Ceiling, "1");
    test("-1", "-1", RoundingMode::Nearest, "1");
    test("-1", "-1", RoundingMode::Exact, "1");

    test("-123", "-1", RoundingMode::Down, "123");
    test("-123", "-1", RoundingMode::Floor, "123");
    test("-123", "-1", RoundingMode::Up, "123");
    test("-123", "-1", RoundingMode::Ceiling, "123");
    test("-123", "-1", RoundingMode::Nearest, "123");
    test("-123", "-1", RoundingMode::Exact, "123");

    test("-123", "-2", RoundingMode::Down, "61");
    test("-123", "-2", RoundingMode::Floor, "61");
    test("-123", "-2", RoundingMode::Up, "62");
    test("-123", "-2", RoundingMode::Ceiling, "62");
    test("-123", "-2", RoundingMode::Nearest, "62");

    test("-125", "-2", RoundingMode::Down, "62");
    test("-125", "-2", RoundingMode::Floor, "62");
    test("-125", "-2", RoundingMode::Up, "63");
    test("-125", "-2", RoundingMode::Ceiling, "63");
    test("-125", "-2", RoundingMode::Nearest, "62");

    test("-123", "-123", RoundingMode::Down, "1");
    test("-123", "-123", RoundingMode::Floor, "1");
    test("-123", "-123", RoundingMode::Up, "1");
    test("-123", "-123", RoundingMode::Ceiling, "1");
    test("-123", "-123", RoundingMode::Nearest, "1");
    test("-123", "-123", RoundingMode::Exact, "1");

    test("-123", "-456", RoundingMode::Down, "0");
    test("-123", "-456", RoundingMode::Floor, "0");
    test("-123", "-456", RoundingMode::Up, "1");
    test("-123", "-456", RoundingMode::Ceiling, "1");
    test("-123", "-456", RoundingMode::Nearest, "0");

    test("-1000000000000", "-1", RoundingMode::Down, "1000000000000");
    test("-1000000000000", "-1", RoundingMode::Floor, "1000000000000");
    test("-1000000000000", "-1", RoundingMode::Up, "1000000000000");
    test(
        "-1000000000000",
        "-1",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "-1000000000000",
        "-1",
        RoundingMode::Nearest,
        "1000000000000",
    );
    test("-1000000000000", "-1", RoundingMode::Exact, "1000000000000");

    test("-1000000000000", "-3", RoundingMode::Down, "333333333333");
    test("-1000000000000", "-3", RoundingMode::Floor, "333333333333");
    test("-1000000000000", "-3", RoundingMode::Up, "333333333334");
    test(
        "-1000000000000",
        "-3",
        RoundingMode::Ceiling,
        "333333333334",
    );
    test(
        "-1000000000000",
        "-3",
        RoundingMode::Nearest,
        "333333333333",
    );

    test("-999999999999", "-2", RoundingMode::Down, "499999999999");
    test("-999999999999", "-2", RoundingMode::Floor, "499999999999");
    test("-999999999999", "-2", RoundingMode::Up, "500000000000");
    test("-999999999999", "-2", RoundingMode::Ceiling, "500000000000");
    test("-999999999999", "-2", RoundingMode::Nearest, "500000000000");

    test("-1000000000001", "-2", RoundingMode::Down, "500000000000");
    test("-1000000000001", "-2", RoundingMode::Floor, "500000000000");
    test("-1000000000001", "-2", RoundingMode::Up, "500000000001");
    test(
        "-1000000000001",
        "-2",
        RoundingMode::Ceiling,
        "500000000001",
    );
    test(
        "-1000000000001",
        "-2",
        RoundingMode::Nearest,
        "500000000000",
    );

    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Down,
        "232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Floor,
        "232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Up,
        "232830643708080",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Ceiling,
        "232830643708080",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        RoundingMode::Nearest,
        "232830643708080",
    );

    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Down,
        "1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Floor,
        "1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Nearest,
        "1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000",
        RoundingMode::Exact,
        "1000000000000",
    );

    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Down,
        "999999999999",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Floor,
        "999999999999",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Up,
        "1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Ceiling,
        "1000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000001",
        RoundingMode::Nearest,
        "999999999999",
    );

    test(
        "-2999999999999999999999999",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "1",
    );
    test(
        "-3000000000000000000000000",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
    );
    test(
        "-3000000000000000000000001",
        "-2000000000000000000000000",
        RoundingMode::Nearest,
        "2",
    );
}

#[test]
#[should_panic]
fn div_round_assign_fail_1() {
    let mut n = Integer::from(10);
    n.div_round_assign(Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_assign_fail_2() {
    let mut n = Integer::from(10);
    n.div_round_assign(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_assign_ref_fail_1() {
    let mut n = Integer::from(10);
    n.div_round_assign(&Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_assign_ref_fail_2() {
    let mut n = Integer::from(10);
    n.div_round_assign(&Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_fail_1() {
    Integer::from(10).div_round(Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_fail_2() {
    Integer::from(10).div_round(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_val_ref_fail_1() {
    Integer::from(10).div_round(&Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_val_ref_fail_2() {
    Integer::from(10).div_round(&Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_ref_val_fail_1() {
    (&Integer::from(10)).div_round(Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_ref_val_fail_2() {
    (&Integer::from(10)).div_round(Integer::from(3), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn div_round_ref_ref_fail_1() {
    (&Integer::from(10)).div_round(&Integer::ZERO, RoundingMode::Floor);
}

#[test]
#[should_panic]
fn div_round_ref_ref_fail_2() {
    (&Integer::from(10)).div_round(&Integer::from(3), RoundingMode::Exact);
}
