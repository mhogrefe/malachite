use common::test_properties;
use malachite_base::num::{IsPowerOfTwo, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;
use std::u32;

#[test]
fn test_is_power_of_two() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_power_of_two(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", true);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
}

#[test]
fn is_power_of_two_properties() {
    test_properties(naturals, |x| {
        let is_power_of_two = x.is_power_of_two();
        if *x != 0 {
            let trailing_zeros = x.trailing_zeros().unwrap();
            assert_eq!(trailing_zeros == x.significant_bits() - 1, is_power_of_two);
            if trailing_zeros <= u64::from(u32::MAX) {
                let trailing_zeros = trailing_zeros as u32;
                assert_eq!(x >> trailing_zeros == 1, is_power_of_two);
            }
        }
    });
}
