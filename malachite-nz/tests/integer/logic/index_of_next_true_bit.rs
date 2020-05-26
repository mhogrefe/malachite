use std::str::FromStr;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitScan;
use malachite_nz_test_util::integer::logic::index_of_next_true_bit::*;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_true_bit_neg;
use malachite_nz::integer::Integer;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_true_bit_neg() {
    let test = |xs, u, out| {
        assert_eq!(limbs_index_of_next_true_bit_neg(xs, u), out);
    };
    test(&[1], 0, 0);
    test(&[1], 100, 100);
    test(&[0b100], 0, 2);
    test(&[0b100], 1, 2);
    test(&[0b100], 2, 2);
    test(&[0b100], 3, 3);
    test(&[0, 0b101], 0, 32);
    test(&[0, 0b101], 20, 32);
    test(&[0, 0b101], 31, 32);
    test(&[0, 0b101], 32, 32);
    test(&[0, 0b101], 33, 33);
    test(&[0, 0b101], 34, 35);
    test(&[0, 0b101], 35, 35);
    test(&[0, 0b101], 36, 36);
    test(&[0, 0b101], 100, 100);
    test(&[0, 0, 0b101], 64, 64);
    test(&[0, 0, 0b101], 66, 67);
    test(&[0, 0, 0b101, 0b101], 96, 97);
    test(&[0, 0, 0b101, 0b101], 98, 99);
}

#[test]
fn test_index_of_next_true_bit() {
    let test = |n, u, out| {
        assert_eq!(Integer::from_str(n).unwrap().index_of_next_true_bit(u), out);
        assert_eq!(
            integer_index_of_next_true_bit_alt(&Integer::from_str(n).unwrap(), u),
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

    test("-21474836480", 0, Some(32));
    test("-21474836480", 20, Some(32));
    test("-21474836480", 31, Some(32));
    test("-21474836480", 32, Some(32));
    test("-21474836480", 33, Some(33));
    test("-21474836480", 34, Some(35));
    test("-21474836480", 35, Some(35));
    test("-21474836480", 36, Some(36));
    test("-21474836480", 100, Some(100));
    test("-92233720368547758080", 64, Some(64));
    test("-92233720368547758080", 66, Some(67));
    test("-396140812663555408336267509760", 96, Some(97));
    test("-396140812663555408336267509760", 98, Some(99));
}
