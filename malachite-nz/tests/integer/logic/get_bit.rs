use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::bit_access::limbs_get_bit_neg;
use malachite_nz::integer::Integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_get_bit_neg() {
    let test = |xs: &[Limb], index: u64, out: bool| {
        assert_eq!(limbs_get_bit_neg(xs, index), out);
    };
    test(&[1], 0, true);
    test(&[1], 100, true);
    test(&[123], 2, true);
    test(&[123], 3, false);
    test(&[123], 100, true);
    test(&[0, 0b1011], 0, false);
    test(&[0, 0b1011], 32, true);
    test(&[0, 0b1011], 33, false);
    test(&[0, 0b1011], 34, true);
    test(&[0, 0b1011], 35, false);
    test(&[0, 0b1011], 100, true);
    test(&[1, 0b1011], 0, true);
    test(&[1, 0b1011], 32, false);
    test(&[1, 0b1011], 33, false);
    test(&[1, 0b1011], 34, true);
    test(&[1, 0b1011], 35, false);
    test(&[1, 0b1011], 100, true);
}

#[test]
fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Integer::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .get_bit(u32::exact_from(index)),
            out
        );
    };

    test("0", 0, false);
    test("0", 100, false);
    test("123", 2, false);
    test("123", 3, true);
    test("123", 100, false);
    test("-123", 0, true);
    test("-123", 1, false);
    test("-123", 100, true);
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
    test("-1000000000000", 12, true);
    test("-1000000000000", 100, true);
    test("4294967295", 31, true);
    test("4294967295", 32, false);
    test("4294967296", 31, false);
    test("4294967296", 32, true);
    test("4294967296", 33, false);
    test("-4294967295", 0, true);
    test("-4294967295", 1, false);
    test("-4294967295", 31, false);
    test("-4294967295", 32, true);
    test("-4294967295", 33, true);
    test("-4294967296", 0, false);
    test("-4294967296", 31, false);
    test("-4294967296", 32, true);
    test("-4294967296", 33, true);
}
