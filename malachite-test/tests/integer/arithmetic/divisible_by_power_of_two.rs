use malachite_base::num::arithmetic::traits::DivisibleByPowerOfTwo;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::integer_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{pairs_of_signed_and_small_unsigned, unsigneds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_small_unsigned, pairs_of_integer_and_small_unsigned_var_1,
    pairs_of_integer_and_small_unsigned_var_2,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn divisible_by_power_of_two_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref x, pow)| {
        let divisible = x.divisible_by_power_of_two(pow);
        assert_eq!(
            integer_to_rug_integer(x).is_divisible_2pow(u32::exact_from(pow)),
            divisible
        );
        if *x != 0 {
            assert_eq!(x.trailing_zeros().unwrap() >= pow, divisible);
        }
        assert_eq!((-x).divisible_by_power_of_two(pow), divisible);
        assert!((x << pow).divisible_by_power_of_two(pow));
        assert_eq!(x >> pow << pow == *x, divisible);
    });

    test_properties(
        pairs_of_integer_and_small_unsigned_var_1,
        |&(ref x, pow)| {
            assert!(x.divisible_by_power_of_two(pow));
            assert!(integer_to_rug_integer(x).is_divisible_2pow(u32::exact_from(pow)));
            if *x != 0 {
                assert!(x.trailing_zeros().unwrap() >= pow);
            }
            assert!((-x).divisible_by_power_of_two(pow));
            assert_eq!(x >> pow << pow, *x);
        },
    );

    test_properties(
        pairs_of_integer_and_small_unsigned_var_2,
        |&(ref x, pow)| {
            assert!(!x.divisible_by_power_of_two(pow));
            assert!(!integer_to_rug_integer(x).is_divisible_2pow(u32::exact_from(pow)));
            if *x != 0 {
                assert!(x.trailing_zeros().unwrap() < pow);
            }
            assert!(!(-x).divisible_by_power_of_two(pow));
            assert_ne!(x >> pow << pow, *x);
        },
    );

    test_properties(integers, |x| {
        assert!(x.divisible_by_power_of_two(0));
    });

    test_properties(unsigneds, |&pow| {
        assert!(Integer::ZERO.divisible_by_power_of_two(pow));
    });

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(x, pow)| {
            assert_eq!(
                x.divisible_by_power_of_two(pow),
                Integer::from(x).divisible_by_power_of_two(pow)
            );
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref x, pow)| {
        assert_eq!(
            x.divisible_by_power_of_two(pow),
            Integer::from(x).divisible_by_power_of_two(pow)
        );
    });
}
