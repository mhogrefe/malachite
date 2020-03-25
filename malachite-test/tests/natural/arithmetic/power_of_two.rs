use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{small_u64s_var_2, small_unsigneds};

#[test]
fn test_power_of_two() {
    let test = |pow, out| assert_eq!(Natural::power_of_two(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(2, "4");
    test(3, "8");
    test(32, "4294967296");
    test(100, "1267650600228229401496703205376");
}

#[test]
fn power_of_two_properties() {
    test_properties_no_special(small_unsigneds, |&pow| {
        let n = Natural::power_of_two(pow);
        assert!(n.is_valid());

        assert_eq!(n, Natural::ONE << pow);
        assert_eq!(n, Natural::low_mask(pow) + Natural::ONE);
        //TODO checked log two
        let mut n = n.clone();
        n.clear_bit(pow);
        assert_eq!(n, 0);
    });

    test_properties_no_special(small_u64s_var_2::<Limb>, |&pow| {
        assert_eq!(Limb::power_of_two(pow), Natural::power_of_two(pow));
    });
}
