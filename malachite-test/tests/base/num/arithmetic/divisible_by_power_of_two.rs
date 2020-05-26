use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_signed_and_small_unsigned_var_1,
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_and_small_unsigned_var_1, signeds,
    unsigneds,
};

fn unsigned_divisible_by_power_of_two_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(x, pow)| {
            let divisible = x.divisible_by_power_of_two(pow);
            if x != T::ZERO {
                assert_eq!(x.trailing_zeros() >= pow, divisible);
            }
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_unsigned_var_1::<T, u64>,
        |&(x, pow)| {
            assert!(!x.divisible_by_power_of_two(pow));
            if x != T::ZERO {
                assert!(x.trailing_zeros() < pow);
            }
        },
    );

    test_properties(unsigneds::<T>, |x| {
        assert!(x.divisible_by_power_of_two(0));
    });

    test_properties(unsigneds, |&pow| {
        assert!(T::ZERO.divisible_by_power_of_two(pow));
    });
}

fn signed_divisible_by_power_of_two_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_small_unsigned::<T, u64>, |&(x, pow)| {
        let divisible = x.divisible_by_power_of_two(pow);
        if x != T::ZERO {
            assert_eq!(x.trailing_zeros() >= pow, divisible);
        }
        if x != T::MIN {
            assert_eq!((-x).divisible_by_power_of_two(pow), divisible);
        }
    });

    test_properties(
        pairs_of_signed_and_small_unsigned_var_1::<T, u64>,
        |&(x, pow)| {
            assert!(!x.divisible_by_power_of_two(pow));
            if x != T::ZERO {
                assert!(x.trailing_zeros() < pow);
            }
            if x != T::MIN {
                assert!(!(-x).divisible_by_power_of_two(pow));
            }
        },
    );

    test_properties(signeds::<T>, |x| {
        assert!(x.divisible_by_power_of_two(0));
    });

    test_properties(unsigneds, |&pow| {
        assert!(T::ZERO.divisible_by_power_of_two(pow));
    });
}

#[test]
fn divisible_by_power_of_two_properties() {
    unsigned_divisible_by_power_of_two_properties_helper::<u8>();
    unsigned_divisible_by_power_of_two_properties_helper::<u16>();
    unsigned_divisible_by_power_of_two_properties_helper::<u32>();
    unsigned_divisible_by_power_of_two_properties_helper::<u64>();
    unsigned_divisible_by_power_of_two_properties_helper::<usize>();

    signed_divisible_by_power_of_two_properties_helper::<i8>();
    signed_divisible_by_power_of_two_properties_helper::<i16>();
    signed_divisible_by_power_of_two_properties_helper::<i32>();
    signed_divisible_by_power_of_two_properties_helper::<i64>();
    signed_divisible_by_power_of_two_properties_helper::<isize>();
}
