use malachite_base::comparison::Max;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use num::BigInt;
use rug;

use common::test_properties;
use malachite_test::common::{bigint_to_integer, rug_integer_to_integer};
use malachite_test::inputs::base::unsigneds;

#[test]
fn test_from_limb() {
    let test = |u: Limb, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(u).to_string(), out);

        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    #[cfg(feature = "32_bit_limbs")]
    test(Limb::MAX, "4294967295");
    #[cfg(feature = "64_bit_limbs")]
    test(Limb::MAX, "18446744073709551615");
}

#[test]
fn from_limb_properties() {
    test_properties(unsigneds, |&u: &Limb| {
        let n = Integer::from(u);
        assert!(n.is_valid());
        assert_eq!(Limb::checked_from(&n), Some(u));

        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });
}
