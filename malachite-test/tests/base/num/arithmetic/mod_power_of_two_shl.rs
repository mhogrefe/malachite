use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, ModPowerOfTwoShl, ModPowerOfTwoShlAssign, ModPowerOfTwoShr,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_u64_var_1, pairs_of_unsigned_and_small_u64_var_2,
    pairs_of_unsigned_and_small_u64_var_3, small_signeds, small_unsigneds,
    triples_of_unsigned_small_signed_and_small_unsigned_var_1,
    triples_of_unsigned_small_unsigned_and_small_unsigned_var_3,
};

fn mod_power_of_two_shl_unsigned_unsigned_helper<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveUnsigned + Rand,
>()
where
    T: ArithmeticCheckedShl<U, Output = T>
        + ModPowerOfTwoShl<U, Output = T>
        + ModPowerOfTwoShlAssign<U>
        + SampleRange,
{
    test_properties_no_special(
        triples_of_unsigned_small_unsigned_and_small_unsigned_var_3::<T, U>,
        |&(n, u, pow)| {
            assert!(n.mod_power_of_two_is_reduced(pow));
            let shifted = n.mod_power_of_two_shl(u, pow);
            assert!(shifted.mod_power_of_two_is_reduced(pow));

            let mut shifted_alt = n;
            shifted_alt.mod_power_of_two_shl_assign(u, pow);
            assert_eq!(shifted_alt, shifted);

            if let Some(shifted_alt) = n.arithmetic_checked_shl(u) {
                assert_eq!(shifted_alt.mod_power_of_two(pow), shifted);
            }
        },
    );

    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(n, pow)| {
        assert_eq!(
            n.mod_power_of_two_shl(U::ZERO, pow),
            n.mod_power_of_two(pow)
        );
    });

    test_properties(
        pairs_of_unsigned_and_small_u64_var_3::<U, T>,
        |&(u, pow)| {
            assert_eq!(T::ZERO.mod_power_of_two_shl(u, pow), T::ZERO);
        },
    );

    test_properties_no_special(small_unsigneds::<U>, |&u| {
        assert_eq!(T::ZERO.mod_power_of_two_shl(u, 0), T::ZERO);
    });
}

fn mod_power_of_two_shl_unsigned_signed_helper<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveSigned + Rand,
>()
where
    T: ArithmeticCheckedShl<U, Output = T>
        + ModPowerOfTwoShl<U, Output = T>
        + ModPowerOfTwoShlAssign<U>
        + ModPowerOfTwoShr<U, Output = T>
        + SampleRange,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties_no_special(
        triples_of_unsigned_small_signed_and_small_unsigned_var_1::<T, U>,
        |&(n, i, pow)| {
            assert!(n.mod_power_of_two_is_reduced(pow));
            let shifted = n.mod_power_of_two_shl(i, pow);
            assert!(shifted.mod_power_of_two_is_reduced(pow));

            let mut shifted_alt = n;
            shifted_alt.mod_power_of_two_shl_assign(i, pow);
            assert_eq!(shifted_alt, shifted);

            if let Some(shifted_alt) = n.arithmetic_checked_shl(i) {
                assert_eq!(shifted_alt.mod_power_of_two(pow), shifted);
            }

            if i != U::MIN {
                assert_eq!(n.mod_power_of_two_shr(-i, pow), shifted);
            }
        },
    );

    test_properties(
        pairs_of_unsigned_and_small_u64_var_3::<T, T>,
        |&(n, pow)| {
            assert_eq!(
                n.mod_power_of_two_shl(U::ZERO, pow),
                n.mod_power_of_two(pow)
            );
        },
    );

    test_properties(pairs_of_signed_and_small_u64_var_1::<U, T>, |&(u, pow)| {
        assert_eq!(T::ZERO.mod_power_of_two_shl(u, pow), T::ZERO);
    });

    test_properties_no_special(small_signeds::<U>, |&u| {
        assert_eq!(T::ZERO.mod_power_of_two_shl(u, 0), T::ZERO);
    });
}

#[test]
fn mod_power_of_two_shl_properties() {
    mod_power_of_two_shl_unsigned_unsigned_helper::<u8, u8>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u8, u16>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u8, u32>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u8, u64>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u8, usize>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u16, u8>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u16, u16>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u16, u32>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u16, u64>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u16, usize>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u32, u8>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u32, u16>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u32, u32>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u32, u64>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u32, usize>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u64, u8>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u64, u16>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u64, u32>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u64, u64>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<u64, usize>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<usize, u8>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<usize, u16>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<usize, u32>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<usize, u64>();
    mod_power_of_two_shl_unsigned_unsigned_helper::<usize, usize>();

    mod_power_of_two_shl_unsigned_signed_helper::<u8, i8>();
    mod_power_of_two_shl_unsigned_signed_helper::<u8, i16>();
    mod_power_of_two_shl_unsigned_signed_helper::<u8, i32>();
    mod_power_of_two_shl_unsigned_signed_helper::<u8, i64>();
    mod_power_of_two_shl_unsigned_signed_helper::<u8, isize>();
    mod_power_of_two_shl_unsigned_signed_helper::<u16, i8>();
    mod_power_of_two_shl_unsigned_signed_helper::<u16, i16>();
    mod_power_of_two_shl_unsigned_signed_helper::<u16, i32>();
    mod_power_of_two_shl_unsigned_signed_helper::<u16, i64>();
    mod_power_of_two_shl_unsigned_signed_helper::<u16, isize>();
    mod_power_of_two_shl_unsigned_signed_helper::<u32, i8>();
    mod_power_of_two_shl_unsigned_signed_helper::<u32, i16>();
    mod_power_of_two_shl_unsigned_signed_helper::<u32, i32>();
    mod_power_of_two_shl_unsigned_signed_helper::<u32, i64>();
    mod_power_of_two_shl_unsigned_signed_helper::<u32, isize>();
    mod_power_of_two_shl_unsigned_signed_helper::<u64, i8>();
    mod_power_of_two_shl_unsigned_signed_helper::<u64, i16>();
    mod_power_of_two_shl_unsigned_signed_helper::<u64, i32>();
    mod_power_of_two_shl_unsigned_signed_helper::<u64, i64>();
    mod_power_of_two_shl_unsigned_signed_helper::<u64, isize>();
    mod_power_of_two_shl_unsigned_signed_helper::<usize, i8>();
    mod_power_of_two_shl_unsigned_signed_helper::<usize, i16>();
    mod_power_of_two_shl_unsigned_signed_helper::<usize, i32>();
    mod_power_of_two_shl_unsigned_signed_helper::<usize, i64>();
    mod_power_of_two_shl_unsigned_signed_helper::<usize, isize>();
}
