use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::{BitScan, CountOnes, LowMask};
use malachite_nz::natural::logic::low_mask::limbs_low_mask;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::{small_u64s_var_4, small_unsigneds};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_low_mask() {
    let test = |bits, out: &[Limb]| assert_eq!(limbs_low_mask(bits), out);
    test(0, &[]);
    test(1, &[1]);
    test(2, &[3]);
    test(3, &[7]);
    test(32, &[4294967295]);
    test(100, &[4294967295, 4294967295, 4294967295, 15]);
}

#[test]
fn test_low_mask() {
    let test = |bits, out| assert_eq!(Natural::low_mask(bits).to_string(), out);
    test(0, "0");
    test(1, "1");
    test(2, "3");
    test(3, "7");
    test(32, "4294967295");
    test(100, "1267650600228229401496703205375");
}

#[test]
fn limbs_low_mask_properties() {
    test_properties_no_special(small_unsigneds, |&bits| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_low_mask(bits)),
            Natural::low_mask(bits)
        );
    });
}

#[test]
fn low_mask_properties() {
    test_properties_no_special(small_unsigneds, |&bits| {
        let n = Natural::low_mask(bits);
        assert!(n.is_valid());

        assert_eq!(n, Natural::power_of_two(bits) - Natural::ONE);
        assert_eq!(n.count_ones(), bits);
        assert_eq!(n.index_of_next_false_bit(0), Some(bits));
    });

    test_properties_no_special(small_u64s_var_4::<Limb>, |&bits| {
        assert_eq!(Limb::low_mask(bits), Natural::low_mask(bits));
    });
}
