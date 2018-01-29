use common::LARGE_LIMIT;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;
use std::u32;

#[test]
fn test_is_power_of_2() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_power_of_2(), out);
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
fn is_power_of_2_properties() {
    // if x != 0, x.is_power_of_2() == (x.trailing_zeros().unwrap() == x.significant_bits() - 1)
    // if x != 0, x.is_power_of_2() == (x >> x.trailing_zeros() == 1)
    let one_natural = |x: Natural| {
        let is_power_of_2 = x.is_power_of_2();
        if x != 0 {
            let trailing_zeros = x.trailing_zeros().unwrap();
            assert_eq!(trailing_zeros == x.significant_bits() - 1, is_power_of_2);
            if trailing_zeros <= u32::MAX.into() {
                let trailing_zeros = trailing_zeros as u32;
                assert_eq!(x >> trailing_zeros == 1, is_power_of_2);
            }
        }
    };

    for n in naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
