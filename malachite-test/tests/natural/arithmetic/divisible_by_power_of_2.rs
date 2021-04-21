use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::natural_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1, unsigneds,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, pairs_of_natural_and_small_unsigned_var_1,
    pairs_of_natural_and_small_unsigned_var_2,
};

#[test]
fn limbs_divisible_by_power_of_2_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, pow)| {
            assert_eq!(
                limbs_divisible_by_power_of_2(limbs, pow),
                Natural::from_limbs_asc(limbs).divisible_by_power_of_2(pow),
            );
        },
    );
}

#[test]
fn divisible_by_power_of_2_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref x, pow)| {
        let divisible = x.divisible_by_power_of_2(pow);
        assert_eq!(
            natural_to_rug_integer(x).is_divisible_2pow(u32::exact_from(pow)),
            divisible
        );
        if *x != 0 {
            assert_eq!(x.trailing_zeros().unwrap() >= pow, divisible);
        }
        assert_eq!((-x).divisible_by_power_of_2(pow), divisible);
        assert!((x << pow).divisible_by_power_of_2(pow));
        assert_eq!(x >> pow << pow == *x, divisible);
    });

    test_properties(
        pairs_of_natural_and_small_unsigned_var_1,
        |&(ref x, pow)| {
            assert!(x.divisible_by_power_of_2(pow));
            assert!(natural_to_rug_integer(x).is_divisible_2pow(u32::exact_from(pow)));
            if *x != 0 {
                assert!(x.trailing_zeros().unwrap() >= pow);
            }
            assert!((-x).divisible_by_power_of_2(pow));
            assert_eq!(x >> pow << pow, *x);
        },
    );

    test_properties(
        pairs_of_natural_and_small_unsigned_var_2,
        |&(ref x, pow)| {
            assert!(!x.divisible_by_power_of_2(pow));
            assert!(!natural_to_rug_integer(x).is_divisible_2pow(u32::exact_from(pow)));
            if *x != 0 {
                assert!(x.trailing_zeros().unwrap() < pow);
            }
            assert!(!(-x).divisible_by_power_of_2(pow));
            assert_ne!(x >> pow << pow, *x);
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(x, pow)| {
            assert_eq!(
                x.divisible_by_power_of_2(pow),
                Natural::from(x).divisible_by_power_of_2(pow)
            );
        },
    );

    test_properties(naturals, |x| {
        assert!(x.divisible_by_power_of_2(0));
    });

    test_properties(unsigneds, |&pow| {
        assert!(Natural::ZERO.divisible_by_power_of_2(pow));
    });
}
