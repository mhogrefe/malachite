use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{small_u64s_var_3, small_unsigneds};

#[test]
fn power_of_two_properties() {
    test_properties_no_special(small_unsigneds, |&pow| {
        let n = Integer::power_of_two(pow);
        assert!(n.is_valid());

        assert_eq!(n, Integer::ONE << pow);
        assert_eq!(n, Integer::low_mask(pow) + Integer::ONE);
        assert_eq!(Natural::exact_from(&n), Natural::power_of_two(pow));
        let mut n = n.clone();
        n.clear_bit(pow);
        assert_eq!(n, 0);
    });

    test_properties_no_special(small_u64s_var_3::<SignedLimb>, |&pow| {
        assert_eq!(SignedLimb::power_of_two(pow), Integer::power_of_two(pow));
    });
}
