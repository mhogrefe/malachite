use std::str::FromStr;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::conversion::traits::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};

#[cfg(feature = "32_bit_limbs")]
use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::base::pairs_of_signeds;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::inputs::integer::pairs_of_integer_and_signed;

#[test]
fn test_assign_signed_double_limb() {
    let test = |u, v: SignedDoubleLimb, out| {
        let mut x = Integer::from_str(u).unwrap();
        x.assign(v);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("123", -456, "-456");
    #[cfg(feature = "32_bit_limbs")]
    {
        test("-123", SignedLimb::MAX.into(), "2147483647");
        test("123", SignedLimb::MIN.into(), "-2147483648");
        test("-123", SignedDoubleLimb::MAX, "9223372036854775807");
        test("123", SignedDoubleLimb::MIN, "-9223372036854775808");
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("-123", SignedLimb::MAX.into(), "9223372036854775807");
        test("123", SignedLimb::MIN.into(), "-9223372036854775808");
        test(
            "-123",
            SignedDoubleLimb::MAX,
            "170141183460469231731687303715884105727",
        );
        test(
            "123",
            SignedDoubleLimb::MIN,
            "-170141183460469231731687303715884105728",
        );
    }
    test("1000000000000", 123, "123");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn assign_signed_double_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedDoubleLimb)| {
            let mut mut_n = n.clone();
            mut_n.assign(i);
            assert!(mut_n.is_valid());
            assert_eq!(mut_n, Integer::from(i));
        },
    );

    test_properties(pairs_of_signeds::<SignedDoubleLimb>, #[allow(
        unused_assignments
    )]
    |&(i, j)| {
        let mut mut_i = i;
        let mut mut_n = Integer::from(i);
        mut_i = j;
        mut_n.assign(j);
        assert_eq!(Integer::from(mut_i), mut_n);
    });
}
