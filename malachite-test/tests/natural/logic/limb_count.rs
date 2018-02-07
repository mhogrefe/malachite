use common::test_properties;
use malachite_base::num::One;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;
use std::u32;

#[test]
fn test_limb_count() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().limb_count(), out);
    };
    test("0", 0);
    test("123", 1);
    test("1000000000000", 2);
    test("4294967295", 1);
    test("4294967296", 2);
    test("18446744073709551615", 2);
    test("18446744073709551616", 3);
}

#[test]
fn limb_count_properties() {
    test_properties(naturals, |x| {
        let limb_count = x.limb_count();
        assert_eq!(*x <= u32::MAX, x.limb_count() <= 1);
        if *x != 0 {
            let n = limb_count as u32;
            assert!(Natural::ONE << ((n - 1) << 5) <= *x);
            assert!(*x < Natural::ONE << (n << 5));
        }
    });
}
