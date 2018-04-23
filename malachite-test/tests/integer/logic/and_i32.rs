use common::test_properties;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::logic::and_i32::integer_and_i32_alt;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_and_i32() {
    let test = |u, v: i32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);

        let n = v & &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v & rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v & &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };

    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("123", 456, "72");
    test("1000000000000", 123, "0");
    test("1000000000001", 123, "1");
    test("12345678987654321", 987_654_321, "579887281");
    test("-123", 0, "0");
    test("-123", 456, "384");
    test("-1000000000000", 123, "0");
    test("-1000000000001", 123, "123");
    test("-12345678987654321", 987_654_321, "407767041");

    test("0", -123, "0");
    test("123", -456, "56");
    test("1000000000000", -123, "1000000000000");
    test("1000000000001", -123, "1000000000001");
    test("12345678987654321", -987_654_321, "12345678407767041");
    test("-123", -456, "-512");
    test("-1000000000000", -123, "-1000000000000");
    test("-1000000000001", -123, "-1000000000123");
    test("-12345678987654321", -987_654_321, "-12345679395421361");
    test(
        "16877400614591900061756902599",
        -1_958_485_034,
        "16877400614591900060882124998",
    );

    test("-3486", -12, "-3488");
    test("-3582", -12, "-3584");
    test("-55835164686", -65_532, "-55835230208");
    test("-60129476622", -65_532, "-60129542144");
    test("-4294901774", -65_532, "-4294967296");
    test(
        "-45671926166590716193855479615826927335145209855",
        -7_684,
        "-45671926166590716193855479615826927335145209856",
    );
}

#[test]
fn and_i32_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i32)| {
            let mut mut_n = n.clone();
            mut_n &= i;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n &= i;
            assert_eq!(rug_integer_to_integer(&rug_n), result, "{} {}", n, i);

            let result_alt = n & i;
            assert_eq!(result_alt, result);

            let result_alt = i & n;
            assert_eq!(result_alt, result);

            assert_eq!(integer_and_i32_alt(&n, i), result);

            //TODO assert_eq!(n & Integer::from(u), result);
            //TODO assert_eq!(Integer::from(u) & n, result);

            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) & i)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n & 0, 0);
        assert_eq!(0 & n, 0);
        assert_eq!(n & -1, *n);
        assert_eq!(-1 & n, *n);
    });

    test_properties(signeds, |&i: &i32| {
        assert_eq!(&Integer::ZERO & i, 0);
        assert_eq!(i & &Integer::ZERO, 0);
        assert_eq!(&Integer::NEGATIVE_ONE & i, i);
        assert_eq!(i & &Integer::NEGATIVE_ONE, i);
    });
}
