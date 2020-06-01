use std::str::FromStr;

use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
use malachite_base::num::conversion::traits::ExactFrom;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_divisible_by_power_of_two() {
    let test = |xs: &[Limb], pow: u64, out: bool| {
        assert_eq!(limbs_divisible_by_power_of_two(xs, pow), out);
    };
    test(&[1], 0, true);
    test(&[1], 1, false);
    test(&[2], 0, true);
    test(&[2], 1, true);
    test(&[2], 2, false);
    test(&[3], 1, false);
    test(&[122, 456], 1, true);
    test(&[0, 0, 1], 64, true);
    test(&[0, 0, 1], 65, false);
    test(&[0, 0, 1], 100, false);
    test(&[3_567_587_328, 232], 11, true);
    test(&[3_567_587_328, 232], 12, true);
    test(&[3_567_587_328, 232], 13, false);
}

#[test]
fn test_divisible_by_power_of_two() {
    let test = |n, pow, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().divisible_by_power_of_two(pow),
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
    test("1000000000000", 0, true);
    test("1000000000000", 12, true);
    test("1000000000000", 13, false);
    test("4294967295", 0, true);
    test("4294967295", 1, false);
    test("4294967296", 0, true);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("18446744073709551615", 0, true);
    test("18446744073709551615", 1, false);
    test("18446744073709551616", 0, true);
    test("18446744073709551616", 64, true);
    test("18446744073709551616", 65, false);
}
