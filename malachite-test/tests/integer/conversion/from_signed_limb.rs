use common::test_properties;
use malachite_base::comparison::{Max, Min};
use malachite_base::conversion::CheckedFrom;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_test::common::{bigint_to_integer, rug_integer_to_integer};
use malachite_test::inputs::base::signeds;
use num::BigInt;
use rug;

#[test]
fn test_from_signed_limb() {
    let test = |i: SignedLimb, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(i).to_string(), out);

        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    #[cfg(feature = "32_bit_limbs")]
    {
        test(SignedLimb::MIN, "-2147483648");
        test(SignedLimb::MAX, "2147483647");
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test(SignedLimb::MIN, "-9223372036854775808");
        test(SignedLimb::MAX, "9223372036854775807");
    }
}

#[test]
fn from_signed_limb_properties() {
    test_properties(signeds, |&i: &SignedLimb| {
        let n = Integer::from(i);
        assert!(n.is_valid());
        assert_eq!(SignedLimb::checked_from(&n), Some(i));

        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
    });
}
