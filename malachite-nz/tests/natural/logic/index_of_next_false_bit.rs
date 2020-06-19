use std::str::FromStr;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitScan;
use malachite_nz_test_util::natural::logic::index_of_next_false_bit::*;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_false_bit;
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_false_bit() {
    let test = |xs, u, out| {
        assert_eq!(limbs_index_of_next_false_bit(xs, u), out);
    };
    test(&[], 0, 0);
    test(&[], 100, 100);
    test(&[0], 0, 0);
    test(&[0], 100, 100);
    test(&[0b100], 0, 0);
    test(&[0b100], 1, 1);
    test(&[0b100], 2, 3);
    test(&[0b100], 3, 3);
    test(&[0, 0b1011], 0, 0);
    test(&[0, 0b1011], 20, 20);
    test(&[0, 0b1011], 31, 31);
    test(&[0, 0b1011], 32, 34);
    test(&[0, 0b1011], 33, 34);
    test(&[0, 0b1011], 34, 34);
    test(&[0, 0b1011], 35, 36);
    test(&[0, 0b1011], 100, 100);
    test(&[0, 0b1011, 0xffff_fff0, u32::MAX, 1], 64, 64);
    test(&[0, 0b1011, 0xffff_fff0, u32::MAX, 1], 68, 129);
}

#[test]
fn test_index_of_next_false_bit() {
    let test = |n, u, out| {
        assert_eq!(
            Natural::from_str(n).unwrap().index_of_next_false_bit(u),
            out
        );
        assert_eq!(
            natural_index_of_next_false_bit_alt(&Natural::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .find_zero(u32::exact_from(u))
                .map(u64::from),
            out
        );
    };
    test("0", 0, Some(0));
    test("0", 100, Some(100));
    test("47244640256", 0, Some(0));
    test("47244640256", 20, Some(20));
    test("47244640256", 31, Some(31));
    test("47244640256", 32, Some(34));
    test("47244640256", 33, Some(34));
    test("47244640256", 34, Some(34));
    test("47244640256", 35, Some(36));
    test("47244640256", 100, Some(100));
    test("680564733841876926631601309731428237312", 64, Some(64));
    test("680564733841876926631601309731428237312", 68, Some(129));
}
