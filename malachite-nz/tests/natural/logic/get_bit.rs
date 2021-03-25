use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz_test_util::natural::logic::get_bit::num_get_bit;
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::bit_access::limbs_get_bit;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_get_bit() {
    let test = |xs: &[Limb], index: u64, out: bool| {
        assert_eq!(limbs_get_bit(xs, index), out);
    };
    test(&[1], 0, true);
    test(&[1], 100, false);
    test(&[123], 2, false);
    test(&[123], 3, true);
    test(&[123], 100, false);
    test(&[0, 0b1011], 0, false);
    test(&[0, 0b1011], 32, true);
    test(&[0, 0b1011], 33, true);
    test(&[0, 0b1011], 34, false);
    test(&[0, 0b1011], 35, true);
    test(&[0, 0b1011], 100, false);
}

#[test]
fn test_get_bit() {
    let test = |n, index, out| {
        assert_eq!(Natural::from_str(n).unwrap().get_bit(index), out);
        assert_eq!(num_get_bit(&BigUint::from_str(n).unwrap(), index), out);
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
    test("1000000000000", 12, true);
    test("1000000000000", 100, false);
}
