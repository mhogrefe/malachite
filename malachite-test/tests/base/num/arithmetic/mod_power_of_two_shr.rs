use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, ModPowerOfTwoShl, ModPowerOfTwoShr, ModPowerOfTwoShrAssign,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_u64_var_1, pairs_of_unsigned_and_small_u64_var_2, small_signeds,
    triples_of_unsigned_small_signed_and_small_unsigned_var_1,
};

fn mod_power_of_two_shr_unsigned_signed_helper<
    T: PrimitiveUnsigned + Rand,
    U: PrimitiveSigned + Rand,
>()
where
    T: ArithmeticCheckedShr<U, Output = T>
        + ModPowerOfTwoShr<U, Output = T>
        + ModPowerOfTwoShrAssign<U>
        + ModPowerOfTwoShl<U, Output = T>
        + SampleRange,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties_no_special(
        triples_of_unsigned_small_signed_and_small_unsigned_var_1::<T, U>,
        |&(n, i, pow)| {
            assert!(n.mod_power_of_two_is_reduced(pow));
            let shifted = n.mod_power_of_two_shr(i, pow);
            assert!(shifted.mod_power_of_two_is_reduced(pow));

            let mut shifted_alt = n;
            shifted_alt.mod_power_of_two_shr_assign(i, pow);
            assert_eq!(shifted_alt, shifted);

            if let Some(shifted_alt) = n.arithmetic_checked_shr(i) {
                assert_eq!(shifted_alt.mod_power_of_two(pow), shifted);
            }

            if i != U::MIN {
                assert_eq!(n.mod_power_of_two_shl(-i, pow), shifted);
            }
        },
    );

    test_properties_no_special(pairs_of_unsigned_and_small_u64_var_2::<T>, |&(n, pow)| {
        assert_eq!(
            n.mod_power_of_two_shr(U::ZERO, pow),
            n.mod_power_of_two(pow)
        );
    });

    test_properties(pairs_of_signed_and_small_u64_var_1::<U, T>, |&(u, pow)| {
        assert_eq!(T::ZERO.mod_power_of_two_shr(u, pow), T::ZERO);
    });

    test_properties_no_special(small_signeds::<U>, |&u| {
        assert_eq!(T::ZERO.mod_power_of_two_shr(u, 0), T::ZERO);
    });
}

#[test]
fn mod_power_of_two_shr_properties() {
    mod_power_of_two_shr_unsigned_signed_helper::<u8, i8>();
    mod_power_of_two_shr_unsigned_signed_helper::<u8, i16>();
    mod_power_of_two_shr_unsigned_signed_helper::<u8, i32>();
    mod_power_of_two_shr_unsigned_signed_helper::<u8, i64>();
    mod_power_of_two_shr_unsigned_signed_helper::<u8, isize>();
    mod_power_of_two_shr_unsigned_signed_helper::<u16, i8>();
    mod_power_of_two_shr_unsigned_signed_helper::<u16, i16>();
    mod_power_of_two_shr_unsigned_signed_helper::<u16, i32>();
    mod_power_of_two_shr_unsigned_signed_helper::<u16, i64>();
    mod_power_of_two_shr_unsigned_signed_helper::<u16, isize>();
    mod_power_of_two_shr_unsigned_signed_helper::<u32, i8>();
    mod_power_of_two_shr_unsigned_signed_helper::<u32, i16>();
    mod_power_of_two_shr_unsigned_signed_helper::<u32, i32>();
    mod_power_of_two_shr_unsigned_signed_helper::<u32, i64>();
    mod_power_of_two_shr_unsigned_signed_helper::<u32, isize>();
    mod_power_of_two_shr_unsigned_signed_helper::<u64, i8>();
    mod_power_of_two_shr_unsigned_signed_helper::<u64, i16>();
    mod_power_of_two_shr_unsigned_signed_helper::<u64, i32>();
    mod_power_of_two_shr_unsigned_signed_helper::<u64, i64>();
    mod_power_of_two_shr_unsigned_signed_helper::<u64, isize>();
    mod_power_of_two_shr_unsigned_signed_helper::<usize, i8>();
    mod_power_of_two_shr_unsigned_signed_helper::<usize, i16>();
    mod_power_of_two_shr_unsigned_signed_helper::<usize, i32>();
    mod_power_of_two_shr_unsigned_signed_helper::<usize, i64>();
    mod_power_of_two_shr_unsigned_signed_helper::<usize, isize>();
}
