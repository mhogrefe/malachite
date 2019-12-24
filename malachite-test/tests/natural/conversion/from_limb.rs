use malachite_base::comparison::Max;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{biguint_to_natural, rug_integer_to_natural};
use malachite_test::inputs::base::unsigneds;

#[test]
fn test_from_limb() {
    let test = |u: Limb, out| {
        let x = Natural::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigUint::from(u).to_string(), out);

        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    #[cfg(feature = "32_bit_limbs")]
    test(Limb::MAX, "4294967295");
    #[cfg(not(feature = "32_bit_limbs"))]
    test(Limb::MAX, "18446744073709551615");
}

#[test]
fn from_limb_properties() {
    test_properties(unsigneds, |&u: &Limb| {
        let n = Natural::from(u);
        assert!(n.is_valid());
        assert_eq!(Limb::checked_from(&n), Some(u));
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });
}
