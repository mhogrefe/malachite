use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use std::str::FromStr;

#[test]
fn test_divisible_by_power_of_2() {
    let test = |n, pow, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().divisible_by_power_of_2(pow),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .is_divisible_2pow(u32::exact_from(pow)),
            out
        );
    };
    test("0", 0, true);
    test("0", 10, true);
    test("0", 100, true);
    test("123", 0, true);
    test("123", 1, false);
    test("-123", 0, true);
    test("-123", 1, false);
    test("1000000000000", 0, true);
    test("1000000000000", 12, true);
    test("1000000000000", 13, false);
    test("-1000000000000", 0, true);
    test("-1000000000000", 12, true);
    test("-1000000000000", 13, false);
    test("4294967295", 0, true);
    test("4294967295", 1, false);
    test("-4294967295", 0, true);
    test("-4294967295", 1, false);
    test("4294967296", 0, true);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("-4294967296", 0, true);
    test("-4294967296", 32, true);
    test("-4294967296", 33, false);
    test("18446744073709551615", 0, true);
    test("18446744073709551615", 1, false);
    test("-18446744073709551615", 0, true);
    test("-18446744073709551615", 1, false);
    test("18446744073709551616", 0, true);
    test("18446744073709551616", 64, true);
    test("18446744073709551616", 65, false);
    test("-18446744073709551616", 0, true);
    test("-18446744073709551616", 64, true);
    test("-18446744073709551616", 65, false);
}
