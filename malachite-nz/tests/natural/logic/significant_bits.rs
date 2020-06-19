use std::str::FromStr;

use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use num::BigUint;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_significant_bits() {
    let test = |xs, out| {
        assert_eq!(limbs_significant_bits::<Limb>(xs), out);
    };
    test(&[0b1], 1);
    test(&[0b10], 2);
    test(&[0b11], 2);
    test(&[0b100], 3);
    test(&[0, 0b1], 33);
    test(&[0, 0b1101], 36);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_significant_bits_fail() {
    limbs_significant_bits::<Limb>(&[]);
}

#[test]
fn test_significant_bits() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().significant_bits(), out);
        assert_eq!(
            u64::wrapping_from(BigUint::from_str(n).unwrap().bits()),
            out
        );
        assert_eq!(
            u64::from(rug::Integer::from_str(n).unwrap().significant_bits()),
            out
        );
    };
    test("0", 0);
    test("100", 7);
    test("1000000000000", 40);
    test("4294967295", 32);
    test("4294967296", 33);
    test("18446744073709551615", 64);
    test("18446744073709551616", 65);
}
