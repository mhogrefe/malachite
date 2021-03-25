use malachite_base::num::logic::traits::CountOnes;
use malachite_nz_test_util::natural::logic::count_ones::{
    natural_count_ones_alt_1, natural_count_ones_alt_2,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_count_ones() {
    let test = |xs, out| {
        assert_eq!(limbs_count_ones(xs), out);
    };
    test(&[], 0);
    test(&[0, 1, 2], 2);
    test(&[1, u32::MAX], 33);
}

#[test]
fn test_count_ones() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().count_ones(), out);
        assert_eq!(
            natural_count_ones_alt_1(&Natural::from_str(n).unwrap()),
            out
        );
        assert_eq!(
            natural_count_ones_alt_2(&Natural::from_str(n).unwrap()),
            out
        );
    };
    test("0", 0);
    test("105", 4);
    test("1000000000000", 13);
    test("4294967295", 32);
    test("4294967296", 1);
    test("18446744073709551615", 64);
    test("18446744073709551616", 1);
}
