use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitScan;
use malachite_nz_test_util::integer::logic::index_of_next_false_bit::*;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_false_bit_neg;
use malachite_nz::integer::Integer;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_false_bit_neg() {
    let test = |xs, u, out| {
        assert_eq!(limbs_index_of_next_false_bit_neg(xs, u), out);
    };
    test(&[1], 0, None);
    test(&[1], 100, None);
    test(&[0b100], 0, Some(0));
    test(&[0b100], 1, Some(1));
    test(&[0b100], 2, None);
    test(&[0b100], 3, None);
    test(&[0, 0b101], 0, Some(0));
    test(&[0, 0b101], 20, Some(20));
    test(&[0, 0b101], 31, Some(31));
    test(&[0, 0b101], 32, Some(34));
    test(&[0, 0b101], 33, Some(34));
    test(&[0, 0b101], 34, Some(34));
    test(&[0, 0b101], 35, None);
    test(&[0, 0b101], 100, None);
    test(&[0, 0, 0b101], 36, Some(36));
    test(&[0, 0, 0b101], 64, Some(66));
    test(&[0, 0, 0b101, 0b101], 96, Some(96));
    test(&[0, 0, 0b101, 0b101], 97, Some(98));
}

//TODO clean from_str

#[test]
fn test_index_of_next_false_bit() {
    let test = |n, u, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().index_of_next_false_bit(u),
            out
        );
        assert_eq!(
            integer_index_of_next_false_bit_alt(&Integer::from_str(n).unwrap(), u),
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

    test("-21474836480", 0, Some(0));
    test("-21474836480", 20, Some(20));
    test("-21474836480", 31, Some(31));
    test("-21474836480", 32, Some(34));
    test("-21474836480", 33, Some(34));
    test("-21474836480", 34, Some(34));
    test("-21474836480", 35, None);
    test("-21474836480", 36, None);
    test("-21474836480", 100, None);
    test("-92233720368547758080", 36, Some(36));
    test("-92233720368547758080", 64, Some(66));
    test("-396140812663555408336267509760", 96, Some(96));
    test("-396140812663555408336267509760", 97, Some(98));
}
