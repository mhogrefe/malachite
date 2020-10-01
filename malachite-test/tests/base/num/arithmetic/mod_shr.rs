use malachite_base::num::arithmetic::traits::{ArithmeticCheckedShr, ModShl, ModShr, ModShrAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::distributions::range::SampleRange;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_positive_unsigned, pairs_of_unsigneds_var_5,
    triples_of_unsigned_signed_and_unsigned_var_1,
};

fn mod_shr_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveSigned + Rand>()
where
    T: ArithmeticCheckedShr<U, Output = T>
        + ModShr<U, Output = T>
        + ModShrAssign<U>
        + ModShl<U, Output = T>
        + SampleRange,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_unsigned_signed_and_unsigned_var_1::<T, U>,
        |&(n, i, m)| {
            assert!(n.mod_is_reduced(&m));
            let shifted = n.mod_shr(i, m);
            assert!(shifted.mod_is_reduced(&m));

            let mut shifted_alt = n;
            shifted_alt.mod_shr_assign(i, m);
            assert_eq!(shifted_alt, shifted);

            if let Some(shifted_alt) = n.arithmetic_checked_shr(i) {
                assert_eq!(shifted_alt % m, shifted);
            }

            if i != U::MIN {
                assert_eq!(n.mod_shl(-i, m), shifted);
            }
        },
    );

    test_properties(pairs_of_unsigneds_var_5::<T>, |&(n, m)| {
        assert_eq!(n.mod_shr(U::ZERO, m), n);
    });

    test_properties(pairs_of_signed_and_positive_unsigned::<U, T>, |&(u, m)| {
        assert_eq!(T::ZERO.mod_shr(u, m), T::ZERO);
    });
}

#[test]
fn mod_shr_properties() {
    mod_shr_helper::<u8, i8>();
    mod_shr_helper::<u8, i16>();
    mod_shr_helper::<u8, i32>();
    mod_shr_helper::<u8, i64>();
    mod_shr_helper::<u8, isize>();
    mod_shr_helper::<u16, i8>();
    mod_shr_helper::<u16, i16>();
    mod_shr_helper::<u16, i32>();
    mod_shr_helper::<u16, i64>();
    mod_shr_helper::<u16, isize>();
    mod_shr_helper::<u32, i8>();
    mod_shr_helper::<u32, i16>();
    mod_shr_helper::<u32, i32>();
    mod_shr_helper::<u32, i64>();
    mod_shr_helper::<u32, isize>();
    mod_shr_helper::<u64, i8>();
    mod_shr_helper::<u64, i16>();
    mod_shr_helper::<u64, i32>();
    mod_shr_helper::<u64, i64>();
    mod_shr_helper::<u64, isize>();
    mod_shr_helper::<usize, i8>();
    mod_shr_helper::<usize, i16>();
    mod_shr_helper::<usize, i32>();
    mod_shr_helper::<usize, i64>();
    mod_shr_helper::<usize, isize>();
}
