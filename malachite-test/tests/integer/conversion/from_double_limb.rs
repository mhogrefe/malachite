use malachite_base::comparison::Max;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::integer::Integer;
use malachite_nz::platform::DoubleLimb;
use num::BigInt;

#[cfg(feature = "32_bit_limbs")]
use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::bigint_to_integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::base::unsigneds;

#[test]
fn test_from_double_limb() {
    let test = |u: DoubleLimb, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(1_000_000_000_000, "1000000000000");
    #[cfg(feature = "32_bit_limbs")]
    test(DoubleLimb::MAX, "18446744073709551615");
    #[cfg(feature = "64_bit_limbs")]
    test(DoubleLimb::MAX, "340282366920938463463374607431768211455");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn from_double_limb_properties() {
    test_properties(unsigneds, |&u: &DoubleLimb| {
        let n = Integer::from(u);
        assert!(n.is_valid());
        assert_eq!(DoubleLimb::checked_from(&n), Some(u));

        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
    });
}
