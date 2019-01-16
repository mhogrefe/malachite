#[cfg(feature = "32_bit_limbs")]
use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::misc::CheckedFrom;
use malachite_base::misc::{Max, Min};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedDoubleLimb;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::bigint_to_integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::base::signeds;
use num::BigInt;

#[test]
fn test_from_signed_double_limb() {
    let test = |i: SignedDoubleLimb, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        assert_eq!(BigInt::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(1_000_000_000_000, "1000000000000");
    test(-1_000_000_000_000, "-1000000000000");
    #[cfg(feature = "32_bit_limbs")]
    {
        test(SignedDoubleLimb::MAX, "9223372036854775807");
        test(SignedDoubleLimb::MIN, "-9223372036854775808");
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test(
            SignedDoubleLimb::MAX,
            "170141183460469231731687303715884105727",
        );
        test(
            SignedDoubleLimb::MIN,
            "-170141183460469231731687303715884105728",
        );
    }
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn from_signed_double_limb_properties() {
    test_properties(signeds, |&i: &SignedDoubleLimb| {
        let n = Integer::from(i);
        assert!(n.is_valid());
        assert_eq!(SignedDoubleLimb::checked_from(&n), Some(i));

        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
    });
}
