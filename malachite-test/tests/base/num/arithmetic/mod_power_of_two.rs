use std::cmp::min;

use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, ModPowerOfTwoIsReduced};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_u64_var_2, pairs_of_signed_and_small_u64_var_3,
    pairs_of_signed_and_small_u64_var_4, pairs_of_signed_and_small_unsigned,
    pairs_of_unsigned_and_small_u64_var_4, pairs_of_unsigned_and_small_unsigned, signeds,
    triples_of_unsigned_small_unsigned_and_small_unsigned,
    triples_of_unsigned_unsigned_and_small_unsigned, unsigneds,
};

fn mod_power_of_two_properties_unsigned_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(
        pairs_of_unsigned_and_small_unsigned::<T, u64>,
        |&(n, pow)| {
            let mut mut_n = n;
            mut_n.mod_power_of_two_assign(pow);
            let result = mut_n;
            assert!(result.mod_power_of_two_is_reduced(pow));
            assert_eq!(n.mod_power_of_two(pow), result);

            let mut mut_n = n;
            mut_n.rem_power_of_two_assign(pow);
            assert_eq!(mut_n, result);
            assert_eq!(n.rem_power_of_two(pow), result);

            assert!(result <= n);
            assert_eq!(result == T::ZERO, n.divisible_by_power_of_two(pow));
            assert_eq!(result.mod_power_of_two(pow), result);
        },
    );

    test_properties(
        triples_of_unsigned_unsigned_and_small_unsigned::<T, u64>,
        |&(x, y, pow)| {
            assert_eq!(
                x.wrapping_add(y).mod_power_of_two(pow),
                x.mod_power_of_two(pow)
                    .wrapping_add(y.mod_power_of_two(pow))
                    .mod_power_of_two(pow)
            );
            assert_eq!(
                x.wrapping_mul(y).mod_power_of_two(pow),
                x.mod_power_of_two(pow)
                    .wrapping_mul(y.mod_power_of_two(pow))
                    .mod_power_of_two(pow)
            );
        },
    );

    test_properties(
        triples_of_unsigned_small_unsigned_and_small_unsigned::<T, u64>,
        |&(n, u, v)| {
            assert_eq!(
                n.mod_power_of_two(u).mod_power_of_two(v),
                n.mod_power_of_two(min(u, v))
            );
        },
    );

    test_properties(unsigneds::<T>, |n| {
        assert_eq!(n.mod_power_of_two(0), T::ZERO);
    });

    test_properties(unsigneds::<u64>, |&pow| {
        assert_eq!(T::ZERO.mod_power_of_two(pow), T::ZERO);
    });
}

fn mod_power_of_two_properties_signed_helper<T: PrimitiveSigned + Rand + SampleRange>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    <T as ModPowerOfTwo>::Output: ExactFrom<T> + PrimitiveUnsigned,
{
    test_properties(pairs_of_signed_and_small_u64_var_2::<T>, |&(n, pow)| {
        let result = n.mod_power_of_two(pow);
        assert!(result.mod_power_of_two_is_reduced(pow));
        assert_eq!(
            result == <T as ModPowerOfTwo>::Output::ZERO,
            n.divisible_by_power_of_two(pow)
        );
        assert_eq!(result.mod_power_of_two(pow), result);
    });

    test_properties(pairs_of_signed_and_small_u64_var_3::<T>, |&(n, pow)| {
        let mut mut_n = n;
        mut_n.mod_power_of_two_assign(pow);
        let result = mut_n;
        assert_eq!(
            n.mod_power_of_two(pow),
            <T as ModPowerOfTwo>::Output::exact_from(result)
        );

        assert!(result >= T::ZERO);
        assert_eq!(result == T::ZERO, n.divisible_by_power_of_two(pow));
        assert_eq!(
            result.mod_power_of_two(pow),
            <T as ModPowerOfTwo>::Output::exact_from(result)
        );
    });

    test_properties(signeds::<T>, |n| {
        assert_eq!(n.mod_power_of_two(0), <T as ModPowerOfTwo>::Output::ZERO);
    });

    test_properties(unsigneds::<u64>, |&pow| {
        assert_eq!(
            T::ZERO.mod_power_of_two(pow),
            <T as ModPowerOfTwo>::Output::ZERO
        );
    });
}

#[test]
fn mod_power_of_two_properties() {
    mod_power_of_two_properties_unsigned_helper::<u8>();
    mod_power_of_two_properties_unsigned_helper::<u16>();
    mod_power_of_two_properties_unsigned_helper::<u32>();
    mod_power_of_two_properties_unsigned_helper::<u64>();
    mod_power_of_two_properties_unsigned_helper::<usize>();
    mod_power_of_two_properties_signed_helper::<i8>();
    mod_power_of_two_properties_signed_helper::<i16>();
    mod_power_of_two_properties_signed_helper::<i32>();
    mod_power_of_two_properties_signed_helper::<i64>();
    mod_power_of_two_properties_signed_helper::<isize>();
}

fn rem_power_of_two_properties_signed_helper<T: PrimitiveSigned + Rand + SampleRange>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_small_unsigned::<T, u64>, |&(n, pow)| {
        let mut mut_n = n;
        mut_n.rem_power_of_two_assign(pow);
        let result = mut_n;
        assert_eq!(n.rem_power_of_two(pow), result);

        if n != T::MIN {
            assert_eq!((-n).rem_power_of_two(pow), -result);
        }
        assert!(result.le_abs(&n));
        assert_eq!(result == T::ZERO, n.divisible_by_power_of_two(pow));
        assert_eq!(result.rem_power_of_two(pow), result);
        assert!(result == T::ZERO || (result > T::ZERO) == (n > T::ZERO));
    });

    test_properties(signeds::<T>, |n| {
        assert_eq!(n.rem_power_of_two(0), T::ZERO);
    });

    test_properties(unsigneds::<u64>, |&pow| {
        assert_eq!(T::ZERO.rem_power_of_two(pow), T::ZERO);
    });
}

#[test]
fn rem_power_of_two_properties() {
    rem_power_of_two_properties_signed_helper::<i8>();
    rem_power_of_two_properties_signed_helper::<i16>();
    rem_power_of_two_properties_signed_helper::<i32>();
    rem_power_of_two_properties_signed_helper::<i64>();
    rem_power_of_two_properties_signed_helper::<isize>();
}

fn neg_mod_power_of_two_properties_helper<T: PrimitiveUnsigned + Rand + SampleRange>() {
    test_properties(pairs_of_unsigned_and_small_u64_var_4::<T>, |&(n, pow)| {
        let mut mut_n = n;
        mut_n.neg_mod_power_of_two_assign(pow);
        let result = mut_n;
        assert!(result.mod_power_of_two_is_reduced(pow));
        assert_eq!(n.neg_mod_power_of_two(pow), result);

        assert_eq!(result == T::ZERO, n.divisible_by_power_of_two(pow));
        assert!(result
            .wrapping_add(n.mod_power_of_two(pow))
            .divisible_by_power_of_two(pow));
        assert_eq!(result.neg_mod_power_of_two(pow), n.mod_power_of_two(pow));
    });

    test_properties(unsigneds::<T>, |n| {
        assert_eq!(n.neg_mod_power_of_two(0), T::ZERO);
    });

    test_properties(unsigneds::<u64>, |&pow| {
        assert_eq!(T::ZERO.neg_mod_power_of_two(pow), T::ZERO);
    });
}

#[test]
fn neg_mod_power_of_two_properties() {
    neg_mod_power_of_two_properties_helper::<u8>();
    neg_mod_power_of_two_properties_helper::<u16>();
    neg_mod_power_of_two_properties_helper::<u32>();
    neg_mod_power_of_two_properties_helper::<u64>();
    neg_mod_power_of_two_properties_helper::<usize>();
}

fn ceiling_mod_power_of_two_properties_helper<T: PrimitiveSigned + Rand + SampleRange>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_signed_and_small_u64_var_4::<T>, |&(n, pow)| {
        let mut mut_n = n;
        mut_n.ceiling_mod_power_of_two_assign(pow);
        let result = mut_n;
        assert_eq!(n.ceiling_mod_power_of_two(pow), result);

        assert!(result <= T::ZERO);
        assert_eq!(result == T::ZERO, n.divisible_by_power_of_two(pow));
    });

    test_properties(signeds::<T>, |n| {
        assert_eq!(n.ceiling_mod_power_of_two(0), T::ZERO);
    });

    test_properties(unsigneds::<u64>, |&pow| {
        assert_eq!(T::ZERO.ceiling_mod_power_of_two(pow), T::ZERO);
    });
}

#[test]
fn ceiling_mod_power_of_two_properties() {
    ceiling_mod_power_of_two_properties_helper::<i8>();
    ceiling_mod_power_of_two_properties_helper::<i16>();
    ceiling_mod_power_of_two_properties_helper::<i32>();
    ceiling_mod_power_of_two_properties_helper::<i64>();
    ceiling_mod_power_of_two_properties_helper::<isize>();
}
