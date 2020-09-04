use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::naturals;

#[test]
fn limb_count_properties() {
    test_properties(naturals, |x| {
        let limb_count = x.limb_count();
        assert_eq!(*x <= Limb::MAX, x.limb_count() <= 1);
        if *x != 0 {
            let n = limb_count;
            assert!(Natural::power_of_two((n - 1) << Limb::LOG_WIDTH) <= *x);
            assert!(*x < Natural::power_of_two(n << Limb::LOG_WIDTH));
        }
    });

    test_properties(unsigneds::<Limb>, |&u| {
        assert!(Natural::from(u).limb_count() <= 1);
    });
}
