use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShl, ModShl, ModShlAssign, ModShr};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{
    pairs_of_signed_and_positive_unsigned, pairs_of_unsigned_and_positive_unsigned,
    pairs_of_unsigneds_var_5, small_signeds, small_unsigneds,
    triples_of_unsigned_signed_and_unsigned_var_1, triples_of_unsigned_unsigned_and_unsigned_var_1,
};

fn mod_shl_unsigned_unsigned_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveUnsigned + Rand>()
where
    T: ArithmeticCheckedShl<U, Output = T>
        + ModShl<U, T, Output = T>
        + ModShlAssign<U, T>
        + SampleRange,
{
    test_properties(
        triples_of_unsigned_unsigned_and_unsigned_var_1::<T, U>,
        |&(n, u, m)| {
            assert!(n.mod_is_reduced(&m));
            let shifted = n.mod_shl(u, m);
            assert!(shifted.mod_is_reduced(&m));

            let mut shifted_alt = n;
            shifted_alt.mod_shl_assign(u, m);
            assert_eq!(shifted_alt, shifted);

            if let Some(shifted_alt) = n.arithmetic_checked_shl(u) {
                assert_eq!(shifted_alt % m, shifted);
            }
        },
    );

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(n, m)| {
        assert_eq!(n.mod_shl(U::ZERO, m), n);
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<U, T>,
        |&(u, m)| {
            assert_eq!(T::ZERO.mod_shl(u, m), T::ZERO);
        },
    );

    test_properties_no_special(small_unsigneds::<U>, |&u| {
        assert_eq!(T::ZERO.mod_shl(u, T::ONE), T::ZERO);
    });
}

fn mod_shl_unsigned_signed_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveSigned + Rand>()
where
    T: ArithmeticCheckedShl<U, Output = T>
        + ModShl<U, Output = T>
        + ModShlAssign<U>
        + ModShr<U, Output = T>
        + SampleRange,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_unsigned_signed_and_unsigned_var_1::<T, U>,
        |&(n, i, m)| {
            assert!(n.mod_is_reduced(&m));
            let shifted = n.mod_shl(i, m);
            assert!(shifted.mod_is_reduced(&m));

            let mut shifted_alt = n;
            shifted_alt.mod_shl_assign(i, m);
            assert_eq!(shifted_alt, shifted);

            if let Some(shifted_alt) = n.arithmetic_checked_shl(i) {
                assert_eq!(shifted_alt % m, shifted);
            }

            if i != U::MIN {
                assert_eq!(n.mod_shr(-i, m), shifted);
            }
        },
    );

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(n, m)| {
        assert_eq!(n.mod_shl(U::ZERO, m), n);
    });

    test_properties(pairs_of_signed_and_positive_unsigned::<U, T>, |&(u, m)| {
        assert_eq!(T::ZERO.mod_shl(u, m), T::ZERO);
    });

    test_properties_no_special(small_signeds::<U>, |&i| {
        assert_eq!(T::ZERO.mod_shl(i, T::ONE), T::ZERO);
    });
}

#[test]
fn mod_shl_properties() {
    mod_shl_unsigned_unsigned_helper::<u8, u8>();
    mod_shl_unsigned_unsigned_helper::<u8, u16>();
    mod_shl_unsigned_unsigned_helper::<u8, u32>();
    mod_shl_unsigned_unsigned_helper::<u8, u64>();
    mod_shl_unsigned_unsigned_helper::<u8, usize>();
    mod_shl_unsigned_unsigned_helper::<u16, u8>();
    mod_shl_unsigned_unsigned_helper::<u16, u16>();
    mod_shl_unsigned_unsigned_helper::<u16, u32>();
    mod_shl_unsigned_unsigned_helper::<u16, u64>();
    mod_shl_unsigned_unsigned_helper::<u16, usize>();
    mod_shl_unsigned_unsigned_helper::<u32, u8>();
    mod_shl_unsigned_unsigned_helper::<u32, u16>();
    mod_shl_unsigned_unsigned_helper::<u32, u32>();
    mod_shl_unsigned_unsigned_helper::<u32, u64>();
    mod_shl_unsigned_unsigned_helper::<u32, usize>();
    mod_shl_unsigned_unsigned_helper::<u64, u8>();
    mod_shl_unsigned_unsigned_helper::<u64, u16>();
    mod_shl_unsigned_unsigned_helper::<u64, u32>();
    mod_shl_unsigned_unsigned_helper::<u64, u64>();
    mod_shl_unsigned_unsigned_helper::<u64, usize>();
    mod_shl_unsigned_unsigned_helper::<usize, u8>();
    mod_shl_unsigned_unsigned_helper::<usize, u16>();
    mod_shl_unsigned_unsigned_helper::<usize, u32>();
    mod_shl_unsigned_unsigned_helper::<usize, u64>();
    mod_shl_unsigned_unsigned_helper::<usize, usize>();

    mod_shl_unsigned_signed_helper::<u8, i8>();
    mod_shl_unsigned_signed_helper::<u8, i16>();
    mod_shl_unsigned_signed_helper::<u8, i32>();
    mod_shl_unsigned_signed_helper::<u8, i64>();
    mod_shl_unsigned_signed_helper::<u8, isize>();
    mod_shl_unsigned_signed_helper::<u16, i8>();
    mod_shl_unsigned_signed_helper::<u16, i16>();
    mod_shl_unsigned_signed_helper::<u16, i32>();
    mod_shl_unsigned_signed_helper::<u16, i64>();
    mod_shl_unsigned_signed_helper::<u16, isize>();
    mod_shl_unsigned_signed_helper::<u32, i8>();
    mod_shl_unsigned_signed_helper::<u32, i16>();
    mod_shl_unsigned_signed_helper::<u32, i32>();
    mod_shl_unsigned_signed_helper::<u32, i64>();
    mod_shl_unsigned_signed_helper::<u32, isize>();
    mod_shl_unsigned_signed_helper::<u64, i8>();
    mod_shl_unsigned_signed_helper::<u64, i16>();
    mod_shl_unsigned_signed_helper::<u64, i32>();
    mod_shl_unsigned_signed_helper::<u64, i64>();
    mod_shl_unsigned_signed_helper::<u64, isize>();
    mod_shl_unsigned_signed_helper::<usize, i8>();
    mod_shl_unsigned_signed_helper::<usize, i16>();
    mod_shl_unsigned_signed_helper::<usize, i32>();
    mod_shl_unsigned_signed_helper::<usize, i64>();
    mod_shl_unsigned_signed_helper::<usize, isize>();
}
