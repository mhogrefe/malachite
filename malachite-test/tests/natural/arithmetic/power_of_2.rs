use malachite_base::num::arithmetic::traits::{Pow, PowerOf2};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{small_u64s_var_2, small_unsigneds};

#[test]
fn power_of_2_properties() {
    test_properties_no_special(small_unsigneds, |&pow| {
        let n = Natural::power_of_2(pow);
        assert!(n.is_valid());

        assert_eq!(n, Natural::ONE << pow);
        assert_eq!(n, Natural::TWO.pow(pow));
        assert_eq!(n, Natural::low_mask(pow) + Natural::ONE);
        //TODO checked log two
        let mut n = n.clone();
        n.clear_bit(pow);
        assert_eq!(n, 0);
    });

    test_properties_no_special(small_u64s_var_2::<Limb>, |&pow| {
        assert_eq!(Limb::power_of_2(pow), Natural::power_of_2(pow));
    });
}
