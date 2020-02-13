use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::TrailingZeros;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_and_small_unsigned, positive_unsigneds,
    signeds_var_1, unsigneds,
};

fn index_of_next_false_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigned_and_small_unsigned::<T, u64>, |&(n, u)| {
        let result = n.index_of_next_false_bit(u).unwrap();
        assert!(result >= u);
        assert!(!n.get_bit(result));
        assert_eq!(result == u, !n.get_bit(u));
        assert_eq!(
            (!n).index_of_next_true_bit(u),
            if result < T::WIDTH {
                Some(result)
            } else {
                None
            }
        );
    });

    test_properties(unsigneds::<T>, |&n| {
        assert_eq!(
            n.index_of_next_false_bit(0),
            Some(TrailingZeros::trailing_zeros(!n))
        );
    });

    test_properties(unsigneds::<u64>, |&u| {
        assert_eq!(T::ZERO.index_of_next_false_bit(u), Some(u));
    });
}

fn index_of_next_true_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(pairs_of_unsigned_and_small_unsigned::<T, u64>, |&(n, u)| {
        let result = n.index_of_next_true_bit(u);
        assert_eq!(result.is_some(), u < n.significant_bits());
        if let Some(result) = result {
            assert!(result >= u);
            assert!(n.get_bit(result));
            assert_eq!(result == u, n.get_bit(u));
        }
    });

    test_properties(positive_unsigneds::<T>, |n| {
        assert_eq!(
            n.index_of_next_true_bit(0),
            Some(TrailingZeros::trailing_zeros(*n))
        );
    });

    test_properties(unsigneds::<u64>, |&u| {
        assert_eq!(T::ZERO.index_of_next_true_bit(u), None);
    });
}

fn index_of_next_false_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_small_unsigned::<T, u64>, |&(n, u)| {
        let result = n.index_of_next_false_bit(u);
        assert_eq!(
            result.is_some(),
            if u >= T::WIDTH {
                n >= T::ZERO
            } else {
                n | (T::ONE << u).wrapping_sub(T::ONE) != T::NEGATIVE_ONE
            }
        );
        if let Some(result) = result {
            assert!(result >= u);
            assert!(!n.get_bit(result));
            assert_eq!(result == u, !n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_true_bit(u), result);
    });

    test_properties(signeds_var_1::<T>, |&n| {
        assert_eq!(
            n.index_of_next_false_bit(0),
            Some(TrailingZeros::trailing_zeros(!n))
        );
    });

    test_properties(unsigneds::<u64>, |&u| {
        assert_eq!(T::ZERO.index_of_next_false_bit(u), Some(u));
        assert_eq!(T::NEGATIVE_ONE.index_of_next_false_bit(u), None);
    });
}

fn index_of_next_true_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_small_unsigned::<T, u64>, |&(n, u)| {
        let result = n.index_of_next_true_bit(u);
        assert_eq!(
            result.is_some(),
            if u >= T::WIDTH {
                n < T::ZERO
            } else {
                n >> u != T::ZERO
            }
        );
        if let Some(result) = result {
            assert!(result >= u);
            assert!(n.get_bit(result));
            assert_eq!(result == u, n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_false_bit(u), result);
    });

    test_properties(signeds_var_1::<T>, |&n| {
        assert_eq!(
            n.index_of_next_true_bit(0),
            Some(TrailingZeros::trailing_zeros(n))
        );
    });

    test_properties(unsigneds::<u64>, |&u| {
        assert_eq!(T::ZERO.index_of_next_true_bit(u), None);
        assert_eq!(T::NEGATIVE_ONE.index_of_next_true_bit(u), Some(u));
    });
}

#[test]
fn index_of_next_false_bit_properties() {
    index_of_next_false_bit_properties_helper_unsigned::<u8>();
    index_of_next_false_bit_properties_helper_unsigned::<u16>();
    index_of_next_false_bit_properties_helper_unsigned::<u32>();
    index_of_next_false_bit_properties_helper_unsigned::<u64>();
    index_of_next_false_bit_properties_helper_unsigned::<usize>();
    index_of_next_false_bit_properties_helper_signed::<i8>();
    index_of_next_false_bit_properties_helper_signed::<i16>();
    index_of_next_false_bit_properties_helper_signed::<i32>();
    index_of_next_false_bit_properties_helper_signed::<i64>();
    index_of_next_false_bit_properties_helper_signed::<isize>();
}

#[test]
fn index_of_next_true_bit_properties() {
    index_of_next_true_bit_properties_helper_unsigned::<u8>();
    index_of_next_true_bit_properties_helper_unsigned::<u16>();
    index_of_next_true_bit_properties_helper_unsigned::<u32>();
    index_of_next_true_bit_properties_helper_unsigned::<u64>();
    index_of_next_true_bit_properties_helper_unsigned::<usize>();
    index_of_next_true_bit_properties_helper_signed::<i8>();
    index_of_next_true_bit_properties_helper_signed::<i16>();
    index_of_next_true_bit_properties_helper_signed::<i32>();
    index_of_next_true_bit_properties_helper_signed::<i64>();
    index_of_next_true_bit_properties_helper_signed::<isize>();
}
