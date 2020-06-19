use std::str::FromStr;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitScan;
use malachite_nz_test_util::natural::logic::index_of_next_true_bit::*;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_true_bit;
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_true_bit() {
    let test = |xs, u, out| {
        assert_eq!(limbs_index_of_next_true_bit(xs, u), out);
    };
    test(&[], 0, None);
    test(&[], 100, None);
    test(&[0], 0, None);
    test(&[0], 100, None);
    test(&[0b100], 0, Some(2));
    test(&[0b100], 1, Some(2));
    test(&[0b100], 2, Some(2));
    test(&[0b100], 3, None);
    test(&[0, 0b1011], 0, Some(32));
    test(&[0, 0b1011], 20, Some(32));
    test(&[0, 0b1011], 31, Some(32));
    test(&[0, 0b1011], 32, Some(32));
    test(&[0, 0b1011], 33, Some(33));
    test(&[0, 0b1011], 34, Some(35));
    test(&[0, 0b1011], 35, Some(35));
    test(&[0, 0b1011], 36, None);
    test(&[0, 0b1011], 100, None);
    test(&[0, 0b1011, 0xfff_ffff, 0, 1], 91, Some(91));
    test(&[0, 0b1011, 0xfff_ffff, 0, 1], 92, Some(128));
}

#[test]
fn test_index_of_next_true_bit() {
    let test = |n, u, out| {
        assert_eq!(Natural::from_str(n).unwrap().index_of_next_true_bit(u), out);
        assert_eq!(
            natural_index_of_next_true_bit_alt(&Natural::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .find_one(u32::exact_from(u))
                .map(u64::from),
            out
        );
    };
    test("0", 0, None);
    test("0", 100, None);
    test("47244640256", 0, Some(32));
    test("47244640256", 20, Some(32));
    test("47244640256", 31, Some(32));
    test("47244640256", 32, Some(32));
    test("47244640256", 33, Some(33));
    test("47244640256", 34, Some(35));
    test("47244640256", 35, Some(35));
    test("47244640256", 36, None);
    test("47244640256", 100, None);
    test("340282366925890223602069384504899796992", 91, Some(91));
    test("340282366925890223602069384504899796992", 92, Some(128));
}
