use common::test_properties;
use malachite_base::misc::Max;
use malachite_base::num::{One, PrimitiveInteger};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::naturals;
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
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
        assert_eq!(*x <= Limb::MAX, x.limb_count() <= 1);
        if *x != 0 {
            let n = limb_count as Limb;
            assert!(Natural::ONE << ((n - 1) << Limb::LOG_WIDTH) <= *x);
            assert!(*x < Natural::ONE << (n << Limb::LOG_WIDTH));
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert!(Natural::from(u).limb_count() <= 1);
    });
}
