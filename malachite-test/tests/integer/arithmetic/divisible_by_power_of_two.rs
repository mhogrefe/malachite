use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_u32};
use std::str::FromStr;

#[test]
fn test_divisible_by_power_of_two() {
    let test = |n, pow, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().divisible_by_power_of_two(pow),
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

#[test]
fn divisible_by_power_of_two_properties() {
    test_properties(
        pairs_of_integer_and_small_u32,
        |&(ref x, pow): &(Integer, u32)| {
            let divisible = x.divisible_by_power_of_two(pow);
            if *x != 0 {
                assert_eq!(x.trailing_zeros().unwrap() >= u64::from(pow), divisible);
            }
            assert_eq!((-x).divisible_by_power_of_two(pow), divisible);
            assert!((x << pow as u32).divisible_by_power_of_two(pow));
            assert_eq!(x >> pow << pow == *x, divisible);
        },
    );

    test_properties(integers, |x| {
        assert!(x.divisible_by_power_of_two(0));
    });

    test_properties(unsigneds, |&pow| {
        assert!(Integer::ZERO.divisible_by_power_of_two(pow));
    });
}