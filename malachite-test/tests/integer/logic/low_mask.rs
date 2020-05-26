use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitScan, LowMask};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{small_u64s_var_2, small_unsigneds};

#[test]
fn low_mask_properties() {
    test_properties_no_special(small_unsigneds, |&bits| {
        let n = Integer::low_mask(bits);
        assert!(n.is_valid());

        assert_eq!(n, Integer::power_of_two(bits) - Integer::ONE);
        assert_eq!(Natural::exact_from(&n), Natural::low_mask(bits));
        assert_eq!(n.index_of_next_false_bit(0), Some(bits));
    });

    test_properties_no_special(small_u64s_var_2::<SignedLimb>, |&bits| {
        assert_eq!(SignedLimb::low_mask(bits), Integer::low_mask(bits));
    });
}
