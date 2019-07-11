#[cfg(feature = "32_bit_limbs")]
use common::test_properties;
use malachite_base::comparison::Max;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::Natural;
use malachite_nz::platform::DoubleLimb;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::biguint_to_natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::base::unsigneds;
use num::BigUint;

#[test]
fn test_from_double_limb() {
    let test = |u: DoubleLimb, out| {
        let x = Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigUint::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(1_000_000_000_000, "1000000000000");
    #[cfg(feature = "32_bit_limbs")]
    test(DoubleLimb::MAX, "18446744073709551615");
    #[cfg(not(feature = "32_bit_limbs"))]
    test(DoubleLimb::MAX, "340282366920938463463374607431768211455");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn from_double_limb_properties() {
    test_properties(unsigneds, |&u: &DoubleLimb| {
        let n = Natural::from(u);
        assert!(n.is_valid());
        assert_eq!(DoubleLimb::checked_from(&n), Some(u));
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
    });
}
