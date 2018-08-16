use common::test_properties;
use malachite_base::num::{DivisibleByPowerOfTwo, EqModPowerOfTwo, ModPowerOfTwo, Zero};
use malachite_nz::integer::Integer;
use malachite_test::inputs::base::pairs_of_signed_and_small_unsigned;
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_unsigned, triples_of_integer_signed_and_small_unsigned,
};
use std::i32;
use std::str::FromStr;

#[test]
fn test_eq_mod_power_of_two_i32() {
    let test = |n, i: i32, pow, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().eq_mod_power_of_two(&i, pow),
            out
        );
    };
    test("0", 256, 8, true);
    test("0", 256, 9, false);
    test("13", 21, 0, true);
    test("13", 21, 1, true);
    test("13", 21, 2, true);
    test("13", 21, 3, true);
    test("13", 21, 4, false);
    test("13", 21, 100, false);
    test("1000000000001", 1, 12, true);
    test("1000000000001", 1, 13, false);
    test("-3", 5, 0, true);
    test("-3", 5, 1, true);
    test("-3", 5, 2, true);
    test("-3", 5, 3, true);
    test("-3", 5, 4, false);
    test("-1", i32::MAX, 0, true);
    test("-1", i32::MAX, 1, true);
    test("-1", i32::MAX, 31, true);
    test("-1", i32::MAX, 32, false);
    test("-13", 11, 0, true);
    test("-13", 11, 1, true);
    test("-13", 11, 2, true);
    test("-13", 11, 3, true);
    test("-13", 11, 4, false);
    test("-999999999999", 1, 12, true);
    test("-999999999999", 1, 13, false);
    test("0", -256, 8, true);
    test("0", -256, 9, false);
    test("-13", -21, 0, true);
    test("-13", -21, 1, true);
    test("-13", -21, 2, true);
    test("-13", -21, 3, true);
    test("-13", -21, 4, false);
    test("-13", -21, 100, false);
    test("-1000000000001", -1, 12, true);
    test("-1000000000001", -1, 13, false);
    test("3", -5, 0, true);
    test("3", -5, 1, true);
    test("3", -5, 2, true);
    test("3", -5, 3, true);
    test("3", -5, 4, false);
    test("0", i32::MIN, 0, true);
    test("0", i32::MIN, 1, true);
    test("0", i32::MIN, 31, true);
    test("0", i32::MIN, 32, false);
    test("13", -11, 0, true);
    test("13", -11, 1, true);
    test("13", -11, 2, true);
    test("13", -11, 3, true);
    test("13", -11, 4, false);
    test("999999999999", -1, 12, true);
    test("999999999999", -1, 13, false);
}

#[test]
fn eq_mod_power_of_two_i32_properties() {
    test_properties(
        triples_of_integer_signed_and_small_unsigned::<i32, u64>,
        |&(ref n, i, pow)| {
            let eq_mod_power_of_two = n.eq_mod_power_of_two(&i, pow);
            assert_eq!(
                n.mod_power_of_two(pow) == Integer::from(i).mod_power_of_two(pow),
                eq_mod_power_of_two,
            );
        },
    );

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, pow)| {
        assert_eq!(
            n.eq_mod_power_of_two(&0i32, pow),
            n.divisible_by_power_of_two(pow),
        );
    });

    test_properties(
        pairs_of_signed_and_small_unsigned::<i32, u64>,
        |&(i, pow)| {
            assert_eq!(
                Integer::ZERO.eq_mod_power_of_two(&i, pow),
                i.divisible_by_power_of_two(pow)
            );
        },
    );
}
